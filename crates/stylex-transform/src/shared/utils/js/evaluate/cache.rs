use super::*;
use stylex_constants::constants::evaluation_errors::expression_too_deep;

/// How many expression levels the fold will descend before refusing.
///
/// The evaluator walks a nested expression recursively, so with no bound of its
/// own its real limit is the thread's stack: past it the process aborts with
/// `fatal runtime error: stack overflow`, which gives a bundler no message, no
/// file, and no chance to finish the rest of the build. This is the bound, and
/// crossing it is an ordinary refusal.
///
/// The value is configuration, not a stack measurement -- [`STACK_SIZE`] is what
/// keeps the stack out of the way, so the ceiling does not have to be whatever a
/// 2 MiB thread happened to survive. Where it comes from, and why the default is
/// sized for hand-written styles rather than for the deepest foldable input, is
/// [`stylex_structures::evaluation_depth`].
///
/// Read off the per-file options rather than passed down the recursion: every
/// arm already carries the state manager, and a ceiling threaded through the
/// arms instead would be a parameter each of them ignores.
#[inline]
fn max_evaluation_depth(traversal_state: &StateManager) -> usize {
  traversal_state.options.max_evaluation_depth
}

/// Grow the stack when less than this is left.
///
/// One level of the fold is not one frame and the arms are not the same size:
/// a nested `Math.max` call descends through argument collection and the callee
/// dispatch, and a debug build keeps every local of a long arm alive across the
/// recursive call. Measured against the most expensive arm, a debug level costs
/// tens of kilobytes, so the zone is sized in megabytes rather than in the
/// hundreds of kilobytes a uniform walk would need.
const RED_ZONE: usize = 1024 * 1024;

/// How much stack to allocate when the red zone is reached.
///
/// Sized to carry a few hundred levels of the most expensive arm in a debug
/// build in a single segment, so even a ceiling raised well past the default
/// allocates once rather than repeatedly. Nothing is allocated for an expression
/// that stays clear of the red zone, which is every expression an author
/// writes.
const STACK_SIZE: usize = 16 * 1024 * 1024;

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // Checked before the structural hash below, which recurses over the same
  // subtree and would be the frame that overflowed if the budget were checked
  // after it. Returning here also leaves `seen` untouched, deliberately: the
  // memo is keyed by the expression's structural hash, which says nothing about
  // the depth the fold happened to reach it at, so this is the one refusal that
  // must not be recorded against the subtree that earned it.
  let ceiling = max_evaluation_depth(traversal_state);

  if traversal_state.evaluation_depth >= ceiling {
    return deopt(path, state, &expression_too_deep(ceiling));
  }

  traversal_state.evaluation_depth += 1;

  // A panic unwinding out of the fold -- a StyleX diagnostic, which is how a
  // refusal in a position requiring a static value is reported -- crosses this
  // boundary safely: `stacker` catches it on the grown stack and resumes the
  // unwind on the original one, so the payload the caller matches on survives.
  let result = stacker::maybe_grow(RED_ZONE, STACK_SIZE, || {
    evaluate_cached_within_budget(path, state, traversal_state, fns)
  });

  traversal_state.evaluation_depth -= 1;

  result
}

/// The fold itself, one level down from the budget check.
///
/// Split out so the increment and the decrement bracket every return path
/// without a guard type: the counter borrows `traversal_state` mutably, which
/// the fold needs for itself, so a `Drop` implementation holding that borrow
/// could not coexist with the call it is guarding. A panic unwinding past the
/// decrement leaves the counter high, which costs nothing -- the panic ends the
/// file's transformation, and the state does not outlive it.
fn evaluate_cached_within_budget(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let cleaned_path_hash = stable_hash_unspanned(path);

  let existing = traversal_state.seen.get(&cleaned_path_hash);

  match existing {
    Some(evaluate_value) => {
      let evaluated_value: &SeenValue = evaluate_value.borrow();

      if evaluated_value.resolved {
        return evaluated_value.value.clone();
      }

      deopt(path, state, PATH_WITHOUT_NODE)
    },
    None => {
      let val = _evaluate(path, state, traversal_state, fns);

      let seen_value = if state.confident {
        SeenValue {
          value: val.clone(),
          resolved: true,
        }
      } else {
        SeenValue {
          value: None,
          resolved: false,
        }
      };

      traversal_state
        .seen
        .entry(cleaned_path_hash)
        .or_insert_with(|| Rc::new(seen_value));

      val
    },
  }
}
