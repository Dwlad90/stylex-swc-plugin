//! The stack every walk that recurses on the bare thread stack runs on.
//!
//! Three walks recurse without a frame budget of their own: the evaluator's
//! descent, the guard's walk into an expression on its way to the engine, and
//! the conversion of an engine value back. Overflowing any of them aborts the
//! process from inside an evaluation whose whole contract is that it may fail,
//! so none of them may run on whatever the thread happened to have left over.
//!
//! Two ways of asking, because the walks differ in one way that matters. The
//! evaluator asks again at every level, so it only ever needs room for the next
//! few. The engine's parser asks once and never again — it descends through a
//! nested literal on whatever stack it was handed — so the fold has to claim
//! room for its whole descent before it starts.

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

/// How much stack one level of a fold is given.
///
/// Measured on a debug build, which is the expensive one: the guard's walk and
/// the conversion back cost about four kilobytes a level, and the engine's
/// parse of the printed source costs most of the rest — eight hundred levels of
/// nested array literal fold inside sixteen megabytes and do not fold inside
/// eight. Sixty-four kilobytes is roughly three times the measurement, which is
/// the margin between a ceiling that refuses and a ceiling that aborts.
const BYTES_PER_LEVEL: usize = 64 * 1024;

/// How many walks can be inside one another when the ceiling is spent.
///
/// Two. The guard walks the expression, and where it reaches a name it walks
/// the *value* that name resolved to — a walk of its own, measured against the
/// whole ceiling because the value is not nested inside the expression that
/// read it. Its frames sit on top of the guard's, so a fold is deepest with one
/// of each fully spent. The conversion out is the third walk and does not
/// count: it runs after the guard's has unwound, as does the engine's parse of
/// the printed source.
///
/// Multiplied into the claim rather than left to the margin on
/// [`BYTES_PER_LEVEL`], so the room is there by construction and stays there if
/// either measurement moves.
const DEEPEST_NESTED_WALKS: usize = 2;

/// The largest claim [`grown_for_depth`] will ever make.
///
/// It is a consequence rather than a setting: the deepest ceiling a caller can
/// reach, at the cost of a level, for as many walks as can nest. Asserted
/// rather than described, because the numbers it multiplies live in two crates
/// and the sentence that says what they come to is not something a compiler
/// checks.
const LARGEST_CLAIM: usize = 1024 * 1024 * 1024;

const _: () =
  assert!(MAX_EVALUATION_DEPTH_LIMIT * DEEPEST_NESTED_WALKS * BYTES_PER_LEVEL == LARGEST_CLAIM);

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
/// The engine's parser is such a caller: it recurses through a nested literal
/// without ever asking for room, so it gets only what was there when it
/// started. Claiming the whole descent up front is what turns "however much
/// this thread happened to have left" into a number, and claiming exactly the
/// configured ceiling is what lets that number *be* the configured ceiling: a
/// project that raises the depth gets the stack its new depth needs, and one
/// that leaves the default alone claims a couple of megabytes it almost always
/// already has, so nothing is allocated at all.
///
/// `ceiling` is a ceiling as
/// [`StateManager::evaluation_ceiling`](crate::shared::structures::state_manager::StateManager::evaluation_ceiling)
/// answers it, which is where it is clamped to something a stack can be claimed
/// for. Nothing is allocated when a segment that size is already underfoot,
/// which is what keeps a fold nested inside another fold from claiming twice.
/// **What it costs.** At the shipped ceiling the claim is a few megabytes, which
/// a thread deep in a transform often does not have left — so a fold on such a
/// thread maps a segment and unmaps it again, measured at 4-6% of a fold on the
/// `engine_fold` benchmarks, and at nothing where the room was already there.
/// Paid rather than avoided: what it buys is that crossing the ceiling is a
/// diagnostic instead of an abort, and every way of paying it less often —
/// claiming once per evaluation, or growing the evaluator's own segment sooner
/// — moves the cost onto files that fold nothing at all.
pub(crate) fn grown_for_depth<R>(ceiling: usize, work: impl FnOnce() -> R) -> R {
  let claim = ceiling * DEEPEST_NESTED_WALKS * BYTES_PER_LEVEL;

  stacker::maybe_grow(claim, claim, work)
}
