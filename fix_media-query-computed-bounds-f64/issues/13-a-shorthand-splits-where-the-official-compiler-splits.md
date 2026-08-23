# 13 — A shorthand splits where the official compiler splits

**What to build:** a shorthand value is divided into parts the way the official
compiler divides it, so that expansion emits CSS declarations rather than
fragments of one. Today `parse_css` returns a flat token list and every
top-level token becomes a part, which for four kinds of input produces output
that is not a declaration:

| authored | official compiler | this crate |
| --- | --- | --- |
| `padding: '10px/1.5'` | `10px` / `1.5` | `10px`, `` / ``, `1.5` |
| `padding: 'a:b'` | `a` / `b` | `a`, `` : ``, `b` |
| `padding: '1px*2'` | `1px*2` (one part) | `1px`, `*`, `2` |
| `padding: '1px !important'` | `1px!important` | `1px`, `!`, `important` |
| `padding: 'A\42 C'` | `A\42` / `C` | `ABC` (one part) |

The first two emit `padding-inline-end: / ` and `padding-inline-end: : ` --
declarations whose value is a delimiter surrounded by spaces. A browser drops
them, so the shorthand silently loses half its sides. The third and fourth are
wrong in the other direction: a value upstream keeps whole is torn into parts,
and `!important` becomes a property value spelled `important`.

Every one is a class-name divergence as well as a correctness bug, so output
from the two compilers cannot be mixed across an SSR and client boundary for any
shorthand containing them.

**Blocked by:** None — can start immediately. Independent of ticket 12, which is
about spacing *within* a part; this is about where one part ends.

**How these were found.** A review of ticket 11 fuzzed 8854 shorthand cases
against `@stylexjs/babel-plugin@0.19.0` and found 372 diffs. Ticket 11's closing
note had twice claimed a smaller remainder from hand-picked probes; the alphabet
is what found these.

**Where the difference lives.** Upstream splits on `postcss-value-parser`'s node
kinds and filters the separator nodes, so a `/` is structure, not a value.
This crate returns strings with no kind attached, so the caller cannot tell a
separator from a value and treats both as parts. The fix likely means carrying
the kind out of `parse_css`, not special-casing characters at the call site --
`/` and `:` are separators at the top level and values inside a function, which
a character test cannot distinguish.

**Two review claims that did not reproduce, recorded so nobody re-chases them:**
quote style is not normalised (`content: "'a'"` and `fontFamily: "'a'"` both
match), and a top-level `+`/`-` is not mis-split (`calc(2px-1px)` matches).

**What the alphabet found that this ticket did not list.** Two more of the same
root cause, both worse than the rows above. A hex colour was read as an
identifier and re-serialised with its leading digit escaped, so `#007bff` was
emitted as `#\30 07bff` -- not the colour the author wrote and not a colour at
all. And an unquoted `url()` did not diverge from upstream, it *aborted the
compiler*: the token walk raised on it deliberately, and upstream compiles it.

**Status:** done

- [x] Every row of the table above matches the official compiler, confirmed
      against `@stylexjs/babel-plugin` from `node_modules`
- [x] No expansion emits a declaration whose value is a bare delimiter -- a
      separator is not a part at all now
- [x] `!important` survives on the part it qualifies, on every part rather than
      on whichever longhand was next in line
- [x] An escaped identifier keeps its escape, matching upstream
- [x] A separator is distinguished from a value by kind rather than by
      character, so `/` inside a function is unaffected
- [x] The fuzz is re-run and its alphabet reported -- see ticket 12's closing
      note for the alphabet and the numbers; the harness is
      `crates/stylex-rs-compiler/parity/fuzz-shorthand-split.ts`
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
