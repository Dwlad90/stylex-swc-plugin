#[cfg(test)]
mod candidate_index {
  use std::cell::Cell;

  use crate::shared::structures::candidate_index::CandidateIndex;

  /// A key the tests can spell without building an expression, since the index
  /// never derives one itself.
  const KEY: u128 = 0x1234_5678_9abc_def0_1234_5678_9abc_def0;
  const OTHER_KEY: u128 = 1;

  #[test]
  fn hands_back_nothing_before_anything_is_recorded() {
    let index: CandidateIndex<u128, usize> = CandidateIndex::default();

    assert!(index.candidates(|| KEY).is_empty());
  }

  #[test]
  fn hands_back_the_handles_recorded_under_a_key() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 7usize);
    index.record(KEY, 9);

    assert_eq!(index.candidates(|| KEY), [7, 9]);
  }

  #[test]
  fn keeps_the_order_handles_were_recorded_in() {
    let mut index = CandidateIndex::default();

    for handle in [9usize, 3, 5] {
      index.record(KEY, handle);
    }

    assert_eq!(index.candidates(|| KEY), [9, 3, 5]);
  }

  #[test]
  fn records_a_handle_once_however_often_it_is_offered() {
    let mut index = CandidateIndex::default();

    for _ in 0..100 {
      index.record(KEY, 4usize);
    }

    assert_eq!(index.candidates(|| KEY), [4]);
  }

  #[test]
  fn keeps_keys_apart() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.record(OTHER_KEY, 2);

    assert_eq!(index.candidates(|| KEY), [1]);
    assert_eq!(index.candidates(|| OTHER_KEY), [2]);
  }

  #[test]
  fn forgetting_drops_only_the_handle_named() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.record(KEY, 2);
    index.forget(&KEY, &1);

    assert_eq!(index.candidates(|| KEY), [2]);
  }

  #[test]
  fn forgetting_the_last_handle_empties_the_index() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.forget(&KEY, &1);

    // Not merely absent from its bucket: an index that kept an empty bucket
    // would stop answering for free, which is the whole of what the empty case
    // buys.
    assert!(
      index
        .candidates(|| panic!("an empty index must not compute a key"))
        .is_empty()
    );
  }

  #[test]
  fn forgetting_what_was_never_recorded_changes_nothing() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.forget(&KEY, &2);
    index.forget(&OTHER_KEY, &1);

    assert_eq!(index.candidates(|| KEY), [1]);
  }

  #[test]
  fn a_handle_can_be_recorded_again_after_being_forgotten() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.forget(&KEY, &1);
    index.record(OTHER_KEY, 1);

    assert!(index.candidates(|| KEY).is_empty());
    assert_eq!(index.candidates(|| OTHER_KEY), [1]);
  }

  #[test]
  fn computes_no_key_while_the_index_holds_nothing() {
    let index: CandidateIndex<u128, usize> = CandidateIndex::default();
    let computed = Cell::new(0);

    index.candidates(|| {
      computed.set(computed.get() + 1);
      KEY
    });

    assert_eq!(computed.get(), 0);
  }

  #[test]
  fn computes_the_key_once_when_the_index_holds_something() {
    let mut index = CandidateIndex::default();
    let computed = Cell::new(0);

    index.record(OTHER_KEY, 1usize);
    index.candidates(|| {
      computed.set(computed.get() + 1);
      KEY
    });

    assert_eq!(computed.get(), 1);
  }

  #[test]
  fn holds_a_name_as_readily_as_a_position() {
    let mut index = CandidateIndex::default();

    index.record(KEY, String::from("styles"));
    index.record(KEY, String::from("other"));
    index.forget(&KEY, &String::from("styles"));

    assert_eq!(index.candidates(|| KEY), [String::from("other")]);
  }

  /// A key that is not a hash at all, which is what the declaration and
  /// top-level-name indexes are keyed by.
  #[test]
  fn narrows_on_a_key_that_is_not_a_hash() {
    let mut index: CandidateIndex<&str, usize> = CandidateIndex::default();

    index.record("styles", 0);
    index.record("styles", 4);
    index.record("theme", 1);

    assert_eq!(index.candidates(|| "styles"), [0, 4]);
    assert_eq!(index.candidates(|| "theme"), [1]);
    assert!(index.candidates(|| "absent").is_empty());
  }

  #[test]
  fn moving_an_entry_forgets_the_key_it_left_and_records_the_one_it_joined() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.move_entry(Some(KEY), Some(OTHER_KEY), 1);

    assert!(index.candidates(|| KEY).is_empty());
    assert_eq!(index.candidates(|| OTHER_KEY), [1]);
  }

  /// The ordering the method exists to encode: forgetting runs first, so an
  /// entry that moves to the key it already held keeps its record.
  #[test]
  fn moving_an_entry_to_the_key_it_already_held_keeps_it() {
    let mut index = CandidateIndex::default();

    index.record(KEY, 1usize);
    index.move_entry(Some(KEY), Some(KEY), 1);

    assert_eq!(index.candidates(|| KEY), [1]);
  }

  #[test]
  fn moving_an_entry_from_or_to_nothing_touches_only_the_side_given() {
    let mut index = CandidateIndex::default();

    // An entry that had no key joins one.
    index.move_entry(None, Some(KEY), 1usize);
    assert_eq!(index.candidates(|| KEY), [1]);

    // And one that gains no key leaves the one it had.
    index.move_entry(Some(KEY), None, 1);
    assert!(index.candidates(|| KEY).is_empty());

    // Neither side is a no-op rather than a panic.
    index.move_entry(None, None, 1);
    assert!(index.candidates(|| KEY).is_empty());
  }

  /// Far past any module, to show the bucket that answers is the one the key
  /// names rather than a walk of everything recorded.
  #[test]
  fn stays_exact_across_a_hundred_thousand_keys() {
    let mut index = CandidateIndex::default();

    for handle in 0..100_000u128 {
      index.record(handle, handle as usize);
    }

    assert_eq!(index.candidates(|| 0), [0]);
    assert_eq!(index.candidates(|| 99_999), [99_999]);
    assert!(index.candidates(|| 100_000).is_empty());
  }

  /// One key that every entry shares -- the worst a collision or a module of
  /// identical calls can do. The index still answers, and still answers with
  /// everything, because narrowing is all it promises: equality is the caller's.
  #[test]
  fn a_bucket_every_handle_shares_still_hands_back_all_of_them() {
    let mut index = CandidateIndex::default();

    for handle in 0..1_000usize {
      index.record(KEY, handle);
    }

    assert_eq!(index.candidates(|| KEY).len(), 1_000);

    for handle in 0..1_000usize {
      index.forget(&KEY, &handle);
    }

    assert!(index.candidates(|| KEY).is_empty());
  }
}
