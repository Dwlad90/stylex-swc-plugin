//! StyleX calls inside a call the engine fold declined.
//!
//! The engine runs a compiler function such as `stylex.keyframes` in most
//! positions (see `engine_fold`). It still declines some calls that hold one:
//! a function of the map that it may not run, an argument it cannot carry, or
//! a part beside the call that it cannot carry, as in
//! `[stylex.keyframes(...) || runtimeValue].join(', ')`. Each part of such a
//! call can still have a value.
//!
//! Here the receiver and each argument that holds such a call are evaluated
//! first, as the parts of any call are evaluated before the method runs, and
//! each is written in as its value. A part is evaluated whole, so a conditional
//! computes only the branch it takes. The call that is left holds only values,
//! so the engine can fold it.

use super::super::*;
use swc_core::ecma::{
  ast::{CallExpr, MemberExpr},
  visit::{Visit, VisitWith},
};

/// `call` with its receiver and each argument that holds a StyleX call replaced
/// by its value, or `None` when no part holds one.
///
/// A part that cannot be evaluated stops the work and leaves its refusal in
/// `state`, and the answer is then `None` too.
pub(super) fn with_compiler_calls_folded(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<CallExpr> {
  let callee = call.callee.as_expr()?;

  // A StyleX call evaluates its own arguments, so it has nothing to fold here.
  if is_a_compiler_function(callee, state) {
    return None;
  }

  // The parts are searched before anything is copied, so a call with no StyleX
  // call in it costs one search of its parts and copies nothing. A part that
  // holds one is searched again below, where it is folded.
  let receiver = callee.as_member().map(|member| &*member.obj);
  let holds_one = receiver
    .into_iter()
    .chain(call.args.iter().map(|arg| &*arg.expr))
    .any(|part| holds_a_compiler_call(part, state));

  if !holds_one {
    return None;
  }

  let mut folded = call.clone();
  let receiver = folded
    .callee
    .as_mut_expr()
    .and_then(|callee| callee.as_mut_member())
    .map(|member| &mut member.obj);

  for part in receiver
    .into_iter()
    .chain(folded.args.iter_mut().map(|arg| &mut arg.expr))
  {
    fold_part(part, state, traversal_state, fns);
  }

  state.confident.then_some(folded)
}

/// Writes `part` in as its value when it holds a StyleX call.
///
/// Once a part has refused, nothing after it is evaluated. A value with no
/// expression form, such as a callback, leaves the part as it was written.
fn fold_part(
  part: &mut Box<Expr>,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) {
  if !state.confident || !holds_a_compiler_call(part, state) {
    return;
  }

  let value = evaluate_cached(part, state, traversal_state, fns);

  if let Some(value) = value.as_ref().and_then(evaluate_result_as_expr) {
    **part = value;
  }
}

/// Whether `callee` reaches one of the compiler's own functions: a name the
/// function map holds, or a member read that starts from one, as in
/// `stylex.keyframes` or `stylex.when.ancestor`.
fn is_a_compiler_function(callee: &Expr, state: &EvaluationState) -> bool {
  let mut root = normalize_expr(callee);

  while let Expr::Member(MemberExpr { obj, .. }) = root {
    root = normalize_expr(obj);
  }

  // The member entries are keyed by an owned name. A module imports StyleX
  // under one or two names, so they are scanned rather than looked up, which
  // builds no key for each call the search visits.
  root.as_ident().is_some_and(|name| {
    state.functions.identifiers.contains_key(&name.sym)
      || state.functions.member_expressions.keys().any(
        |source| matches!(source, ImportSources::Regular(import) if import == name.sym.as_str()),
      )
  })
}

/// Whether `part` calls one of the compiler's own functions anywhere in it.
fn holds_a_compiler_call(part: &Expr, state: &EvaluationState) -> bool {
  let mut search = Search {
    state,
    found: false,
  };

  part.visit_with(&mut search);

  search.found
}

/// The walk of [`holds_a_compiler_call`].
struct Search<'s> {
  state: &'s EvaluationState,
  found: bool,
}

impl Visit for Search<'_> {
  fn visit_call_expr(&mut self, call: &CallExpr) {
    self.found |= call
      .callee
      .as_expr()
      .is_some_and(|callee| is_a_compiler_function(callee, self.state));

    if !self.found {
      call.visit_children_with(self);
    }
  }
}
