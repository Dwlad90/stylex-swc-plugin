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
`String(x)`. A [valid callee](../stylex-js/CONTEXT.md) is the wider set, because
it also admits globals that only contribute methods: `Math` is a valid callee so
that `Math.round(1.5)` folds, and is not a callable global, so a bare `Math(x)`
is rejected rather than folded. Only a global with no binding in scope is one at
all — a declared `String` is an ordinary function and is called, not folded.
_Avoid_: built-in function, global function, wrapper call

**Binding write**:
A binding whose value can differ from its declaration initializer, either
rebound or mutated in place. Both make the initializer an unsound stand-in at
the use site, so both deopt identically and share one set keyed by full SWC `Id`
— a write to a shadowing binding never deopts the one it shadows.
_Avoid_: mutation, reassignment, dirty binding

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
