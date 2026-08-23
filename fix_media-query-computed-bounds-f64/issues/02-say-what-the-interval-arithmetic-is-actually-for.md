# 02 — Say what the interval arithmetic is actually for

**What to build:** a maintainer reading the media query interval merge is told
the truth about why it computes in double precision. The current doc comment
justifies the width purely as comparison hygiene and states that emission
narrows back to single precision — which stops being true once 01 lands, so the
comment actively misleads the next reader.

The companion guard that refuses a ceiling the boundary cannot represent was
added for the single-precision world. Re-examine it against its own test:
remove it only if the test still passes without it, keep it if the
representability limit was guarding something that survives at double
precision. Either way the finding is reported rather than decided silently.

**Blocked by:** 01.

**Status:** done

- [x] The interval-merge doc comment describes the arithmetic's actual purpose
      and no longer claims emission narrows
- [x] The ceiling guard is either kept with a stated reason or removed with its
      test shown to still pass
- [x] The finding — kept or removed, and why — is recorded in the ticket's
      closing note
- [x] `cargo test` passes with no behaviour change

## Closing note

Delivered.

**The commit this ticket names is the wrong one.** `5fb8dcfa1` ("refuse a
ceiling the boundary cannot represent") is about `maxEvaluationDepth` crossing
the NAPI boundary and touches no media query code. The commit that added the
double-precision interval arithmetic is `043a1de9c` ("keep the negated bound
nudge out of f32"), and that is what was re-examined.

**The guard: absorbed, not removed.** It was the temporary widening
(`f64::from(length.value)`) the merge did for the length of a comparison. With
`Length` holding a double, the conversion is gone and the width comes from the
field. Nothing was deleted as dead: its test --
`negated_bound_at_a_width_f32_cannot_nudge_is_still_a_contradiction` -- passes
unchanged and stays, because it pins the behaviour rather than the mechanism
that delivers it. Its rationale was rewritten to describe the world it now
lives in.
