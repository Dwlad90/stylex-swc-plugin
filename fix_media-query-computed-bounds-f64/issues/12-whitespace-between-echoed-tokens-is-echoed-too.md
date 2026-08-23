# 12 — Whitespace between echoed tokens is echoed too

**What to build:** an author who writes a value the shorthand expansion path
passes through gets their spacing back, not a re-spacing of it. `parse_css`
discards whitespace tokens and `join_css` re-inserts a single space between
every node, so the authored spacing is replaced by a canonical one:

| authored | official compiler | this crate |
| --- | --- | --- |
| `calc(1.50px*2)` | `calc(1.50px*2)` | `calc(1.50px * 2)` |
| `calc(100%/3)` | `calc(100%/3)` | `calc(100%/3)` |

The second row is not a typo -- it already matches, because `join_css`
suppresses the space around a slash and a comma. That is the shape of the bug:
the exceptions were added one delimiter at a time, and `*` never got one.

Because the emitted text feeds the class-name hash, the first row is a
class-name divergence: `.xkqwiw` here against `.x1aash7n` upstream, so output
from the two compilers cannot be mixed across an SSR and client boundary for
any value whose spacing this re-writes.

**Blocked by:** None — can start immediately.

**How it was found, which is the argument for the shape of the fix.** Ticket 11
made the *numbers* on this path echo rather than reprint, and its closing note
claimed one remaining divergence. A review then fuzzed the path and found this
in 633 of about 3950 cases. Adding an exception for `*` would pass the table
above and leave the same claim unearned for the next delimiter. The path either
echoes its input or canonicalizes it, and upstream echoes.

**Prior art, and its limit.** Ticket 11 echoes a numeric literal by looking up
its span from the token's offset. Whitespace is not a span this crate keeps at
all -- it is filtered before `join_css` runs -- so the fix is a different one:
either carry the whitespace tokens through, or drive emission from source spans
rather than from a node list. Establish which before writing either; the
existing `join_css` exceptions become dead once it echoes, and deleting them is
how the fix shows it is not another exception.

**Status:** ready-for-agent

- [ ] A value the expansion path passes through is emitted with the author's own
      spacing, including no spacing
- [ ] `calc(1.50px*2)` and every row of the table above match the official
      compiler, confirmed against `@stylexjs/babel-plugin` from `node_modules`
- [ ] The `join_css` delimiter exceptions are gone rather than extended, or the
      reason they survive is recorded
- [ ] A fuzz over the shape the review used is run again and reported, so the
      completeness claim this time is earned rather than asserted
- [ ] Ticket 11's escaped-unit divergence is either closed with this or
      explicitly left, not silently absorbed
- [ ] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
