# stylex-transform

The SWC `VisitMut` transform and the state it carries across phases. This is
where the compiler's JavaScript evaluator lives, and where every `stylex.*` call
is recognised and rewritten.

## Language

**State manager**:
`StateManager` — everything the transform knows about the file it is part-way
through: imports, declarations, discovered style objects, caches, and the
current [transformation cycle](../stylex-enums/CONTEXT.md). One per file; passed
by mutable reference through the whole visitor.
_Avoid_: context, session, environment, state

**Pre-scan**:
The walk at the start of the `Discover` cycle that records module-wide facts a
visitor cannot ask for later — every import source, every bound name, the scope
spans of local bindings, every [binding the module declares](#declared-binding),
and every binding that is written to. SWC visitors have no parent pointers and
no scope chain, so the pre-scan stands in for both.
_Avoid_: first pass, collection phase, analysis

**Declared binding**:
A name this module binds, held under its full SWC `Id`. The one question the
[chain](#reference-resolution-chain) asks of the reference implementation's
scope chain — does a binding exist for this exact reference — and the whole of
what it needs, because the resolver has already given every binding a
`SyntaxContext` of its own. Asked of a reference and never of a name: the global
`NaN` beside a `function f(NaN)` matches nothing, while a reference inside that
function matches the parameter. Recorded for every binding form JavaScript
spells, a function parameter included, because a parameter is the one form no
declaration list or import table can answer for; TypeScript's three reach the
[pre-scan](#pre-scan) already lowered to `var` or `const`. Held for the module
being compiled, so a name bound inside an _imported_ file carries a context this
set never saw and falls to the global — which fails safe, refusing nothing that
should fold.
_Avoid_: scope, bound name, symbol table

**Confident**:
Whether an evaluation produced a value the compiler may rely on.
`EvaluateResult.confident` is the gate: a non-confident result carries a `deopt`
expression and a reason instead of a usable value, and every caller must stop
rather than guess.
_Avoid_: successful, resolved, known, static

**Deopt**:
The expression an evaluation fell back to when it could not stay confident — the
code that must be emitted for the runtime to finish the job. Recorded on the
result, not thrown.
_Avoid_: bailout, failure, fallback, error

**Callable global**:
A JavaScript global the evaluator folds when the global _itself_ is called —
`String(x)`, `Number(x)`, `Array(x)`, `Object(x)`. A
[valid callee](../stylex-js/CONTEXT.md) is the wider set, because
it also admits globals that only contribute methods: `Math` is a valid callee so
that `Math.round(1.5)` folds, and is not a callable global, so a bare `Math(x)`
is rejected rather than folded. Only a global with no binding in scope is one at
all — a declared `String` is an ordinary function and is called, not folded.
_Avoid_: built-in function, global function, wrapper call

**Engine fold**:
Folding a method call by evaluating it in an embedded JavaScript engine instead
of matching its name against a table. A table is finite, so the method it does
not list is the next bug report; evaluating covers `String.prototype`,
`Array.prototype` and `Object.prototype` at once, and covers a chain, because
the receiver of a call is itself a candidate and the whole chain is evaluated
once. An expression qualifies when every leaf of it resolves to a value the
bridge can carry, which is what the [guard](#fold-guard) decides. The engine is
handed the expression alone and knows nothing of the module, so a name the guard
resolved crosses beside the source as a [transport](#transport) argument rather
than being looked up. What comes back is the evaluator's own value
type, not a syntax node: an array answers the list an array literal answers and
an object the object an object literal answers, so a folded value reaches
everywhere a value the author wrote reaches.
_Avoid_: boa fold, reflection, dynamic dispatch

**Fold guard**:
The predicate in front of an [engine fold](#engine-fold), and the whole of that
fold's behaviour: what it admits is answered by the language rather than by any
code here, so every boundary the compiler owns is a refusal the guard states.
Four are not about the scope. A **locale-sensitive** method is refused because
the engine has no locale data and would answer from the root locale, which is a
wrong value rather than no value. An **escaping** property -- `constructor`,
`call`, `apply`, `bind` -- is refused because it walks off the value that was
written and onto the language's function graph, where `Function` compiles a
string into a body that answers differently on every build and can write to a
prototype the next fold reads. A **length-amplifying** call is bounded, on the
argument written and again on the string that comes back, because nothing in the
engine bounds allocation; a callback body is refused outright, since a written
bound bounds one evaluation and a callback runs once per element. And **nesting**
is bounded on both sides: on the way in because the engine's parser recurses, and
on the way out because the depth of an answer is not what the width bound
measures. An overflow inside an evaluation that is allowed to fail aborts the
build instead of reporting anything. Each applies at every link of a chain, since
a chain hides its middle links.

A **mutating** method is not among them. It was, on the reasoning that matching
upstream would carry mutation into an otherwise pure evaluator; measured, the
reference compiler does not refuse one at all. It folds it on any receiver not
reachable by name and disqualifies the **binding** instead — with no position
check, so a read above the mutation is dead too — which leaves the engine
mutating only a temporary nothing can name afterwards. That rule belongs to
binding resolution and already existed there.

The guard reads values, not only syntax: a leaf qualifies when it is written into
the expression, bound by a callback's parameters, or a name the module resolves
to a value the bridge carries. So giving a value a name no longer changes whether
the call on it folds. What it costs is that the walk can evaluate, which is why
every refusal answerable from a name alone is applied before the walk begins and
only an expression the guard intends to fold pays to have its names read.

A name the guard could not read is not a refusal but a **candidacy** answer: the
call is simply not this module's, and the dispatch below owns it — which is what
keeps `Math` and the callable globals folding. Reading a name to decide that is a
[speculative read](#speculative-read). A receiver naming one of those globals is
handed back the same way, but only where it is the call the caller asked about: a
static _inside_ a chain is a link nothing else is ever handed, so handing it back
would take the whole chain down with it. Nested, the engine answers it, and the
names the reference compiler refuses — the nondeterministic and mutating statics
it lists — are refused here with it.
_Avoid_: whitelist, allowlist, filter, validator

**Transport**:
How a value the [guard](#fold-guard) resolved reaches the engine. An expression
that resolved names is printed as an arrow taking them as parameters and called
with their values, so `s.toLowerCase()` is handed over as `(s)=>s.toLowerCase()`
applied to the string `s` holds; one that resolved none is evaluated as itself,
because wrapping it costs a function object and a VM frame that measured 44% of
the cheapest fold. The author's own name is the parameter name,
which means nothing is rewritten and a callback parameter of the same name
shadows the value exactly as it does in the module. Chosen over substituting a
literal into the printed text, which would reprint and reparse the whole value at
every use site and could not carry a value with no literal spelling; and over
registering names on the engine, which is one leaked instance per thread shared by
every file that thread compiles, where a name left behind would be a cross-file
correctness bug. Because the value is an argument rather than text, the printed
source stays the size of the expression however large the value is — so the value
carries a size bound of its own.
_Avoid_: injection, substitution, interpolation, binding the engine

**Carried value**:
A value the bridge copies inward: a string, a number, a boolean, `null`, an
array, or a plain object, nested to any depth of those. Everything else the
evaluator can answer — a function configuration, a callback, the environment
object, an unresolved theme reference, an AST-keyed map — is handed back rather
than refused, so the dispatch below keeps answering for it. A theme reference
therefore crosses only as the `var(--…)` string it already resolved to, because
resolving it is what mutates compiler state and that happens before the bridge.

What a _name_ may hold is narrower than what the bridge carries, and
deliberately: a number and a boolean cross as an element or a property, where
they are part of the value the receiver is, but a name holding one alone is a
receiver of its own and `Number.prototype` is not reachable yet. The bounds are
counted on the value rather than on the syntax that named it — a name is three
characters whatever it holds — and counted across every name one fold carries,
in text, in entries and in nesting.
_Avoid_: primitive, scalar, serialisable, carryable string

**Speculative read**:
Resolving a name to decide whether a fold is _possible_, as opposed to folding.
The distinction is load-bearing because a refusal raised under one is not the
subtree's answer: the [guard](#fold-guard) hands the call back, the dispatch below
evaluates the same name in earnest, and it has to find the state and the sentence
it would have had. So a speculative read puts back the evaluation's confidence
and deopt, and the per-file memo withholds the refusal — while still memoising a
value that did resolve, since only the refusal is the speculation's own. The same
distinction the depth ceiling already needed, for the same reason.
_Avoid_: dry run, probe, tentative evaluation, trial fold

**Refused fold**:
A deopt raised by a fold that recognised its callee and will not produce a
value. Every refusal in this area is one — there is no separate error-raising
path — so whether an author sees a failed build or working runtime code depends
on where the call sat, not on which refusal it was: in a static position the
`stylex.*` call it belongs to fails, and inside a dynamic style function the
call is left for the runtime instead. A refusal carries the rule that refused
it, in this compiler's own words: message text is not a parity obligation, and
the comparison harness compares class name, rule text and style-object shape
rather than sentences. A call the guard never recognised is not a refusal — it
is not the fold's, and the dispatch behind it decides instead. Some refusals
therefore borrow a diagnostic from further down the pipeline instead of routing
a value to it, for
the reasons in
[docs/adr/0001](./docs/adr/0001-a-refused-fold-borrows-a-later-diagnostic.md).
_Avoid_: fold error, hard error, invalid call

**Hole**:
An array slot with no element in it. Two arrive by different routes and are
answered differently. One `Array(n)` created by counting rather than by listing
is held as the same absent value a confidently evaluated element with no value
already is, so it joins as nothing and reaches the style-array check unchanged —
a style array cannot contain one, which is where a counted array is refused, the
fold itself succeeding. One an author _wrote_, as in `[, 1]`, refuses the whole
array instead: the reference implementation evaluates element paths and a hole's
path carries no node, so both compilers answer
`Could not resolve the code being evaluated`. Dropping it is what refusing
replaced — a dropped hole shortens the array, and `[, 1].length` answering `1`
and `height: [, '2px']` emitting `height: 2px` were each a value the source does
not describe. Inside a dynamic style's body the refusal is not an error at all:
the value falls to the runtime, which is what the reference implementation emits
there.
_Avoid_: empty slot, gap, undefined element

**Member lookup**:
What a property read asks of a string or an array, decided once for all three
receiver kinds: `length`, an index, a property the receiver does not carry, or a
computed key with no name the evaluator could read. The classification carries
the key it read, so the refusal a receiver gives names the same key the
classification saw — three private copies of the property test is how one author
mistake came to earn three different diagnostics.
_Avoid_: property access, index check, member kind

**Written slot**:
The element count the language reports for an array, read from the literal as
written rather than from what evaluating it produced. A written [hole](#hole) is
why the count comes from the AST at all: it occupies a slot the language counts
and refuses the array it is in, so the count is answered from the source ahead
of the receiver being evaluated. A spread never reaches the count: evaluating
the array refuses it first, as a [spread refusal](#spread-refusal). The receiver
is unwrapped before it is read, because a parenthesis is not a different
receiver. _Avoid_: array length, element count, size

**Spread refusal**:
The single answer every spread in a value position earns —
`Unsupported expression: SpreadElement` — given before the spread's operand is
evaluated, and the same whatever the operand is and wherever the spread sits: an
array element, a call argument, at any nesting. Upstream gets this for free by
evaluating elements and arguments as _paths_, where a spread is a node kind with
no fold; this evaluator reads through to the operand, so it refuses explicitly
instead. One constant, `SPREAD_ELEMENT`, because a site that spelled the node
kind itself is a divergence no test names.
_Avoid_: spread not supported, unsupported spread, illegal spread

**Own enumerable properties**:
What a spread operand contributes to the object it is spread into —
`Object.assign` semantics, which is what the reference implementation calls.
A number, a boolean, `null`, `undefined` and a function have none, so spreading
one is not an error and adds nothing; a string and an array have their indices.
Two readings are refused rather than answered, both because this evaluator
cannot write them down: an astral string, whose code units are lone surrogates
no Rust string holds, and a value held in a representation of the evaluator's
own. An array carrying a written [hole](#hole) never reaches the question — it
is refused for the hole, before the spread reads it. _Avoid_: spread properties,
object keys, assigned properties

**Own key order**:
The order an object's properties are enumerated in, and so the order their
declarations reach the stylesheet: every array-index key first in ascending
numeric order, then every other key in insertion order. A key is an array index
only in its canonical decimal spelling, so `"0"` is one and `"00"` is not.
Decided once, on the finished property set, because an index key can arrive
from a literal, a spread or a computed key.
_Avoid_: property order, insertion order, key sorting

**Object method receiver**:
What the argument of `Object.keys`/`values`/`entries` reads as. Four answers,
not two, because an absent object spells three different things: a receiver that
is not an object contributes no own keys and folds to `[]`, as `Object.keys(5)`
does; one holding an element with no expression form refuses, since answering
`[]` would write a shorter list into the stylesheet than the source describes;
and a nullish receiver refuses under the sentence the language itself raises,
which complains about the receiver rather than about the list. The last two
differ in nothing but that sentence, and the sentence is the whole of what a
refused build hands an author. A [folded function map](#folded-function-map) is
a receiver like any other and reads through its object form, because a value
classified as "not an object" by one of the three readers of own keys and as an
object by the other two is how `Object.keys` came to answer `[]` for a map the
same compiler spreads correctly. _Avoid_: object argument, keys source

**Winning operand**:
The side of `||`, `&&` or `??` the fold keeps, returned as the value it already
was rather than re-created as a literal — so a winning object stays an object
and a winning array stays an array. The three operators are decided on their own
evaluator node, because they answer with an operand where every other binary
operator answers with a coercion of both.
_Avoid_: result, chosen branch, short-circuit value

**Coercion bridge**:
A helper that answers one of the [coercion crate](../stylex-js/CONTEXT.md)'s
questions over `EvaluateResultValue` rather than over an expression, deciding
for each evaluator-only variant what the value it stands for would have
answered. There is exactly one per question and every caller goes through it,
because a private copy of a question is how two call sites come to disagree
about the same value.
_Avoid_: adapter, wrapper, converter

**String operand**:
An evaluated side of `+` that already _is_ a string, which is what decides
whether the operator concatenates or adds — either side being one is enough.
The question is asked of the operands, never of whether coercing them to
numbers happened to succeed: `'1'` coerces perfectly well, and answering on the
coercion is how `'1' + 2` came to fold to `3`.
_Avoid_: stringable operand, non-numeric operand

**Reference resolution chain**:
The one ordered question the evaluator asks of an identifier it could not fold
from the injected function map: which binding does this name, and what does that
binding fold to. Eight steps in the reference implementation's order, each
citing the line range it mirrors, because both compilers agree on every step's
verdict and the sequence is the only thing left for them to disagree by. One
step is deliberately absent and says so at its position rather than being
missing. The questions the rest ask are an [import specifier
kind](#import-specifier-kind), a [declared binding](#declared-binding), a
[binding write](#binding-write) and an [early reference](#early-reference). The
order, and why it is upstream's rather than this compiler's, is [ADR
0003](./docs/adr/0003-one-ordered-chain-resolves-a-reference.md). _Avoid_:
identifier lookup, binding resolver, evaluate fallback

**Folded function map**:
What a reference resolves to when its name is a key of the injected function
map, which is consulted ahead of the [chain](#reference-resolution-chain) and
keyed by name rather than by binding — so a dynamic style's parameter spelling
`stylex` folds to the namespace import's map and not to the parameter.
Deliberate on both compilers: an arrow parameter is injected into that same map
so a nested `create()` can see it, which is why the map cannot be keyed by
binding.

The map holds the StyleX API surface reachable off one namespace, which is not
all functions: `when` is a config and `env` is an object of the `env` option's
entries. So its value type is the wider one the member-expression map beside it
has always held. A value in one and not the other is how `Object.keys` of the
namespace came to answer a list its own member reads contradict. `env` is
registered into the fold only where a `create` call sets its evaluation up: the
other calls that build a function map leave the namespace name unregistered on
purpose, so a bare `stylex` written where a static value belongs refuses rather
than materializing and dropping the declaration.

The fold carries no expression form, so every position that needs one
**materializes** it as the object it stands for -- its keys, each carrying a
function -- and validation then refuses whichever half it reads: a style value
and a namespace refuse the key as neither pseudo nor at-rule nor `default`, and
a spread copies the key onto the style object where the function is refused for
not being a style value. One function answers for all of them, so the sentence a
build stops on cannot depend on which position asked. Materialized in one place
rather than at any consumer, and never where the identifier resolves, because
`when` is read off the same map as a callee and needs its own form there.

Which keys depends on what the name was registered as, and the answer mirrors
the reference implementation's registration rather than this compiler's types. A
map stands for one key per entry, each carrying that entry's own object. A
single **function config** -- `keyframes`, `firstThatWorks`, `positionTry` --
stands for `{ fn }`, the one key a callable carries upstream. A config holding a
marker map, which is a bare `when` import or the `when` entry of the namespace's
map, stands for the marker names. An evaluated array is materialized too,
through the same fold a static namespace uses, and what it holds is then decided
by namespace validation rather than at the value position -- an element that is
not a string or a number is refused there, with the message the reference
implementation gives. Every other evaluated shape with no expression form is
refused rather than materialized, a [theme reference](#theme-reference) among
them, as is `defaultMarker` -- an index map here, and a bare function upstream.
_Avoid_: shadowed namespace, identifier map hit, function config fold

**Theme reference**:
What an import of a `defineVars` group resolves to: the group as a whole, named
by the hash of the file that declares it. It carries no expression form and
cannot be materialized the way a [folded function map](#folded-function-map) is,
because the keys it would need live in the other file -- so the CSS a style
value needs comes from a _member_ read off it (`zIndex.ten` is
`var(--x1ew7r74)`), and the group read without one is refused wherever a value
belongs. Refused, not dropped: answering "no value" there compiled the object as
if the declaration had not been written. _Avoid_: theme object, vars object,
defineVars value

**Import specifier kind**:
Which of `{ c }`, `c` or `* as c` bound the name a reference reads, answered by
the same lookup that matched the reference and travelling with the declaration
it belongs to. The three kinds get three answers from the first two steps of the
[chain](#reference-resolution-chain). A named specifier resolves to a theme
reference. A default one is refused outright, because a theme file is read
through its named exports and a default binding names a value from a file this
compiler never evaluates. A namespace specifier binds the whole export object
and so names no export at all, which leaves nothing for a theme reference to be
built from: it resolves nothing and falls through to the chain's terminal
refusal. The question is about the specifier and not about the declaration,
because one declaration carries two kinds at once:
`import tokens, { colors } from 'colors.stylex.js'` must refuse `tokens` and
still resolve `colors`. What a specifier is matched by is its **local binding**
and nothing else: an `import { spacing as sp }` binds `sp`, and the name it was
aliased away from binds nothing in this module, so no reference resolves through
it.
_Avoid_: import kind, import shape

**Binding write**:
A binding whose value can differ from its declaration initializer, either
rebound or mutated in place. Both make the initializer an unsound stand-in at
the use site and both refuse with the same text, but they are recorded apart —
**reassignment** for a name given a new value, **mutation** for a value changed
under a name that still points at it — because the
[chain](#reference-resolution-chain) probes them as the two sequential steps the
reference implementation probes. One walk fills both, and crossing a member hop
is what makes a write the second kind. Keyed by full SWC `Id`, so a write to a
shadowing binding never deopts the one it shadows, and a write to a name this
module does not declare never deopts a global that spells it. What a write is
refused against is the _binding_, not a declarator: destructured names,
parameters, `catch` bindings and hoisted `function` / `class` declarations are
all bindings a write makes stale, and each is refused for the write rather than
for whatever a later step would have found.

A third kind sits behind those two. A write more than one member hop from the
binding — `obj.a.b = 1` — is a **deep mutation**, which the reference
implementation does not count as a mutation of `obj` at all: it folds the
initializer and bakes in a value that has since changed. This refuses instead,
but only for a binding whose initializer the chain would actually inline, so a
`function`, a `class` or a destructured name keeps the refusal it already had
rather than being told its value is not constant.
_Avoid_: dirty binding, stale binding, nested mutation, transitive write

**Early reference**:
A reference that begins before the declarator naming it ends, so the program
does not hold the value where it is read. The sibling of a binding write: both
make the declaration initializer an unsound stand-in at the use site, and both
deopt. Declarations are collected module-wide with no notion of position, so
this is decided by comparing the parser's byte positions rather than by the
lookup — and a [synthesized node](#synthesized-node) is never one, having no
position to compare.
_Avoid_: forward reference, hoisted read, out-of-order declaration

**Synthesized node**:
An AST node this compiler built rather than read, carrying `DUMMY_SP` because no
source text spells it. Shorthand expansion and injected function mappers both
produce them. Every question answered from a position has to exempt them: byte
zero sorts before every authored node, so comparing one answers a fact about its
having been built.
_Avoid_: generated node, dummy node, fake node

**Key span index**:
Where every style namespace key of the module's _own parsed source_ is written,
collected in one walk and held beside that memoized source on the
[state manager](#state-manager). What the `file:line` annotation on `$$css` is
resolved from: the annotation asks for the authored position of every namespace
of every `stylex.create` call, and answering each by walking the source made a
`dev` build quadratic in the size of a file that is one long list of styles. A
key two namespaces spell is several candidates, ranked by how much of the
compiled call each reproduces; a tie resolves to nothing, because a wrong
`file:line` is worse than none. Distinct from the state manager's span cache,
which memoizes the _answers_ this index is asked for, keyed by the lookup rather
than by the key.
_Avoid_: namespace key index, key map, position table

**Framed declaration**:
The binding a refusal is _about_, recorded so its code frame names the line that
binding was declared on rather than the line it was read from — which is what
`@stylexjs/babel-plugin` frames, and the line an author has to go and change. A
name is recorded rather than a position: a span from this compiler's parse
indexes this compiler's source map, while the frame's positions live in the one
it built for the file, so the name is resolved against the module the frame
re-parsed. Recorded per refused expression, because a refused dynamic style
falls through to an inline style instead of stopping the build, and a later
diagnostic must not inherit an earlier refusal's position. A name that module
does not declare falls back to locating the read. _Avoid_: deopt span,
declaration span cache, reported position

**File offset**:
How far into its own file a position sits, and the only thing the
[key span index](#key-span-index)'s proximity tie-break may compare. Two
`BytePos` in this compiler can name the same character and hold different
numbers: the index is built from a module re-parsed into the code frame's
shared, process-global source map, while the call it places is read out of the
per-transform one, and a source map gives each file a start position after the
previous file's end. So the two agree only for the first file a process
compiles. A file offset can only be built from a position and the
[module base](#module-base) it belongs to, and exposes no way to read the number
back out, so the subtraction cannot be skipped at a new call site.
_Avoid_: byte position, column, index

**Module base**:
Where the module being transformed starts, in the source map it was parsed into
— the thing a position is measured against to become a
[file offset](#file-offset). Its own type for two reasons: both arguments would
otherwise be `BytePos`, so transposing them compiles and answers zero for every
candidate; and it must have no default, because a base nobody recorded would be
byte zero, which turns every offset straight back into the raw position. Where a
base may be genuinely unavailable it is spelled as absent rather than defaulted,
so a lookup that never got one loses the proximity tie-break instead of silently
ranking by "earliest in the file". _Avoid_: module start, origin, offset base

**Call lookup**:
The half of a key-span lookup that belongs to the `stylex.create` _call_ rather
than to one of its namespaces: the sibling keys every namespace of that call
ranks against, the proximity anchor, the span cache key's call-side digest, and
the call wrapped as an expression for the value-matching fallback. Built once
per call, because building any of it per namespace makes the call quadratic in
its own namespace count — the same shape the [key span index](#key-span-index)
removed one level up. One type rather than four arguments so that they cannot
describe different calls: a digest paired with another call's keys is a wrong
span cached under a key that looks right. The wrapper inside it is a deep clone,
so it is built on the first namespace that needs one and never for a call whose
namespaces all hit the span cache. _Avoid_: call keys, sibling context, lookup
context

**Seen value**:
A memoized evaluation, keyed by the
[structural hash](../stylex-utils/CONTEXT.md) of the expression. `resolved`
distinguishes a completed evaluation from one
currently in progress, which is how cyclic references terminate.

The key covers the whole remaining subtree and is taken again at every level, so
what the memo costs grows about quadratically with depth -- and is nearly all of
what folding a deep expression costs. Why it stays that way, and what an
incremental key would take, is
[docs/adr/0005](./docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md). This
is also one of the two consumers that acts on a hash hit without confirming
equality, which is why that key is 128 bits wide. _Avoid_: cache entry, memo

**Evaluation depth**:
How many levels of the fold are currently standing, counted on the
[state manager](#state-manager) rather than on the evaluation, because the
evaluation's confidence forks -- a logical operand and a computed key each get
their own -- while the stack it is accounting for does not. Crossing the ceiling
is a [refused fold](#refused-fold), not an abort. The fold also grows its own
stack, so the ceiling is a policy rather than whatever a 2 MiB thread survived.

Always in **fold levels**, never in levels of source nesting -- a member read
spends two, a spread spends two, an array element spends one for the array, and
a parenthesis spends none. A number quoted in source terms is wrong for some
other shape, which is why every boundary is measured rather than derived.

The ceiling itself is `maxEvaluationDepth`, resolved in
[stylex-structures](../stylex-structures/CONTEXT.md) from the option, then the
environment, then the default. Why both halves are needed, and what the ceiling
costs against the reference implementation, is
[docs/adr/0004](./docs/adr/0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md).
_Avoid_: recursion limit, nesting level, stack depth

**Pre-rule**:
A style entry that has been recognised but not yet turned into CSS —
`PreRuleValue` plus the pseudos and at-rules it sits under. `PreRuleSet`
composes several; `NullPreRule` is the empty one. Compiling a pre-rule yields
`ComputedStyle` (class name, injectable style, and the original authored paths
that produced it).
_Avoid_: draft rule, intermediate style, raw rule

**Blank value**:
A style value that carries no CSS text — an empty or whitespace-only string.
A declaration built from one is `color:`, which no browser accepts, so the
property is left undeclared and compiles to `null` exactly as an authored
`null` does. Distinct from **falsy**, which is a JS question and disagrees at
both ends: `0` is falsy but spells a value, and `" "` is truthy but spells
nothing.

Judged _after_ transformation, because transformation is what decides whether a
value spells anything — a blank `content` is quoted into `""`, which does. A
blank entry of a fallback array drops there too, before the `var()` chain is
composed, so the class name is hashed from the entries that survive: a blank
entry beside `red` yields the class name a lone `red` yields.
_Avoid_: empty value, falsy value, null value

**Producer / consumer**:
A `stylex` call that creates styles (`create`, `defineVars`, `defineConsts`,
`keyframes`, `createTheme`, `positionTry`, `viewTransitionClass`) versus one
that spends them (`props`, `attrs`). They run in separate cycles because a
consumer needs every producer in the file already transformed.
_Avoid_: definition/usage, source/sink

**Transformer**:
The implementation of one producer API, under `shared/transformers/`. It is the
compile-time counterpart of the runtime function it is named for, so
`stylex_create.rs` is where `stylex.create` semantics actually live.
_Avoid_: handler, visitor, rewriter

**Property registration**:
The `@property` rule the `create` transformer injects for each CSS variable a
dynamic style function writes. Its `inherits` descriptor is decided per
variable: `true` only when some segment of the variable's authored path is a
pseudo _element_ (a `::` prefix), because a pseudo element can reach a variable
no other way; every other case — including pseudo _classes_ such as `:hover` —
registers `inherits: false`.
_Avoid_: at-property, var declaration, custom property rule

**Runtime binding**:
The `sx` import the transform injects when compiled output needs runtime help.
`get_stylex_runtime_binding` reuses an existing import source when it can, and
consults the pre-scan's bound names and scope spans so the injected name is
never one the module already uses or shadows.
_Avoid_: import, helper, inject binding
