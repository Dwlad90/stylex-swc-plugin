# 12 — Whitespace between echoed tokens is echoed too

**What to build:** an author who writes a value the shorthand expansion path
passes through gets their spacing back, not a re-spacing of it. `parse_css`
discards whitespace tokens and `join_css` re-inserts spacing by rule, so the
authored spacing is replaced by a canonical one -- and it diverges in *both*
directions:

| authored | official compiler | this crate |
| --- | --- | --- |
| `calc(1.50px*2)` | `calc(1.50px*2)` | `calc(1.50px * 2)` |
| `calc(100% / 3)` | `calc(100% / 3)` | `calc(100%/3)` |
| `min(1px , 2px)` | `min(1px,2px)` | `min(1px,2px)` |
| `calc(2px-1px)` | `calc(2px-1px)` | `calc(2px-1px)` |
| `calc(1px  +  2px)` | `calc(1px + 2px)` | `calc(1px + 2px)` |

Because the emitted text feeds the class-name hash, rows one and two are
class-name divergences: `.xkqwiw` against `.x1aash7n`, and `.xxol412` against
`.xfffg4`.

**Blocked by:** None — can start immediately.

**The first draft of this ticket had the mechanism half wrong, and the wrong
half was load-bearing.** It said `join_css` suppresses the space around a slash
and a comma, and that deleting both exceptions was the fix. Measured, they are
not the same:

- The **comma** carve-out is faithful. Upstream drops a separator node's
  surrounding space too, so `min(1px , 2px)` matches today. Deleting it would
  *introduce* a divergence.
- The **slash** carve-out is itself a divergence, in the opposite direction.
  Inside `calc()` upstream emits no separator node and echoes the spacing, so
  suppressing it turns `calc(100% / 3)` into `calc(100%/3)`.

So the fix is not "delete the exceptions". It is: echo the authored spacing,
and keep the comma behaviour because upstream also normalizes there --
justified rather than deleted.

**Prior art, and its limit.** Ticket 11 echoes a numeric literal by looking up
its span from the token's offset. Whitespace is not a span this crate keeps at
all -- it is filtered before `join_css` runs -- so the fix is a different one:
either carry the whitespace tokens through, or drive emission from source spans.
Establish which before writing either.

**How it was closed.** Neither of those two. The splitting path no longer runs
on this crate's own token walk at all: it runs on the value scanner already in
the workspace, which is the same library upstream splits on. Echoing the
authored spacing is then not a feature that had to be written -- the scanner
never discarded it -- and the whole `parse_css`/`join_css` token walk was
deleted rather than taught to preserve spans. Ticket 13 was closed by the same
change, which is why they share a commit.

**Not in this ticket.** The top-level splitting bugs -- a `/` or `:` emitted as
a value, a delimiter splitting a value upstream keeps whole, `!important`
unrecognised -- are ticket 13. They are a different defect that happens to live
in the same function, and they are more severe: they emit text that is not a CSS
declaration.

**Status:** done

- [x] Every row of the table above matches the official compiler, confirmed
      against `@stylexjs/babel-plugin` from `node_modules`
- [x] The comma behaviour is kept with its reason stated, not deleted alongside
      the slash -- it is not a carve-out any more but a consequence: a comma
      inside a function is a separator node carrying the whitespace on either
      side of it, and printing the node prints only the comma
- [x] Ticket 11's escaped-unit divergence is closed with this, not absorbed:
      `1\70x` comes back as `1\70x`. The unit used to be read off the token
      rather than the source, which printed what it escapes to
- [x] The fuzz that found this is re-run and its alphabet reported --
      `crates/stylex-rs-compiler/parity/fuzz-shorthand-split.ts`, 33 token
      classes x 14 joiners x 8 properties = 122232 subjects. Value-spelling
      divergences went 73505 -> 0
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
