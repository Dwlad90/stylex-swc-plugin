# 15 — The function map read where it is not a map

Status: `needs-triage`
Blocked by: None

**What was found:** Ticket 08 made a folded function map reach namespace
validation from a dynamic style's value position. Four neighbouring positions
read the same fold and still diverge, each for its own reason -- one of which is
now owned by [16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md),
and a second of which 16 closed outright (case 3 below). Measured against
`@stylexjs/babel-plugin` 0.19.0 under the parity harness's configuration.

All four share one root: the reference implementation's `identifiers` is a plain
JS object, so *every* entry has a key set and a string coercion. This compiler's
`FunctionConfigType` has four shapes, only one of which (`Map`) carries keys, and
`FunctionConfig` carries none at all.

### 1. A static namespace value reaches a different consumer

```js
export const styles = stylex.create({ a: { height: stylex } });
```

| | |
| --- | --- |
| Babel | `Invalid pseudo or at-rule.` |
| here | `a > A style value can only contain an array, string or number.` |

No shadowing involved -- this is the namespace import read as a value. A static
namespace is evaluated by `nodes/object_expression.rs`, whose terminal arm
deopts with `ILLEGAL_PROP_VALUE` rather than materializing. Ticket 08 covered
`evaluate_partial_object_recursively`, which only walks a dynamic style's body.
`object_expression.rs` is the general object evaluator and is read by
`defineVars` and `createTheme` too, so materializing there is a wider decision
than materializing at the create-call consumer was.

### 2. A named import of a function-map entry that is not a `Map`

Split out into [16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md):
it is the one case of the four that emits CSS the reference implementation
refuses, rather than refusing with different words, so it is owned on its own and
does not wait on the rest of this issue.

### 3. A `FunctionConfig` read off the map — **closed**

```js
export const styles = stylex.create({ dyn: (stylex) => ({ height: stylex.when }) });
```

| | |
| --- | --- |
| Babel | `Invalid pseudo or at-rule.` -- `stylexWhen` is an object of the when functions |
| here | `Invalid pseudo or at-rule.` |

Closed as a side effect of
[16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md), and the
reasoning recorded here was wrong. This said reaching upstream's message "means
the when surface carrying its names, not a change at the consumer". It did not:
the marker map behind the config already carries the names, so once the consumer
materialized a single function config the keys were there to refuse. Nothing
about the when surface changed.

Pinned in
`validation_stylex_create_test::invalid_values::when_read_off_a_shadowed_namespace_is_refused_as_a_namespace`.

### 4. The fold coerced to a string

```js
export const styles = stylex.create({ dyn: (stylex) => ({ height: `${stylex}px` }) });
export const other  = stylex.create({ dyn: (stylex) => ({ [stylex]: '1px' }) });
```

| input | Babel | here |
| --- | --- | --- |
| template | `height:[object Object]px` | `height:px` |
| computed key | `.x…[object Object]{[object object]:1px}` | `dyn > A style value can only contain an array, string or number.` |

Both compilers emit nonsense; they disagree about which nonsense. `[object
Object]` is what JS gives for an object in a template, and this compiler
coerces the fold to the empty string instead. Lowest value of the four, and the
only one where agreeing means reproducing a coercion neither compiler intends.

- [x] Case 3 fixed, by 16, and the reasoning recorded here corrected
- [ ] Cases 1 and 4 are either fixed or recorded as a decided divergence
- [ ] Corpus entries carry the verdict each is known to read
