use super::*;
use stylex_constants::constants::evaluation_errors::expression_too_deep;

/// Grow the stack when less than this is left.
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
/// that stays clear of the red zone, which is everything under the default
/// ceiling.
const STACK_SIZE: usize = 16 * 1024 * 1024;

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // Checked before the structural hash below, which recurses over the same
  // subtree and would be the frame that overflowed if the budget were checked
  // after it.
  //
  // Returning here leaves *this* node out of `seen`, which is deliberate: the
  // memo is keyed by a structural hash that says nothing about the depth the
  // fold reached the node at, so recording "no" against the subtree that earned
  // it would answer for the same subtree written shallowly. The ancestors above
  // it are still marked unresolved, exactly as they are for any other refusal --
  // that is the in-progress marker cycles terminate on, and a depth refusal is
  // not special enough to change it.
  // The ceiling is read off the per-file options rather than threaded down the
  // recursion: every arm already carries the state manager, and a ceiling passed
  // through the arms instead would be a parameter each of them ignores. Where the
  // number comes from, and why the default is sized for hand-written styles
  // rather than for the deepest foldable input, is
  // `stylex_structures::evaluation_depth`.
  let ceiling = traversal_state.options.max_evaluation_depth;

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
  // Walks the whole remaining subtree, at every level, and is therefore nearly
  // all of what folding a deep expression costs -- the memo that exists to avoid
  // repeated work pays for the subtree to decide whether it can avoid it. Left
  // that way deliberately: the numbers, and why an incremental key is a
  // correctness question rather than a refactor, are
  // `docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`.
  //
  // The lookup below acts on the key *alone* -- no `eq_ignore_span` confirms it,
  // unlike the JSX-spread bucket -- so the key's width is what stands between a
  // collision and a wrong folded value. It is 128 bits for that reason. A
  // confirm here would cost a subtree compare on every hit and a stored clone of
  // every memoized expression, which is the trade the ADR records rejecting.
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
