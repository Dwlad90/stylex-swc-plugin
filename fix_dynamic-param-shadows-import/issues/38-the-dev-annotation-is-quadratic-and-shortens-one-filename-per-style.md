# 38 — The `dev` annotation is quadratic, and shortens the same filename once per style

Status: `resolved`
Blocked by: None — 36 left the first half of this as remaining work, 37 built the
bench that made it measurable.

**What was measured.** A release build, `dev` on and `dev` off, over slices of
`apps/rollup-large-example/lotsOfStyles.js` at 100 / 400 / 1 600 `stylex.create`
calls. Production is flat at 3.5 µs per line across the whole range. `dev` is
not: 14.1, 20.3 and 53.3 µs per line, so the `dev` penalty grows from 3.8x to
**15.5x** with the size of the file. A developer editing a large stylesheet waits
on that.

Attribution at 1 600 creates (29 351 lines), one transform:

| stage                                 | ms    | share |
| ------------------------------------- | ----- | ----- |
| whole `dev` transform                 | 1 565 | 100%  |
| `add_source_map_data`                 | 1 482 | 95%   |
| ├─ `KeySpanFinder` whole-program walk | 1 218 | 78%   |
| └─ `create_short_filename`            | 192   | 12%   |
| whole `dev=false` transform           | 103   | —     |

Two costs, and neither is the annotation itself:

- **the walk is the quadratic term 36 left behind.** 11.3 µs per lookup at 100
  creates, 44 at 400, 220 at 1 600 — linear per lookup, and there is one lookup
  per namespace, which is `O(namespaces x file size)`;
- **`create_short_filename` is called once per style with the same path every
  time.** Each call runs `env::current_dir()` and reads the package boundaries
  around both that and the file. Linear, but 12% of a transform at 1 600 creates
  and **54%** of what was left once the walk was gone.

- [x] Index every (object, namespace key) -> span in one pass over the memoized
      source, reproducing the disambiguation rather than approximating it
- [x] Memoize the short filename per path
- [x] Every existing snapshot byte-identical afterwards
- [x] Re-measure the curve, and say whether the committed bench can see a return

## Answer

**A `dev` transform is now linear in file size, and 1.4x a production one
instead of 15.5x.**

Two changes, one per cost:

- `crates/stylex-transform/src/shared/structures/key_span_index.rs` collects
  every (object, namespace key) -> span of the memoized source in **one** walk,
  held beside that source on the state manager and dropped with it. Each lookup
  is a hash hit followed by ranking the handful of candidates that actually
  spell the key, so `O(namespaces x file size)` becomes `O(file size)`;
- the short filename is memoized per path on `CacheState`, so
  `create_short_filename` — `env::current_dir()` plus the package boundaries
  around both it and the file — runs once per module rather than once per style.

Measured with the same harness as the numbers above (release build, best of
three, one transform):

| creates | lines  | dev before | dev after | speedup   | µs/line before | after |
| ------- | ------ | ---------- | --------- | --------- | -------------- | ----- |
| 100     | 1 886  | 26.5 ms    | 10.2 ms   | 2.6x      | 14.1           | 5.4   |
| 400     | 6 893  | 140.2 ms   | 34.5 ms   | 4.1x      | 20.3           | 5.0   |
| 1 600   | 29 351 | 1 635.9 ms | 154.3 ms  | **10.6x** | 55.7           | 5.3   |

The per-line cost is now flat across a 16x range where it used to quadruple,
which is what removing a superlinear term looks like rather than shaving a
constant. The `dev` penalty over `dev=false` was 3.8x / 5.9x / 15.5x and is now
1.4x / 1.4x / 1.5x — it no longer grows with the file at all. Both sides of the
table are uninstrumented release builds; the attribution above was taken with
per-stage timers in, which cost ~2% of the `dev` leg.

**From the committed bench** (`transform_debug_bench`, criterion, against 37's
recorded table on the same machine):

| point    | 37's number | now     | change |
| -------- | ----------- | ------- | ------ |
| dev/25   | 5.87 ms     | 2.60 ms | 2.3x   |
| dev/50   | 10.60 ms    | 4.17 ms | 2.5x   |
| dev/100  | 25.12 ms    | 9.10 ms | 2.8x   |
| prod/25  | 1.89 ms     | 1.86 ms | —      |
| prod/50  | 3.04 ms     | 3.06 ms | —      |
| prod/100 | 6.57 ms     | 6.54 ms | —      |

Production is unchanged at all three sizes, which is the check that this touched
only the debug path. Per create, `dev` went from 235 / 212 / 251 µs to 104 / 83 / 91.

**What stands behind the disambiguation.** The index keeps the old finder's
ranking: namespace-value-key overlap, then sibling-key overlap, then proximity
to the compiled call, with a tie resolving to `DUMMY_SP` — because a wrong
`file:line` is worse than none. Two properties the old walk had implicitly and
the index has to state, and one it changes:

- one candidate per (object, key), not per property, so a key written twice in
  one object literal is not read as two objects disagreeing. The last
  occurrence wins, which is the property a runtime object literal keeps;
- the sibling-key list keeps duplicates, because the overlap count did;
- **changed:** a duplicate key's _value_ keys now come from the last occurrence
  as well. The old walk moved its answer to the later property but kept the
  earlier one's value keys whenever the later value was not an object literal,
  so `{ root: { color: 'red' }, root: someVar }` ranked as though `root` still
  spelled `color` — and that is enough to flip which of two calls wins, or to
  turn a win into a tie and so into no annotation at all. The surviving property
  is the one the compiled call was built from, so it is the one whose value may
  be compared against it. Pinned by
  `a_duplicate_key_is_ranked_by_the_value_that_survives_it`, which the old
  behaviour fails.

Seven unit tests in `shared/structures/tests/key_span_index_test.rs` cover
those, the two-calls-one-key case both ways round, and the rank ordering — the
last moved out of `build_code_frame_error_tests.rs`, where it used to test the
deleted `KeySpanCandidate`. All 27 test binaries pass with no snapshot changes.

**The committed bench can see a return.** Reverting either change makes dev/100
go from 9.10 ms to 25.12 ms at an unchanged prod/100, so the existing 25 / 50 /
100 series flags it as a 2.8x regression. It could not have _caught_ the
quadratic when it was introduced — at those sizes the old per-create cost was
flat (235 / 212 / 251 µs) because the quadratic term was only ~14% of a
100-create transform — but the paired `dev`/`prod` legs at one size are enough
to guard the fix, and the curve out to 1 600 creates is what the table above is
for. Growing the committed fixture past 100 creates was declined: 37 pinned its
size by a test because `benchmark/fixtures.v1.json` points at the same file, and
re-cutting it would reshape a JS trend series under an unchanged name.

Re-measured after the review cleanups below, which moved the same work behind a
`NamespaceKeyQuery`: dev/100 8.95 ms and prod/100 6.52 ms, both within noise of
the table above -- the refactor cost nothing.

## Found in review

- the four values a lookup carries -- the key, the call's other namespace keys,
  the namespace's own value keys, the compiled call's position -- travelled
  together through three functions. They are now one `NamespaceKeyQuery`, built
  from the compiled call by the index module, which is also where the three
  helpers that read them moved to: describing a namespace and matching one are
  the same concept, and it was split across two files;
- `collect_object_lit_keys` had been duplicated into the index. It now lives
  once in `utils/ast/helpers.rs` beside `namespace_name_from_prop_key`, written
  in terms of the `prop_as_key_value` helper that was already there;
- the bench's own doc claimed the 25/50/100 series reads a quadratic curve apart
  from a linear one. It does not at those sizes -- the pre-fix per-create cost
  across it was flat -- so the comment now says what the series actually guards
  and where the curve was measured instead.

### Second round

Both axes again, this time with the whole change in front of them:

- **the disambiguation had a third implicit property, and it changed.** Spelled
  out above; pinned by a test the old behaviour fails. This was the only
  behavioural finding, and it was found by diffing the deleted walk against the
  new index line by line rather than by reading either;
- the rank was a bare `(usize, usize, Reverse<Option<u32>>)` whose doc had to
  re-explain what each position meant, with a `candidate_rank` constructor
  existing only so a test could build one. It is now a `CandidateRank` struct
  with named fields and a derived `Ord`, so the field order _is_ the precedence
  and the test constructs the value directly;
- `KeySpanIndexBuilder` was a one-field wrapper around the index it was
  building. `impl Visit for KeySpanIndex` deletes it;
- four places re-walked "the call's first argument, if it is an object literal".
  One `first_object_arg` now answers that, and `object_lo` answers "where is
  this object written, if anything wrote it";
- `IndexedCandidate` and `NamespaceKeyQuery` named the same two concepts four
  ways (`sibling_names`/`value_keys` against
  `sibling_keys`/`namespace_value_keys`) while `resolve` compared them field to
  field. Aligned, and the two overlap counts are one `overlap` helper;
- the "unreachable in practice" error was written out twice, comment included.
  One `missing_memoized_module(state)`;
- `SeenModuleSource` held a `Program` that both readers immediately matched back
  into a `Module`, one of them with an unreachable `_ => return None`. It holds
  the `Module` now, and both matches are gone;
- the glossary coined **Namespace key index** and then listed the names the code
  actually uses under `_Avoid_`. The term is **Key span index**, which is what
  the type, the file and the field are called, and the entry now says how it
  differs from the span cache beside it;
- two branches had no test: an object argument that names nothing, and an object
  with no position of its own being placed by its call. Both have one now, the
  second built from a synthesized module because parsing cannot produce it.

Not acted on: the branch name (`fix_dynamic-style-parameter-shadowing-an-imported-binding`)
describes none of this, which is true of tickets 29 onwards on it and is a
naming question for the branch rather than for this change. The
`collect_object_lit_keys` move into `utils/ast/helpers.rs` and the bench doc
rewrite were both called adjacent to what was asked; they stay, because the
helper was duplicated the moment the index needed it and the bench doc made a
claim about its own series that the measurements here contradict.

The atomic-commit finding is answered by committing this as two changes: the
index, then the short-filename memo.

## What is still not covered

- `get_key_span_from_source_code_impl` still builds `Expr::Call(call_expr.clone())`
  on every lookup, and after the first one per module nothing reads it — the
  clone only feeds the synthetic-module fallback for a source that cannot be
  read. 0.9 µs a lookup, ~3% of a `dev` transform at 1 600 creates. Left because
  making it lazy means threading a closure through `memoize_module`.
- `add_source_map_data` is still 90% of what a `dev` transform costs over a
  production one; after these two changes that is the annotation's own work
  (namespace-key hashing, the cache-key digest, one `CodeFrame` per lookup)
  spread thinly rather than one term to remove.
- Whether `dev` should keep implying `debug` is still the product question 36
  left open.
