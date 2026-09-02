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

**Status:** ready-for-agent

- [ ] Each exclusion names the ticket that closes it, per the issue-tracker
      convention
- [ ] Whether the evaluator's exclusion can now be removed is decided
      explicitly, not left implicit
- [ ] The spec's line-count and proportion claims read the measured numbers
- [ ] All three places that carry an exclusion list agree with each other
- [ ] The workspace gate is green, including a coverage run that confirms the
      exclusion list is what this ticket settled on
