# 21 — A shadowed `defaultMarker` param reports an internal shape

Status: `needs-triage`
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
`both-reject` — both compilers refuse, and the verdict cannot report that the
words differ ([17](./17-the-corpus-cannot-report-a-changed-refusal.md)).

- [ ] The terminal arm of `validate_namespace` is decided, here or with 18
- [ ] The example refuses with the reference implementation's exact text
- [ ] The pinned sentence is updated rather than deleted
