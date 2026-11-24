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
//! so the engine can fold it. Where the engine declines it again, the dispatch
//! continues on this call, so no part is evaluated a second time.

use super::super::*;
use crate::growable_stack::grown_per_level;
use smallvec::SmallVec;
use stylex_state::functions::FunctionConfig;
use swc_core::ecma::{
  ast::{CallExpr, MemberExpr, Stmt},
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
  // With no function in the map, no part can call one.
  if state.functions.identifiers.is_empty() && state.functions.member_expressions.is_empty() {
    return None;
  }

  let callee = call.callee.as_expr()?;

  // A StyleX call evaluates its own arguments, so it has nothing to fold here.
  if is_a_compiler_function(callee, state) {
    return None;
  }

  // The parts are searched once, before anything is copied, so a call with no
  // StyleX call in it costs one search of its parts and copies nothing.
  let receiver = callee.as_member().map(|member| &*member.obj);
  let holding = receiver
    .into_iter()
    .chain(call.args.iter().map(|arg| &*arg.expr))
    .map(|part| holds_a_compiler_call(part, state))
    .collect::<SmallVec<[bool; 8]>>();

  if !holding.contains(&true) {
    return None;
  }

  let mut folded = call.clone();
  let receiver = folded
    .callee
    .as_mut_expr()
    .and_then(|callee| callee.as_mut_member())
    .map(|member| &mut member.obj);

  let parts = receiver
    .into_iter()
    .chain(folded.args.iter_mut().map(|arg| &mut arg.expr));

  for (part, holds_one) in parts.zip(holding) {
    if holds_one {
      fold_part(part, state, traversal_state, fns);
    }
  }

  state.confident.then_some(folded)
}

/// Writes `part`, which holds a StyleX call, in as its value.
///
/// Once a part has refused, nothing after it is evaluated. A value with no
/// expression form, such as a callback, leaves the part as it was written.
fn fold_part(
  part: &mut Box<Expr>,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) {
  if !state.confident {
    return;
  }

  let value = evaluate_cached(part, state, traversal_state, fns);

  if let Some(value) = value.as_ref().and_then(evaluate_result_as_expr) {
    **part = value;
  }
}

/// Whether `callee` reaches one of the compiler's own functions: a name the
/// function map holds as a function of the compiler, or a member read that
/// starts from one, as in `stylex.keyframes` or `stylex.when.ancestor`.
///
/// A name the map holds as a plain value, as a callback binds its parameters,
/// is not one.
fn is_a_compiler_function(callee: &Expr, state: &EvaluationState) -> bool {
  let functions = &state.functions;
  let mut root = normalize_expr(callee);
  let mut property = None;

  while let Expr::Member(MemberExpr { obj, prop, .. }) = root {
    property = Some(prop);
    root = normalize_expr(obj);
  }

  let Some(name) = root.as_ident() else {
    return false;
  };

  let named = functions
    .identifiers
    .get(&name.sym)
    .is_some_and(|entry| is_a_compiler_entry(entry));

  // The member entries are keyed by an owned name. A module imports StyleX
  // under one or two names, so they are scanned rather than looked up, which
  // builds no key for each call the search visits.
  named
    || property.is_some_and(|property| {
      functions
        .member_expressions
        .iter()
        .any(|(source, members)| {
          // A computed property names no entry this can read, so the import
          // alone answers for it.
          matches!(source, ImportSources::Regular(import) if import == name.sym.as_str())
            && property.as_ident().is_none_or(|property| {
              members
                .get(&property.sym)
                .is_some_and(|entry| is_a_compiler_entry(entry))
            })
        })
    })
}

/// Whether `entry` is a function of the compiler, and not a plain value that a
/// callback bound to a parameter.
fn is_a_compiler_entry(entry: &FunctionConfigType) -> bool {
  !matches!(
    entry,
    FunctionConfigType::Regular(FunctionConfig {
      fn_ptr: FunctionType::Mapper(_) | FunctionType::ThemeRefMapper(_) | FunctionType::Callback(_),
      ..
    })
  )
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
///
/// It asks for room at each level, as every walk of this crate does, so a deep
/// part cannot overflow the stack.
struct Search<'s> {
  state: &'s EvaluationState,
  found: bool,
}

impl Visit for Search<'_> {
  fn visit_expr(&mut self, expr: &Expr) {
    grown_per_level(|| expr.visit_children_with(self));
  }

  fn visit_stmt(&mut self, stmt: &Stmt) {
    grown_per_level(|| stmt.visit_children_with(self));
  }

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

#[cfg(test)]
#[path = "tests/compiler_calls_tests.rs"]
mod compiler_calls_tests;
