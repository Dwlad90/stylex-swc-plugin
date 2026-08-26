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
//! The guard asks whether every leaf of an expression *resolves to a value the
//! bridge can carry*, not whether it is written out — so giving a value a name
//! does not change whether the call on it folds. A name the guard resolved
//! becomes a parameter of a printed arrow and its value an argument to it, so
//! `s.toLowerCase()` is handed over as `(s) => s.toLowerCase()` called with the
//! string `s` holds. See [`Transport`] for why the value travels beside the
//! source rather than inside it.
//!
//! A fold answers one of two things: the value, or the rule that refused it.
//! There is no silent refusal — a call the guard recognised and declined says
//! which rule declined it, rather than falling through to the caller's
//! `Unsupported expression: CallExpression`. A call the guard never recognised
//! is not a refusal at all: it is simply not this module's, and the dispatch
//! below it decides what happens instead.

use std::{borrow::Cow, cell::RefCell, mem::ManuallyDrop};

use boa_engine::{
  Context, JsError, JsObject, JsResult, JsString, JsValue, Source, js_string, property::PropertyKey,
};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, Ident,
      KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleItem, Null, ObjectLit, Pat, Prop,
      PropName, PropOrSpread, Stmt, Tpl,
    },
    codegen::Config,
  },
};

use stylex_ast::ast::convertors::atom_utf16_length;
use stylex_ast::ast::factories::{
  create_arrow_expression_with_params, create_binding_ident, create_ident,
  create_ident_key_value_prop, create_object_lit,
};
use stylex_constants::constants::evaluation_errors::{
  amplification_inside_a_callback, array_length_too_large, bound_value_too_large, engine_threw,
  escaping_property, expression_too_deep, folded_string_too_large, locale_sensitive_method,
  numeric_literal_receiver, object_size_too_large, unbounded_amplified_length,
  uncallable_printed_fold, unfoldable_fold_result,
};
use stylex_js::helpers::is_valid_callee;

use super::{evaluate_cached, evaluate_result_vec_to_array_expr, helpers::get_binding};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
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
  /// The module the expression was written in, read through the evaluator. A
  /// name it resolves to a carryable value becomes a parameter of the printed
  /// arrow; a name it cannot is not this module's call.
  Module,
  /// A callback's own parameters, which the engine binds itself when it invokes
  /// the callback with values from the receiver. A name that is not one of them
  /// is still resolved against the module.
  Params(&'a [Atom]),
}

/// How much nesting is left, and the one refusal spent at the bottom of it.
///
/// Both directions across the bridge count the same budget for the same
/// reason — the walk in recurses on the bare thread stack and so does the
/// conversion out — so they share the counter and the sentence rather than
/// keeping two that could drift apart.
///
/// Reaching the bottom is a refusal and not a "not mine", which is the one
/// place the two are worth telling apart. This bound is the engine parser's
/// stack, so it does not move when a project raises the evaluator's ceiling,
/// and under a raised ceiling the older path would fold what this declines —
/// so the refusal costs a fold. It is taken because the two ceilings no longer
/// carry the same number, and a bound this module owns has to answer in this
/// module's words: handing the shape back instead makes which sentence an
/// author reads depend on which of two disagreeing ceilings they crossed.
/// Handing it back is at least safe now — the nested array that reached the
/// older `join` refuses rather than panicking — so what remains is the
/// diagnostic, and Ticket 11 owns unifying the two ceilings.
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

/// The names the guard resolved and the values it resolved them to, ready to
/// cross the bridge inward.
///
/// The value travels *beside* the printed source rather than inside it: the
/// expression is printed as an arrow taking these names as parameters, and the
/// values are passed as arguments. Substituting a literal into the text instead
/// would reprint and reparse the whole value at every use site, and a value with
/// no literal spelling could not cross at all — where an argument keeps the
/// printed text the size of the expression however large the value is.
///
/// The author's own name is the parameter name. Nothing has to be rewritten for
/// that, and a callback parameter of the same name shadows it in the printed
/// arrow exactly as it does in the module the author wrote.
///
/// Chosen over registering the names on the engine because the engine is one
/// leaked instance per thread, shared by every file that thread compiles: a name
/// left behind or shadowed there would be a cross-file correctness bug.
#[derive(Default)]
struct Transport {
  params: Vec<Atom>,
  values: Vec<Wtf8Atom>,
}

impl Transport {
  /// Records `value` under `name`, or does nothing where the name is already
  /// carried.
  ///
  /// One parameter per name however often the expression reads it, because a
  /// repeated parameter is a syntax error in the arrow this is printed into. The
  /// second reading resolves to the same value — the evaluator memoises it — so
  /// dropping it loses nothing. A scan rather than a set, because an expression
  /// carries a handful of names and hashing them would cost more than comparing
  /// them.
  fn bind(&mut self, name: &Atom, value: Wtf8Atom) {
    if self.params.contains(name) {
      return;
    }

    self.params.push(name.clone());
    self.values.push(value);
  }

  /// The resolved values as the arguments the printed arrow is called with,
  /// bounded.
  ///
  /// The bound is on the value rather than on the syntax that named it, which is
  /// the whole reason it is applied here: a name is three characters whatever it
  /// holds, so the printed expression says nothing about how much is about to be
  /// copied into the engine. Bounded by the number that bounds a folded string
  /// on the way out, because it is the same measurement on the other side.
  ///
  /// Nesting needs no bound of its own while a resolved value crosses only as a
  /// string: a string is one level deep by construction, and a value that is
  /// nested at all is refused for not being one.
  ///
  /// Bounded on the running total and not on each value, because what is about to
  /// be copied into the engine is all of them: eight names each one code unit
  /// under the limit is eight megabytes, and a per-value check would wave every
  /// one of them through. The name reported is the one that crossed the line,
  /// which is the one an author can shorten.
  fn arguments(&self) -> Result<Vec<JsValue>, Decline> {
    let mut arguments = Vec::with_capacity(self.values.len());
    let mut carried = 0u64;

    for (name, value) in self.params.iter().zip(&self.values) {
      // Counted in UTF-16 code units, which is what the engine's own strings are
      // measured in and what bounds a folded string on the way out. Saturating
      // because the sum is a bound to refuse on, and a wrapped one would admit.
      carried = carried.saturating_add(atom_utf16_length(value) as u64);

      if carried as f64 > MAX_AMPLIFIED_LENGTH {
        return Err(Decline::rule(bound_value_too_large(
          name,
          MAX_AMPLIFIED_LENGTH,
        )));
      }

      arguments.push(JsValue::from(carry_string(value)));
    }

    Ok(arguments)
  }
}

/// What the walk needs that the expression does not carry: the evaluator, so a
/// name can be resolved to the value it holds, and the transport the resolved
/// values are collected into.
struct Reader<'a> {
  state: &'a mut EvaluationState,
  traversal_state: &'a mut StateManager,
  fns: &'a FunctionMap,
  transport: Transport,
}

impl Reader<'_> {
  /// The string `ident` resolves to, or `None` where it resolves to nothing the
  /// bridge carries.
  ///
  /// Resolved through the evaluator's own memoised entry point rather than by a
  /// second reading of its own, so a binding this fold reads is the binding
  /// every other position reads — including the disqualifications that live
  /// there: a reassigned binding, one mutated in place, and one read above its
  /// own declaration all answer nothing here because they answer nothing there.
  ///
  /// A string is the only value carried, so a theme reference crosses only as
  /// the string it already resolved to and never as the reference itself.
  ///
  /// The read is a speculation and is marked as one, so nothing it refuses is
  /// left behind: the evaluation's confidence and deopt are put back, and the
  /// memo withholds the refusal. A name this module could not read is not a
  /// refusal — the dispatch below owns the call, evaluates the same name itself,
  /// and has to find both the state and the sentence it would have had.
  fn resolve(&mut self, ident: &Ident) -> Option<Wtf8Atom> {
    let reference = Expr::Ident(ident.clone());
    let Reader {
      state,
      traversal_state,
      fns,
      ..
    } = self;

    // The putting-back is the whole contract, so it belongs to the two states
    // that own the fields rather than to a sequence written out here that a
    // later edit could return past.
    speculate(state, traversal_state, |state, traversal_state| {
      evaluate_cached(&reference, state, traversal_state, fns)
    })
    .and_then(as_carryable_string)
  }
}

/// Runs `read` as a [speculative
/// read](../../../../../CONTEXT.md#speculative-read), and puts back everything it
/// refused.
///
/// `None` where the read refused, so a caller cannot mistake a refusal for a
/// value: the confidence that says which it was is gone by the time the caller
/// sees the answer, which is the point.
///
/// The two flags are saved and restored rather than cleared, so a fold reached
/// from inside another speculation stays inside it for as long as that one lasts.
/// Written as one function taking a closure rather than as a save, a call and a
/// restore at the call site, because the restore is the contract and a `return`
/// added between the halves would silently drop it.
fn speculate(
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  read: impl FnOnce(&mut EvaluationState, &mut StateManager) -> Option<EvaluateResultValue>,
) -> Option<EvaluateResultValue> {
  let confident = state.confident;
  let deopt_path = state.deopt_path.take();
  let deopt_reason = state.deopt_reason.take();
  let speculating = traversal_state.speculating;

  traversal_state.speculating = true;

  let read = read(state, traversal_state);

  let value = match state.confident {
    true => read,
    false => None,
  };

  traversal_state.speculating = speculating;
  state.confident = confident;
  state.deopt_path = deopt_path;
  state.deopt_reason = deopt_reason;

  value
}

/// One resolved string as the engine's own string type.
///
/// Two readings of the same atom, because a JavaScript string literal can hold
/// an unpaired surrogate and no Rust `str` can. The engine's strings are UTF-16,
/// so the ill-formed reading carries such a value across *exactly* — where the
/// outward direction has to substitute the replacement character, since a
/// `Lit::Str` is what it has to land in. A valid value takes the direct reading
/// rather than being taken apart into code units it would only be rebuilt from.
fn carry_string(value: &Wtf8Atom) -> JsString {
  match value.as_str() {
    Some(text) => JsString::from(text),
    None => JsString::from(&value.to_ill_formed_utf16().collect::<Vec<u16>>()[..]),
  }
}

/// The string an evaluated value carries, where it carries one.
///
/// Deliberately narrow. Every other shape the evaluator can answer — an array, a
/// plain object, a number, a function configuration, an unresolved theme
/// reference — is a value some later ticket widens the bridge to, and admitting
/// one before its own tests exist would fold it silently.
fn as_carryable_string(value: EvaluateResultValue) -> Option<Wtf8Atom> {
  match value {
    EvaluateResultValue::Expr(Expr::Lit(Lit::Str(string))) => Some(string.value),
    _ => None,
  }
}

/// Folds `call` through the engine.
///
/// `None` leaves the existing path in charge: the call is not one this module
/// handles, which is a question of syntax and of what the module's names hold,
/// not a refusal. `Some` is the fold's own answer — the value, or the rule that
/// declined it.
pub(crate) fn try_fold(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<Result<EvaluateResultValue, Refusal>> {
  let guard = Guard {
    scope: Scope::Module,
    depth: Depth::FULL,
  };

  let mut reader = Reader {
    state,
    traversal_state,
    fns,
    transport: Transport::default(),
  };

  match fold(call, guard, &mut reader) {
    Ok(value) => Some(Ok(value)),
    Err(Decline::NotACandidate) => None,
    Err(Decline::Rule(reason)) => Some(Err(reason)),
  }
}

fn fold(
  call: &CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<EvaluateResultValue, Decline> {
  let method = admit_call(call, guard, reader)?;

  let arguments = reader.transport.arguments()?;
  let source = print_fold(call, &reader.transport.params);

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

    let folded = apply(&source, &arguments, &mut engine, outward)
      .and_then(|value| to_value(&value, &mut engine, outward));

    *slot = Some(engine);

    folded
  })
}

/// Compiles the printed arrow and calls it with the transported values.
///
/// Two steps rather than one evaluation *when there is something to pass*,
/// because the values cross as arguments rather than as text. Every step can
/// throw and all of them are answered the same way: a throw is an answer, not a
/// failure of this module — the language throws on `[].reduce(f)` too — so the
/// engine's own sentence is what the author reads rather than a generic refusal
/// standing in for it.
///
/// A fold that resolved no name is evaluated directly, which is the whole of why
/// this branches. Wrapping it in an arrow and invoking that arrow costs a
/// function object and a VM frame on top of the expression itself: measured, it
/// is +44% on the cheapest leg of the benchmark and +24% on the chain, paid by
/// exactly the folds that gained nothing from the transport, since every
/// expression that folded before this work resolves no name. The branch is on one
/// question — did the guard resolve anything — and both arms hand the same
/// expression to the same engine, so it is not the two-tables-that-must-agree
/// this module exists to remove.
///
/// The compiled value is a function by construction, so the refusal for one that
/// is not stands in for a broken invariant rather than for anything an author can
/// write. It is a refusal all the same: this runs inside an evaluation whose
/// whole contract is that it may fail, where an assertion would abort a build
/// that a deopt would only leave to the runtime.
fn apply(
  source: &str,
  arguments: &[JsValue],
  engine: &mut Context,
  outward: Outward,
) -> Result<JsValue, Decline> {
  let evaluated = engine
    .eval(Source::from_bytes(source))
    .map_err(|error| outward.threw(&error))?;

  if arguments.is_empty() {
    return Ok(evaluated);
  }

  let Some(callable) = evaluated.as_callable() else {
    return Err(Decline::rule(uncallable_printed_fold(outward.method)));
  };

  callable
    .call(&JsValue::undefined(), arguments, engine)
    .map_err(|error| outward.threw(&error))
}

/// Whether this thread is holding an engine — the observable half of "built on
/// first use and never before".
///
/// Test-only, and reading the slot rather than counting constructions, because
/// what the claim is about is whether an engine exists after an input the fold
/// declined. Paired with [`forget_engine`], since a test asserting an engine was
/// *not* built has to start from a thread that has none.
#[cfg(test)]
pub(super) fn holds_an_engine() -> bool {
  ENGINE.with_borrow(|slot| slot.is_some())
}

/// Drops this thread's engine reference without dropping the engine, which is
/// what the slot's `ManuallyDrop` already does at thread exit and for the same
/// reason: the collector lives in a thread-local of its own and the drop order
/// between the two is not defined.
#[cfg(test)]
pub(super) fn forget_engine() {
  ENGINE.with_borrow_mut(|slot| {
    slot.take();
  });
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

/// Whether every value `expr` needs is written into it, bound by the guard's
/// scope, or resolvable from the module — so the printed arrow and the values
/// beside it are together something the engine can evaluate alone.
///
/// One walk serves both questions this module asks — the receiver's and the
/// arguments' — so a shape accepted in one position cannot silently be refused
/// in the other. Deliberately narrow: a shape it does not recognise is not a
/// candidate.
fn admit_value(expr: &Expr, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let inner = guard.descend()?;

  match expr {
    // A name in the callback's own parameters is bound by the engine when it
    // invokes the callback. Any other name is asked of the module, and becomes a
    // parameter of the printed arrow carrying the value it resolved to.
    Expr::Ident(ident) => {
      if let Scope::Params(params) = guard.scope
        && params.contains(&ident.sym)
      {
        return Ok(());
      }

      match reader.resolve(ident) {
        Some(value) => {
          reader.transport.bind(&ident.sym, value);

          Ok(())
        },
        None => Err(Decline::NotACandidate),
      }
    },
    // A regular expression and a BigInt have no value this evaluator carries,
    // and neither does the reference implementation fold one.
    Expr::Lit(Lit::Regex(_) | Lit::BigInt(_)) => Err(Decline::NotACandidate),
    Expr::Lit(_) => Ok(()),
    Expr::Paren(paren) => admit_value(&paren.expr, inner, reader),
    Expr::Unary(unary) => admit_value(&unary.arg, inner, reader),
    Expr::Bin(bin) => {
      admit_value(&bin.left, inner, reader)?;
      admit_value(&bin.right, inner, reader)
    },
    Expr::Cond(cond) => {
      admit_value(&cond.test, inner, reader)?;
      admit_value(&cond.cons, inner, reader)?;
      admit_value(&cond.alt, inner, reader)
    },
    // A template literal is written-out syntax whose holes are values in their
    // own right, so it is walked like any other composite and printed back
    // exactly as it was written. A tagged one is a different node and is not
    // here: its tag is a call this module cannot see the body of.
    Expr::Tpl(Tpl { exprs, .. }) => {
      for hole in exprs {
        admit_value(hole, inner, reader)?;
      }

      Ok(())
    },
    // A named property read: `x.length` inside a callback, and `({a:1}).a` as
    // a receiver. A computed one is a lookup that needs the scope.
    //
    // The property is answered before the receiver is walked, because the name
    // alone decides it and the walk now resolves bindings — so a read that no
    // receiver could make safe must not cost a resolution first.
    Expr::Member(MemberExpr {
      obj,
      prop: MemberProp::Ident(name),
      ..
    }) => {
      if lists(&ESCAPING_PROPERTIES, &name.sym) {
        return Err(Decline::rule(escaping_property(&name.sym)));
      }

      admit_value(obj, inner, reader)
    },
    Expr::Array(ArrayLit { elems, .. }) => {
      for elem in elems {
        match elem {
          Some(ExprOrSpread { spread: None, expr }) => admit_value(expr, inner, reader)?,
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

        admit_value(value, inner, reader)?;
      }

      Ok(())
    },
    // A chained call: the receiver is itself a fold. Printing the whole chain
    // and evaluating it once is what makes `[…].map(…).join('-')` work, which
    // two separate method tables cannot agree on.
    Expr::Call(call) => admit_call(call, inner, reader).map(|_| ()),
    _ => Err(Decline::NotACandidate),
  }
}

/// Whether a call is a method call this module can hand to the engine whole.
///
/// Every boundary is checked here rather than at the outermost call, because a
/// chain hides its middle links: `"AB".toLocaleLowerCase().trim()` is a `trim`
/// whose receiver needs a locale.
///
/// Nearly everything answerable from syntax is answered before the walk, because
/// the walk resolves bindings and resolution is the only expensive thing here. So
/// the shape of the callee, the spelling of the method name, a receiver written
/// as a number and a length nothing bounds are all settled while they are still
/// free, and only an expression this module intends to fold pays to have its
/// names read. The one exception is named where it is applied.
///
/// That ordering is why candidacy is *not* settled first. A rule firing over a
/// call that was never this module's would stop it reaching the dispatch below —
/// which is where `Math` and the callable globals still fold — so the one
/// candidacy question resolution would otherwise pay for is asked up front:
/// a receiver naming one of those globals is handed straight back.
fn admit_call<'a>(
  call: &'a CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<&'a Atom, Decline> {
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

  // A receiver naming a global whose methods the dispatch below folds — `Math`
  // and `Object` today — is that dispatch's call and not this one's. Tickets 07
  // and 09 are where those surfaces move here and this question goes away.
  //
  // Only where the module declares no binding of that name, which is the same
  // question the callee branch of that dispatch asks. A locally-declared shadow
  // is the module's own value and is resolved like any other name: measured,
  // `const String = 'abc'; String.toUpperCase()` folds to `ABC` in the reference
  // compiler, so treating the name as the global would refuse an input it
  // compiles. The lookup is one map read and no evaluation, so it stays in front
  // of the walk with the other cheap answers.
  if is_valid_callee(obj) && get_binding(obj, reader.traversal_state).is_none() {
    return Err(Decline::NotACandidate);
  }

  if lists(&LOCALE_SENSITIVE_METHODS, &method.sym) {
    return Err(Decline::rule(locale_sensitive_method(&method.sym)));
  }

  if receiver_is_a_written_number(obj) {
    return Err(Decline::rule(numeric_literal_receiver(&method.sym)));
  }

  admit_amplification(&method.sym, obj, &call.args, guard.scope)?;

  admit_value(obj, guard, reader)?;

  for arg in &call.args {
    admit_argument(arg, guard, reader)?;
  }

  // The one rule left behind the walk, and deliberately. It is a name check like
  // the three above and would cost nothing in front of them, but a chain of
  // escaping reads is refused outermost-first there — so
  // `''.constructor.constructor('return 1').call()` would be named for its
  // `call` rather than for the `constructor` that is the whole of the reason.
  // The walk reaches the receiver's reads first, which is the sentence worth
  // reading, and the resolution it costs is one binding on a call already
  // certain to refuse.
  if lists(&ESCAPING_PROPERTIES, &method.sym) {
    return Err(Decline::rule(escaping_property(&method.sym)));
  }

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
fn admit_argument(arg: &ExprOrSpread, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  if arg.spread.is_some() {
    return Err(Decline::NotACandidate);
  }

  match arg.expr.as_ref() {
    Expr::Arrow(arrow) => admit_arrow(arrow, guard, reader),
    expr => admit_value(expr, guard, reader),
  }
}

/// Whether an arrow reads nothing but its own parameters and names the module
/// resolves. Anything else would need a scope the engine does not have.
///
/// A block body is refused rather than analysed: statements bind, assign and
/// loop, and none of that is modelled here.
fn admit_arrow(arrow: &ArrowExpr, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
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

  admit_value(body, guard.binding(&params), reader)
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
  // `typeof` names the kind rather than a word list of this module's own: the
  // engine already answers this question, exhaustively, and its answer is the
  // one an author would use for the value they wrote.
  let Some(object) = value.as_object() else {
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
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
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
  }

  let keys = read(outward, || object.own_property_keys(engine))?;

  if keys.len() as u64 > MAX_FOLDED_ENTRIES {
    return Err(Decline::rule(object_size_too_large(MAX_FOLDED_ENTRIES)));
  }

  let mut props = Vec::with_capacity(keys.len());

  for key in keys {
    // A symbol key has no spelling in an object literal, so an object carrying
    // one cannot be written back out whole — and writing it out partly would
    // fold a value the source does not describe. `PropertyKey` has three
    // variants and all three are answered, as the crate's error policy asks.
    let name = match &key {
      PropertyKey::String(string) => string.to_std_string_lossy(),
      PropertyKey::Index(index) => index.get().to_string(),
      PropertyKey::Symbol(_) => {
        return Err(Decline::rule(unfoldable_fold_result(
          "object with a symbol key",
        )));
      },
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

  expr.ok_or_else(|| Decline::rule(unfoldable_fold_result("value of an unreadable kind")))
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
      "array with no readable length",
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

/// The call as the minified source the engine is handed: an arrow over the names
/// the guard resolved, whose values [`apply`] passes to it as arguments — or the
/// call alone where it resolved none.
///
/// The bare form is not a second path so much as the absence of one: an arrow
/// over no parameters, invoked immediately, is the same expression with a
/// function object and a VM frame added, and [`apply`] carries the measurement
/// that says what those cost. Printing the call itself is what lets an expression
/// that names nothing pay nothing.
///
/// The module is assembled here rather than by `create_module`, which takes
/// `&Expr` and clones it — so going through it means cloning the subtree once
/// to build the `Expr` and once more inside. The printer needs an owned tree
/// either way, because it drops the spans in place before emitting.
fn print_fold(call: &CallExpr, params: &[Atom]) -> String {
  let folded = Expr::Call(call.clone());

  let printed = match params.is_empty() {
    true => folded,
    false => create_arrow_expression_with_params(
      params
        .iter()
        .map(|name| Pat::Ident(create_binding_ident(create_ident(name))))
        .collect(),
      folded,
    ),
  };

  let module = Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(printed),
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
