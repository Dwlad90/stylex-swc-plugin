# 18 — A theme object read as a style value is dropped

Status: `ready-for-agent`
Blocked by: None

**What was found:** A theme import read as a style value *without* a member
access compiles to nothing. The reference implementation refuses it. We emit no
rule, no error, and no warning -- the declaration disappears.

```js
import { zIndex } from './vars/zIndex.stylex.js';
export const styles = stylex.create({ w: { zIndex: zIndex } });
```

| | |
| --- | --- |
| Babel 0.19.0 | `A style value can only contain an array, string or number.` |
| here | compiles, emits nothing |

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration -- `haste` resolution, one source string.

| input, inside `create({ … })` | Babel 0.19.0 | here |
| --- | --- | --- |
| `w: { zIndex: zIndex }` | refuses | `ok`, no rule |
| `w: { color: 'red', zIndex: zIndex }` | refuses | `ok`, only the `color` rule |
| `w: { zIndex: { default: zIndex } }` | refuses | `ok`, no rule |
| `w: { margin: zIndex }` | refuses | `ok`, no rule |
| `w: { zIndex: \`${zIndex}\` }` | `z-index:x1q8i56t` | `ok`, no rule |
| `w: { zIndex: t }`, `import * as t` | `Referenced constant is not defined.` | `ok`, no rule |
| `dyn: (a) => ({ zIndex: zIndex, color: a })` | refuses | `Style value must evaluate to a static expression.` |

The mechanism is one line. `nodes/object_expression.rs:288` answers a
`ThemeRef` with `None`:

```rust
EvaluateResultValue::ThemeRef(_) => None,
```

and the caller reads `if let Some(value) = value` before pushing the property --
so `None` means "no property", not "refuse". Every arm beside it either folds to
an expression or deopts; this is the only one that drops the declaration on the
floor. A member read (`zIndex._10`) never reaches it, which is why this went
unnoticed: that is the shape every test and every fixture writes.

The one row where the reference implementation *emits* rather than refuses is
the template, and it emits the theme's hash as a bare string -- `z-index:
x1q8i56t`. Worth confirming that is intended upstream before mirroring it;
refusing may be the better answer even though it diverges.

Found while reviewing [08](./08-reject-a-folded-map-as-a-namespace.md), whose
`materialize_style_value` docstring claimed the style-value fall-through was
reachable by two shapes. A theme reference is the third, and it is the one the
spec opens with -- so this is the same seam, reached by a shape ticket 08 did not
audit. Related to [14](./14-an-array-style-value-inside-a-dynamic-style.md) and
[15](./15-the-function-map-read-where-it-is-not-a-map.md): all three are
"an evaluated shape with no expression form, read where a style value belongs".

Worth deciding together with 15 case 1, which is the same consumer
(`object_expression.rs`) refusing with the wrong words rather than not refusing
at all.

- [ ] A bare theme object read as a style value refuses, with the reference
      implementation's text
- [ ] A member read off the same import still resolves to its theme reference
- [ ] The dropped-declaration path is gone: no arm of that match answers `None`
      in a way the caller reads as "no property"
- [ ] The namespace-import spelling is decided too -- upstream reads
      `Referenced constant is not defined.`
- [ ] The template row is measured against upstream intent before it is mirrored
- [ ] Corpus entries for each row above, with the verdict each is known to read
