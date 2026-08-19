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
spans of local bindings, and every binding that is written to. SWC visitors have
no parent pointers and no scope chain, so the pre-scan stands in for both.
_Avoid_: first pass, collection phase, analysis

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

**Refused fold**:
A deopt raised by a fold that recognised its callee and will not produce a
value. Every refusal in this area is one — there is no separate error-raising
path — so whether an author sees a failed build or working runtime code depends
on where the call sat, not on which refusal it was: in a static position the
`stylex.*` call it belongs to fails, and inside a dynamic style function the
call is left for the runtime instead. Some refusals therefore borrow a
diagnostic from further down the pipeline instead of routing a value to it, for
the reasons in
[docs/adr/0001](./docs/adr/0001-a-refused-fold-borrows-a-later-diagnostic.md).
_Avoid_: fold error, hard error, invalid call

**Hole**:
An array element `Array(n)` created by counting rather than by listing. Held as
the same absent value a confidently evaluated element with no value already is,
so it joins as nothing and reaches the style-array check unchanged. A style
array cannot contain one, which is where a counted array is refused — the fold
itself succeeds.
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
written rather than from what evaluating it produced. The two differ for a
hole, which evaluation drops before it becomes a value, so a hole is why the
count comes from the AST at all. A spread never reaches the count: evaluating
the array refuses it first, as a [spread refusal](#spread-refusal). The
receiver is unwrapped before it is read, because a parenthesis is not a
different receiver.
_Avoid_: array length, element count, size

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
no Rust string holds, and an array carrying a [hole](#hole), which is dropped
before it becomes a value so the keys after it would shift.
_Avoid_: spread properties, object keys, assigned properties

**Own key order**:
The order an object's properties are enumerated in, and so the order their
declarations reach the stylesheet: every array-index key first in ascending
numeric order, then every other key in insertion order. A key is an array index
only in its canonical decimal spelling, so `"0"` is one and `"00"` is not.
Decided once, on the finished property set, because an index key can arrive
from a literal, a spread or a computed key.
_Avoid_: property order, insertion order, key sorting

**Object method receiver**:
What the argument of `Object.keys`/`values`/`entries` reads as. Three answers
rather than two, because an absent object spells two opposite things: a receiver
that is not an object contributes no own keys and folds to `[]`, as
`Object.keys(5)` does, while a receiver holding an element with no expression
form cannot be read at all and refuses. Answering `[]` for the second would write
a shorter list into the stylesheet than the source describes.
_Avoid_: object argument, keys source

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

**Binding write**:
A binding whose value can differ from its declaration initializer, either
rebound or mutated in place. Both make the initializer an unsound stand-in at
the use site, so both deopt identically and share one set keyed by full SWC `Id`
— a write to a shadowing binding never deopts the one it shadows.
_Avoid_: mutation, reassignment, dirty binding

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

**Seen value**:
A memoized evaluation, keyed by the
[structural hash](../stylex-utils/CONTEXT.md) of the expression. `resolved`
distinguishes a completed evaluation from one
currently in progress, which is how cyclic references terminate.
_Avoid_: cache entry, memo

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
