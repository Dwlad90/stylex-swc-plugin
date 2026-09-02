# Ticket 27 — what was measured

One machine, one session. Every number below is a **directional micro-result**:
best-of-N on an ad-hoc harness, not the bootstrapped verdict
`guidelines/PERFORMANCE.md` requires of a blocking comparison. Read them as
"which way, and roughly how far", never as a percentage to quote.

## 1. The per-namespace ordering step

`compute_key_span_cache_key` runs once per namespace of every call, before the
span-cache check, and it has to hash the namespace's value keys in a canonical
order. It collected a heap `Vec<&Atom>` and sorted it stably. It now collects an
inline `SmallVec` and sorts it unstably -- the stable sort allocates scratch
space of its own, so both halves matter. Timed side by side over the same key
sets, best of seven:

| value keys | heap `Vec` + stable sort | inline buffer + unstable sort |
| ---------- | ------------------------ | ----------------------------- |
| 1          | 17 ns                    | 3 ns                          |
| 2          | 22 ns                    | 7 ns                          |
| 4          | 32 ns                    | 19 ns                         |
| 8          | 67 ns                    | 55 ns                         |
| 16         | 202 ns                   | 191 ns                        |
| 32         | 701 ns                   | 665 ns                        |
| 128        | 3 475 ns                 | 3 174 ns                      |

Cheaper everywhere, and most where real namespaces sit. The byte stream the key
hashes is unchanged, so the `v5` version string stands.

### What this replaced, and why

The first attempt held the value keys as a sorted, deduplicated `Vec` and
answered membership from it with `binary_search`, which removed the second
collection altogether. **Measured, that was a regression**, because `Atom`'s two
operators are not comparable in cost: `Hash` writes one precomputed word, while
`Ord` compares the strings. Membership in the shape `overlap` asks it, best of
seven:

| value keys | hash set | binary search | linear scan |
| ---------- | -------- | ------------- | ----------- |
| 1          | 2 ns     | 3 ns          | 1 ns        |
| 4          | 7 ns     | 26 ns         | 10 ns       |
| 8          | 14 ns    | 82 ns         | 23 ns       |
| 16         | 28 ns    | 263 ns        | 91 ns       |
| 32         | 59 ns    | 732 ns        | 337 ns      |
| 128        | 222 ns   | 5 073 ns      | 5 705 ns    |

`rank` runs that test once per candidate, and a module of N calls spelling one
key gives N candidates, so the hash set stays. Only the ordering moved.

## 2. `transform_debug_bench`, re-run for shape

The bench exists to hold the debug path's cost *flat* per create. This is a new
baseline (`ticket-27-after`), not a delta: no pre-change leg was run, because
the effect above is far under the noise a paired whole-transform comparison can
resolve. The two columns are unpaired runs, so the derived penalty is a shape
check and nothing more. Raw run: `ticket-27-debug-bench.log`.

| creates | dev      | prod     | derived penalty per create |
| ------- | -------- | -------- | -------------------------- |
| 25      | 2.161 ms | 1.620 ms | 21.6 us                    |
| 50      | 3.526 ms | 2.653 ms | 17.5 us                    |
| 100     | 7.582 ms | 5.713 ms | 18.7 us                    |

| namespaces per call | dev      | prod     | derived per namespace |
| ------------------- | -------- | -------- | --------------------- |
| 8                   | 274.3 us | 164.4 us | 13.7 us               |
| 32                  | 1.083 ms | 878.1 us | 6.4 us                |
| 128                 | 8.457 ms | 7.645 ms | 6.3 us                |

Flat, which is what the bench asks. **This baseline closes when ticket 17
lands**: linking the allocator the addon ships changes every number here.

## 3. The candidate index's per-record bucket scan

Filling one bucket, best of five, against filling distinct keys:

| entries | one bucket        | distinct keys  |
| ------- | ----------------- | -------------- |
| 1       | 83 ns/record      | 42 ns/record   |
| 10      | 20.8 ns/record    | 50.0 ns/record |
| 100     | 13.8 ns/record    | 27.9 ns/record |
| 1 000   | 61.5 ns/record    | 25.4 ns/record |
| 10 000  | 464.4 ns/record   | 17.5 ns/record |
| 100 000 | 4 693.3 ns/record | 20.3 ns/record |

Quadratic, as the code says, and reaching the thousands takes a module of
thousands of byte-identical `stylex.create` calls. Left alone on those numbers,
recorded in `CandidateIndex::record`.

## 4. The bench profile's debug information

`strip = "none"` was applied. Counted rather than read off the flags: the
`stylex_styleq` bench binary now carries **95** symbols matching `stylex`,
where the inherited `strip = "symbols"` left **0**, and measures 2 723 104
bytes. Both match what
[ticket 19](../issues/19-bench-profile-strips-its-own-debuginfo.md) predicted.
The change strips no code, only symbols, so no timing series moves with it.

## What was not measured

`crates/stylex-utils/src/identifier.rs` dropped one of its two allocations and
was not timed. Every production caller passes no member key and is once per
export, and `stylex-state/src/theme_ref.rs` already caches the result, so there
is nothing here a benchmark could resolve. The change stands on the allocation
it removes, not on a number.
