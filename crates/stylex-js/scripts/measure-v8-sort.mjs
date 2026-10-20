// Measures the rows that `src/tests/array_sort_tests.rs` holds against V8.
//
// Each scenario builds its input from the MINSTD generator, as the Rust test
// does, and sorts the indices `0..n` with `Array.prototype.sort`. The script
// prints, for each scenario, the number of comparator calls, an FNV-1a
// checksum of the `(a, b)` operands in call order, an FNV-1a checksum of the
// sorted order, and the first eight items of that order.
//
// Usage: node crates/stylex-js/scripts/measure-v8-sort.mjs
//
// Run it on the Node whose V8 the port follows (see `src/array_sort.rs`).

/** The MINSTD generator, `x = x * 48271 mod (2^31 - 1)`, exact in a double. */
function minstd(seed) {
  let state = seed;
  return () => {
    state = (state * 48_271) % 2_147_483_647;
    return state;
  };
}

/** FNV-1a over 32-bit values. */
function fnv(values) {
  let hash = 2_166_136_261;
  for (const value of values) {
    hash = Math.imul((hash ^ value) >>> 0, 16_777_619) >>> 0;
  }
  return hash;
}

const bit = random => random() % 2;

/** `K[a] - K[b]`: a consistent order. */
const byKey = (_, keys) => (a, b) => keys[a] - keys[b];

/** `P[a] - P[b]` unless it is 0, else `K[a] - K[b]`. */
const byPriority = (priorities, keys) => (a, b) => {
  const difference = priorities[a] - priorities[b];
  return difference !== 0 ? difference : keys[a] - keys[b];
};

/** An answer from MINSTD for each call: less, greater, a tie or `NaN`. */
const atRandom = seed => () => {
  const random = minstd(seed);
  return () => [-1, 1, 0, Number.NaN][random() % 4];
};

/**
 * `K[a] - K[b]` for the first `after` calls, then the answers of `script` in
 * call order, then `rest` for every later call.
 */
const scripted = (after, script, rest) => (_, keys) => {
  let call = 0;
  return (a, b) => {
    const index = call++;
    if (index < after) return keys[a] - keys[b];
    return index - after < script.length ? script[index - after] : rest;
  };
};

/** The scenario, by the name of its Rust test. */
const SCENARIOS = {
  many_equal_keys: [1000, 7, (_, r) => [0, r() % 50], byKey],
  alternating_runs: [
    3000,
    11,
    (i, r) => {
      const [chunk, j] = [Math.floor(i / 128), i % 128];
      return [0, chunk % 2 === 1 ? 100_000 - j * 3 - bit(r) : j * 3 + bit(r) + ((chunk * 7) % 50)];
    },
    byKey,
  ],
  interleaved_blocks: [
    2048,
    13,
    i => [
      0,
      i < 1024
        ? Math.floor(i / 16) * 32 + (i % 16)
        : Math.floor((i - 1024) / 16) * 32 + 16 + (i % 16),
    ],
    byKey,
  ],
  nan_short: [37, 17, (_, r) => [r() % 5 === 0 ? NaN : r() % 4, r() % 10], byPriority],
  nan_run_boundary: [65, 19, (_, r) => [r() % 4 === 0 ? NaN : r() % 3, r() % 7], byPriority],
  nan_long: [700, 23, (_, r) => [r() % 6 === 0 ? NaN : r() % 3, r() % 100], byPriority],
  nan_long_runs: [
    1500,
    29,
    (i, r) => {
      const priority = r() % 9 === 0 ? NaN : Math.floor(i / 200);
      const offset = i % 200;
      return [priority, offset < 100 ? offset : 300 - offset];
    },
    byPriority,
  ],
  infinite: [
    400,
    31,
    (_, r) => [r() % 3 === 0 ? Infinity : r() % 3 === 0 ? -Infinity : 1, r() % 20],
    byPriority,
  ],
  // A run of 300, then 40 items that fall between its items: the merge is
  // `merge_high`, and each side wins often enough in a row to gallop.
  merge_high_gallops: [340, 37, highGallop, byKey],
  merge_high_gallops_nan: [
    340,
    37,
    (i, r) => [i % 9 === 0 ? NaN : 0, highGallop(i, r)[1]],
    byPriority,
  ],
  random_short: [65, 0, () => [0, 0], atRandom(41)],
  random_long: [1000, 0, () => [0, 0], atRandom(43)],
  // Two runs of 40. The first 79 calls find them; then the merge trims
  // nothing from the first run and all of the second.
  merge_at_empties_the_second_run: [80, 0, i => [0, i < 40 ? i : i - 40], scripted(79, [-1], 1)],
};

/** Keys of a run of 300, then 40 that fall between them. */
function highGallop(i) {
  return [0, i < 300 ? i * 10 : (i - 300) * 75 + 5];
}

for (const [name, [n, seed, make, comparatorOf]] of Object.entries(SCENARIOS)) {
  const random = minstd(seed);
  const priorities = [];
  const keys = [];
  for (let i = 0; i < n; i++) {
    const [priority, key] = make(i, random);
    priorities.push(priority);
    keys.push(key);
  }

  const compare = comparatorOf(priorities, keys);
  const calls = [];
  const items = Array.from({ length: n }, (_, i) => i);
  items.sort((a, b) => {
    calls.push(a, b);
    return compare(a, b);
  });

  console.log(
    `${name}: calls ${calls.length / 2}, call_sum ${fnv(calls)}, ` +
      `order_sum ${fnv(items)}, head [${items.slice(0, 8).join(', ')}]`
  );
}
