# 27 — An index read off an array refuses where the reference implementation folds one

Status: `needs-triage`
Blocked by: None

**What was found:** Reading an element out of an array by index refuses with
`Unsupported index: 0` where the reference implementation folds the element.

```js
const FALLBACKS = ['1px'];
export const styles = stylex.create({ s: { height: FALLBACKS[0] } });
```

| input | Babel 0.19.0 | this compiler |
| --- | --- | --- |
| `height: A[0]` where `const A = ['1px']` | `.x…{height:1px}` | `Unsupported index: 0` |
| `height: B` where `const B = [A[0], '2px']` | `.x…{height:1px;height:2px}` | the same refusal, at the inner read |

An array literal *a fold produced* is indexed — `Object.keys({a:1})[0]` folds —
and an array the evaluator holds as its own value is not. The gap is named at
the site, in the `Vec` arm of `nodes/member_expression.rs`, as its own scope:
teaching that representation to be indexed is the work, and the refusal is
deliberate in the meantime rather than accidental.

Inside a dynamic style's body the refusal is not an error, which is how the
second row above was found: the value falls to the runtime as
`height: var(--x-height)` where upstream folds two static fallbacks. So the
divergence is `acceptance divergent` in a static namespace and
`structurally divergent` in a dynamic style — the same missing fold, wearing two
verdicts.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration, while measuring ticket 14.

- [ ] An evaluated array answers an index in range
- [ ] An index past the end answers `undefined`, as the object arm already does
      for a key an object does not carry
- [ ] Corpus rows for both positions, static and dynamic
