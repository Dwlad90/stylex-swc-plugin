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

**Status:** ready-for-agent

- [ ] Every bench asserts that its subject produced the output it times, or the
      file says why it does not
- [ ] The three deliberate refusal benches assert that they refused
- [ ] `guidelines/PERFORMANCE.md` says how the rule is checked, if it is checked
- [ ] Any row whose timing moved is named in the re-baseline record
