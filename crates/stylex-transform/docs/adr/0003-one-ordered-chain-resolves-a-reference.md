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

The default-import step landed later and shares that exception rather than
adding one: `import NaN from 'tokens.stylex.js'` puts a default specifier and a
global on one binding, and the step ahead answers, as it does upstream. Both
answers are refusals there, so only the text differs.

The namespace arm of step 1 landed later still, and it is the one specifier kind
that does _not_ take the exception: `import * as NaN from 'tokens.stylex.js'`
resolves nothing at step 1, so the globals step behind it is reached and refuses
for the binding. Both answers are refusals again, and again only the text
differs.

**The steps that would change output were left changing nothing.** Two of the
reference implementation's steps had no counterpart here when the chain was
assembled — a default-import refusal and the `undefined` / `Infinity` / `NaN`
refusal. Both were reachable divergences and both were somebody else's commit.
They sat at their upstream positions carrying a comment and, for the globals, a
guard that reproduced what the old order already did, so that change was a move
and the next one a decision.

Both have since been decided. The default-import step refuses, because the
divergence behind it was measured before the step was written rather than
assumed from the reference implementation's shape: a default import of a theme
file resolved to a theme reference here and emitted `var()` for a variable the
theme file does not define, where upstream refused. That is
`modules-1266-default-theme-import` in the parity corpus, `acceptance divergent`
before the step and `both reject` after it. The step asks about the _specifier_
rather than about the declaration, because one declaration carries a default and
a named specifier at once and the two steps give them opposite answers.

**A namespace specifier resolves nothing, and that was a decision, not a third
step.** Upstream's step 1 excludes an `ImportNamespaceSpecifier` alongside the
default one, but for a different reason and with a different consequence: the
step reads `importSpecifierNode.imported`, a field a namespace node does not
carry, so the exclusion is a guard on the step's input rather than a verdict on
the import kind — and unlike a default specifier, a namespace one is given no
refusal of its own. It falls through every step behind it and lands on the
terminal `UNDEFINED_CONST`.

Mirroring that was not obviously right, because this compiler accepted such an
import and refusing it breaks modules that compile today. What decided it was
measuring what those modules compile _to_. The namespace arm synthesized the
reference's own **local alias** as the export name, so `import * as tokens from
'vars.stylex.js'` folded `tokens.color` to a variable hashed from
`vars.stylex.js//tokens.color`, while the file's exported group is `vars` and
the variable it defines is hashed from `vars.stylex.js//vars.color`. The two
were compared against the reference implementation's own output for the theme
file. The consequences, all measured and all in the parity corpus:

- a module reading one token through a named import and through a namespace
  import at once emitted **two different custom properties** for it
  (`modules-1266-a-namespace-theme-import-beside-a-named-one`);
- the spelling the namespace form actually calls for, `tokens.vars.color`,
  folded to a third variable nothing defines
  (`modules-1266-a-namespace-theme-import-read-through-its-group`);
- the only spelling that agreed with the theme file was an alias spelled like
  the exported group, which is a coincidence rather than a resolution
  (`modules-1266-a-namespace-theme-import-aliased-to-the-export-name`).

A `var()` nothing defines renders as nothing and reports as nothing, so what the
arm was preserving was a silent wrong render, not a capability: the same
variable is reachable through the named import both compilers resolve. So the
arm gives up the resolution and takes upstream's fall-through, which also
replaces two refusals given for the wrong reason — a namespace import of a
non-theme file read as a path-resolution failure, and a namespace group read
where a value belongs read as a shape the value position rejects.

The globals step asks what the reference
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

**Keep resolving a namespace theme import, and record why.** The option the
namespace arm's ticket opened with, and the one that reads as the conservative
choice: it accepts modules upstream refuses, and refusing them is a breaking
change. Rejected on the measurement rather than on the parity argument — the
resolution being given up hashes the local alias, so the module it "accepts"
gets a custom property nothing defines unless the alias happens to be spelled
like the exported group. Keeping it would mean recording that a wrong render is
preferred to a build error, which is not a trade this compiler makes anywhere
else.

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

**A missing step is visible as a missing step.** The absent one reads as a gap
in a numbered sequence with a comment explaining the gap, not as a step nobody
thought of. Where the gap is still open, its ticket is named at it.

**An absent step is measured before it is ruled out, not reasoned about.** The
default-import step was written after putting the input through both compilers,
and the one remaining gap — a binding carrying a folded value — is a gap because
the field it reads is provably always false upstream, not because nobody looked.
A step left out on the strength of how the two implementations _look_ is the
shape this chain exists to avoid.

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
