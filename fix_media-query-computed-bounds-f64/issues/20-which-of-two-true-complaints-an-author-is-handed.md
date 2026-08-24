# 20 — Which of two true complaints an author is handed

**What to build:** a value with more than one fault earns the same complaint
from both compilers, so a refused build reads the same whichever compiler
stopped it.

This compiler asks whether a generated declaration carries a `;`, `{` or `}`
before it runs the guards it shares with the reference compiler. So
`padding: '/*;calc(1px'` — rule-breaking *and* unclosed — is refused here for
the token and there for the unclosed function. Both refuse, so no output
depends on it; what differs is the whole of what an author whose build stopped
is handed.

Found by the generated corpus, which reaches 254 rows of it across two shared
guards (unclosed function, unclosed string). Pinned as the `first refusal to
fire` family in `crates/stylex-rs-compiler/parity/lib/refusal-families.ts`, and
pinned in the curated corpus by two `edge.json` entries, so it is accounted for
rather than unread.

Closing it means running the local injection guard *after* the guards the
reference compiler also has, which is a change to diagnostic order and not to
any emitted value. The risk to weigh is the other direction: a value whose only
fault is the token would then be refused by a later pass, and every curated row
that currently reads `declaration-terminating token` has to still read it.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] `padding: '/*;calc(1px'` and `padding: '/*;"a'` read `both-reject` rather
      than `both-reject-divergent`, in both harnesses
      — verified under `legacy-expand-shorthands`, which is the resolution the
      generated corpus these two were quoted from runs at: the shorthand is cut
      at the semicolon, so the part carrying `calc(` is not the part carrying
      the `/*`. Under the default resolution the same two strings read
      `acceptance-divergent` instead, because the whole value is one unclosed
      comment and that guard has to speak before anything parses it. That is
      the pre-existing `unclosed comment` divergence, which the reference
      compiler answers by emitting the `/*`, and is not what this ticket moved.
      The curated corpus asks the same question at default resolution through
      `color: 'red;calc(1px'` and `color: 'red;"a'`, and both read
      `both-reject`.
- [x] Every row the `declaration-terminating token` family accounts for today
      still reads that refusal, so the reorder buys agreement without trading a
      diagnostic away
- [x] The `first refusal to fire` family is removed once nothing reaches it —
      a family claiming no row is already a failure, so it cannot be left behind
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
