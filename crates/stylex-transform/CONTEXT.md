# stylex-transform

The SWC `VisitMut` transform: where every `stylex.*` call is recognised and
rewritten. What an authored expression folds to is one crate down, in
[stylex-evaluator](../stylex-evaluator/CONTEXT.md); the state the transform
carries across phases is below that, in
[stylex-state](../stylex-state/CONTEXT.md).

## Language

**Pre-scan**:
The walk at the start of the `Discover` cycle that records module-wide facts a
visitor cannot ask for later — every import source, every bound name, the scope
spans of local bindings, every [binding the module
declares](../stylex-evaluator/CONTEXT.md#declared-binding), and every binding
that is written to. SWC visitors have no parent pointers and no scope chain, so
the pre-scan stands in for both.
_Avoid_: first pass, collection phase, analysis

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
