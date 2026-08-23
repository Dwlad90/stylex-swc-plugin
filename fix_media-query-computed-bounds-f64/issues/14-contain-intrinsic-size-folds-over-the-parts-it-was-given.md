# 14 — `contain-intrinsic-size` folds over the parts it was given

**What was built:** the `containIntrinsicSize` expansion reduces the value's
parts, and it now reduces the parts rather than a four-sided view of them.

`auto` is a qualifier in this property, not a size: `auto 1px` means "1px,
remembered", so the two belong to one axis and the expansion folds them into a
single part before assigning axes. The fold was running over
`split_value_required`, which repeats a missing side to fill four — so a
one-part value arrived as four copies of itself and each copy joined the one
before it:

| authored | official compiler | this crate |
| --- | --- | --- |
| `auto` | `auto` / `auto` | `auto auto` / `auto auto` |
| `300px auto` | `300px` / `auto` | `300px` / `auto 300px` |

The second is the same cause read from the other end: the repeated fourth side
handed the trailing `auto` a size to swallow that nobody wrote.

**How it was found.** Not by a ticket. It was every *value-spelling* divergence
the shorthand split fuzz still reported once tickets 12 and 13 landed — 306
subjects, all of them this property and these two shapes. The existing unit
tests asserted the expansion's *shape* (two pairs, named right) and never its
text, which is how a wrong value sat under passing tests.

A review of this ticket found a third shape the table above misses, from the
same predicate: the fold skipped an *empty* part where upstream skips only an
absent one, and no part of a split value is absent. An unterminated comment
contributes an empty part, so `containIntrinsicSize: 'auto /*'` sized only the
width where upstream sizes both axes.

**Blocked by:** 12, 13 — not logically; the divergence was simply unreadable
underneath theirs.

**Status:** done

- [x] Both rows match the official compiler, confirmed against
      `@stylexjs/babel-plugin` from `node_modules`
- [x] The fold reads the part list, and the part list is a named function rather
      than a second parse — `value_parts`, which `split_value` is now built from
- [x] The axes are asserted by text, not only by property name, so the next
      wrong value fails a test
- [x] The fuzz reports no remaining value-spelling divergence: 306 -> 0
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

**What the fuzz still reports, and why it is not this.** 7023 of 122232
subjects, every one of them `acceptance-divergent` and every one the same
class: this compiler refuses a value containing `;` and upstream compiles it.
That refusal is deliberate — the guard exists so a value cannot terminate its
own rule and splice arbitrary CSS into the stylesheet — and it is described in
`parity/README.md` beside the harness. A value nested past 64 is refused the
same way and for the same kind of reason. Neither is a splitting defect, and
neither is in the counts above, which are value-spelling only.
