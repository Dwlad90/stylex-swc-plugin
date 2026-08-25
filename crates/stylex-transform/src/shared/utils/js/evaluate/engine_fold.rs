//! Folds a self-contained method call by evaluating it, rather than by
//! matching its name against a table of methods.
//!
//! A table is finite by construction, so the method it does not list is the
//! next bug report. Handing the call to a JavaScript engine instead covers
//! `String.prototype`, `Array.prototype` and `Object.prototype` at once, and
//! covers a chain for free: the receiver of a call is itself a candidate, so
//! `["1px","solid"].concat(["red"]).join(" ")` is printed and evaluated once
//! instead of being split across two tables that have to agree.
//!
//! Everything therefore rests on the guard in front of the engine. What it
//! lets through is answered by the language; what it holds back is a boundary
//! with a reason, and each one is named where it is applied below.

use std::{cell::RefCell, mem::ManuallyDrop};

use boa_engine::{Context, JsValue, Source, js_string};
use swc_core::{
  atoms::Atom,
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt,
      KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleItem, Null, ObjectLit, Pat, Prop,
      PropName, PropOrSpread, Stmt,
    },
    codegen::Config,
  },
};

use stylex_js::helpers::is_mutating_array_method;

use crate::shared::utils::log::build_code_frame_error::{CodeFrame, print_module};

/// Methods whose answer depends on locale data the engine does not carry.
///
/// It resolves them against the root locale, so `"i".toLocaleUpperCase("tr")`
/// comes back `I` where the language says `İ`, and a collation comparison
/// answers by code point. Folding those would put a wrong value in the
/// stylesheet, which is worse than not folding at all, so they are refused and
/// the caller's existing refusal reports them.
///
/// `toLocaleString` is here for a receiver the name cannot be separated from.
/// On an object it delegates to `toString` and carries no locale at all, and
/// refusing it costs one of the object methods that would otherwise fold. On a
/// number it formats — `(1234.5).toLocaleString("de-DE")` is `1.234,5` in the
/// language and `1234.5` here — and which receiver a chain will produce is not
/// knowable before evaluating it. One name cannot be both admitted and refused,
/// so it is refused, and the object case is the price.
const LOCALE_SENSITIVE_METHODS: [&str; 4] = [
  "localeCompare",
  "toLocaleLowerCase",
  "toLocaleUpperCase",
  "toLocaleString",
];

/// Property names that walk off the value that was written and onto the
/// language's function graph.
///
/// `constructor` on a literal is `String`, whose own `constructor` is
/// `Function`, which compiles a string into a function. So
/// `"".constructor.constructor("return Date.now()").call()` is a chain every
/// other rule here admits — two named property reads on a literal, then a call
/// — whose answer is a different number on every build, and whose body can
/// assign to `String.prototype` in an engine every later fold shares. `call`,
/// `apply` and `bind` are what turn an unapplied function back into a call, so
/// they are refused with it.
const ESCAPING_PROPERTIES: [&str; 4] = ["constructor", "call", "apply", "bind"];

/// Methods whose result length is set by an argument rather than by the
/// receiver, and so are the only ones a single small argument can blow up.
const LENGTH_AMPLIFYING_METHODS: [&str; 3] = ["repeat", "padStart", "padEnd"];

/// How long a string one of those may be asked to build.
///
/// The engine bounds recursion and VM stack depth by default, and this module
/// bounds loop iterations itself ([`MAX_LOOP_ITERATIONS`]), because Boa's own
/// default for that is `u64::MAX`. What none of them bound is allocation:
/// growth inside a native builtin is not a counted loop. So
/// `"x".repeat(200000000)` is a typo that folds — agreeing with the language —
/// at several gigabytes of resident memory, and a compiler that dies there is
/// worse than one that declines the fold. A million UTF-16 code units is two
/// megabytes in the engine and instant, and no CSS value is a megabyte long.
const MAX_AMPLIFIED_LENGTH: f64 = 1_000_000.0;

/// How many elements a folded array may carry back.
///
/// [`MAX_AMPLIFIED_LENGTH`] bounds a string, and a string can still be turned
/// into one element per code unit: `"x".repeat(999999).split("")` folds to an
/// array literal of a million AST nodes, which costs far more as a tree than it
/// did as text. A fallback list in a real declaration holds a handful of values,
/// so this is generous by three orders of magnitude and still refuses that.
const MAX_FOLDED_ARRAY_LENGTH: u64 = 10_000;

/// How deeply nested an expression this module will hand to the engine.
///
/// Nesting is not free for whoever parses it. The engine's parser descends
/// through a nested array literal recursively and, measured on a debug build,
/// overflows its stack somewhere around a hundred levels — which aborts the
/// process from inside an evaluation whose whole contract is that it may fail.
/// Refusing deeper input is what turns that crash back into a diagnostic.
///
/// One level is spent per step of the walk, and a leaf spends one too, so this
/// admits an expression nested 31 levels and refuses the 32nd. The number is
/// deliberately the same as
/// `stylex_structures::evaluation_depth::DEFAULT_MAX_EVALUATION_DEPTH`, for the
/// same reason and with a wide margin over the measured limit rather than an
/// exact match to it. It is *not* that setting: raising the ceiling on the
/// evaluator's own recursion says nothing about the engine's stack, and a
/// project that raised it must not lose the diagnostic here.
const MAX_ENGINE_NESTING: usize = 32;

/// How many loop iterations an evaluation may run.
///
/// Boa's own default is `u64::MAX` — `RuntimeLimits::loop_iteration` documents
/// it as no limit — so a loop is unbounded until it is said to be bounded. The
/// guard refuses the shapes that reach a loop body, and this is the second
/// answer behind that one: a bound the engine enforces whatever the guard let
/// through. Ten million iterations of an empty loop is well under a second, and
/// no folded CSS value is reached by counting that far.
const MAX_LOOP_ITERATIONS: u64 = 10_000_000;

/// Whether `name` is one of the names in `list`.
///
/// The lists above are three to four entries, so a scan beats any structure
/// that would have to hash first, and one function keeps the four call sites
/// from each spelling the double reference their own way.
fn lists(list: &[&str], name: &Atom) -> bool {
  list.contains(&&**name)
}

thread_local! {
  /// One engine per thread, created on the first fold that needs it and reused
  /// for every later one. A file with no foldable method call never builds it.
  ///
  /// Reuse is what makes the guard load-bearing rather than merely tidy: a fold
  /// that reached a prototype would be read by every later fold in the build,
  /// including one in another file, so the boundaries below are what keeps one
  /// engine safe to share. Reuse also costs: the engine interns each distinct
  /// source it is handed and never reclaims it, measured at roughly half a
  /// kilobyte per distinct folded call site, which a real corpus keeps in the
  /// low megabytes for the life of the process.
  ///
  /// `ManuallyDrop` is not a convenience: the engine's garbage collector lives
  /// in a thread-local of its own, and the order two thread-locals are dropped
  /// in is not defined. Dropping this one after the collector's underflows a
  /// reference count, and that panic runs inside a destructor, which aborts the
  /// process instead of unwinding. Leaking one engine per thread at exit is the
  /// price of not aborting.
  static ENGINE: RefCell<Option<ManuallyDrop<Context>>> = const { RefCell::new(None) };
}

/// Where a bare identifier in an expression is allowed to get its value from.
#[derive(Clone, Copy)]
enum Scope<'a> {
  /// Nowhere. The expression has to carry its whole value with it, because the
  /// engine is handed the expression alone and knows nothing of the module.
  Nothing,
  /// A callback's own parameters, which the engine binds itself when it invokes
  /// the callback with values from the receiver.
  Params(&'a [Atom]),
}

/// What the guard carries as it walks: where a bare identifier may come from,
/// and how much nesting is left before the expression is refused as too deep.
#[derive(Clone, Copy)]
struct Guard<'a> {
  scope: Scope<'a>,
  levels_left: usize,
}

impl<'a> Guard<'a> {
  /// The guard one level in, or `None` at the bound — which every walk reads as
  /// a refusal, so depth is answered the same way as any other shape it
  /// declines.
  fn descend(self) -> Option<Self> {
    match self.levels_left {
      0 => None,
      levels_left => Some(Self {
        levels_left: levels_left - 1,
        ..self
      }),
    }
  }

  /// The same remaining depth, with a callback's parameters now in scope.
  fn binding(self, params: &'a [Atom]) -> Self {
    Self {
      scope: Scope::Params(params),
      ..self
    }
  }
}

/// Folds `call` through the engine, or answers `None` to leave the existing
/// path in charge.
pub(crate) fn try_fold(call: &CallExpr) -> Option<Expr> {
  // Only a method call on a self-contained receiver is a candidate. Anything
  // else would need the surrounding scope, which the engine does not have.
  let guard = Guard {
    scope: Scope::Nothing,
    levels_left: MAX_ENGINE_NESTING,
  };

  if !is_foldable_call(call, guard) {
    return None;
  }

  let source = print_call(call);

  ENGINE.with_borrow_mut(|slot| {
    // Taken, not borrowed in place. A panic unwinding out of the engine is
    // caught at the NAPI boundary and the process carries on, so an engine left
    // in the slot would be reused by every later fold with its VM stack
    // abandoned mid-frame. Taking it means an unwind leaves the slot empty and
    // the next fold builds a fresh engine; the abandoned one leaks, which is
    // what `ManuallyDrop` already makes it do at thread exit.
    let mut engine = slot.take().unwrap_or_else(new_engine);

    let folded = match engine.eval(Source::from_bytes(&source)) {
      Ok(value) => to_expr(&value, &mut engine),
      // A throw is an answer, not a failure of this module: the caller's
      // existing refusal already reports it. Nothing to fold.
      Err(_) => None,
    };

    *slot = Some(engine);

    folded
  })
}

/// A context with the one runtime limit its default leaves open.
fn new_engine() -> ManuallyDrop<Context> {
  let mut engine = Context::default();

  engine
    .runtime_limits_mut()
    .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

  ManuallyDrop::new(engine)
}

/// Whether every value `expr` needs is either written into it or bound by the
/// guard's scope, so printing it yields source the engine can evaluate alone.
///
/// One walk serves both questions this module asks — a receiver that must be
/// wholly self-contained is the same walk with an empty scope — so a shape
/// accepted in one position cannot silently be refused in the other.
/// Deliberately narrow: a shape it does not recognise is refused.
fn carries_its_own_value(expr: &Expr, guard: Guard) -> bool {
  let Some(inner) = guard.descend() else {
    return false;
  };

  match expr {
    Expr::Ident(ident) => match guard.scope {
      Scope::Nothing => false,
      Scope::Params(params) => params.contains(&ident.sym),
    },
    // A regular expression and a BigInt have no value this evaluator carries,
    // and neither does the reference implementation fold one.
    Expr::Lit(lit) => !matches!(lit, Lit::Regex(_) | Lit::BigInt(_)),
    Expr::Paren(paren) => carries_its_own_value(&paren.expr, inner),
    Expr::Unary(unary) => carries_its_own_value(&unary.arg, inner),
    Expr::Bin(bin) => {
      carries_its_own_value(&bin.left, inner) && carries_its_own_value(&bin.right, inner)
    },
    Expr::Cond(cond) => {
      carries_its_own_value(&cond.test, inner)
        && carries_its_own_value(&cond.cons, inner)
        && carries_its_own_value(&cond.alt, inner)
    },
    // A named property read: `x.length` inside a callback, and `({a:1}).a` as
    // a receiver. A computed one is a lookup that needs the scope.
    Expr::Member(MemberExpr { obj, prop, .. }) => match prop {
      MemberProp::Ident(name) if !lists(&ESCAPING_PROPERTIES, &name.sym) => {
        carries_its_own_value(obj, inner)
      },
      _ => false,
    },
    Expr::Array(ArrayLit { elems, .. }) => elems.iter().all(|elem| match elem {
      Some(ExprOrSpread { spread: None, expr }) => carries_its_own_value(expr, inner),
      // A hole is `undefined` and a spread needs the scope; both stay out.
      _ => false,
    }),
    // A key and a value written out, which is a value the engine can be handed
    // whole. `__proto__` is the one key that is not: written as a plain
    // property it sets the prototype rather than a member, so the receiver the
    // engine sees is not the object the source appears to describe. It is left
    // in because the reference implementation folds it identically and every
    // route off the prototype is refused above — not because the walk models
    // it.
    Expr::Object(ObjectLit { props, .. }) => props.iter().all(|prop| match prop {
      PropOrSpread::Prop(prop) => match prop.as_ref() {
        Prop::KeyValue(KeyValueProp { key, value }) => {
          matches!(
            key,
            PropName::Ident(_) | PropName::Str(_) | PropName::Num(_)
          ) && carries_its_own_value(value, inner)
        },
        _ => false,
      },
      PropOrSpread::Spread(_) => false,
    }),
    // A chained call: the receiver is itself a fold. Printing the whole chain
    // and evaluating it once is what makes `[…].map(…).join('-')` work, which
    // two separate method tables cannot agree on.
    Expr::Call(call) => is_foldable_call(call, inner),
    _ => false,
  }
}

/// Whether a call is a method call this module can hand to the engine whole.
///
/// Every boundary is checked here rather than at the outermost call, because a
/// chain hides its middle links: `["b","a"].sort().join("-")` is a `join` whose
/// receiver mutates.
fn is_foldable_call(call: &CallExpr, guard: Guard) -> bool {
  let Callee::Expr(callee) = &call.callee else {
    return false;
  };

  let Expr::Member(MemberExpr { obj, prop, .. }) = callee.as_ref() else {
    return false;
  };

  // A computed method name is a lookup the guard cannot resolve, even when it
  // is written as a literal.
  let MemberProp::Ident(method) = prop else {
    return false;
  };

  // A mutating method is refused rather than folded. The reference
  // implementation folds `["a","b"].push("c")` to `3` by reflecting on a real
  // array, and an engine reproduces that exactly — so the refusal has to be
  // stated. Matching it would mean carrying mutation into an evaluator whose
  // every other answer is pure, to serve input nobody writes.
  if is_mutating_array_method(prop) {
    return false;
  }

  if lists(&ESCAPING_PROPERTIES, &method.sym) {
    return false;
  }

  if lists(&LOCALE_SENSITIVE_METHODS, &method.sym) {
    return false;
  }

  if receiver_is_a_written_number(obj) {
    return false;
  }

  if !amplification_is_bounded(&method.sym, obj, &call.args, guard.scope) {
    return false;
  }

  carries_its_own_value(obj, guard) && call.args.iter().all(|arg| is_argument_foldable(arg, guard))
}

/// Whether the receiver is a number written into the source as a literal.
///
/// Every `Number.prototype` method throws upstream on one of those — the
/// reference implementation applies the method without a receiver, so
/// `(5).toFixed(2)` reports that `toFixed` requires a Number. Refusing keeps
/// both compilers rejecting the same input. A number that a fold *produced* is
/// a different shape and folds in both, as does a negated literal, which is a
/// unary expression rather than a literal.
fn receiver_is_a_written_number(receiver: &Expr) -> bool {
  match receiver {
    Expr::Paren(paren) => receiver_is_a_written_number(&paren.expr),
    Expr::Lit(lit) => matches!(lit, Lit::Num(_)),
    _ => false,
  }
}

/// Whether a length-amplifying call is bounded well enough to evaluate.
///
/// The length asked for has to be written into the source as a number under
/// [`MAX_AMPLIFIED_LENGTH`], and the receiver must not itself be a call:
/// per-link bounds alone would let `"x".repeat(1000000).repeat(1000000)`
/// multiply two allowed lengths into one that is not. With both in place the
/// most a fold can build is one bounded string per amplifying call written, so
/// the source file bounds the total.
fn amplification_is_bounded(
  method: &Atom,
  receiver: &Expr,
  args: &[ExprOrSpread],
  scope: Scope,
) -> bool {
  if !lists(&LENGTH_AMPLIFYING_METHODS, method) {
    return true;
  }

  // A written bound bounds one evaluation. A callback body is evaluated once
  // per element of a receiver this guard never measured, so the same bound is
  // multiplied by a count the source never states: `"x".repeat(999999)
  // .split("").map(() => "y".repeat(999999))` is two calls, each inside the
  // bound, building a terabyte between them. The count cannot be read here, so
  // an amplifying call inside a callback is refused whatever its argument says.
  if matches!(scope, Scope::Params(_)) {
    return false;
  }

  if matches!(receiver, Expr::Call(_)) {
    return false;
  }

  match args.first() {
    // `"x".padStart()` amplifies nothing, so there is no length to bound.
    None => true,
    Some(ExprOrSpread { spread: None, expr }) => match expr.as_ref() {
      Expr::Lit(Lit::Num(length)) => length.value <= MAX_AMPLIFIED_LENGTH,
      _ => false,
    },
    Some(_) => false,
  }
}

/// An argument is foldable when it carries its own value, or is an arrow
/// function reading nothing but its own parameters — the callback shape `map`,
/// `filter` and `reduce` take.
fn is_argument_foldable(arg: &ExprOrSpread, guard: Guard) -> bool {
  if arg.spread.is_some() {
    return false;
  }

  match arg.expr.as_ref() {
    Expr::Arrow(arrow) => is_closed_arrow(arrow, guard),
    expr => carries_its_own_value(expr, guard),
  }
}

/// Whether an arrow reads nothing but its own parameters. Anything else would
/// need a scope the engine does not have.
///
/// A block body is refused rather than analysed: statements bind, assign and
/// loop, and none of that is modelled here.
fn is_closed_arrow(arrow: &ArrowExpr, guard: Guard) -> bool {
  let mut params = Vec::with_capacity(arrow.params.len());

  for param in &arrow.params {
    match param {
      Pat::Ident(ident) => params.push(ident.sym.clone()),
      _ => return false,
    }
  }

  let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
    return false;
  };

  carries_its_own_value(body, guard.binding(&params))
}

/// Converts an engine value back to an AST literal, or answers `None` when it
/// has no representation the evaluator carries.
fn to_expr(value: &JsValue, engine: &mut Context) -> Option<Expr> {
  to_expr_within(value, engine, MAX_ENGINE_NESTING)
}

/// The conversion, with a nesting bound of its own.
///
/// [`MAX_ENGINE_NESTING`] bounds the expression handed to the engine and
/// [`MAX_FOLDED_ARRAY_LENGTH`] bounds how wide an array comes back, but neither
/// bounds how *deep* the answer is: `[0, 1, 2, …].reduce((a, b) => [a, b])` is
/// two elements wide at every level and one level deeper per element, so the
/// width check never fires and this recursion runs off the stack. That is an
/// abort rather than an unwind, which the `catch_unwind` at the NAPI boundary
/// cannot turn back into a diagnostic — the one failure this module must not
/// have. Same ceiling as the input side, for the same reason.
fn to_expr_within(value: &JsValue, engine: &mut Context, levels_left: usize) -> Option<Expr> {
  let levels_left = levels_left.checked_sub(1)?;

  // Read through the accessors rather than matching variants: the engine's
  // value is nan-boxed by default and an enum only under a feature, and both
  // answer these.
  if let Some(truth) = value.as_boolean() {
    return Some(Expr::Lit(Lit::Bool(truth.into())));
  }

  if let Some(number) = value.as_number() {
    return Some(Expr::Lit(Lit::Num(number.into())));
  }

  if value.is_null() {
    return Some(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })));
  }

  if let Some(string) = value.as_string() {
    // The bound on an amplifying argument bounds what one written call may be
    // asked to build; this bounds what actually came back, whatever produced
    // it. The array arm below has had such a bound from the start, and a string
    // is the other shape a fold can return at size.
    if string.len() as f64 > MAX_AMPLIFIED_LENGTH {
      return None;
    }

    // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
    // unpaired surrogate cannot survive this step. Substituting the replacement
    // character keeps the declaration text identical to what the reference
    // implementation writes to disk, and diverges only in the class name.
    return Some(Expr::Lit(Lit::Str(string.to_std_string_lossy().into())));
  }

  match value.as_object() {
    Some(object) if object.is_array() => {
      let length = object.get(js_string!("length"), engine).ok()?.as_number()? as u64;

      if length > MAX_FOLDED_ARRAY_LENGTH {
        return None;
      }

      let mut elems = Vec::with_capacity(usize::try_from(length).ok()?);

      for index in 0..length {
        let element = object.get(index, engine).ok()?;

        elems.push(Some(ExprOrSpread {
          spread: None,
          expr: Box::new(to_expr_within(&element, engine, levels_left)?),
        }));
      }

      Some(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
      }))
    },
    // A plain object, a function, a symbol, `undefined` and a BigInt have no
    // literal the evaluator carries, so the existing path keeps them.
    _ => None,
  }
}

/// The call as the minified source the engine is handed.
///
/// The module is assembled here rather than by `create_module`, which takes
/// `&Expr` and clones it — so going through it means cloning the subtree once
/// to build the `Expr` and once more inside. The printer needs an owned tree
/// either way, because it drops the spans in place before emitting.
fn print_call(call: &CallExpr) -> String {
  let module = Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(Expr::Call(call.clone())),
    }))],
    shebang: None,
  };

  print_module(
    &CodeFrame::new(),
    module,
    Some(
      Config::default()
        .with_minify(true)
        .with_omit_last_semi(true),
    ),
  )
}
