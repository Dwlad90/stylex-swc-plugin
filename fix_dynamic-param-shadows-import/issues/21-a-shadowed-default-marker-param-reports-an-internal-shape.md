# 21 — A shadowed `defaultMarker` param reports an internal shape

Status: `resolved`
Blocked by: None

**What was found:** `defaultMarker` is the one function-map entry the reference
implementation registers as a *function* rather than as an object, so a dynamic
style's parameter shadowing it is refused as an illegal value and not as a
namespace. This compiler registers it as an index map with no value form, and the
sentence the build stops on names that shape instead of the input.

```js
import { create, defaultMarker } from '@stylexjs/stylex';
export const styles = create({ dyn: (defaultMarker) => ({ height: defaultMarker }) });
```

| | |
| --- | --- |
| Babel 0.19.0 | `A style value can only contain an array, string or number.` |
| here (`cargo test`) | `[UNIMPLEMENTED] IndexMap values are not supported in this context.` |
| here (rs-compiler) | `.x16ye13r{height:var(--x-height)}` plus an `@property` rule |

The third row is [22](./22-the-stripped-specifier-the-fold-never-sees.md), not
this ticket: with the shadowing parameter as the specifier's only occurrence the
specifier is elided before the transform runs, so nothing is registered and the
parameter becomes a runtime value. The second row is what this ticket owns.

Measured while resolving
[16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md), which fixed
every entry of the family that folds to an object. This one does not, and the
reason it is left out is not effort:

`identifiers[name] = () => stylexDefaultMarker(state.options)`
(`visitors/stylex-create.js:191`) is a function, and
`basic-validation.js:47` refuses any style value that is neither a literal, an
array nor a plain object. This compiler's `validate_namespace`
(`shared/utils/validators.rs`) has a permissive `_ => {}` terminal arm where
upstream throws `ILLEGAL_PROP_VALUE`, so there is no expression that could be
materialized here and be refused for being one. Closing this means giving that
arm a refusal, which decides the answer for every non-literal value the validator
currently passes over — a wider question than one entry, and probably the same
question as
[18](./18-a-theme-object-read-as-a-style-value-is-dropped.md).

Pinned as it stands in
`validation_stylex_create_test::invalid_values::a_dynamic_param_shadowing_a_named_default_marker_import_reports_an_internal_shape`,
so the day the sentence changes is visible. Corpus row
`modules-1266-param-shadows-a-named-default-marker-import`, which reads
`both-reject-divergent` since
[17](./17-the-corpus-cannot-report-a-changed-refusal.md) landed: the verdict now
compares the two sentences, so this divergence is one the corpus reports.

That row did not measure this subject until 17 re-measured it. It carried a
second create call, written to keep the `defaultMarker` specifier alive, whose
`default: defaultMarker()` both compilers refuse — so the module refused before
the shadowed parameter was read and the row measured that call. The call is
gone; a shadowing parameter as the specifier's only occurrence keeps the
specifier now, which is what
`modules-1266-param-shadows-a-named-import-referenced-nowhere-else` holds.

## The static position, measured while closing 15

The same entry read where a *static* value belongs, no shadowing involved:

```js
import { create, defaultMarker } from '@stylexjs/stylex';
export const styles = create({ a: { height: defaultMarker } });
```

| | |
| --- | --- |
| Babel 0.19.0 | `A style value can only contain an array, string or number.` |
| here | `a > height > Referenced value is not a constant.` |

A second sentence naming something other than the input, and the same root: with
`defaultMarker` an index map, `function_fold_to_object`
(`js/evaluate/mod.rs`) has no arm for it and every position refuses without
materializing. [15](./15-the-function-map-read-where-it-is-not-a-map.md) closed
the static position for every entry of the family that *does* fold to an object,
which is why this one now stands out in two positions rather than one -- and the
static sentence is the worse of the two, because "not a constant" says the
opposite of what is true of an import of a compiler API.

Both positions close together if the answer here is to register `defaultMarker`
as the function it is upstream rather than as an index map, which is worth
measuring before the wider `validate_namespace` question: a bare function in a
style value is already refused with upstream's exact text, as the spread of a
`{ fn }` wrapper now shows.

- [x] The terminal arm of `validate_namespace` is decided, here or with 18 --
      decided as *not this ticket's*: nothing had to be taught to refuse a value
      it was passing over, so the arm is untouched and the question stays with
      [18](./18-a-theme-object-read-as-a-style-value-is-dropped.md)
- [x] The example refuses with the reference implementation's exact text
- [x] The pinned sentence is updated rather than deleted
- [x] The static position's sentence is decided with the dynamic one, not apart
      from it

## Resolved

The registration was the whole of it, and the wider `validate_namespace`
question turned out not to be this ticket's after all -- it is untouched, and
stays with [18](./18-a-theme-object-read-as-a-style-value-is-dropped.md).

A reference to `defaultMarker` folds to a **function** now, where every other
entry of the family folds to the object `{ fn }` -- which is exactly how the
reference implementation registers the two: `identifiers[name] = () =>
stylexDefaultMarker(state.options)` against `identifiers[name] = { fn: … }`. So
a style value refuses this one for not being a style value and refuses a wrapped
one for being a map of conditions with an invalid pseudo. Both sentences are
upstream's; which one an entry earns is decided by how it is registered, and
nothing had to be taught to refuse a value it was passing over.

The placeholder is the same function in both, extracted as
`fold_placeholder_function`, so the two cannot come to refuse for shapes that
differ by more than the wrapper.

Both positions closed together, as the ticket asked:

| | before | now, and upstream's |
| --- | --- | --- |
| the shadowing parameter | `[UNIMPLEMENTED] IndexMap values are not supported in this context.` | `A style value can only contain an array, string or number.` |
| the static read | `a > height > Referenced value is not a constant.` | the same |

Two neighbours measured with them:

- **A spread** of the bare function contributes nothing, where a spread of a
  wrapped entry contributes its `fn` key. `identical` to upstream.
- **`stylex.defaultMarker`** -- the namespace read rather than the named import
  -- is `undefined` in both compilers: the name is registered for a *call* and
  not as something the namespace object holds. Refused as a value in both.

`get_var_decl_by_ident`'s index-map arm is left as the broken-invariant guard it
is: reaching it now means the identifier step did not answer first, which is a
different fault from the one this ticket reported.

Measured: `modules-1266-param-shadows-a-named-default-marker-import` reads
`both-reject` where it read `both-reject (diverged)`, and the three new rows
read `both-reject`, `both-reject` and `identical`. Pinned in
`validation_stylex_create_test::invalid_values` across all four positions and in
`crates/stylex-rs-compiler/__test__/importElision.spec.ts`, whose `defaultMarker`
case now names upstream's sentence rather than the index map.
