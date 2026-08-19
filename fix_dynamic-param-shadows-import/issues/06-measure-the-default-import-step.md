# 06 — Measure the default-import step, then mirror it or rule it out

Status: `ready-for-agent`
Blocked by: 04

**What to build:** A verdict on step 2 of the chain — either the step, or a
comment saying why there is no step.

The reference implementation refuses a reference that resolves to a *default*
import specifier, with a distinct message. We treat one as a theme reference
like any other. Our message constant for it is commented out alongside the two
that 03 and 05 revive — but unlike those two, there is no measured divergence
behind it yet.

So measure first. Put a default import of a theme file through both compilers
and compare. If the outputs differ, mirror the step and revive the constant. If
they agree, leave the step out and record *at the site* that the difference is
deliberate and what was measured — an absent step with no explanation is what
invites the next reader to add it speculatively.

Either outcome is a complete ticket. The deliverable is the verdict, not the
code.

- [ ] Both compilers measured on a default theme import, result recorded
- [ ] If they diverge: the step lands, the constant is revived, corpus entry
      added with the verdict it reads
- [ ] If they agree: a comment at the step's position saying so, and the
      agreeing case added as a corpus guard
