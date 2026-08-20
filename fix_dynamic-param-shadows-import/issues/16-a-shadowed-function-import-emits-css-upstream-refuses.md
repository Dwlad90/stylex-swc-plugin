# 16 — A shadowed function import emits CSS upstream refuses

Status: `resolved`
Blocked by: None — 22 landed

**What to build:** A dynamic style whose parameter shadows a named import of a
function-map entry must refuse, as the reference implementation does, instead of
emitting a declaration for it.

```js
import { create, keyframes } from '@stylexjs/stylex';
export const styles = create({ dyn: (keyframes) => ({ height: keyframes }) });
```

| | |
| --- | --- |
| Babel 0.19.0 | `Invalid pseudo or at-rule.` |
| here | `.x16ye13r{height:var(--x-height)}` plus an `@property` rule |

Split out of [15](./15-the-function-map-read-where-it-is-not-a-map.md) because it
is the only one of that ticket's four cases that ships CSS rather than a
different sentence, and a wrong-output defect deserves a commit a bisect can
land on.

Same fold as [08](./08-reject-a-folded-map-as-a-namespace.md), different entry
shape. Upstream registers every entry as a plain object, so `keyframes` folds to
`{ fn: keyframes }` and namespace validation refuses the key `fn`. This compiler
registers it as `FunctionConfigType::Regular`, and `nodes/identifier.rs` answers
a `Regular` whose `fn_ptr` is neither `Mapper`, `ThemeRefMapper` nor
`DefaultMarker` by deopting with `Function not found` — and a deopt inside a
dynamic style is the inline-style path. So the parameter becomes a runtime value
and the build succeeds where upstream fails.

`firstThatWorks` and a bare `when` import behave identically. `defaultMarker` is
registered as `IndexMap` and deopts with `NON_CONSTANT`; measure it before
deciding whether it belongs here.

Ticket 08 materializes a `Map` at the style-value consumer. A `Regular` entry has
no key set to materialize, so this one is not the same change: either the entry
carries the key upstream gives it (`fn`), or the identifier step stops deopting
for it. Decide which, and say why, before writing either.

The guard that must not move: `types`. Upstream never registers it in
`identifiers` at all, so the parameter stands and the module compiles — and this
compiler agrees today only because the entry deopts into that same inline-style
path. Making the entries beside it refuse must leave `types` compiling.
Recorded as `modules-1266-param-shadows-the-types-import`.

- [x] The example refuses with the reference implementation's exact text.
      It did so under `cargo test` when this ticket landed; through the
      rs-compiler pipeline it took
      [22](./22-the-stripped-specifier-the-fold-never-sees.md), which stopped
      the type-stripping pass from eliding the specifier in a JavaScript module.
      A TypeScript input still compiles it, which is
      [24](./24-the-typescript-half-of-the-stripped-specifier.md).
- [x] `firstThatWorks` and a bare `when` import refuse the same way
- [x] `types` still compiles to an inline style
- [x] `defaultMarker` is measured and either fixed here or recorded
- [x] The corpus row `modules-1266-param-shadows-a-named-function-map-import`
      stops reading `acceptance-divergent`

## Resolution

**The entry carries the key upstream gives it**, and the object it stands for is
built at the consumer -- the same split ticket 08 chose, one entry shape further
down.

The reason is that `{ fn }` is not a fact about `keyframes`. It is what a
function config *is* upstream: `identifiers[name] = { fn: keyframes }`
(`visitors/stylex-create.js:185`) is the whole registration, and `fn` is its only
enumerable key. `firstThatWorks`, `positionTry`, `stylex-keyframes.js:76` and both
`defineVars` and `createTheme` spell every callable the same way. So a reference
to a config folds to the config, and the object with one key `fn` is what a
reader that needs an object builds from it. Registering `Map({ fn: ... })` at the
table instead would have been a lie at the call site: `call_expression.rs`
refuses a `Map` callee with `NON_CONSTANT`, so `keyframes(...)` would have
stopped working.

Two changes:

- `nodes/identifier.rs` -- the terminal arm of `FunctionConfigType::Regular`
  answers `EvaluateResultValue::FunctionConfig(func.clone())` where it used to
  `deopt` with `Function not found`. The `DefaultMarker` arm collapsed into it:
  it was already answering exactly that, rebuilt by hand. `Mapper` and
  `ThemeRefMapper` stay, because those two are this compiler's encodings of a
  *value* -- a bound argument and a theme reference -- rather than of a callable.
- `materialize_style_value` in `core/evaluate_stylex_create_arg.rs` -- a
  `FunctionConfig` materializes as `{ fn: null }`, or, behind
  `FunctionType::DefaultMarker`, as the marker map's keys. That second case is a
  bare `when` import, the one entry upstream registers as the marker object
  itself, so `ancestor` and not `fn` is the key its refusal lands on. Same
  sentence either way, which is why the key set is pinned rather than assumed.

Nothing in the suite pinned `Function not found`; it named an internal lookup and
no caller could act on it.

The deopt was the whole defect. A deopt inside a dynamic style is the inline-style
path, so the parameter became a runtime value and the module compiled -- for
`keyframes`, `firstThatWorks`, `positionTry`, a bare `when` and every alias of
each. All of them now read `Invalid pseudo or at-rule.`, byte-identical to the
reference implementation, and so do nine hostile shapes measured beside them: an
unclosed media query, an unclosed `calc(`, an unterminated quote, an unknown
pseudo-class, a bracket condition, a custom property, three nested
pseudo-classes, a non-ASCII alias, and the fold read twice or read as one of
several parameters. In each the fold is what the refusal names, because validation
reaches the key it cannot read before anything parses the malformed CSS beside it.

### Wider than the dynamic style

The fold is not scoped to a dynamic style, so other readers of a config changed
too. Measured, not reasoned about -- the first draft of this section claimed
`defineVars` and `createTheme` had started reading upstream's `Default value is
not defined for a variable.`, and writing the test for it showed that was false:

- `defineVars({ a: keyframes })` still reads `Only static values are allowed
  inside of a defineVars() call.` where upstream reads `Default value is not
  defined for "a" variable.` The identifier step folds, but materialization lives
  at the create call's style-value consumer and was not extended to this one, so
  the position still refuses a value with no expression form. Unchanged by this
  ticket, and now pinned in
  `validation_stylex_define_vars_test::a_folded_function_map_read_as_a_variable_value_is_refused`
  so the claim cannot drift again.
- A static `create({ a: { height: keyframes } })` moved from `Function not found`
  to `A style value can only contain an array, string or number.` Still not
  upstream's `Invalid pseudo or at-rule.` -- the static object evaluator deopts
  rather than materializing, which is
  [15](./15-the-function-map-read-where-it-is-not-a-map.md)'s first case -- but it
  now classifies the value instead of naming a lookup. Pinned, and recorded as
  corpus row `modules-1266-a-named-function-map-import-read-as-a-static-value`.
- **`15`'s third case closed as a side effect.**
  `create({ dyn: (stylex) => ({ height: stylex.when }) })` now reads
  `Invalid pseudo or at-rule.`, upstream's text. That ticket recorded it as
  needing "the when surface carrying its names, not a change at the consumer";
  measuring it disproved that -- the marker map behind the config already carries
  the names, so materializing at the consumer was enough. Pinned in
  `validation_stylex_create_test::invalid_values::when_read_off_a_shadowed_namespace_is_refused_as_a_namespace`
  and struck from 15.

The fold reaches every `FunctionType` that encodes a callable, which is wider
than the three `{ fn }` names this ticket set out to fix. `nodes/identifier.rs`
names each variant explicitly rather than folding under a catch-all, so the
breadth is a stated decision and a new variant has to be decided rather than
silently answered as a config object.

`types` compiles as before, and now for upstream's reason rather than by accident:
neither compiler registers it for a create call, so nothing folds and the
parameter stands. The corpus note that claimed this compiler did register it was
wrong and is corrected.

### Two findings that are not this seam's

- **`defaultMarker`** is the one entry upstream registers as a function, so it
  refuses as an illegal value and not as a namespace. Reaching that sentence needs
  `validate_namespace`'s permissive terminal arm to refuse, which decides the
  answer for every non-literal value it passes over. Measured, pinned as it
  stands, filed as
  [21](./21-a-shadowed-default-marker-param-reports-an-internal-shape.md).
- **The reported source still diverges through the napi pipeline**, and not
  because of the fold. `typescript_strip` runs ahead of the transform and elides a
  specifier with no value reference; a shadowing parameter is not a reference, so
  with nothing else reading `keyframes` the specifier is gone before anything
  could register it. Under `cargo test`, which runs the resolver but not
  `typescript_strip`, the same source refuses. This is
  [01](./01-pin-why-an-unused-import-spares-the-shadowed-parameter.md)'s
  mechanism in the direction that costs a refusal, and it is a pipeline-ordering
  decision -- filed as
  [22](./22-the-stripped-specifier-the-fold-never-sees.md).

  **Closed by 22.** The pipeline keeps every import specifier of a JavaScript
  module now, so the reported source refuses there too. The corpus row therefore
  split. `modules-1266-param-shadows-a-named-function-map-import`
  keeps the entry alive with a `keyframes` call beside the dynamic style, which is
  the shape that reaches the seam this ticket owns, and reads `both-reject`. The
  reported bare shape moved to
  `modules-1266-param-shadows-a-named-import-referenced-nowhere-else`, which
  read `acceptance-divergent` until 22 landed and reads `both-reject` now. Both
  rows measure what they claim.

Six corpus rows added -- `firstThatWorks`, a bare `when`, `positionTry`,
`defaultMarker`, the elided specifier, the static read -- and the whole `modules`
set reports `changed 0`.
