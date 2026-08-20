# 04 — Give the identifier chain one ordered home

Status: `resolved`
Blocked by: 02, 03

**What to build:** Nothing, from the outside. Every input that compiles today
compiles the same way afterwards, and every refusal keeps its text.

This is the prefactor that makes 05, 06 and 07 easy changes. Resolving an
identifier to its binding is one ordered question with one answer, and it is
currently spread across the identifier arm of the evaluator's dispatch as a
sequence assembled ad hoc. Give it its own module and put the steps in the
reference implementation's order, each carrying the upstream line range it
mirrors:

1. import specifier → theme reference
2. default-import specifier — 06 decides whether this step exists
3. binding reassigned → not a constant
4. binding mutated in place → not a constant
5. used before declaration → from 03
6. the binding carries a folded value — deliberately absent; the reference
   implementation only ever sets that field through an API this plugin never
   calls. A comment, not code.
7. `undefined` / `Infinity` / `NaN` — 05 changes this step
8. resolve the declaration initializer, else the class/function/undefined
   refusals

Two things make this safe to do as a pure move. The reorder is *inert*: once
resolution is binding-aware, at most one of {import specifier, declaration}
matches a given reference, so moving the initializer read after the import check
changes no outcome. And the class and function declaration refusals already
match the reference implementation's text byte for byte, so they move unchanged.

Steps 3 and 4 are spelled as two sequential probes even though one binding-write
set answers both today. Split that set in two — reassignments and in-place
mutations — filled by the same single walk, so each step probes what the
reference implementation probes. Both refuse with the same text, so the split
changes no outcome; it costs one hash set and buys a line-for-line mapping. The
documented escape-into-a-call unsoundness carries over verbatim, including the
reason it is accepted.

Lands with the decision recorded as an ADR beside the two existing ones in this
crate — they are decisions about how this same evaluator refuses things, and the
chain's order is exactly what a future reader would otherwise re-litigate — and
the seam's name in the crate glossary.

- [x] The chain lives in one module, one ordered function, steps in upstream
      order with their upstream line ranges cited
- [x] The binding-write set is split in two, filled by the same walk
- [x] No snapshot changes, no corpus verdict changes, no message text changes
- [x] ADR recorded beside the existing ones in this crate
- [x] Glossary entry naming the seam

## Comments

Landed as `26bc51018`, with the one measurement it turned up as `34859c8af`.

`resolve_reference` in `shared/utils/js/evaluate/binding.rs`. The upstream line
ranges cited there are `599-650 / 652-654 / 656-658 / 660-662 / 664-666 /
668-669 / 670-683 / 685-690`; the spec's were each one line low, and the spec
has been corrected so tickets 05 and 06 do not inherit stale numbers.

`ModuleBindingsCollector` now classifies each write as it records it -- a
`WriteKind` that flips to `Mutation` on the first member hop, so `n++`
reassigns and `o.n++` mutates, out of the walk that already ran. Two sets on
`StateManager`, two sequential probes in the chain, each spelled probe-first so
the guard's scan of the declaration list runs only for a name some write was
recorded against and step 4 costs nothing once step 3 has refused.

Verification: `cargo test --workspace --all-features` 6206 passed / 0 failed,
`cargo clippy --workspace --all-features --all-targets` clean, `cargo fmt` clean,
`pnpm typecheck && pnpm lint:check && pnpm format:check` green, `parity --set
modules` 0 changed verdicts over 56 subjects. No snapshot, fixture or message
text changed.

### The reorder is not inert -- one pair, measured both ways

The checkbox above claims no corpus verdict changes, and that holds for every
recorded entry. But the audit for it turned up one input the reorder *does*
change, which the ticket's "every input that compiles today compiles the same
way afterwards" did not cover:

```js
import { zIndex as NaN } from 'zIndex.stylex.js';
```

`NaN` is a legal binding name, so this is the only shape where an import
specifier and one of the three folded globals name the same binding -- and a
`SyntaxContext` cannot separate them, because they *are* one binding. Moving the
globals step behind the import step changes which one answers.

Measured on both compilers rather than argued: the shape reads `acceptance
divergent` at `0f9d2bcd2` and `identical` at `26bc51018`. So the reorder closed
a divergence rather than opening one, which is the outcome upstream's order was
adopted for. Recorded as `modules-1266-import-aliased-to-a-global-name` with the
verdict pinned, plus a unit test over all three global names, and stated in ADR
0003 rather than left as a footnote here.

### A narrowing carried over, not introduced -- worth its own ticket

Steps 3 and 4 guard on there being a `VarDeclarator`, where upstream asks
whether a *binding* exists at all. So a hoisted `function` or `class` that is
reassigned falls past both probes and is refused for its declaration kind
instead of for the write. Both compilers refuse, only the text differs, and the
behaviour is exactly what it was before this commit -- but the step now claims a
line-for-line mapping, so the narrowing is written down at the step and pinned by
a test rather than left for a reader to notice. Closing it is a message change
and belongs to a ticket of its own.

### Left undone, deliberately

`parity/corpus/harvested.json` is still stale, and `pnpm test` still fails on
`@stylexswc/postcss-value-parser` for that reason -- reproduced identically at
`0f9d2bcd2` with this work stashed. Ticket 03 recorded the same thing; it is
still worth its own commit and still is not this one's.
