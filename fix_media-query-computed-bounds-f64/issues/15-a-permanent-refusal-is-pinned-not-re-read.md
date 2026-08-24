# 15 — A permanent refusal is pinned, not re-read

**What to build:** `pnpm parity` reads zero unexpected rows, so that a reader
who does not already know the corpus can tell a new divergence from a
permanent one.

Today it prints 32 rows that are neither agreement nor regression, and every
one is a refusal this compiler makes on purpose. Grouped by why:

| rows | refusal |
| --- | --- |
| 19 | `;`, `{` or `}`, which would terminate the declaration |
| 5 | an unclosed comment, which would swallow the rules after it |
| 3 | an unprefixed custom property — a StyleX rule, not a CSS one |
| 1 | a value nested past the recursion budget |
| 3 | the reference compiler throwing a `TypeError` of its own |
| 1 | `toString: "notfn"`, where the reference emits one rule per character |

None of these is closable toward the reference compiler, and two of them
should not be: agreement on the first row means emitting CSS that escapes its
own declaration, and agreement on the fifth means reproducing a crash.

The corpus already has the mechanism — `expected` on an entry, and the
`changed` count that reports when a pinned verdict stops holding. It is the
values that are missing. The type's own documentation makes the argument:
"without it a permanent divergence and a new one print the same, so the corpus
can only be read by someone who already knows which is which."

**Blocked by:** None — can start immediately.

**Status:** done

- [x] `pnpm parity` reports zero rows that are neither agreement nor a recorded
      expectation, and `changed` stays 0
- [x] Each pinned entry carries the reason the divergence is permanent, in
      terms of what agreement would cost — not "known difference"
- [x] The refusal families are named once, somewhere a later harness can reuse
      the same names rather than inventing its own
- [x] A deliberately broken expectation is shown to report loudly, so the gate
      is demonstrated rather than assumed
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Outcome

`parity/lib/refusal-families.ts` names the seven reasons this compiler diverges
on purpose — one entry each, carrying the verdict a member reads, the test for
whether a row is one, and the reason stated as what agreement would cost. Both
harnesses read that list.

Pinning is by family rather than by `expected` on an entry, because half the
permanent rows live in `corpus/harvested.json`, which is regenerated wholesale
from the Rust sources: a value written there is lost on the next harvest. The
per-entry mechanism is untouched and still wins where an entry carries one.

`pnpm parity` now reports 0 unexpected rows over 1029 subjects (35 pinned +
`changed` 0). Three curated entries were added to `edge.json` for the seventh
family, which only the generated corpus reached: two values that are both
rule-breaking and unclosed, where the two compilers name different true
complaints, and one declaration-terminating token inside a function body.

**Measured on the checked-in corpus, which is stale.** `parity:harvest:check`
was already failing before this work — 76 harvestable declarations are missing
from `harvested.json`, none of them from a test added here. Zero-unexpected is
therefore unproven for those 76. Regenerating also rewrites `cases.rs` in
another crate, so it is ticket 21 rather than a footnote to this one.

The gate: a family claiming no row at all exits non-zero, for the same reason a
changed `expected` does. An unexpected row deliberately does not — a divergence
nobody has looked at is information, and a corpus of degenerate values would
otherwise fail every run.

Demonstrated rather than assumed, which took a seam. The deciding half of the
harness moved to `parity/lib/report.ts` — where a row stands, the summary, the
family grouping, and `fails()` — leaving `parity-values.ts` to print. Twelve
tests in `__tests__/report.test.ts` exercise it end to end: a changed
expectation fails, an emptied family fails, an unexpected row does not, a
filtered run is not asked, and a reworded diagnostic un-pins its rows so the
unexpected count is what moves. `__tests__/refusal-families.test.ts` covers
the predicate itself and every near miss a family must *not* claim.
