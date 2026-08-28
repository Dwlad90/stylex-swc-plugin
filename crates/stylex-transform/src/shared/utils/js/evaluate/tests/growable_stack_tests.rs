//! What the two ways of asking for stack actually give a walk.
//!
//! Neither is testable through the compiler at the size it is sized for: the
//! stages around the fold run out of room on an input long before the largest
//! configurable ceiling is reached, so a case written as source measures those
//! stages rather than these claims. And under-sizing either one does not report
//! — it aborts the process — so a suite that never asked for the largest claim
//! would be finding out on somebody's build.
//!
//! So the recursion here is written rather than parsed: a frame of a known size,
//! repeated a known number of times, is the only thing that can be pointed at a
//! specific number of bytes.

use stylex_structures::evaluation_depth::MAX_EVALUATION_DEPTH_LIMIT;

use super::growable_stack::{claim_for, grown_for_depth, grown_per_level};
use super::source_evaluation::on_a_thread_of;

/// What one written level costs, and comfortably more than a level of any walk
/// the fold runs — the point is to spend measurable stack, not to imitate one.
const FRAME: usize = 16 * 1024;

/// A thread small enough that every case below has to be given room it did not
/// start with. Under a megabyte the runtime's own guard page arithmetic starts
/// to matter, so this is the smallest honest floor rather than the smallest.
const SMALL_THREAD: usize = 1024 * 1024;

/// Which of the two subjects a descent is.
#[derive(Clone, Copy)]
enum Asking {
  /// Room asked for at every level, as every walk this compiler owns does.
  AtEveryLevel,
  /// Room asked for once and never again, as the engine's parser does — the
  /// caller the claim exists for.
  Never,
}

/// Recurses `levels` deep, keeping a [`FRAME`] alive across each call, and
/// answers how deep it went.
fn descend(levels: usize, asking: Asking) -> usize {
  let mut frame = [0u8; FRAME];

  frame[0] = levels as u8;

  if levels == 0 {
    return std::hint::black_box(&frame)[0] as usize;
  }

  let deeper = match asking {
    Asking::AtEveryLevel => grown_per_level(|| descend(levels - 1, asking)),
    Asking::Never => descend(levels - 1, asking),
  };

  std::hint::black_box(&frame);

  deeper + 1
}

/// A walk that asks at every level runs far past the thread it started on.
///
/// Eight megabytes of frames on a one-megabyte thread: without the growth this
/// is an abort rather than a failure, which is why it is asserted at a size the
/// thread plainly cannot hold rather than just over its edge.
#[test]
fn asking_by_the_level_carries_a_walk_past_the_thread_it_started_on() {
  let levels = 8 * 1024 * 1024 / FRAME;

  assert_eq!(
    on_a_thread_of(SMALL_THREAD, move || descend(levels, Asking::AtEveryLevel)),
    levels
  );
}

/// A descent that never asks again gets what was claimed for it, which is the
/// whole point of claiming: the frames below the claim are spent without a
/// single further check.
#[test]
fn a_claim_carries_a_descent_that_never_asks_again() {
  // Two levels of written frame per level claimed, since a claimed level is
  // sized for the engine's parser and this frame is not it.
  let ceiling = 128;
  let levels = ceiling * 2;

  assert_eq!(
    on_a_thread_of(SMALL_THREAD, move || grown_for_depth(ceiling, || descend(
      levels,
      Asking::Never
    ))),
    levels
  );
}

/// The largest claim the option surface can ask for can actually be made.
///
/// The ceiling is clamped to this number precisely so that the claim behind it
/// is one an allocation can satisfy, and nothing else checks that it is: a claim
/// that could not be mapped fails inside an evaluation, at whatever depth some
/// project configured, rather than here.
#[test]
fn the_largest_ceiling_a_project_can_configure_can_be_claimed() {
  let room = on_a_thread_of(SMALL_THREAD, || {
    grown_for_depth(MAX_EVALUATION_DEPTH_LIMIT, stacker::remaining_stack)
  });

  let claimed = claim_for(MAX_EVALUATION_DEPTH_LIMIT);

  // Nothing to assert where the platform does not answer how much is left. The
  // claim was still made — reaching here at all means it was satisfied.
  //
  // Read from inside the claim, so the frames between the boundary and the
  // reading are already spent: what is asserted is that the whole claim is
  // underfoot but for those.
  if let Some(bytes) = room {
    assert!(
      bytes + 4096 >= claimed,
      "the largest claim left {} bytes underfoot rather than {}",
      bytes,
      claimed
    );
  }
}

/// Nothing is claimed when the room is already there, which is what keeps a fold
/// nested inside another fold from claiming twice.
#[test]
fn a_claim_that_is_already_covered_allocates_nothing() {
  let ceiling = 8;

  let (before, inside) = on_a_thread_of(SMALL_THREAD, move || {
    let before = stacker::remaining_stack();
    let inside = grown_for_depth(ceiling, stacker::remaining_stack);

    (before, inside)
  });

  // A megabyte thread already holds a claim of half of one, so the walk stays
  // where it was: what is left underfoot shrinks only by the frames between the
  // two readings. Asserted as a near-equality rather than as "no more than
  // before", since a claim that *did* allocate would also leave less.
  if let (Some(before), Some(inside)) = (before, inside) {
    assert!(
      inside <= before && before - inside < FRAME,
      "a covered claim moved the walk onto {} bytes from {}",
      inside,
      before
    );
  }
}

/// A panic crosses both kinds of grown stack with its payload intact.
///
/// Load-bearing rather than incidental: a refusal in a position that requires a
/// static value is reported by panicking with a StyleX diagnostic, so a payload
/// that did not survive the boundary would turn every deep refusal into a panic
/// nobody can read.
#[test]
fn a_panic_crosses_a_grown_stack_with_its_payload() {
  for way in ["per level", "claimed"] {
    let raised = std::panic::catch_unwind(|| match way {
      "per level" => grown_per_level(|| panic!("{}", way)),
      _ => grown_for_depth(MAX_EVALUATION_DEPTH_LIMIT, || panic!("{}", way)),
    });

    let payload = raised.expect_err("the panic did not cross the boundary");

    assert_eq!(
      payload.downcast_ref::<String>().map(String::as_str),
      Some(way)
    );
  }
}
