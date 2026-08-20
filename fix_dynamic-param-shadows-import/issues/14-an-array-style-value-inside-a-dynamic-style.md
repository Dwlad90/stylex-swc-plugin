# 14 — An array style value inside a dynamic style

Status: `needs-triage`
Blocked by: None

**What was found:** Every array written as a style value *inside a dynamic
style's body* aborts the build with `Style value must evaluate to a static
expression.` The reference implementation compiles them.

```js
export const styles = stylex.create({ dyn: (h) => ({ height: ['1px', '2px'] }) });
```

The same array in a static namespace compiles. The difference is the consumer:
a dynamic style's body is walked by `evaluate_partial_object_recursively`
(`shared/utils/core/evaluate_stylex_create_arg.rs`), and its two style-value
positions read the evaluated value through `as_expr()`. An array evaluates to
`EvaluateResultValue::Vec`, which has no expression form, so the value aborts
there rather than being folded to an `Expr::Array` the way
`object_expression.rs` folds it for a static namespace via
`evaluate_result_vec_to_array_expr`.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration — `haste` resolution, one source string. Every row below is
`ERR Style value must evaluate to a static expression.` on this compiler:

| input, inside `create({ dyn: (h) => ({ … }) })` | Babel 0.19.0 |
| --- | --- |
| `height: ['1px', '2px']` | `.x…{height:1px;height:2px}` |
| `margin: [1, 2]` | `.x…{margin:1px;margin:2px}` |
| `height: F` where `const F = ['1px','2px']` | `.x…{height:1px;height:2px}` |
| `height: { default: ['1px','2px'] }` | `.x…{height:1px;height:2px}` |
| `height: []` | no rule, no error |
| `height: [null, '2px']` | `.x…{height:2px}` |
| `height: [, '2px']` | the hole makes it dynamic: `height:var(--x-height)` |
| `height: [['1px'], '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [{a:1}, '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [undefined, '2px']` | `A style array value can only contain strings or numbers.` |
| `height: [true, '2px']` | `A style array value can only contain strings or numbers.` |

Two shapes already agree, and are the reason this went unnoticed:
`stylex.firstThatWorks('1px','2px')` and `[...xs, '2px']` — the first answers an
expression rather than a `Vec`, the second is refused before the array is folded.

Not fixed under ticket 08, which is about a folded function map at the same two
positions. The overlap is one input — `height: [stylex, '1px']` with the
parameter shadowing the namespace import, where the reference implementation
reads `A style array value can only contain strings or numbers.` and this
compiler cannot reach that message until the `Vec` case is folded. Recorded in
the corpus as divergent, with this ticket named as what closes it.

The shapes above are the work: an empty array, a `null` element, a hole, a
nested array, and an `undefined` element each have their own answer upstream,
and none of them is the array's element list read straight through.

- [ ] Every row above agrees with the reference implementation
- [ ] The two positions no longer abort for an array
- [ ] Corpus entries for each shape, with the verdict each is known to read
