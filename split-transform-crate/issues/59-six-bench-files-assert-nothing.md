# 59 — Six bench files assert nothing

**What to build:** `guidelines/PERFORMANCE.md` says a bench must assert that its
subject produced the output it exists to time. Six of the fourteen bench files
do not, and they hold 179 of the 268 measurements. Close the gap, or write down
which files the rule does not cover and why.

| Bench file | Crate | Measurements | Assertions |
| --- | --- | ---: | ---: |
| `value_parser_bench.rs` | `postcss-value-parser` | 7 | 0 |
| `css_types_bench.rs` | `stylex-css-parser` | 72 | 0 |
| `properties_bench.rs` | `stylex-css-parser` | 63 | 0 |
| `token_parser_bench.rs` | `stylex-css-parser` | 28 | 0 |
| `css_generation_bench.rs` | `stylex-css` | 6 | 0 |
| `normalize_value_bench.rs` | `stylex-css` | 3 | 0 |
| | | **179** | |

The other eight files carry 6 to 25 assertions each.

**The rule, and why it is written that way.** `guidelines/PERFORMANCE.md`:
"Assert what the bench is measuring, in the bench. A refusal, a deopt, a
swallowed panic and a cache hit are all fast, and a curve that flattens because
the work stopped happening is indistinguishable from a win." Every one of the
179 measurements is the same shape: `black_box(parser.parse(input))`, with the
`Result` dropped. A parser that starts refusing every input gets faster and
nothing reports it.

**This is not a theory.** The re-baseline found three benches that already time
nothing, all of them in these six files:

- `PropertyCreation/transform_create`, 801.74 ps, and
  `PropertyDisplay/transform_display`, 7.32 ns
  ([ticket 57](./57-two-benches-measure-nothing.md));
- `InlineStyleToCssString/rtl_flippable_pairs`, which measures no flipping
  ([ticket 58](./58-a-bench-named-for-work-it-does-not-do.md)).

None was caught by the benches. All three were found by a person reading the
numbers afterwards, which is the failure mode the rule exists to prevent.

**Three benches deliberately time a refusal**, in `token_parser_bench.rs`:
`BasicParsers/never_parser` (53.5 ns), `Repetition/zero_or_more_empty`
(62.8 ns) and `Repetition/one_or_more_failure` (93.9 ns). Timing an error path
on purpose is legitimate and the names say so. They still want an assertion,
because nothing today stops one of them from starting to answer `Ok` and reading
as a win.

**What an assertion costs.** It runs once per iteration inside `b.iter`, so keep
it cheap: check the discriminant, or a length, not a full equality against a
built expected value. The eight files that already comply show the shape.

**Do not change any timing.** An assertion inside the timed closure adds to the
measurement. Put the check outside `b.iter` where the input is fixed, and inside
only where the result varies per iteration. Say in the record which rows moved,
because a bench that gains a timed assertion starts a new series.

**Where the numbers are:**
[`../bench/ticket-17.md`](../bench/ticket-17.md), "Four things this run found".

**Found by:** the re-baseline run of
[ticket 17](./17-link-mimalloc-in-every-bench.md).

**Blocked by:** None. Tickets 57 and 58 fix two instances; this one closes the
gap that let them through. Doing them first is fine and does not change this
ticket.

**Status:** done

- [x] Every bench *file* carries a check that its subject produced the output
      it times. All fourteen files, and a fifteenth the table did not list.
      Read the wording: the guard is per file, not per measurement -- see "What
      the guard holds, and what it does not" below
- [x] The three deliberate refusal benches state the answer they expect. Two of
      the three refuse; the third answers `Ok(vec![])`, which the check now
      records rather than assumes
- [x] `guidelines/PERFORMANCE.md` says how the rule is checked:
      `every_bench_asserts_what_it_measures`, beside the allocator check
- [x] Every row whose timing moved is named in the re-baseline record --
      [`../bench/ticket-57-58-59.md`](../bench/ticket-57-58-59.md)

## Comments

**The table was one file short.**
`crates/stylex-styleq/benches/performance_bench.rs` carries 16 measurements and
no check either, which makes it seven files and 195 measurements rather than
six and 179. The guard found it; the count by hand had not.

**The checks found nine more measurements that were timing a refusal.** None of
them was reported by anything:

| Benchmark | What it handed the parser |
| --- | --- |
| `Length/parse_length_100%` | a percentage, which is not a `<length>` |
| `Angle/parse_angle_2π rad` | `π`, which is not a CSS number |
| `BoxShadow/single_box_shadow_0..2` | a shadow with no colour (3 cases) |
| `BoxShadow/box_shadow_list_0..2` | the same three, through the list parser |
| `ParserComparison/parse_complex_values` | `calc(...)` to a `Length` parser |

The colour is not optional in this parser and it is not optional in the
reference either, so those six are the bench's error and not the parser's. Each
input was replaced with one the case's own name describes.

**The checks also found a correctness defect, in `stylex-styleq`.** Writing the
counts for `performance_bench.rs` needed an answer to compare against, and the
answers disagreed with themselves: the same merge read 2 class names or 4
depending on what had been asked for earlier, and a style asked for on its own
after a merge read as an empty class name. The cache was one flat map where the
reference keeps a chain, so a chunk cut for one merge was answered to another.
Fixed in the same change, with four tests, and checked against the reference
implementation. See the re-baseline record.

**Where the checks live.** Outside `b.iter` in every case, because every input
in these files is fixed, so one answer speaks for every iteration and the
timing does not move for the check itself. The timings that did move, moved for
another reason -- see the record.

**One region of `styleq.rs` is still counted uncovered, and no test can reach
it.** The workspace gate went from ten uncovered regions to one. The ten were
real and are closed -- the two the cache fix removed, the `is_undefined` default
and its two wrapper delegations, and the chunk-dedupe arm. The one that remains
has no source location: `scripts/coverage-missing.sh` reports zero distinct
source regions unexercised, and the JSON export carries no region whose count is
zero in every instantiation. It is the monomorphization artifact that script's
own header describes -- `styleq.rs` is generic over the value type and is
compiled into three test binaries, and llvm-cov's per-file summary does not
merge the instantiation records the way the script does.

It predates this work: the same single instance sat behind the ten. Two
instantiation gaps were closed on the way to finding that out -- the crate's own
test binary now runs one merge, compiled and inline, so the generic walk is
instantiated where it is defined rather than only in the integration binary.
What is left needs either a newer nightly or a change to production code whose
only purpose is the tool, which `guidelines/stack/RUST.md` refuses.

**An undefined inline-style value is skipped, and that is kept here.** Writing
the counts also turned up a second disagreement with the reference: an inline
style property whose value is `undefined` was entered into the defined set and
broke the cache chain, where `styleq.js` skips the whole body for one. So the
property was held against a later style that could still have declared it. The
skip is in this change, with the cache fix it was found beside, and is pinned
by three cases in `styleq_test.rs`. It is a merge-semantics change rather than a
cache one, so it is named here rather than left to be read out of the diff.

**What the guard holds, and what it does not.** The guard is file-level. One
`assert!`, `assert_eq!` or `assert_ne!` anywhere in a `benches/*.rs` passes the
whole file, and so does one `ASSERTIONS: none`. What it catches is a file that
checks nothing, which is the state all seven were in. What it cannot catch is a
new measurement added beside a checked sibling: `transform_create` in
`properties_bench.rs` timed 801 ps -- nothing at all -- inside a file that
already held checks, and the guard passed it.

Raising the guard to one check per `bench_function` was considered and not
done. There is no reading that separates a check for one measurement from a
check for another: the files that do check well, `performance_bench.rs` among
them, gather every check into one block above the measurements and name each by
the benchmark id, while others check a shared fixture once for a group. Only 16
of 36 named benchmarks repeat their id anywhere else in their file, so a reader
keyed on the id would fail 20 measurements that are checked and pass any file
that spells a name twice. That is a guard a reader stops believing, which is
worse than one whose limit is written down. The limit is written here and on
`asserts_what_it_measures` itself; a new measurement is covered by review.

**What the rule costs to keep.** `every_bench_asserts_what_it_measures` reads
each `benches/*.rs` and fails on a file with no check, the way
`every_bench_says_which_allocator_it_measures` fails on one that names no
allocator. A bench that times something no check can read writes
`ASSERTIONS: none` with the reason. The reader ignores comment lines, so a file
that keeps the comment and drops the check does not pass.
