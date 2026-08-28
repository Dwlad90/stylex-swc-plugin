use super::growable_stack;
use super::*;
use stylex_constants::constants::evaluation_errors::expression_too_deep;

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  folded_once(path, state, traversal_state, |state, traversal_state| {
    _evaluate(path, state, traversal_state, fns)
  })
}

/// A value one level of a fold can answer with, and what the memo keeps of it.
///
/// Everything the memo stores is an [`EvaluateResultValue`], because that is
/// what the tree can hold. A caller whose own value carries more than that says
/// here how it crosses in each direction -- and, by saying it, accepts that the
/// extra does not survive a remembered answer.
pub(crate) trait Memoized: Sized {
  /// A remembered answer, read back as this caller's own value.
  fn from_memo(remembered: EvaluateResultValue) -> Self;

  /// This value, as the one the memo stores.
  fn to_memo(&self) -> EvaluateResultValue;
}

/// The evaluator's own value, which is what the memo holds, so both directions
/// are the identity.
impl Memoized for EvaluateResultValue {
  fn from_memo(remembered: Self) -> Self {
    remembered
  }

  fn to_memo(&self) -> Self {
    self.clone()
  }
}

/// One level of a fold, through the ceiling and through the memo.
///
/// [`evaluate_cached`] is this with the evaluator's own dispatch as the fold.
/// The binary-expression node passes a fold of its own, because what it folds a
/// `+` to carries a measurement no expression can hold — but the level it
/// costs, the memo it reads and writes, and the refusal it records all have to
/// stay one behaviour, and that is what lives here. A second copy of it would be
/// a second answer to "how deep is this", which is the one question a memo
/// keyed by structure alone cannot be asked twice.
///
/// [`Memoized`] is how a caller's value crosses into the memo and back. One
/// carrying more than the tree can hold — a measured string — loses the extra on
/// the way through and keeps it everywhere else, which is exactly the trade: the
/// memo answers for a subtree written twice, and the measurement for a subtree
/// grown once.
pub(crate) fn folded_once<T: Memoized>(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fold: impl FnOnce(&mut EvaluationState, &mut StateManager) -> Option<T>,
) -> Option<T> {
  descend_one_level(path, state, traversal_state, |state, traversal_state| {
    memoized_fold(path, state, traversal_state, fold)
  })
}

/// The outer half of [`folded_once`]: `descend` costs a level of the evaluation
/// ceiling, or `path` is refused because there are none left.
///
/// Split from the memo so the level is charged before the structural hash runs,
/// which is where the comment below starts.
fn descend_one_level<T>(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  descend: impl FnOnce(&mut EvaluationState, &mut StateManager) -> Option<T>,
) -> Option<T> {
  // Checked before the descent, which for the memo begins with a structural hash
  // over the same subtree -- the frame that would overflow if the budget were
  // checked after it.
  //
  // Returning here leaves *this* node out of `seen`, which is deliberate: the
  // memo is keyed by a structural hash that says nothing about the depth the
  // fold reached the node at, so recording "no" against the subtree that earned
  // it would answer for the same subtree written shallowly.
  //
  // The ancestors have to be left out for the same reason, which is what
  // `depth_refused` is for. Leaving them in was a bug: the entry a memo write
  // makes is post-order -- after `_evaluate` returns, not before it recurses --
  // so it is not the in-progress marker a cycle terminates on and nothing needs
  // it to be there. What it did instead was answer for a shallow reading of any
  // subtree the refusal happened inside, so a dynamic style deep enough to
  // refuse decided whether the *next* namespace folded, and property order
  // decided the emitted CSS.
  //
  // What the ceiling counts, given that, is the levels the fold *descends*, not
  // the levels an expression is written with: a hit answers without descending,
  // so a subtree already folded elsewhere costs one level rather than its own
  // height, and an expression can fold beside a sibling that warmed its inner
  // subtree where it would refuse alone. Charging a hit for the height it skips
  // was built and measured: the height a subtree records is the deepest the fold
  // went *anywhere* under it, which for the object of a member chain is the
  // whole object literal rather than the one member a read plucks out of it, so
  // the charge refused two of the member-chain boundaries this suite pins. Left
  // as it is because the asymmetry only ever folds *more* -- upstream has no
  // ceiling at all, so every case it decides differently is a case upstream
  // folds too, and no working input becomes a build error.
  //
  // The ceiling is read off the per-file options rather than threaded down the
  // recursion: every arm already carries the state manager, and a ceiling passed
  // through the arms instead would be a parameter each of them ignores. Where the
  // number comes from, and why the default is sized for hand-written styles
  // rather than for the deepest foldable input, is
  // `stylex_structures::evaluation_depth`.
  let ceiling = traversal_state.evaluation_ceiling();

  if traversal_state.evaluation_depth == 0 {
    // A new top-level fold. Whatever the previous one refused, its unwind is
    // over, so the frames of this one are free to record their own answers.
    traversal_state.depth_refused = false;
  }

  if traversal_state.evaluation_depth >= ceiling {
    traversal_state.depth_refused = true;

    // `deopt` answers with the evaluator's own value type, and this helper
    // answers with the caller's, so the refusal is recorded here and the `None`
    // returned separately.
    deopt(path, state, &expression_too_deep(ceiling));

    return None;
  }

  traversal_state.evaluation_depth += 1;

  let result = growable_stack::grown_per_level(|| descend(state, traversal_state));

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
fn memoized_fold<T: Memoized>(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fold: impl FnOnce(&mut EvaluationState, &mut StateManager) -> Option<T>,
) -> Option<T> {
  // Walks the whole remaining subtree, at every level, and is therefore nearly
  // all of what folding a deep expression costs -- the memo that exists to avoid
  // repeated work pays for the subtree to decide whether it can avoid it. Left
  // that way deliberately: the numbers are
  // `docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`, and the incremental
  // key that would remove the quadratic was built, measured 14-42% slower on
  // every real fixture, and not kept --
  // `docs/adr/0006-an-incremental-memo-key-was-built-and-measured-slower.md`.
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
        return evaluated_value.value.clone().map(T::from_memo);
      }

      deopt(path, state, PATH_WITHOUT_NODE);

      None
    },
    None => {
      let val = fold(state, traversal_state);

      if state.confident {
        let remembered = val.as_ref().map(T::to_memo);

        traversal_state
          .seen
          .entry(cleaned_path_hash)
          .or_insert_with(|| {
            Rc::new(SeenValue {
              value: remembered,
              resolved: true,
            })
          });
      } else if traversal_state.owns_its_refusals() {
        // Recorded so the same refusal is not re-walked, but only where the
        // refusal was about the subtree itself -- which is the one question
        // `owns_its_refusals` answers, and where the reasons it can answer `false`
        // are written down.
        traversal_state
          .seen
          .entry(cleaned_path_hash)
          .or_insert_with(|| {
            Rc::new(SeenValue {
              value: None,
              resolved: false,
            })
          });
      }

      val
    },
  }
}
