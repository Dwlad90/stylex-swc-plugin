# 27 — An index read off an array refuses where the reference implementation folds one

Status: `resolved`
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

- [x] An evaluated array answers an index in range
- [x] An index past the end answers `undefined`, as the object arm already does
      for a key an object does not carry
- [x] Corpus rows for both positions, static and dynamic

## Resolved

Both array receivers read an index now, on the language's own terms rather than
on the representation's:

- A **canonical digit key** names a slot, however it was written. `A[0]`,
  `A["0"]` and `A[1.0]` are one element, and `A["00"]` is not an element at all
  -- a digit test alone would have read it as slot zero, which is the one way
  this can be confidently wrong rather than merely refused.
- **Past the end is `undefined`**, which is what a key an object does not carry
  already answered. That is what lets `A[7] ?? '2px'` fold; answering no value
  sent the whole declaration to the runtime instead. Bare, the `undefined`
  reaches the style-value check and is refused there.
- The array literal a fold produced took the same reading, so the two receivers
  cannot drift again. It read a slot only where the index was written as a
  numeric literal, and answered *no value at all* past the end -- a confident
  `None` its callers read as nothing to see.

The receiver still refuses ahead of the index where it cannot be counted: a
spread stands for however many elements its value holds, and a hole has no
value. A string still refuses an index, for the reason it always did -- its
element is a single UTF-16 code unit, which can be an unpaired surrogate no Rust
string holds.

`index_slot` in `nodes/member_expression.rs` is where the canonical-spelling
rule lives, once, for both receivers.

Measured against `@stylexjs/babel-plugin` 0.19.0 through the parity harness:
`modules-an-index-read-off-an-array-in-a-static-namespace` and
`modules-an-index-read-off-an-array-in-a-dynamic-style` both read `identical`
where they read `acceptance-divergent` and `structurally-divergent`, and
`modules-1266-an-index-read-past-the-end` is new and `identical`. The whole
corpus reports no changed verdict.

Pinned in `evaluate/tests/array_index_tests.rs` -- every key shape, both
receivers, the overflow, the non-ASCII digits and the shapes that still refuse
-- and in `transform_stylex_create_test::array_index_reads`, which asks what the
compiler emits in both style-value positions.
