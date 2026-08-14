# 04 — `Number(x)` around a style value compiles

**What to build:** Numeric coercion folds the way JavaScript does, so a numeric
token written as a string produces the same rule as the number itself.

This is the largest correctness surface in the spec, which is why it stands
apart from ticket 03. Rust's float parsing is not a substitute for the
language's: it rejects `0x1f` and surrounding whitespace, which JavaScript
accepts, and it accepts `inf` and `nan`, which JavaScript rejects. Each of those
divergences produces a *wrong stylesheet* rather than a failed build — silently
incorrect CSS, not a visible error — so the coercion is worth implementing
against the numeric-literal grammar rather than approximating.

`NaN` is a legitimate result and flows into the value, as it does upstream.

The string cases, all measured against the reference implementation:

| input        | result     |
| ------------ | ---------- |
| `''`         | `0`        |
| `'  10  '`   | `10`       |
| `'10'`       | `10`       |
| `'0x1f'`     | `31`       |
| `'0b101'`    | `5`        |
| `'0o17'`     | `15`       |
| `'1e3'`      | `1000`     |
| `'Infinity'` | `Infinity` |
| `'-0'`       | `-0`       |
| `'10px'`     | `NaN`      |
| `'inf'`      | `NaN`      |
| `'nan'`      | `NaN`      |

And the non-string cases: `null` is `0`, `undefined` is `NaN`, booleans are `1`
and `0`, an empty array is `0`, a single-element array coerces its element, a
longer array is `NaN`, and a plain object is `NaN`.

The wide matrix is why the coercions are a separate test seam from the transform
fixtures — routing forty cases through full transforms would be slow and would
obscure which coercion failed. The matrix belongs in unit tests beside the
crate's existing predicate tests; the transform fixtures carry a representative
few, including the hexadecimal case that motivates the whole grammar.

**Blocked by:** 03 — reuses the globals enum, the callback variant, and the
apply-site wiring introduced there.

**Status:** done

- [x] The coercion is unit-tested across the full matrix above, genuinely wide
      rather than sampled
- [x] Transform fixtures cover numeric coercion in a style value, including
      `'0x1f'` folding to `31`
- [x] `NaN` flows into the value rather than raising
- [x] The Rust float spellings `inf` and `nan` are `NaN`, not infinity
- [x] Expected values are taken from measured reference output

**Found while measuring:** `Number(fn)` folds to `NaN` upstream rather than
failing, where `String(fn)` needs the source text this evaluator does not keep.
No function's source text is a numeric literal, so the number is knowable
without it — the coercion answers `NaN` rather than deopting, and the two
callees deliberately differ on this input.

The parity check ran the coercion against a JavaScript runtime over 10,522
generated strings — every two- and three-piece combination of sign, radix
prefix, digit, point, exponent, whitespace and suffix fragments — with no
divergence. That harness is a development instrument and is not committed.
