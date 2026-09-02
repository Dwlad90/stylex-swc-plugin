# Ticket 09 — bench A/B across the crate move

The three evaluation benches moved from `stylex-transform/benches` to
`stylex-evaluator/benches`. Criterion baseline identities are per crate, so
criterion cannot diff the two legs itself. Both legs were measured on one
machine in one session and paired by benchmark id afterwards.

## Method

| Item     | Value                                                             |
| -------- | ----------------------------------------------------------------- |
| Machine  | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1                       |
| Profile  | `bench` (`lto = true`, `debug = true`)                            |
| Settings | `--sample-size 20 --warm-up-time 2 --measurement-time 4 --noplot` |
| Before   | benches on `stylex_transform`, saved as `before-move`             |
| After    | the same three files on `stylex_evaluator`, saved as `after-move` |
| Paired   | 28 of 28 measurements; no benchmark id changed                    |

Only the bench sources moved. No assertion, input or fixture changed, and the
one edited line repoints `transform_fixtures_dir` at the sibling crate.

## Result

| Measure      | Value             |
| ------------ | ----------------- |
| Measurements | 28                |
| Median       | +1.90%            |
| Range        | −2.10% to +7.71%  |
| Slower       | 25 of 28          |
| Within ±4%   | 23 of 28          |

## No evaluator code changed, so no measurement here can be a code cost

This is settled by the diff rather than by the numbers. `git diff HEAD -M`
shows two of the three bench files byte-identical to their previous versions
and the third differing by one path expression. Not one line of
`crates/stylex-evaluator/src` is touched. Whatever the +1.90% is, it is not the
evaluator doing more work.

**The `EngineFoldRoundTrip/engine` legs are not the control this looked like.**
They hand source straight to a warm engine and enter none of the evaluator's
fold, so they do control for a source change -- but a source change was already
ruled out above. Against the mechanism that is actually in play they control for
nothing, because that mechanism reaches them too. Their +4.23% beside the `fold`
legs' +1.21% is one more reading of the same shift, not evidence about it.

## What the shift is: a smaller link graph

25 of 28 legs are slower. Under a sign test that is p≈2e-4, so the binary
really is about 1.9% slower and calling it noise would be wrong.

A bench target links its own crate and that crate's dependency subgraph. Before
the move each of these binaries linked `stylex_transform` and roughly twenty
crates behind it -- css, css-parser, styleq, path-resolver, codegen,
state-index -- to call `stylex_evaluator::evaluate`. After the move each links
the evaluator's subgraph alone. Both crates are `rlib` only, so ticket 13's
`cdylib` hazard cannot recur and fat LTO gets the bitcode either way; but it is
a different unit of it, with a different inlining budget and different function
placement. That is the whole of the 1.9%.

## Consequence: the pre-move series for these three benches is closed

These ids now describe a link configuration the shipped `.node` does not have.
The `.node` still links `rs-compiler → transform → evaluator`, which is what the
before leg resembled. **Discard the pre-move criterion baselines for these three
benches rather than diffing against them**, and read the numbers from here on as
a fresh series. Ratios inside one leg -- `fold` against `engine`, one depth
against the next -- carry across the move; absolute times do not.

This is the price the ticket accepted knowingly. Profiling the evaluator on its
own is what the move is for, and a bench that links only the evaluator is a
sharper instrument for that even though it is a worse model of the shipped link.

## Two outliers, both accounted for

`StructuralKeyFallback/object/128` reads +7.71% where its `/129` twin reads
−0.84%, and the pair runs the same code down to one extra property. Ticket 13
recorded that same leg swinging 15 points between configurations, so it is the
noisiest measurement in the set. `EngineFoldRoundTrip/engine/array-answer` at
+6.54% executes no evaluator code at all.

Each group is n=4 at `--sample-size 20`, measured once. That is enough to
establish the uniform direction across all 28 and too little to separate any two
groups from each other; no claim above rests on a between-group gap.

Logs: `bench-09-before.log`, `bench-09-after.log`.
