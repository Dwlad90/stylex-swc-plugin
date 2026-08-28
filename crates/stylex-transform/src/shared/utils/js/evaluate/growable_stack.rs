//! The stack every walk that recurses on the bare thread stack runs on.
//!
//! Several descents recurse without a frame budget of their own: the evaluator's
//! own, the guard's walk into an expression on its way to the engine, the
//! carriage of a resolved value inward, the conversion of an engine value back,
//! the printing of the source the engine is handed, and the engine's parse of
//! it. Overflowing any of them aborts the process from inside an evaluation
//! whose whole contract is that it may fail, so none of them may run on whatever
//! the thread happened to have left over.
//!
//! Two ways of asking, and one rule that decides which a descent gets: **a
//! descent that can ask again at the next level does**. Every walk this compiler
//! owns can, so each spends a level and claims room for the one after it, and a
//! walk that stops early pays for the levels it actually descended. The two that
//! cannot are the ones this compiler does not write: SWC's printer clones the
//! expression and writes it out, and the engine's parser reads it back, and both
//! descend through a nested literal on whatever stack they were handed. So the
//! fold claims their whole descent up front, around them and around nothing
//! else.

use stylex_structures::evaluation_depth::MAX_EVALUATION_DEPTH_LIMIT;

/// Room to keep in front of a walk that asks again at the next level.
///
/// One level is not one frame and the frames are not the same size: a nested
/// `Math.max` call descends through argument collection and the callee
/// dispatch, and a debug build keeps every local of a long arm alive across the
/// recursive call. Measured against the most expensive arm, a debug level costs
/// tens of kilobytes, so the zone is sized in megabytes rather than in the
/// hundreds of kilobytes a uniform walk would need.
const HEADROOM_PER_LEVEL: usize = 1024 * 1024;

/// How much stack a walk that asks by the level is given when it is grown.
///
/// Sized to carry a few hundred levels of the most expensive arm in a debug
/// build in a single segment, so even a ceiling raised well past the default
/// allocates once rather than repeatedly.
const SEGMENT: usize = 16 * 1024 * 1024;

/// How much stack one nesting level of the print, the parse and the evaluation
/// is given.
///
/// Only those, because only those run on this claim — every walk this compiler
/// owns asks for its own room, and the print has unwound before the parse
/// begins, so what is sized here is the deeper of the two rather than the sum.
/// Measured on a debug build, which is the expensive one, by claiming nothing
/// and shrinking the thread instead: sixty-four megabytes carry three thousand
/// three hundred levels of nested array literal and not three thousand five
/// hundred, so a level costs about twenty kilobytes. Sixty-four kilobytes is
/// three times that, which is the margin between a ceiling that refuses and a
/// ceiling that aborts.
const BYTES_PER_LEVEL: usize = 64 * 1024;

/// How much deeper than the ceiling the printed source may nest.
///
/// The claim is made from the ceiling, and the ceiling bounds the **walk**
/// rather than the text. An operand a short circuit never reaches is printed
/// without being walked — the engine decides the short circuit itself — so the
/// printer and the parser both descend through nesting the guard never spent a
/// level on. Two is what the deepest such shape in the suite comes to at the
/// shipped ceiling, and it is a margin rather than a bound: nothing in the guard
/// stops a dead operand nesting deeper still.
///
/// Multiplied into the claim rather than left to the margin on
/// [`BYTES_PER_LEVEL`], so the room is there by construction and stays there if
/// either measurement moves.
const UNWALKED_NESTING: usize = 2;

/// The largest claim [`grown_for_depth`] will ever make.
///
/// It is a consequence rather than a setting: the deepest ceiling a caller can
/// reach, at the cost of a level, for as deep as the text of it can nest.
/// Asserted rather than described, because the numbers it multiplies live in two
/// crates and the sentence that says what they come to is not something a
/// compiler checks.
const LARGEST_CLAIM: usize = 1024 * 1024 * 1024;

const _: () =
  assert!(MAX_EVALUATION_DEPTH_LIMIT * UNWALKED_NESTING * BYTES_PER_LEVEL == LARGEST_CLAIM);

/// Runs `work` with room for a few more levels, for a caller that will ask
/// again at the next one.
///
/// A panic unwinding out of `work` — a StyleX diagnostic, which is how a
/// refusal in a position requiring a static value is reported — crosses the
/// boundary safely: `stacker` catches it on the grown stack and resumes the
/// unwind on the original one, so the payload the caller matches on survives.
pub(crate) fn grown_per_level<R>(work: impl FnOnce() -> R) -> R {
  stacker::maybe_grow(HEADROOM_PER_LEVEL, SEGMENT, work)
}

/// Runs `work` with room for a descent `ceiling` levels deep, for a caller that
/// will not ask again.
///
/// SWC's printer and the engine's parser are those callers: each recurses
/// through a nested literal without ever asking for room, so each gets only what
/// was there when it started. Claiming the whole descent up front is what turns "however
/// much this thread happened to have left" into a number, and claiming in
/// proportion to the configured ceiling is what ties that number to it: a
/// project that raises the depth gets the stack its new depth needs, and one
/// that leaves the default alone claims a couple of megabytes it almost always
/// already has, so nothing is allocated at all.
///
/// `ceiling` is a ceiling as
/// [`StateManager::evaluation_ceiling`](crate::shared::structures::state_manager::StateManager::evaluation_ceiling)
/// answers it, which is where it is clamped to something a stack can be claimed
/// for. Nothing is allocated when a segment that size is already underfoot,
/// which is what keeps a fold nested inside another fold from claiming twice.
///
/// **What it costs, and who pays.** Only a call the guard admitted: the claim is
/// made after the walk and around the printing and the engine's own work, so a
/// call expression declined for any reason never reaches it. That matters because the guard runs
/// on every call expression the evaluator visits and almost none of them fold —
/// wrapped around the walk as well, this mapped and unmapped a segment per
/// declined call, on a file that folds nothing at all. What is left is a cost on
/// folds, which is where the fold's cost belongs, and it buys the thing the
/// ceiling exists for: crossing it is a diagnostic instead of an abort.
pub(crate) fn grown_for_depth<R>(ceiling: usize, work: impl FnOnce() -> R) -> R {
  let claim = claim_for(ceiling);

  stacker::maybe_grow(claim, claim, work)
}

/// The stack [`grown_for_depth`] asks for at `ceiling`.
///
/// Named rather than written into the call, so what the claim comes to can be
/// asserted against instead of restated — a test that spelled the arithmetic out
/// again would agree with itself whichever number was wrong.
pub(crate) fn claim_for(ceiling: usize) -> usize {
  ceiling * UNWALKED_NESTING * BYTES_PER_LEVEL
}
