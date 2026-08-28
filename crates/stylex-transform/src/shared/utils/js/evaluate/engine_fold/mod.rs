//! Folds a self-contained method call by evaluating it, rather than by
//! matching its name against a table of methods.
//!
//! A table is finite by construction, so the method it does not list is the
//! next bug report. Handing the call to a JavaScript engine instead covers
//! `String.prototype`, `Array.prototype` and `Object.prototype` at once, along
//! with the `Math` and `Object` statics, and covers a chain for free: the
//! receiver of a call is itself a candidate, so
//! `["1px","solid"].concat(["red"]).join(" ")` is printed and evaluated once
//! instead of being split across two tables that have to agree.
//!
//! Everything therefore rests on the guard in front of the engine. What it
//! lets through is answered by the language; what it holds back is a boundary
//! with a reason, and each one is named where it is applied.
//!
//! The guard asks whether every leaf of an expression *resolves to a value the
//! bridge can carry*, not whether it is written out — so giving a value a name
//! does not change whether the call on it folds. A name the guard resolved
//! becomes a parameter of a printed arrow and its value an argument to it, so
//! `s.toLowerCase()` is handed over as `(s) => s.toLowerCase()` called with the
//! string `s` holds. See [`Transport`](transport::Transport) for why the value
//! travels beside the source rather than inside it.
//!
//! A name holding a *function* crosses by the other route, because a function
//! has no value an argument could carry: the declaration it came from is
//! printed as the parameter's default and nothing is passed for it. So a
//! callback given a name folds exactly as the same arrow written out in place
//! does, down to which spelling shadows which. Calling such a name is the same
//! carriage and a different question — see [`Position`] for the one thing on
//! this bridge that reads where a call sits.
//!
//! A fold answers one of two things: the value, or the rule that refused it.
//! There is no silent refusal — a call the guard recognised and declined says
//! which rule declined it, rather than falling through to the caller's
//! `Unsupported expression: CallExpression`. A call the guard never recognised
//! is not a refusal at all: it is simply not this module's, and the dispatch
//! below it decides what happens instead.
//!
//! The fold is four concerns and a module each, so the guard — the subject of
//! everything above — reads on its own rather than inside the whole bridge:
//! [`guard`] walks the expression and applies every boundary named here,
//! [`transport`] carries the values it resolved inward, [`amplification`]
//! answers how much a call would build, and [`outward`] reads the engine's
//! answer back.
//!
//! Owning the engine is a fifth and lives in [`engine`]: which expressions may
//! fold is decided in the guard, where they run is decided there, and nothing in
//! that answer turns on a rule the guard applies.
//!
//! What is left here is the way in — [`try_fold`] — and the vocabulary the four
//! share: the refusal, the depth budget and the two allocation ceilings.

mod amplification;
mod engine;
mod guard;
mod outward;
mod transport;

use engine::{ENGINE, Engine, print_fold, threw};
use guard::{Admitted, Guard, Position, Reader, Repeats, Scope, Walk, admit_an_applied_global};
use outward::{Outward, to_value};

pub(super) use guard::unshadowed_global;

// Read by the evaluator's own tests, which sit one level up.
#[cfg(test)]
pub(super) use engine::{compiled_expressions, forget_engine, holds_an_engine};

use std::borrow::Cow;

use boa_engine::JsValue;
use swc_core::ecma::ast::{CallExpr, Expr};

use stylex_constants::constants::evaluation_errors::{
  expression_too_deep, uncallable_printed_fold,
};

use super::{evaluate_result_vec_to_array_expr, growable_stack};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
};

/// What one fold may allocate, in the two units that cost separately.
///
/// The engine bounds recursion and VM stack depth by default, and this module
/// bounds loop iterations itself
/// ([`MAX_LOOP_ITERATIONS`](engine::MAX_LOOP_ITERATIONS)), because Boa's own
/// default for that is `u64::MAX`. What none of them bound is allocation:
/// growth inside a native builtin is not a counted loop. So
/// `"x".repeat(200000000)` is a typo that folds — agreeing with the language —
/// at gigabytes of resident memory, and a compiler that dies there is worse than
/// one that declines the fold.
///
/// Two numbers rather than one, because a value costs in two ways that do not
/// stand in for each other: a bounded string can still become one element per
/// code unit, which costs far more as a tree than it did as text, and a thousand
/// empty arrays hold no text at all and are still a thousand values to build.
///
/// Carried rather than read at each of the six sites that spend them -- three
/// each, a resolved value on the way in and the answer on the way back -- for
/// the reason the depth ceiling is carried: the same number bounds both
/// directions, and sites that each reached for the option could come to
/// disagree about which number that is. Where the values
/// come from, and what each costs, is `stylex_structures::fold_ceilings`.
#[derive(Clone, Copy)]
struct Ceilings {
  /// UTF-16 code units of string — the unit the engine's own strings are
  /// measured in.
  characters: u64,
  /// Array elements and object properties.
  entries: u64,
}

/// Whether `name` is one of the names in `list`.
///
/// Every list it is asked about — the guard's two and amplification's two — is
/// three to ten entries, so a scan beats any structure that would have to hash
/// first, and one function keeps every call site from spelling the double
/// reference its own way. Takes a `&str` so a name read off a member property
/// and one read out of a string literal ask the same question rather than one of
/// them reaching past the helper.
fn lists(list: &[&str], name: &str) -> bool {
  list.contains(&name)
}

/// The words a [refused fold](../../../../../CONTEXT.md) hands the caller,
/// ready to be raised as a deopt.
///
/// Borrowed where the rule has one fixed sentence and owned where it names the
/// method or the limit it refused on, so the common path allocates nothing.
pub(crate) type Refusal = Cow<'static, str>;

/// One evaluated value as the expression it spells, where it spells one.
///
/// An array is the one case that has to be rebuilt rather than cloned, by the
/// evaluator's own conversion rather than by a second copy of it here. Shared
/// between the two positions that ask — a folded property on the way out, and a
/// resolved amplification count on the way in — so the two cannot come to
/// disagree about which values have an expression form. It sits here for the
/// reason [`lists`] does: neither direction owns it, and the one that held it
/// would be imported by the other.
fn as_expr(value: &EvaluateResultValue) -> Option<Expr> {
  match value {
    EvaluateResultValue::Expr(expr) => Some(expr.clone()),
    EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(items),
    _ => None,
  }
}

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

/// How much nesting is left, and the one refusal spent at the bottom of it.
///
/// Every walk across the bridge counts the same budget for the same reason ---
/// the guard's walk in, the conversion of a value it resolved, and the
/// conversion out all recurse on the stack — so they share the counter and
/// the sentence rather than keeping three that could drift apart.
///
/// The budget is the project's configured evaluation depth, which is also what
/// the evaluator's own descent spends. One number, because both walks run on
/// the same grown stack for the same reason, and two would mean an author who
/// raised the configured depth still met a ceiling nobody set.
///
/// Reaching the bottom is a refusal and not a "not mine": the shape is one an
/// author can shorten, and saying so is more use than handing it back to a path
/// that would refuse it in vaguer words.
#[derive(Clone, Copy)]
struct Depth {
  /// Levels left before the walk refuses.
  left: usize,
  /// The ceiling the refusal names — kept beside the count because a walk
  /// that has spent its budget no longer knows what it started with.
  ceiling: usize,
}

impl Depth {
  /// A full budget, at the start of a walk or a conversion.
  fn full(ceiling: usize) -> Self {
    Self {
      left: ceiling,
      ceiling,
    }
  }

  /// A full budget again, for a walk that starts where this one stands.
  ///
  /// A value the guard resolved and a value the engine answered are each
  /// converted by a walk of their own, and neither is nested inside the
  /// expression that reached them — so each is measured against the whole
  /// ceiling rather than against what the walk before it had left.
  fn restart(self) -> Self {
    Self::full(self.ceiling)
  }

  /// One level in, or the depth refusal at the bound — so depth is answered
  /// the same way as any other rule the guard applies.
  fn descend(self) -> Result<Self, Decline> {
    match self.left {
      0 => Err(Decline::rule(expression_too_deep(self.ceiling))),
      left => Ok(Self {
        left: left - 1,
        ..self
      }),
    }
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
  let ceiling = traversal_state.evaluation_ceiling();
  let ceilings = Ceilings {
    characters: traversal_state.character_ceiling() as u64,
    entries: traversal_state.entry_ceiling() as u64,
  };

  let guard = Guard {
    scope: Scope::Module,
    depth: Depth::full(ceiling),
    ceilings,
    // The expression the caller asked about is evaluated once.
    repeats: Repeats::Times(1),
    callback: None,
  };

  let mut reader = Reader::new(state, traversal_state, fns, ceilings);

  // Grown before the first recursive step rather than at every step: the
  // engine's parser descends through a nested literal without ever asking for
  // room, so the whole fold has to run on a stack that was already large enough
  // when it started.
  let mut walk = Walk::new(guard, &mut reader);

  match growable_stack::grown_for_depth(ceiling, || fold(call, &mut walk)) {
    Ok(value) => Some(Ok(value)),
    Err(Decline::NotACandidate) => None,
    Err(Decline::Rule(reason)) => Some(Err(reason)),
  }
}

fn fold(call: &CallExpr, walk: &mut Walk) -> Result<EvaluateResultValue, Decline> {
  let admitted = walk.admit_call(call, Position::Outermost)?;

  // A theme reference crosses as the string its own `toString` answers, which is
  // every member the group has except the members themselves. So an expression
  // that reads a property as a value is handed back rather than folded: the
  // engine would read that property off a string and answer `undefined`, which
  // is a wrong declaration where a hand-back is merely a narrower one — the
  // dispatch below resolves the member this compiler's own way and folds it.
  //
  // Asked of the whole expression rather than of the read, because the read need
  // not be on the reference: `[colors][0].primary` reads a property off an
  // element, and nothing at that member says which value it will land on.
  if walk.carried_a_theme_reference() && walk.read_a_property_as_a_value() {
    return Err(Decline::NotACandidate);
  }

  let method = admitted.name();

  let source = print_fold(call, walk.parameters());

  ENGINE.with_borrow_mut(|slot| {
    // Taken, not borrowed in place. A panic unwinding out of the engine is
    // caught at the NAPI boundary and the process carries on, so an engine left
    // in the slot would be reused by every later fold with its VM stack
    // abandoned mid-frame. Taking it means an unwind leaves the slot empty and
    // the next fold builds a fresh engine; the abandoned one leaks, which is
    // what `ManuallyDrop` already makes it do at thread exit.
    let mut engine = match slot.take() {
      Some(engine) => engine,
      None => Engine::new()?,
    };

    let outward = Outward {
      method,
      depth: walk.guard.depth.restart(),
      ceilings: walk.guard.ceilings,
    };

    let applied = match admitted {
      Admitted::Global(global) => admit_an_applied_global(global, &mut engine.context),
      Admitted::Method(_) | Admitted::Named(_) => Ok(()),
    };

    let folded = applied
      .and_then(|()| walk.arguments(&mut engine.context, method))
      .and_then(|arguments| apply(&source, &arguments, &mut engine, outward))
      .and_then(|value| {
        // A theme reference crossed as a string, so an answer that is still an
        // object may *be* that reference — `Object(colors)` hands its argument
        // straight back — and a string standing where the group stood has lost
        // every member it had. Handed back rather than refused: the dispatch
        // below holds the reference itself and answers for it, where a refusal
        // here would fail a build it can compile.
        //
        // Read off the answer rather than predicted from the call, because what
        // a fold hands back is a property of the whole chain and not of the
        // method that ends it.
        if walk.carried_a_theme_reference() && value.is_object() {
          return Err(Decline::NotACandidate);
        }

        to_value(&value, &mut engine.context, outward)
      });

    *slot = Some(engine);

    folded
  })
}

/// Evaluates the printed expression and, where there is something to pass,
/// calls what it evaluated to with the transported values.
///
/// Two steps rather than one *when there is something to pass*, because the
/// values cross as arguments rather than as text. Every step can throw and all
/// of them are answered the same way: a throw is an answer, not a failure of
/// this module — the language throws on `[].reduce(f)` too — so the engine's own
/// sentence is what the author reads rather than a generic refusal standing in
/// for it.
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
/// Both arms go through the memo, because what is memoised is the compiled
/// script and neither arm has to be a function for that.
///
/// The compiled value is a function by construction, so the refusal for one that
/// is not stands in for a broken invariant rather than for anything an author can
/// write. It is a refusal all the same: this runs inside an evaluation whose
/// whole contract is that it may fail, where an assertion would abort a build
/// that a deopt would only leave to the runtime.
fn apply(
  source: &str,
  arguments: &[JsValue],
  engine: &mut Engine,
  outward: Outward,
) -> Result<JsValue, Decline> {
  let evaluated = engine.eval(source, outward.method)?;

  if arguments.is_empty() {
    return Ok(evaluated);
  }

  let Some(callable) = evaluated.as_callable() else {
    return Err(Decline::rule(uncallable_printed_fold(outward.method)));
  };

  callable
    .call(&JsValue::undefined(), arguments, &mut engine.context)
    .map_err(|error| threw(outward.method, &error))
}
