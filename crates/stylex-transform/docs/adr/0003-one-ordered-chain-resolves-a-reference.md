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

**A refusal reports against the declaration, as upstream does.** Upstream deopts
on `binding.path` at lines 626, 647, 653, 657, 661, 665 and 673, and on the
reference only at 687. So its code frame names the line the binding was
_declared_ on, which is the line a reader has to go and change; only the tail
refusal names the read. Every step here does the same, and the declaration-kind
refusal at the tail does too — upstream reaches that one through
`evaluateCached(path.resolve())`, whose argument is the declaration, which is why
its frame prints the `function` line rather than the read.

What is recorded is the binding's **name**, not its position. A `Span` from this
compiler's parse indexes this compiler's source map; the code frame owns another,
built from the text it registered for the file, so the same byte offset means
something else in each. `utils::log::declaration_span` therefore finds the
declaration in the module the frame already re-parsed — the same trade
`key_span_index` makes for namespace keys, and it survives an earlier loader
having rewritten the values. An ident re-spanned onto the declaration does not
work, and was tried: `find_expression_span` searches for the first node
`eq_ignore_span`-equal to the recorded expression, and `eq_ignore_span` ignores
spans by definition, so the read still matches first.

The name is recorded against the refused expression rather than as one "current
refusal", because a refusal is not always the end of a build: a dynamic style's
refused value falls through to an inline style, so a later diagnostic about
something else must not inherit this binding's position.

Measured against 0.19.0, this compiler now frames the same line _and column_ for
every shape:

| Input                                         | Both frame                    |
| --------------------------------------------- | ----------------------------- |
| `let c = 'red'; c = 'blue'` read below        | `2:5` — `c = 'red'`           |
| `const o = {…}; o.c = 'blue'` read below      | `2:7` — the declarator        |
| a read above `const c = 'red'`                | `3:7` — the declarator        |
| `import vars from './vars.stylex.js'`         | `2:8` — `vars`                |
| `import { token } from 'no-such-package/…'`   | `2:10` — `token`              |
| `let NaN;` read as `zIndex`                   | `2:5` — `NaN`                 |
| `function f() {}` / `class K {}` read below   | `2:1` — the whole declaration |
| a namespace-imported token (the tail refusal) | the read                      |

A name the re-parsed module does not declare falls back to locating the
expression: the frame's text is not always the text the reference was resolved
against, and the read's own line is a better answer than none.

Nothing in the transform suites could see any of this. A `stylex_test_panic!`
matches the message and the frame is written separately, and the parity corpus
compares verdicts and messages with the text that says _where_ removed — which
is why the divergence survived until the two implementations were read side by
side. The guards are therefore split in two: `resolution_order.rs` pins which
binding each refusal names, and the code frame's own suite pins the line that
name resolves to.

**A write is refused for the write, whatever declared the binding.** Both write
steps are guarded by upstream's own question — `binding &&`, at 656 and 660 —
which is whether the module declares the binding the write was recorded against.
An earlier version asked instead whether a `VarDeclarator` existed, and that is
narrower than the language: a name bound by destructuring, a parameter, a
`catch` binding and a hoisted `function` or `class` all have no declarator, so a
write to one fell past both steps. What answered then was whichever later step
the reference reached — the tail refusal calling a destructured binding an
undefined constant, or the kind refusal calling a reassigned `function` a
`function` — which named something true about the reference, and nothing about
the problem.
Measured on 0.19.0, all five shapes answer `Referenced value is not a constant.`
there, framed at the declaration, and now do here.

The guard also became cheaper than the lookup it replaced: `declares_binding` is
a hash probe keyed by full `Id`, where the declarator lookup walked the
declaration list. Keyed by `Id` is what keeps it sound — a write recorded
against a shadowing binding cannot refuse the binding it shadows, and a write to
a name this module does not declare cannot refuse a global that spells it.

It is not literally upstream's question. Upstream asks its scope chain, so a
binding out of scope at the reference answers nothing there, where this asks a
module-wide set. The syntax context inside the `Id` is what closes the gap: the
resolver gives a shadowing binding its own, and the write sets are keyed the same
way, so a reference only ever meets writes recorded against the binding it names.
Wider by construction, equal on every shape measured, and wrong in the direction
that refuses rather than the one that folds.

**The mutation probe over-approximates, and knowingly — where it protects
something.** `add_target_root_write` walks a member chain to its root, so
`obj.a.b = 1` and `state.items.push(…)` both reach `obj` and `state`. Upstream's
`isMutated` asks that the reference's _own_ parent be the member the write lands
on, so it sees no mutation in either, folds the initializer, and bakes in a value
that has since changed. Refusing instead is a divergence this keeps: it only ever
refuses input upstream compiles, never the other way round.

What it may not do is change an answer that already agreed, and left to itself it
did. A write below the first hop is recorded as its own kind — a **deep
mutation** — and the chain asks it of a `VarDeclarator` rather than of the
binding. A declarator is the only shape whose initializer this chain would
inline, so it is the only shape where a stale value can reach the stylesheet;
everything else already refuses for its own reason and now keeps that reason.
Measured on 0.19.0: `function paint() {}` beside `paint.a.b = 1` is
`Unsupported expression: FunctionDeclaration` on both sides, where refusing it
for the write would have diverged, and `const theme = {…}; theme.colors.primary =
'blue'` is still refused here and still folded there.

One hop stays exactly upstream's question, and the two probes agree with each
other on it, which is what the split was for.
