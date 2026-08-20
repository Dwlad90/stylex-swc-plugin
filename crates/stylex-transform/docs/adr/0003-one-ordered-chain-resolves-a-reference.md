# One ordered chain resolves a reference, in the reference implementation's order

**Status:** accepted

Resolving a reference to the binding it names is one question with one answer,
and the evaluator used to answer it as a sequence assembled where each step
happened to be needed: the write probe first because it was the cheapest, the
declarator read next because it was the common case, the import specifier last
because it was added last. Every step was right on its own and the order was
nobody's decision.

It now lives in `shared/utils/js/evaluate/binding.rs` as one function, and the
order is `evaluate-path.js` 0.19.0's, each step citing the line range it mirrors.
The order is the load-bearing part: this compiler and the reference
implementation agree on every step's verdict, so the only way left for them to
disagree about an input is by asking two applicable questions in a different
sequence. A citation beside each step is what lets that be checked by reading
rather than by measuring.

Two properties make the reorder safe to perform as a pure move, and both are
worth stating because neither is obvious.

**At most one step can answer — with one exception, measured.** Resolution is
keyed on the full SWC `Id`, so a reference matches an import specifier or a
declarator, never both: a shadowing binding carries a syntax context of its own.
Moving the declarator read behind the import check therefore changes no outcome
on anything a parser can produce. The unit tests beside the chain assemble the
collision anyway, with an import and a declarator sharing one context, because
"inert" is a claim about the order and that is the only place it is visible.

The exception is the `undefined` / `Infinity` / `NaN` step, which a syntax
context cannot protect, because those names are ordinary bindings to the
language: `import { zIndex as NaN }` makes one import specifier and one global
name the same binding, and the reorder puts the import ahead of the global. That
one pair changed, and it changed toward the reference implementation — measured
on both compilers before and after, and recorded in the parity corpus as
`modules-1266-import-aliased-to-a-global-name`, which reads as a divergence at
the commit before this one and as agreement at this one. It is the whole extent
of the reorder's observable effect.

**The steps that would change output were left changing nothing.** Two of the
reference implementation's steps had no counterpart here when the chain was
assembled — a default-import refusal and the `undefined` / `Infinity` / `NaN`
refusal. Both were reachable divergences and both were somebody else's commit.
They sat at their upstream positions carrying a comment and, for the globals, a
guard that reproduced what the old order already did, so that change was a move
and the next one a decision.

The globals step has since been decided, and asks what the reference
implementation asks: whether a binding exists for the reference, refusing when
one does. That is what a
[declared binding](../../CONTEXT.md) is for, and the
question is answered from the pre-scan's `Id`-keyed set rather than from the
declaration list — because the binding the step is really about is a dynamic
style's parameter, which leaves no declarator behind for a list to hold. Keying
by `Id` is what keeps the answer scope-aware without a scope chain: the resolver
gives the parameter a context of its own, so a reference to the global `NaN`
elsewhere in the same module matches nothing.

## Considered options

**Leave the sequence where it is and document it.** The order would still be
correct, and the next step added would still be added wherever it was needed. A
comment saying "these run in this order" is not a place; it does not tell a
reader where step 4 goes.

**A scope tree, resolving references the way Babel's `path.scope` does.** The
honest mirror of the reference implementation, and rejected: the SWC resolver
already runs ahead of this pass and already makes `SyntaxContext` authoritative
for shadowing. A hand-rolled scope tree would be a second answer to the one
question the resolver exists to answer, and the two would drift.

**One write probe instead of two.** The two probes — a rebound binding and a
value mutated in place — refuse with the same text, so a single set answers both
and the split buys no outcome. It buys the mapping: upstream asks
`constantViolations` and then `isMutated`, and a chain where one step stands for
two of theirs is a chain a reader has to hold a correction in mind while reading.
The cost is one extra `FxHashSet`, filled by the walk that already ran.

## Consequences

**Adding a step means placing it.** The upstream line ranges give every step a
position, so a new one goes where its counterpart sits rather than where it is
convenient — and a step with no upstream counterpart has to say so.

**A missing step is visible as a missing step.** The absent ones read as gaps in
a numbered sequence with a comment explaining the gap, not as steps nobody
thought of. Their tickets are named at the gap.

**A write is now classified at collection.** `ModuleBindingsCollector` records
which kind of write it saw — a reassignment or an in-place mutation — rather than
only that it saw one. The classification falls out of the walk it already
performed: crossing a member hop makes the write a mutation, because the root
binding keeps pointing at the same value.

**The escape-into-a-call unsoundness carries over unchanged.** Only writes
spelled out syntactically in the module are recorded; `const a = []; mutate(a);`
is not one. Deopting on every identifier passed as an argument would disable
evaluation for nearly every StyleX module, so it stays a known unsoundness,
accepted rather than overlooked, and it is accepted identically by both probes.
