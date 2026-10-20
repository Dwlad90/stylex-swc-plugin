// Tests for V8's TimSort.
// Source: crates/stylex-js/src/array_sort.rs
//
// The V8 rows are measured on Node 24 (V8 13.6.233) by
// `scripts/measure-v8-sort.mjs`, which builds the same input from the same
// MINSTD generator and sorts an index array with `Array.prototype.sort`,
// recording every comparator call. A row holds the
// number of calls, an FNV-1a checksum of the `(a, b)` operands in call order,
// and an FNV-1a checksum of the sorted order. Equal checksums mean the port
// makes each comparison V8 makes, in V8's order, and ends in V8's order.
//
// Every test sorts `usize` items with a `usize` error, so the sort is one
// instantiation here and the coverage gate scores all the tests together.

use std::cell::RefCell;

use super::*;

/// The MINSTD generator, `x = x * 48271 mod (2^31 - 1)`. Its products stay
/// below 2^53, so JavaScript computes it exactly too.
struct Minstd(u64);

impl Minstd {
  fn next(&mut self) -> u64 {
    self.0 = self.0 * 48_271 % 2_147_483_647;
    self.0
  }
}

/// FNV-1a over 32-bit values, as `Math.imul` computes it.
fn fnv(values: impl IntoIterator<Item = usize>) -> u32 {
  let mut hash: u32 = 2_166_136_261;

  for value in values {
    hash ^= value as u32;
    hash = hash.wrapping_mul(16_777_619);
  }

  hash
}

/// How a scenario's comparator answers.
#[derive(Clone, Copy)]
enum Comparator {
  /// `K[a] - K[b]`: a consistent order.
  Key,
  /// `P[a] - P[b]` unless it is 0, else `K[a] - K[b]`. With a `NaN` or an
  /// infinite priority, this is not a consistent order.
  Priority,
  /// An answer from a MINSTD generator of this seed for each call: less,
  /// greater, a tie or `NaN`, whatever the items are.
  Random(u64),
  /// `K[a] - K[b]` for the first `after` calls, then the answers of `script`
  /// in call order, then `rest` for every later call.
  Scripted {
    after: usize,
    script: &'static [f64],
    rest: f64,
  },
}

struct Measured {
  calls: usize,
  call_sum: u32,
  order_sum: u32,
  head: [usize; 8],
}

/// Sorts `0..n` as the measuring script does and reports what it saw.
fn measure(
  n: usize,
  seed: u64,
  make: impl Fn(usize, &mut Minstd) -> (f64, f64),
  comparator: Comparator,
) -> Measured {
  let mut random = Minstd(seed);
  let (priorities, keys): (Vec<f64>, Vec<f64>) = (0..n).map(|i| make(i, &mut random)).unzip();
  let calls = RefCell::new(Vec::new());
  let mut items: Vec<usize> = (0..n).collect();
  let mut answers = Minstd(match comparator {
    Comparator::Random(seed) => seed,
    _ => 1,
  });

  let sorted: Result<(), usize> = array_tim_sort(&mut items, |&a, &b| {
    let call = calls.borrow().len() / 2;
    calls.borrow_mut().extend([a, b]);

    Ok(match comparator {
      Comparator::Key => keys[a] - keys[b],
      Comparator::Priority => {
        let difference = priorities[a] - priorities[b];
        if difference != 0.0 {
          difference
        } else {
          keys[a] - keys[b]
        }
      },
      Comparator::Random(_) => [-1.0, 1.0, 0.0, f64::NAN][(answers.next() % 4) as usize],
      Comparator::Scripted {
        after,
        script,
        rest,
      } => match call.checked_sub(after) {
        None => keys[a] - keys[b],
        Some(index) => script.get(index).copied().unwrap_or(rest),
      },
    })
  });
  assert_eq!(sorted, Ok(()));

  let calls = calls.into_inner();
  let mut head = [0; 8];
  head.copy_from_slice(&items[..8]);

  Measured {
    calls: calls.len() / 2,
    call_sum: fnv(calls),
    order_sum: fnv(items),
    head,
  }
}

fn assert_v8(measured: Measured, calls: usize, call_sum: u32, order_sum: u32, head: [usize; 8]) {
  assert_eq!(measured.head, head);
  assert_eq!(measured.calls, calls);
  assert_eq!(measured.call_sum, call_sum);
  assert_eq!(measured.order_sum, order_sum);
}

fn bit(random: &mut Minstd) -> f64 {
  (random.next() % 2) as f64
}

#[test]
fn many_equal_keys_make_the_comparisons_v8_makes() {
  let measured = measure(
    1000,
    7,
    |_, random| (0.0, (random.next() % 50) as f64),
    Comparator::Key,
  );

  assert_v8(
    measured,
    8346,
    598_553_585,
    1_372_345_211,
    [123, 175, 253, 255, 261, 307, 340, 353],
  );
}

#[test]
fn alternating_runs_make_the_comparisons_v8_makes() {
  let measured = measure(
    3000,
    11,
    |i, random| {
      let (chunk, j) = ((i / 128) as f64, (i % 128) as f64);
      let key = if chunk % 2.0 == 1.0 {
        100_000.0 - j * 3.0 - bit(random)
      } else {
        j * 3.0 + bit(random) + (chunk * 7.0) % 50.0
      };
      (0.0, key)
    },
    Comparator::Key,
  );

  assert_v8(
    measured,
    14_198,
    3_199_290_819,
    721_027_905,
    [0, 1, 2816, 2, 1024, 2817, 3, 1025],
  );
}

#[test]
fn interleaved_blocks_gallop_as_v8_gallops() {
  let measured = measure(
    2048,
    13,
    |i, _| {
      let key = if i < 1024 {
        (i / 16 * 32 + i % 16) as f64
      } else {
        ((i - 1024) / 16 * 32 + 16 + i % 16) as f64
      };
      (0.0, key)
    },
    Comparator::Key,
  );

  assert_v8(
    measured,
    3072,
    1_890_501_006,
    671_419_845,
    [0, 1, 2, 3, 4, 5, 6, 7],
  );
}

/// A priority that is `NaN` one time in five, below 64 items: binary
/// insertion alone.
#[test]
fn nan_priorities_in_a_short_list_sort_as_v8_sorts() {
  let measured = measure(
    37,
    17,
    |_, random| {
      let priority = if random.next() % 5 == 0 {
        f64::NAN
      } else {
        (random.next() % 4) as f64
      };
      (priority, (random.next() % 10) as f64)
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    141,
    2_139_140_696,
    1_626_029_543,
    [15, 24, 3, 8, 29, 31, 22, 19],
  );
}

/// 65 items: the first list long enough for two runs and one merge.
#[test]
fn nan_priorities_at_the_run_boundary_sort_as_v8_sorts() {
  let measured = measure(
    65,
    19,
    |_, random| {
      let priority = if random.next() % 4 == 0 {
        f64::NAN
      } else {
        (random.next() % 3) as f64
      };
      (priority, (random.next() % 7) as f64)
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    246,
    1_529_760_124,
    2_413_596_973,
    [5, 10, 15, 19, 9, 17, 3, 13],
  );
}

#[test]
fn nan_priorities_in_a_long_list_sort_as_v8_sorts() {
  let measured = measure(
    700,
    23,
    |_, random| {
      let priority = if random.next() % 6 == 0 {
        f64::NAN
      } else {
        (random.next() % 3) as f64
      };
      (priority, (random.next() % 100) as f64)
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    3090,
    1_149_026_584,
    182_444_109,
    [34, 23, 35, 17, 4, 3, 25, 11],
  );
}

#[test]
fn nan_priorities_over_long_runs_sort_as_v8_sorts() {
  let measured = measure(
    1500,
    29,
    |i, random| {
      let priority = if random.next() % 9 == 0 {
        f64::NAN
      } else {
        (i / 200) as f64
      };
      let offset = i % 200;
      let key = if offset < 100 { offset } else { 300 - offset };
      (priority, key as f64)
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    5002,
    482_073_038,
    3_546_005_451,
    [0, 1, 2, 3, 4, 5, 6, 7],
  );
}

/// `Infinity - Infinity` is `NaN` too.
#[test]
fn infinite_priorities_sort_as_v8_sorts() {
  let measured = measure(
    400,
    31,
    |_, random| {
      let priority = if random.next() % 3 == 0 {
        f64::INFINITY
      } else if random.next() % 3 == 0 {
        f64::NEG_INFINITY
      } else {
        1.0
      };
      (priority, (random.next() % 20) as f64)
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    2427,
    4_049_990_009,
    3_837_625_699,
    [5, 7, 8, 11, 12, 15, 25, 37],
  );
}

/// Keys of a run of 300, then of 40 items that fall between its items. The
/// merge of the two runs is `merge_high`, and each side wins often enough in
/// a row to gallop.
fn high_gallop_key(i: usize) -> f64 {
  if i < 300 {
    (i * 10) as f64
  } else {
    ((i - 300) * 75 + 5) as f64
  }
}

#[test]
fn merge_high_gallops_as_v8_gallops() {
  let measured = measure(340, 37, |i, _| (0.0, high_gallop_key(i)), Comparator::Key);

  assert_v8(
    measured,
    671,
    2_027_395_964,
    4_174_909_381,
    [0, 300, 1, 2, 3, 4, 5, 6],
  );
}

/// The same input with a `NaN` priority on every ninth item.
#[test]
fn merge_high_with_nan_priorities_sorts_as_v8_sorts() {
  let measured = measure(
    340,
    37,
    |i, _| {
      let priority = if i % 9 == 0 { f64::NAN } else { 0.0 };
      (priority, high_gallop_key(i))
    },
    Comparator::Priority,
  );

  assert_v8(
    measured,
    357,
    1_287_087_313,
    2_839_715_361,
    [0, 1, 2, 3, 4, 5, 6, 7],
  );
}

/// A comparator that answers at random still gives V8's order: the answers
/// come in call order, and each call is the one V8 makes.
#[test]
fn a_random_comparator_sorts_as_v8_sorts() {
  assert_v8(
    measure(65, 0, |_, _| (0.0, 0.0), Comparator::Random(41)),
    266,
    180_744_368,
    1_079_752_199,
    [0, 1, 2, 33, 3, 34, 10, 15],
  );
  assert_v8(
    measure(1000, 0, |_, _| (0.0, 0.0), Comparator::Random(43)),
    5780,
    3_711_921_848,
    3_620_645_125,
    [381, 6, 9, 47, 0, 1, 16, 568],
  );
}

/// Two runs of 40. The first 79 calls find them. Then the merge trims nothing
/// from the first run, because the first item of the second run sorts before
/// it, and all of the second run, because each of its items sorts after the
/// last item of the first: the merge ends with nothing to merge.
#[test]
fn a_merge_that_empties_the_second_run_ends_as_v8_ends() {
  let measured = measure(
    80,
    0,
    |i, _| (0.0, (if i < 40 { i } else { i - 40 }) as f64),
    Comparator::Scripted {
      after: 79,
      script: &[-1.0],
      rest: 1.0,
    },
  );

  assert_v8(
    measured,
    90,
    2_244_254_514,
    1_385_968_469,
    [0, 1, 2, 3, 4, 5, 6, 7],
  );
}

/// A run of 60, then one of 20. The merge keeps both runs whole, so it is
/// `merge_high`. There every later answer is "greater": B wins, gallops, and
/// the gallop takes all of what is left of B.
#[test]
fn a_gallop_in_merge_high_that_empties_the_second_run_ends_as_v8_ends() {
  let measured = measure(
    80,
    0,
    |i, _| (0.0, (if i < 60 { i } else { i - 60 }) as f64),
    Comparator::Scripted {
      after: 79,
      script: &[-1.0, -1.0],
      rest: 1.0,
    },
  );

  assert_v8(
    measured,
    96,
    2_434_862_606,
    1_262_322_905,
    [0, 1, 2, 3, 4, 5, 6, 7],
  );
}

/// With a consistent order, V8's sort is a stable sort, so it agrees with
/// `sort_by` on every input: lengths around each run boundary, and inputs
/// that are sorted, reversed, constant or random.
#[test]
fn a_consistent_order_sorts_as_a_stable_sort_does() {
  let mut random = Minstd(97);

  for length in (0..140).chain([255, 256, 257, 1000, 4096, 10_000]) {
    let shapes: [Vec<u64>; 5] = [
      (0..length).map(|_| random.next() % 8).collect(),
      (0..length).map(|_| random.next()).collect(),
      (0..length as u64).collect(),
      (0..length as u64).rev().collect(),
      vec![3; length],
    ];

    for keys in shapes {
      // The items are positions into the keys, so the test can see which of
      // two equal keys comes first.
      let mut items: Vec<usize> = (0..keys.len()).collect();
      let mut expected = items.clone();
      expected.sort_by_key(|&item| keys[item]);

      let sorted: Result<(), usize> =
        array_tim_sort(&mut items, |&a, &b| Ok(keys[a] as f64 - keys[b] as f64));

      assert_eq!(sorted, Ok(()));
      assert_eq!(items, expected, "length {length}");
    }
  }
}

/// A comparator error stops the sort at that comparison, and the items keep
/// their first order, as V8 leaves the array when a comparator throws.
#[test]
fn an_error_stops_the_sort_and_keeps_the_original_order() {
  for length in [2, 10, 64, 65, 500] {
    for fail_at in [0, 1, 7, 100, 600, 2000] {
      let mut items: Vec<usize> = (0..length).rev().collect();
      let original = items.clone();
      let mut calls: usize = 0;

      let sorted = array_tim_sort(&mut items, |a, b| {
        calls += 1;
        if calls > fail_at {
          return Err(calls);
        }
        Ok(*a as f64 - *b as f64)
      });

      match sorted {
        Err(at) => {
          assert_eq!(at, fail_at + 1);
          assert_eq!(items, original);
        },
        Ok(()) => assert_eq!(items, (0..length).collect::<Vec<_>>()),
      }
    }
  }
}

#[test]
fn a_list_of_fewer_than_two_items_is_never_compared() {
  let mut never = |_: &usize, _: &usize| -> Result<f64, usize> { panic!("compared") };

  assert_eq!(array_tim_sort(&mut [], &mut never), Ok(()));
  assert_eq!(array_tim_sort(&mut [1], &mut never), Ok(()));
}

/// A comparator that answers at random is no order at all. The sort still
/// ends, keeps every item once, and does not panic.
#[test]
fn a_random_comparator_keeps_every_item_once() {
  for seed in 1..=50 {
    let mut random = Minstd(seed);

    for length in [2, 3, 31, 64, 65, 129, 1000, 5000] {
      let mut items: Vec<usize> = (0..length).collect();
      let sorted: Result<(), usize> = array_tim_sort(&mut items, |_, _| {
        Ok(match random.next() % 4 {
          0 => -1.0,
          1 => 1.0,
          2 => 0.0,
          _ => f64::NAN,
        })
      });

      assert_eq!(sorted, Ok(()));
      items.sort_unstable();
      assert_eq!(items, (0..length).collect::<Vec<_>>());
    }
  }
}

/// A comparator that answers "less" for every pair makes each gallop run to
/// the end of its run, so a merge empties a run inside the gallop.
#[test]
fn a_comparator_that_is_always_less_keeps_every_item_once() {
  for answer in [-1.0, 1.0, 0.0] {
    for length in [65, 200, 1000, 3000] {
      let mut items: Vec<usize> = (0..length).collect();
      let sorted: Result<(), usize> = array_tim_sort(&mut items, |_, _| Ok(answer));

      assert_eq!(sorted, Ok(()));
      items.sort_unstable();
      assert_eq!(items, (0..length).collect::<Vec<_>>());
    }
  }
}

#[test]
fn the_minimum_run_length_is_the_length_below_64_and_32_to_64_above() {
  assert_eq!(compute_min_run_length(0), 0);
  assert_eq!(compute_min_run_length(63), 63);
  assert_eq!(compute_min_run_length(64), 32);
  assert_eq!(compute_min_run_length(65), 33);
  assert_eq!(compute_min_run_length(1000), 63);
  assert_eq!(compute_min_run_length(1024), 32);
  for length in 64..5000 {
    assert!(
      (32..=64).contains(&compute_min_run_length(length)),
      "{length}"
    );
  }
}

/// Inputs made of natural runs of many lengths, ascending and descending, so
/// that the run stack meets each of its merge rules. With a consistent order
/// the result is the stable sort's.
#[test]
fn runs_of_many_lengths_meet_every_merge_rule() {
  let mut random = Minstd(41);

  for _ in 0..300 {
    let mut keys: Vec<u64> = Vec::new();
    let target = 64 + (random.next() % 3000) as usize;

    while keys.len() < target {
      let length = 1 + (random.next() % 300) as usize;
      let start = random.next() % 1000;
      let step = random.next() % 3;
      let run = (0..length as u64).map(|i| start + i * step);

      if random.next().is_multiple_of(2) {
        keys.extend(run);
      } else {
        keys.extend(run.rev());
      }
    }

    let mut items: Vec<usize> = (0..keys.len()).collect();
    let mut expected = items.clone();
    expected.sort_by_key(|&item| keys[item]);

    let sorted: Result<(), usize> =
      array_tim_sort(&mut items, |&a, &b| Ok(keys[a] as f64 - keys[b] as f64));

    assert_eq!(sorted, Ok(()));
    assert_eq!(items, expected);
  }
}

/// A long sorted run with one smaller item after it leaves a run of one item
/// at the end.
#[test]
fn a_last_item_after_a_long_run_is_a_run_of_its_own() {
  let mut items: Vec<usize> = (1..100).chain([0]).collect();
  let sorted: Result<(), usize> = array_tim_sort(&mut items, |a, b| Ok(*a as f64 - *b as f64));

  assert_eq!(sorted, Ok(()));
  assert_eq!(items, (0..100).collect::<Vec<_>>());
}

/// Two runs that the merge trims to one item each: B's one item goes first,
/// and A's one item stays where it is.
#[test]
fn two_runs_trimmed_to_one_item_each_merge() {
  // A: 0..31 then 100. B: 50, then 100 and up. B's first item goes after A's
  // first 31 items, and A's last item goes before B's second.
  let keys: Vec<u64> = (0..31).chain([100, 50]).chain(100..131).collect();
  let mut items: Vec<usize> = (0..keys.len()).collect();
  let mut expected = items.clone();
  expected.sort_by_key(|&item| keys[item]);

  let sorted: Result<(), usize> =
    array_tim_sort(&mut items, |&a, &b| Ok(keys[a] as f64 - keys[b] as f64));

  assert_eq!(sorted, Ok(()));
  assert_eq!(items, expected);
}

/// A comparator error at each call in turn, over inputs that merge low and
/// high and gallop both ways: every comparison in the sort can end it, and
/// each time the items keep their first order.
#[test]
fn an_error_at_any_comparison_ends_the_sort() {
  let mut random = Minstd(43);
  let alternating: Vec<u64> = (0..400_u64)
    .map(|i| {
      if (i / 50) % 2 == 0 {
        i
      } else {
        1000 - i + random.next() % 3
      }
    })
    .collect();
  // A long run, then a short one that interleaves with it: a high merge.
  let long_then_short: Vec<u64> = (0..300_u64)
    .map(|i| i * 2)
    .chain((0..40_u64).map(|i| i * 15 + 1))
    .collect();

  // Runs of mixed lengths and directions, as in the merge-rule test.
  let mut mixed: Vec<u64> = Vec::new();
  while mixed.len() < 600 {
    let length = 1 + random.next() % 120;
    let start = random.next() % 500;
    mixed.extend((0..length).map(|i| start + i));
  }

  for keys in [alternating, long_then_short, mixed] {
    assert_every_failure_ends_the_sort(&keys);
  }
}

fn assert_every_failure_ends_the_sort(keys: &[u64]) {
  let original: Vec<usize> = (0..keys.len()).collect();

  let mut total: usize = 0;
  let mut items = original.clone();
  let sorted: Result<(), usize> = array_tim_sort(&mut items, |&a, &b| {
    total += 1;
    Ok(keys[a] as f64 - keys[b] as f64)
  });
  assert_eq!(sorted, Ok(()));

  for fail_at in 1..=total {
    let mut calls: usize = 0;
    let mut items = original.clone();
    let sorted = array_tim_sort(&mut items, |&a, &b| {
      calls += 1;
      if calls == fail_at {
        return Err(calls);
      }
      Ok(keys[a] as f64 - keys[b] as f64)
    });

    assert_eq!(sorted, Err(fail_at));
    assert_eq!(items, original);
  }
}
