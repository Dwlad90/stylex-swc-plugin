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
//!
//! A fold answers one of two things: the value, or the rule that refused it.
//! There is no silent refusal — a call the guard recognised and declined says
//! which rule declined it, rather than falling through to the caller's
//! `Unsupported expression: CallExpression`. A call the guard never recognised
//! is not a refusal at all: it is simply not this module's, and the dispatch
//! below it decides what happens instead.

use std::{borrow::Cow, cell::RefCell, mem::ManuallyDrop};

use boa_engine::{
  Context, JsError, JsObject, JsResult, JsValue, Source, js_string, property::PropertyKey,
};
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

use stylex_ast::ast::factories::{create_ident_key_value_prop, create_object_lit};
use stylex_constants::constants::evaluation_errors::{
  amplification_inside_a_callback, array_length_too_large, engine_threw, escaping_property,
  expression_too_deep, folded_string_too_large, locale_sensitive_method, numeric_literal_receiver,
  object_size_too_large, unbounded_amplified_length, unfoldable_fold_result,
};

use super::evaluate_result_vec_to_array_expr;
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  utils::{
    common::order_own_keys,
    log::build_code_frame_error::{CodeFrame, print_module},
  },
};

/// Methods whose answer depends on locale data the engine does not carry.
///
/// It resolves them against the root locale, so `"i".toLocaleUpperCase("tr")`
/// comes back `I` where the language says `İ`, and a collation comparison
/// answers by code point. Folding those would put a wrong value in the
/// stylesheet, which is worse than not folding at all.
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

/// How many entries a folded array or object may carry back.
///
/// [`MAX_AMPLIFIED_LENGTH`] bounds a string, and a string can still be turned
/// into one element per code unit: `"x".repeat(999999).split("")` folds to an
/// array literal of a million AST nodes, which costs far more as a tree than it
/// did as text. An object pays the same price per property, so it is bounded by
/// the same number rather than by one of its own. A fallback list in a real
/// declaration holds a handful of values, and a nested style object a handful of
/// conditions, so this is generous by three orders of magnitude and still
/// refuses that.
const MAX_FOLDED_ENTRIES: u64 = 10_000;

/// How deeply nested an expression this module will hand to the engine, and how
/// deeply nested a value it will carry back.
///
/// Nesting is not free for whoever parses it. The engine's parser descends
/// through a nested array literal recursively and, measured on a debug build,
/// overflows its stack somewhere around a hundred levels — which aborts the
/// process from inside an evaluation whose whole contract is that it may fail.
/// Refusing deeper input is what turns that crash back into a diagnostic, and
/// the same bound applies on the way out because a loop inside the engine can
/// nest a value deeper than any expression that reached it.
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

/// The words a [refused fold](../../../../../CONTEXT.md) hands the caller,
/// ready to be raised as a deopt.
///
/// Borrowed where the rule has one fixed sentence and owned where it names the
/// method or the limit it refused on, so the common path allocates nothing.
pub(crate) type Refusal = Cow<'static, str>;

/// Why the guard did not hand a call to the engine — the outcome, where
/// [`Refusal`] is the half of it an author reads.
///
/// Only one of the two arms is a refusal in the glossary's sense. A call that
/// is not a candidate was decided from syntax before any rule ran: nothing
/// declined it, so nothing is reported and the dispatch below this module owns
/// the call. A rule that fired declined a call the guard did recognise, and
/// carries the words for it.
enum Decline {
  NotACandidate,
  Rule(Refusal),
}

impl Decline {
  fn rule(reason: impl Into<Refusal>) -> Self {
    Self::Rule(reason.into())
  }
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

/// How much nesting is left, and the one refusal spent at the bottom of it.
///
/// Both directions across the bridge count the same budget for the same
/// reason — the walk in recurses on the bare thread stack and so does the
/// conversion out — so they share the counter and the sentence rather than
/// keeping two that could drift apart.
#[derive(Clone, Copy)]
struct Depth(usize);

impl Depth {
  /// A full budget, at the start of a walk or a conversion.
  const FULL: Self = Self(MAX_ENGINE_NESTING);

  /// One level in, or the depth refusal at the bound — so depth is answered the
  /// same way as any other rule the guard applies.
  fn descend(self) -> Result<Self, Decline> {
    match self.0 {
      0 => Err(Decline::rule(expression_too_deep(MAX_ENGINE_NESTING))),
      left => Ok(Self(left - 1)),
    }
  }
}

/// What the guard carries as it walks: where a bare identifier may come from,
/// and how much nesting is left before the expression is refused as too deep.
#[derive(Clone, Copy)]
struct Guard<'a> {
  scope: Scope<'a>,
  depth: Depth,
}

impl<'a> Guard<'a> {
  /// The guard one level in.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }

  /// The same remaining depth, with a callback's parameters now in scope.
  fn binding(self, params: &'a [Atom]) -> Self {
    Self {
      scope: Scope::Params(params),
      ..self
    }
  }
}

/// Folds `call` through the engine.
///
/// `None` leaves the existing path in charge: the call is not one this module
/// handles, which is a question of syntax and not a refusal. `Some` is the
/// fold's own answer — the value, or the rule that declined it.
pub(crate) fn try_fold(call: &CallExpr) -> Option<Result<EvaluateResultValue, Refusal>> {
  let guard = Guard {
    scope: Scope::Nothing,
    depth: Depth::FULL,
  };

  match fold(call, guard) {
    Ok(value) => Some(Ok(value)),
    Err(Decline::NotACandidate) => None,
    Err(Decline::Rule(reason)) => Some(Err(reason)),
  }
}

fn fold(call: &CallExpr, guard: Guard) -> Result<EvaluateResultValue, Decline> {
  let method = admit_call(call, guard)?;

  let source = print_call(call);

  ENGINE.with_borrow_mut(|slot| {
    // Taken, not borrowed in place. A panic unwinding out of the engine is
    // caught at the NAPI boundary and the process carries on, so an engine left
    // in the slot would be reused by every later fold with its VM stack
    // abandoned mid-frame. Taking it means an unwind leaves the slot empty and
    // the next fold builds a fresh engine; the abandoned one leaks, which is
    // what `ManuallyDrop` already makes it do at thread exit.
    let mut engine = slot.take().unwrap_or_else(new_engine);

    let outward = Outward {
      method,
      depth: Depth::FULL,
    };

    let folded = match engine.eval(Source::from_bytes(&source)) {
      Ok(value) => to_value(&value, &mut engine, outward),
      // A throw is an answer, not a failure of this module — the language
      // throws on `[].reduce(f)` too — so the engine's own sentence is what the
      // author reads, rather than a generic refusal standing in for it.
      Err(error) => Err(outward.threw(&error)),
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

/// What the outward bridge carries as it converts a value back: the method
/// whose answer it is reading, so a refusal can name it, and how much nesting
/// is left before the value is refused as too deep.
///
/// The method is carried because the engine's own sentence does not always
/// name it — `"abc".unsupported()` throws `not a callable function`, which
/// tells an author nothing the code frame has not already shown them.
#[derive(Clone, Copy)]
struct Outward<'a> {
  method: &'a Atom,
  depth: Depth,
}

impl Outward<'_> {
  /// The bridge one level in.
  ///
  /// A value nested deeper than the guard admits on the way in can still be
  /// built on the way out, by a loop the engine ran rather than by syntax the
  /// author wrote. Bounded for the reason the input is bounded: the conversion
  /// recurses on the bare thread stack.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }

  /// A throw, in the engine's own words under this compiler's naming of the
  /// call that produced it.
  fn threw(self, error: &JsError) -> Decline {
    Decline::rule(engine_threw(self.method, &error.to_string()))
  }
}

/// Whether every value `expr` needs is either written into it or bound by the
/// guard's scope, so printing it yields source the engine can evaluate alone.
///
/// One walk serves both questions this module asks — a receiver that must be
/// wholly self-contained is the same walk with an empty scope — so a shape
/// accepted in one position cannot silently be refused in the other.
/// Deliberately narrow: a shape it does not recognise is not a candidate.
fn admit_value(expr: &Expr, guard: Guard) -> Result<(), Decline> {
  let inner = guard.descend()?;

  match expr {
    Expr::Ident(ident) => match guard.scope {
      Scope::Nothing => Err(Decline::NotACandidate),
      Scope::Params(params) if params.contains(&ident.sym) => Ok(()),
      Scope::Params(_) => Err(Decline::NotACandidate),
    },
    // A regular expression and a BigInt have no value this evaluator carries,
    // and neither does the reference implementation fold one.
    Expr::Lit(Lit::Regex(_) | Lit::BigInt(_)) => Err(Decline::NotACandidate),
    Expr::Lit(_) => Ok(()),
    Expr::Paren(paren) => admit_value(&paren.expr, inner),
    Expr::Unary(unary) => admit_value(&unary.arg, inner),
    Expr::Bin(bin) => {
      admit_value(&bin.left, inner)?;
      admit_value(&bin.right, inner)
    },
    Expr::Cond(cond) => {
      admit_value(&cond.test, inner)?;
      admit_value(&cond.cons, inner)?;
      admit_value(&cond.alt, inner)
    },
    // A named property read: `x.length` inside a callback, and `({a:1}).a` as
    // a receiver. A computed one is a lookup that needs the scope.
    //
    // Whether the receiver carries its own value is asked first, so a read the
    // guard cannot see the value of stays the dispatch below's rather than
    // becoming a refusal this module raised over it.
    Expr::Member(MemberExpr {
      obj,
      prop: MemberProp::Ident(name),
      ..
    }) => {
      admit_value(obj, inner)?;

      if lists(&ESCAPING_PROPERTIES, &name.sym) {
        return Err(Decline::rule(escaping_property(&name.sym)));
      }

      Ok(())
    },
    Expr::Array(ArrayLit { elems, .. }) => {
      for elem in elems {
        match elem {
          Some(ExprOrSpread { spread: None, expr }) => admit_value(expr, inner)?,
          // A hole is `undefined` and a spread needs the scope; both stay out.
          _ => return Err(Decline::NotACandidate),
        }
      }

      Ok(())
    },
    // A key and a value written out, which is a value the engine can be handed
    // whole. `__proto__` is the one key that is not: written as a plain
    // property it sets the prototype rather than a member, so the receiver the
    // engine sees is not the object the source appears to describe. It is left
    // in because the reference implementation folds it identically and every
    // route off the prototype is refused above — not because the walk models
    // it.
    Expr::Object(ObjectLit { props, .. }) => {
      for prop in props {
        let PropOrSpread::Prop(prop) = prop else {
          return Err(Decline::NotACandidate);
        };

        let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() else {
          return Err(Decline::NotACandidate);
        };

        if !matches!(
          key,
          PropName::Ident(_) | PropName::Str(_) | PropName::Num(_)
        ) {
          return Err(Decline::NotACandidate);
        }

        admit_value(value, inner)?;
      }

      Ok(())
    },
    // A chained call: the receiver is itself a fold. Printing the whole chain
    // and evaluating it once is what makes `[…].map(…).join('-')` work, which
    // two separate method tables cannot agree on.
    Expr::Call(call) => admit_call(call, inner).map(|_| ()),
    _ => Err(Decline::NotACandidate),
  }
}

/// Whether a call is a method call this module can hand to the engine whole.
///
/// Every boundary is checked here rather than at the outermost call, because a
/// chain hides its middle links: `"AB".toLocaleLowerCase().trim()` is a `trim`
/// whose receiver needs a locale.
///
/// Candidacy is settled before any rule fires, and both questions are settled
/// before anything is printed. The order is what keeps a rule from answering
/// for a call that was never this module's: a receiver the guard cannot see the
/// value of belongs to the dispatch below, and a rule raised over it would stop
/// the call reaching that dispatch — which is where `Math` and the callable
/// globals still fold. It costs nothing today, because the walk is syntax and
/// resolves nothing. Ticket 05 is where the walk starts resolving bindings and
/// the cheap rules have to move back in front of it.
fn admit_call<'a>(call: &'a CallExpr, guard: Guard) -> Result<&'a Atom, Decline> {
  let Callee::Expr(callee) = &call.callee else {
    return Err(Decline::NotACandidate);
  };

  let Expr::Member(MemberExpr { obj, prop, .. }) = callee.as_ref() else {
    return Err(Decline::NotACandidate);
  };

  // A computed method name is a lookup the guard cannot resolve, even when it
  // is written as a literal.
  let MemberProp::Ident(method) = prop else {
    return Err(Decline::NotACandidate);
  };

  admit_value(obj, guard)?;

  for arg in &call.args {
    admit_argument(arg, guard)?;
  }

  if lists(&ESCAPING_PROPERTIES, &method.sym) {
    return Err(Decline::rule(escaping_property(&method.sym)));
  }

  if lists(&LOCALE_SENSITIVE_METHODS, &method.sym) {
    return Err(Decline::rule(locale_sensitive_method(&method.sym)));
  }

  if receiver_is_a_written_number(obj) {
    return Err(Decline::rule(numeric_literal_receiver(&method.sym)));
  }

  admit_amplification(&method.sym, obj, &call.args, guard.scope)?;

  Ok(&method.sym)
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
/// multiply two allowed lengths into one that is not. And the call must not sit
/// inside a callback, which runs once per element of a receiver nothing here
/// measured, so a bound written once is multiplied by a count the source never
/// states. With all three in place the most a fold can build is one bounded
/// string per amplifying call written, so the source file bounds the total.
fn admit_amplification(
  method: &Atom,
  receiver: &Expr,
  args: &[ExprOrSpread],
  scope: Scope,
) -> Result<(), Decline> {
  if !lists(&LENGTH_AMPLIFYING_METHODS, method) {
    return Ok(());
  }

  // A written bound bounds one evaluation. A callback body is evaluated once
  // per element of a receiver this guard never measured, so the same bound is
  // multiplied by a count the source never states: `"x".repeat(999999)
  // .split("").map(() => "y".repeat(999999))` is two calls, each inside the
  // bound, building a terabyte between them. The count cannot be read here, so
  // an amplifying call inside a callback is refused whatever its argument says.
  if matches!(scope, Scope::Params(_)) {
    return Err(Decline::rule(amplification_inside_a_callback(method)));
  }

  let unbounded = || Decline::rule(unbounded_amplified_length(method, MAX_AMPLIFIED_LENGTH));

  if matches!(receiver, Expr::Call(_)) {
    return Err(unbounded());
  }

  match args.first() {
    // `"x".padStart()` amplifies nothing, so there is no length to bound.
    None => Ok(()),
    Some(ExprOrSpread { spread: None, expr }) => match expr.as_ref() {
      Expr::Lit(Lit::Num(length)) if length.value <= MAX_AMPLIFIED_LENGTH => Ok(()),
      _ => Err(unbounded()),
    },
    Some(_) => Err(unbounded()),
  }
}

/// An argument is admitted when it carries its own value, or is an arrow
/// function reading nothing but its own parameters — the callback shape `map`,
/// `filter` and `reduce` take.
fn admit_argument(arg: &ExprOrSpread, guard: Guard) -> Result<(), Decline> {
  if arg.spread.is_some() {
    return Err(Decline::NotACandidate);
  }

  match arg.expr.as_ref() {
    Expr::Arrow(arrow) => admit_arrow(arrow, guard),
    expr => admit_value(expr, guard),
  }
}

/// Whether an arrow reads nothing but its own parameters. Anything else would
/// need a scope the engine does not have.
///
/// A block body is refused rather than analysed: statements bind, assign and
/// loop, and none of that is modelled here.
fn admit_arrow(arrow: &ArrowExpr, guard: Guard) -> Result<(), Decline> {
  let mut params = Vec::with_capacity(arrow.params.len());

  for param in &arrow.params {
    match param {
      Pat::Ident(ident) => params.push(ident.sym.clone()),
      _ => return Err(Decline::NotACandidate),
    }
  }

  let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
    return Err(Decline::NotACandidate);
  };

  admit_value(body, guard.binding(&params))
}

/// Converts an engine value into the evaluator's own value type, or declines
/// with the rule that refused it.
///
/// The evaluator's type rather than a bare syntax node, so a folded value
/// reaches every place a value the author wrote reaches: an array answers the
/// `Vec` an array literal answers, and an object the `Expr::Object` an object
/// literal answers. Answering a syntax node instead is what left a folded array
/// and an evaluated one in two dispatch arms that disagree about which methods
/// they carry.
fn to_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  // Read through the accessors rather than matching variants: the engine's
  // value is nan-boxed by default and an enum only under a feature, and both
  // answer these.
  let literal = if let Some(truth) = value.as_boolean() {
    Lit::Bool(truth.into())
  } else if let Some(number) = value.as_number() {
    Lit::Num(number.into())
  } else if value.is_null() {
    Lit::Null(Null { span: DUMMY_SP })
  } else if let Some(string) = value.as_string() {
    // The bound on an amplifying argument bounds what one written call may be
    // asked to build; this bounds what actually came back, whatever produced
    // it. The array arm below has had such a bound from the start, and a string
    // is the other shape a fold can return at size.
    if string.len() as f64 > MAX_AMPLIFIED_LENGTH {
      return Err(Decline::rule(folded_string_too_large(MAX_AMPLIFIED_LENGTH)));
    }

    // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
    // unpaired surrogate cannot survive this step. Substituting the replacement
    // character keeps the declaration text identical to what the reference
    // implementation writes to disk, and diverges only in the class name.
    Lit::Str(string.to_std_string_lossy().into())
  } else {
    return to_object_value(value, engine, outward);
  };

  Ok(EvaluateResultValue::Expr(Expr::Lit(literal)))
}

/// The half of [`to_value`] that needs the engine to read the value back:
/// arrays and plain objects, and the refusal for everything else.
fn to_object_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  let Some(object) = value.as_object() else {
    return Err(Decline::rule(unfoldable_fold_result(describe(value))));
  };

  let inner = outward.descend()?;

  if object.is_array() {
    let length = read_length(&object, engine, outward)?;
    let mut items = Vec::with_capacity(length as usize);

    for index in 0..length {
      let element = read(outward, || object.get(index, engine))?;

      items.push(to_value(&element, engine, inner)?);
    }

    return Ok(EvaluateResultValue::Vec(items));
  }

  if !object.is_ordinary() {
    return Err(Decline::rule(unfoldable_fold_result(describe(value))));
  }

  let keys = read(outward, || object.own_property_keys(engine))?;

  if keys.len() as u64 > MAX_FOLDED_ENTRIES {
    return Err(Decline::rule(object_size_too_large(MAX_FOLDED_ENTRIES)));
  }

  let mut props = Vec::with_capacity(keys.len());

  for key in keys {
    // A symbol key has no spelling in an object literal, so an object carrying
    // one cannot be written back out whole — and writing it out partly would
    // fold a value the source does not describe. No input the guard admits
    // today produces one, since a symbol is reachable only through a bare
    // global; it is a refusal rather than an assumption because widening the
    // guard is what the rest of this work does.
    let name = match &key {
      PropertyKey::String(string) => string.to_std_string_lossy(),
      PropertyKey::Index(index) => index.get().to_string(),
      PropertyKey::Symbol(_) => return Err(Decline::rule(unfoldable_fold_result("a symbol key"))),
    };

    let element = read(outward, || object.get(key.clone(), engine))?;
    let expr = as_property_value(to_value(&element, engine, inner)?)?;

    props.push(create_ident_key_value_prop(&name, expr));
  }

  // Ordered by the same rule an object the author wrote is ordered by, rather
  // than by trusting two implementations of own-key order to agree.
  Ok(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
    order_own_keys(props),
  ))))
}

/// One folded value as the expression an object property carries.
///
///
/// An array is the one case that has to be rebuilt rather than moved: a `Vec`
/// is the shape the evaluator wants at the top of a value, and an object
/// literal wants a nested array literal in the same position. Rebuilt by the
/// evaluator's own conversion rather than by a second copy of it here, so a
/// folded property and an evaluated one cannot come to disagree about what an
/// array element may be.
fn as_property_value(value: EvaluateResultValue) -> Result<Expr, Decline> {
  let expr = match &value {
    EvaluateResultValue::Expr(expr) => Some(expr.clone()),
    EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(items),
    // Every arm of `to_value` answers one of the two above, so nothing else is
    // reachable by construction — and answers a refusal rather than panicking
    // if that ever stops holding.
    _ => None,
  };

  expr.ok_or_else(|| Decline::rule(unfoldable_fold_result("an unreadable value")))
}

/// An array's `length`, bounded: the count the conversion loop below reads.
///
/// The two ways it can fail say different things, because they are different
/// faults. A length past [`MAX_FOLDED_ENTRIES`] is the bound, and names it. A
/// `length` that is not a count at all — not a number, or negative — is not the
/// bound and must not claim to be; it is a value the bridge cannot read, and is
/// refused as one.
fn read_length(object: &JsObject, engine: &mut Context, outward: Outward) -> Result<u64, Decline> {
  let length = read(outward, || object.get(js_string!("length"), engine))?;

  let Some(length) = length.as_number().filter(|length| *length >= 0.0) else {
    return Err(Decline::rule(unfoldable_fold_result(
      "an array with no readable length",
    )));
  };

  if length > MAX_FOLDED_ENTRIES as f64 {
    return Err(Decline::rule(array_length_too_large(
      MAX_FOLDED_ENTRIES as usize,
    )));
  }

  Ok(length as u64)
}

/// A read back out of the engine, with a throw carried in the engine's words.
///
/// Reading a property can run a getter, which can throw, so the outward bridge
/// needs the same answer the evaluation itself gets rather than a second one.
fn read<T>(outward: Outward, read: impl FnOnce() -> JsResult<T>) -> Result<T, Decline> {
  read().map_err(|error| outward.threw(&error))
}

/// What kind of value the fold could not carry, in the words a refusal uses.
fn describe(value: &JsValue) -> &'static str {
  if value.is_undefined() {
    "undefined"
  } else if value.is_symbol() {
    "a symbol"
  } else if value.is_bigint() {
    "a BigInt"
  } else if value.as_object().is_some_and(|object| object.is_callable()) {
    "a function"
  } else {
    "an object of a kind with no literal form"
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
