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

**Status:** ready-for-agent

- [ ] `pnpm parity` reports zero rows that are neither agreement nor a recorded
      expectation, and `changed` stays 0
- [ ] Each pinned entry carries the reason the divergence is permanent, in
      terms of what agreement would cost — not "known difference"
- [ ] The refusal families are named once, somewhere a later harness can reuse
      the same names rather than inventing its own
- [ ] A deliberately broken expectation is shown to report loudly, so the gate
      is demonstrated rather than assumed
- [ ] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
