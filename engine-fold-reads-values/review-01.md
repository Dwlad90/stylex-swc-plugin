# Code review — ticket 01

Reviewed: the working tree against `ee47b0359` (`git diff HEAD` plus the two
untracked files, 5 files). Two axes, run as independent sub-agents so neither
could rerank the other: **Standards** (this repo's documented standards, plus
the Fowler smell baseline) and **Spec** (does it match what ticket 01 asked
for).

Status: reviewed, findings triaged below. Every row marked **fixed** was applied
before the change was committed.

## Standards

No hard violations. `guidelines/PERFORMANCE.md` was checked rule by rule — the
paired fixture registration, the `GLOBALS.set` scope, "assert what the bench is
measuring", "a fixture must compile on the revision before the change", and the
manifest shape — and all hold.

| # | Finding | Verdict |
| --- | --- | --- |
| S1 | Data Clumps / Primitive Obsession: `LEGS` is a `&[(&str, &str, &str)]` whose three strings travel together, positionally, through the assertion and both loops. | **fixed** — a three-field `Leg` struct, each field with the sentence that says what it is for |
| S2 | Duplicated Code: `fn parse` is byte-identical to `evaluate_depth_bench.rs`'s. | **kept** — five benches each carry their own; there is no shared bench-helper module, and introducing one for this diff is the wrong place to start |
| S3 | Duplicated constant: `MAX_LOOP_ITERATIONS` restates `engine_fold.rs`'s private one, and nothing fails if the two drift. | **kept** — the doc comment already argues it: reaching into the module under measurement would measure a different engine than the one that ships. The drift risk is real and unfixable without widening a private API for a bench |
| S4 | `Cargo.toml`'s `[[bench]]` list is alphabetical; the new entry went in at position two. Taplo does not sort array-of-tables. | **fixed** — moved first |

## Spec

| # | Finding | Verdict |
| --- | --- | --- |
| P1 | **Checkbox 2 was only half met.** The cold-start legs asserted `Ok(_)` and nothing more, so a context answering `undefined` for every leg would have passed and still reported ~120 µs — the exact failure mode the checkbox exists to stop, on the one group that reaches no fold. | **fixed** — `assert_engine_answers` compares the engine's own `String(value)` against the leg's recorded answer. One recorded answer covers both sides, because `fold_text` now renders an array the way the language renders it |
| P2 | The Answer called the `fold`/`engine` gap "the guard walk, the print and the conversion back". The `fold` leg enters through `evaluate`, so it also carries the evaluator's entry cost. The gap is an upper bound. | **fixed** — bench header, the group's doc and the Answer all say "at most" and name what else is inside it |
| P3 | Cold start was one measurement reported four times: 114-126 µs across legs whose warm engine cost spans 2.3-10.7 µs, which is context construction and nothing else. Four columns implied a leg-dependence that is not there. | **fixed** — one leg, `build-and-answer`, with the per-shape spread recorded in the comment that justifies dropping the other three |
| P4 | The one non-noise number in the branch/base table, `array-answer`, came from a bench source that differs from the committed one. | **fixed** — the Answer now carries the arm to add, so the column is reproducible. The arm stays out of the committed bench because on this branch it is unreachable, which is the smell S3 of the 02-04 review deleted once already |
| P5 | Checkbox 1 says the benchmark "runs inside the global scope the transform requires"; the cold-start group does not. | **not a finding** — that group is the engine alone and reaches no `Mark::new()`. Both the header and the group say so |
| P6 | "identical to the reference compiler" is asserted by no test. | **kept, handed on** — measured once by hand and now labelled as such in the Answer. The corpus that would pin it is ticket 14's |

## What the two axes agreed on

Nothing, and that is the useful part here. Standards read the diff as a shape
problem — a tuple that should be a type, a list out of order — and Spec read it
as a claims problem — three sentences in the Answer that the code did not
support. The one finding neither reached from the other's direction, P1, is the
one that mattered: a guard that looked present, was named in a checkbox, and
would not have fired.

## Correction to this review's own first pass

P3 was going to be argued rather than measured — "four legs is more information
than one". The per-shape numbers were already on hand and said the opposite. The
same trap the 02-04 review ended on: closing a finding by explaining instead of
by measuring.
