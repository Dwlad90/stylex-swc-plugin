# stylex-transform

The SWC `VisitMut` transform: where every `stylex.*` call is recognised and
rewritten. What an authored expression folds to is one crate down, in
[stylex-evaluator](../stylex-evaluator/CONTEXT.md). The state the transform
carries across phases is below that, in
[stylex-state](../stylex-state/CONTEXT.md). The cycle order itself is a
[transformation cycle](../stylex-enums/CONTEXT.md).

## Language

**Pre-scan**:
The walk that records module-wide facts a visitor cannot ask for later, in two
modes that collect different things. `for_sx()` runs at the start of `Discover`,
and only when `sxPropName` is set: every import source, every bound name, the
scope spans of local bindings, every [declared
binding](../stylex-evaluator/CONTEXT.md#declared-binding), and every binding
written to. `writes_only()` runs after `Discover`, only for a module that
reaches evaluation, and records the declared bindings and binding writes alone.
So an import source or a scope span is only ever populated on the `sx` path.
_Avoid_: first pass, collection phase, analysis

**Pre-rule**:
A style entry recognised but not yet CSS — `PreRuleValue` (from
[stylex-structures](../stylex-structures/CONTEXT.md)) plus the pseudos and
at-rules it sits under, composed by `PreRuleSet` or empty as `NullPreRule`. Compiling one
yields a `CompiledResult`: the `ComputedStyle` list (class name, injectable
style, authored paths), or `Null`, which is the blank value below.
_Avoid_: draft rule, intermediate style, raw rule

**Blank value**:
A style value that carries no CSS text — an empty or whitespace-only string. A
declaration built from one is `color:`, which no browser accepts, so the
property is left undeclared and compiles to `null`, exactly as an authored
`null` does. Blank is not falsy: `0` is falsy but spells a value.

Judged _after_ transformation, because transformation decides whether a value
spells anything — a blank `content` is quoted into `""`, which does. A blank
entry of a fallback array drops before the `var()` chain is composed, so the
class name is hashed from the entries that survive.
_Avoid_: empty value, falsy value, null value

**Producer / consumer**:
A `stylex` call that creates styles against one that spends them. They run in
separate cycles, because a consumer needs every producer in the file already
transformed. The producers are `create`, `defineVars`, `defineConsts`,
`keyframes`, `createTheme`, `positionTry`, `viewTransitionClass`,
`defaultMarker`, `defineMarker` and the three `unstable_*Nested` calls; the
consumers are `props`, `attrs` and the callable `stylex(...)`, which merges.
Both lists are complete, and the names live in
[stylex-constants](../stylex-constants/CONTEXT.md).
_Avoid_: definition/usage, source/sink

**Transformer**:
One implementation under `shared/transformers/`. Most are one producer API, so
`stylex_create.rs` holds `stylex.create`. Two are not: `stylex_types.rs` is the
`stylex.types.*` value helpers, and `named_rule.rs` the fold `keyframes` and
`positionTry` share.
_Avoid_: handler, visitor, rewriter

**Property registration**:
The `@property` rule the `create` transformer injects for each CSS variable a
dynamic style function writes. `inherits` is `true` only when some segment of
the variable's authored path is a pseudo _element_ (a `::` prefix), and `false`
otherwise, pseudo _classes_ such as `:hover` included.
_Avoid_: at-property, var declaration, custom property rule

**Runtime binding**:
The value-level `stylex` namespace binding an `sx` runtime call is written
against, resolved by `get_stylex_runtime_binding`. It reuses an existing
namespace or default import where nothing shadows it at the call site, and
otherwise prepends `import * as <name> from '<source>'` — under the uid
`_stylex` where an import source or a `stylex` binding already exists, and plain
`stylex` where neither does. Not the CSS runtime injection, which
`runtimeInjection` gates separately.
_Avoid_: import, helper, inject binding
