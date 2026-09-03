# 25 — Settle the coverage exclusions and correct the shrink claim

**What to build:** The split was meant to shrink the surface excluded from the
coverage gate. It grew instead. Two crates were added to the exclusion list,
one of which existed before this work and was gated and green — repopulating
it took a previously-gated crate off the gate, and that exclusion landed
inside the same commit that moved the code. The guidelines forbid adding an
exclusion without justification, and this repo's own issue-tracker convention
requires a partial acceptance criterion to name the ticket that closes it;
neither exclusion names one.

The spec's headline claim — that the excluded surface drops from roughly 32k
lines to roughly 20k, about a third — is not what happened. The measured
figures in the closing ticket are 34,304 to 31,744 lines and four to six
excluded crates, about 7%.

Make the exclusion list a bounded, documented exception and make the spec's
numbers true.

**Blocked by:** 24

**Status:** resolved

- [x] Each exclusion names the ticket that closes it, per the issue-tracker
      convention
- [x] Whether the evaluator's exclusion can now be removed is decided
      explicitly, not left implicit
- [x] The spec's line-count and proportion claims read the measured numbers
- [x] All three places that carry an exclusion list agree with each other
- [x] The workspace gate is green, including a coverage run that confirms the
      exclusion list is what this ticket settled on

## Answer

### The exclusion list is now a bounded exception

`guidelines/STRUCTURE.md` splits the six rows into two groups. Four are
permanent and state why. Two are temporary and name the ticket that removes
them, `11-cover-the-state-crate` and `15-cover-the-evaluator-crate`, together
with the reason both arrived: each crate was covered *transitively* through the
transform, which is itself off the gate, and the crate boundary is what stopped
that counting. No line that was covered became uncovered.

The rule is stated as a rule, not as a description: a row is either permanent
with a reason, or temporary with a remover. Both scripts that hold a copy of the
list now carry the same split in a comment and point at `STRUCTURE.md` for it,
so a reader who reaches the list through a script is not sent looking.

### The evaluator's exclusion stays, and here is the number

Re-measured on the tip of `feat_split-transform-crate`:

| Crate | Regions | Functions | Lines | Unexercised regions |
| --- | ---: | ---: | ---: | ---: |
| `stylex_evaluator` | 66.86% | 75.22% | 70.93% | 2347 across 28 files |
| `stylex_state` | 43.71% | 45.00% | 42.38% | 1486 across 5 files |

The gate is zero uncovered lines and zero uncovered regions. Neither crate is
near it, so neither can leave the list before its ticket lands. That is the
explicit decision the criterion asked for, and both figures are recorded on
tickets 11 and 15, whose own headline numbers were taken before ticket 24 and
were a few points stale.

### There were four lists, not three; now there are three

`test:coverage:workspace` in the root `package.json`, `EXCLUDED_CRATES` in
`scripts/coverage-missing.sh` and the `case` in
`scripts/packages/test/coverage.sh` already held the same six crates, and still
do -- no crate is added or removed by this ticket.

The stale copy was a fourth one nobody had counted: the illustrative
`cargo llvm-cov` invocation in `STRUCTURE.md` still excluded
`stylex_css_parser`, which is on none of the three lists, and omitted
`stylex_state` and `stylex_evaluator`, which are on all three. A reader copying
that command would have run a different gate from CI's.

Repairing that copy would have left the same trap armed, and the file two
paragraphs below would still have claimed there are three lists. The copy is
deleted instead: the bullet now names `pnpm test:coverage:workspace`, describes
the flags in prose and says to run the script rather than a copy of it. Four
enumerations become three, and "three lists" is true again.

### What the spec claimed, and what happened

The headline claim was that the excluded surface drops from ~32k lines to ~20k,
about a third. That read the *transform's* own shrink as if it were the whole
excluded surface.

| Measurement | Baseline `e8887ab8f` | Now | Change |
| --- | ---: | ---: | --- |
| `stylex-transform` source lines | 32327 | 13858 | -57% |
| Lines the gate never measures | 34304, 4 crates | 31744, 6 crates | -7% |
| Same, once tickets 11 and 15 land | 34304, 4 crates | 15835, 4 crates | -54% |

The transform beat its target; the excluded surface did not, because two of the
crates it shed took temporary exclusions of their own. Both readings are now in
the spec: user story 5 carries the measured numbers and an amendment explaining
the arithmetic error, the "Expected end state" note contrasts plan against
measurement, and the "Modules under test" amendment -- which named only
`stylex-state` -- now names `stylex-evaluator` as well, which was the second
crate the rule it amends was breached for.

Line counts use the gate's own view: every `*.rs` under `src/` less any path
matching `(tests?|benches?|examples)/`. The 31744 and 34304 figures reproduce
ticket 10's exactly.

### Gate

`pnpm test:coverage:workspace` green at 100.00% of regions, functions and lines
over the six-crate exclusion list this ticket settled on. Workspace check,
clippy, test and doc-test green; format, lint and shellcheck green.


## Comments

**Review, same day.** Three reviewers ran over the change: standards, spec and
performance. The spec reviewer re-derived all five line counts independently and
every one matched. The performance reviewer confirmed the inline comments inside
the bash array expand to exactly the six crate names, with no comment text in
any element, and found no cache or runtime effect. Four findings were acted on:

1. **The "three lists" claim stayed false.** `STRUCTURE.md` held a fourth copy
   of the list -- the very copy this ticket had to repair -- and the prose below
   it still said three. Deleting the copy, rather than repairing it, is what
   makes the claim true and stops the same rot recurring.
2. **`coverage.sh` carried a false rule.** The comment said the list uses
   hyphens where the other two use underscores. That holds for five of six
   crates; `stylex_compiler_rs` is the directory `stylex-rs-compiler`, reordered
   rather than de-underscored, so a reader applying the rule looks for
   `stylex-compiler-rs` and finds nothing. The comment now names the exception.
3. **The guideline quietly legalised a backlogged remover.** The spec requires a
   temporary exclusion whose "immediate follow-up removes" it. The first draft of
   the rule dropped "immediate", so an indefinite deferral would have passed.
   The rule now says a temporary row is an exception rather than a deferral, and
   must say why the coverage could not travel with the code.
4. **The guideline hard-coded tracker state.** It said both tickets are
   backlogged, which rots the moment either moves and duplicates state the
   tracker owns. It now points at the tracker for the state.

One finding was heard and not acted on. Both reviewers read the inline
`# permanent` / `# temporary` annotations in `EXCLUDED_CRATES` as a duplicate
that will go stale. The annotation stays there and only there: that array is
the list a maintainer edits, one crate per line, so the note sits on the row it
describes and cannot drift onto another. The `case` in `coverage.sh` is one
pipe-joined pattern where no note can attach to a row, so it carries a pointer
only. Classification therefore lives in two places, down from the three the
first draft had, and the reasons live in one.