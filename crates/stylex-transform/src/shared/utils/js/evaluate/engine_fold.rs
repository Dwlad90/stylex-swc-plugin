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
      ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, KeyValueProp,
      Lit, MemberExpr, MemberProp, Null, ObjectLit, Pat, Prop, PropName, PropOrSpread,
    },
    codegen::Config,
  },
};

use stylex_js::helpers::is_mutating_array_method;

use crate::shared::utils::log::build_code_frame_error::{CodeFrame, create_module, print_module};

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

/// Methods whose result length is set by an argument rather than by the
/// receiver, and so are the only ones a single small argument can blow up.
const LENGTH_AMPLIFYING_METHODS: [&str; 3] = ["repeat", "padStart", "padEnd"];

/// How long a string one of those may be asked to build.
///
/// The engine bounds loop iterations, recursion and stack depth, but not
/// allocation: growth inside a native builtin is not a counted loop. So
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

thread_local! {
  /// One engine per thread, created on the first fold that needs it and reused
  /// for every later one. A file with no foldable method call never builds it.
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

  let source = print_expr(&Expr::Call(call.clone()));

  ENGINE.with_borrow_mut(|slot| {
    let engine = slot.get_or_insert_with(|| ManuallyDrop::new(Context::default()));

    match engine.eval(Source::from_bytes(&source)) {
      Ok(value) => to_expr(&value, engine),
      // A throw is an answer, not a failure of this module: the caller's
      // existing refusal already reports it. Nothing to fold.
      Err(_) => None,
    }
  })
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
    Expr::Member(MemberExpr { obj, prop, .. }) => {
      carries_its_own_value(obj, inner) && matches!(prop, MemberProp::Ident(_))
    },
    Expr::Array(ArrayLit { elems, .. }) => elems.iter().all(|elem| match elem {
      Some(ExprOrSpread { spread: None, expr }) => carries_its_own_value(expr, inner),
      // A hole is `undefined` and a spread needs the scope; both stay out.
      _ => false,
    }),
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

  if LOCALE_SENSITIVE_METHODS.contains(&&*method.sym) {
    return false;
  }

  if receiver_is_a_written_number(obj) {
    return false;
  }

  if !amplification_is_bounded(&method.sym, obj, &call.args) {
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
fn amplification_is_bounded(method: &Atom, receiver: &Expr, args: &[ExprOrSpread]) -> bool {
  if !LENGTH_AMPLIFYING_METHODS.contains(&&**method) {
    return true;
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
          expr: Box::new(to_expr(&element, engine)?),
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

fn print_expr(expr: &Expr) -> String {
  print_module(
    &CodeFrame::new(),
    create_module(expr),
    Some(
      Config::default()
        .with_minify(true)
        .with_omit_last_semi(true),
    ),
  )
}
