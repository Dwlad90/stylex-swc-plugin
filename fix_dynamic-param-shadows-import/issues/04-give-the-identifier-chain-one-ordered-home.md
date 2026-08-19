# 04 — Give the identifier chain one ordered home

Status: `ready-for-agent`
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

- [ ] The chain lives in one module, one ordered function, steps in upstream
      order with their upstream line ranges cited
- [ ] The binding-write set is split in two, filled by the same walk
- [ ] No snapshot changes, no corpus verdict changes, no message text changes
- [ ] ADR recorded beside the existing ones in this crate
- [ ] Glossary entry naming the seam
