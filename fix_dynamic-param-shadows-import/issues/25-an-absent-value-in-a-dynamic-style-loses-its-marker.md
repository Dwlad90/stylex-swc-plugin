# 25 — An absent value in a dynamic style loses its marker

Status: `needs-triage`
Blocked by: None

**What was found:** A style value that is absent — `null`, an empty string, or
an array whose every element is one — keeps its property in the emitted style
object upstream and loses it here, but only inside a *dynamic* style's body. The
static namespace agrees with the reference implementation.

```js
export const styles = stylex.create({ dyn: (h) => ({ height: null }) });
```

| compiler | emitted |
| --- | --- |
| Babel 0.19.0 | `const _temp = { kZKoxP: "", "$$css": true }; … h => [_temp, {}]` |
| this compiler | `(h) => ({})` |

The empty string is how an absence is spelled in a compiled style object, and it
is what unsets an earlier declaration of the same property when two styles
merge. Dropping the key means a dynamic style can no longer unset what a static
one beside it declared, which is a difference the rule text cannot show — no
rule is emitted either way.

The static position already agrees:
`create({ s: { height: [null, null] } })` emits `kZKoxP: null` in both
compilers, and `absent_style_values.rs` pins that half.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration. Found while measuring ticket 14 — an array of nothing but
absences collapses to exactly this shape, which is why the corpus row that
records it sits with the array rows:

| input, inside `create({ dyn: (h) => ({ … }) })` | verdict |
| --- | --- |
| `height: null` | structurally divergent |
| `height: ['', '']` | structurally divergent |
| `height: [null, null]` | structurally divergent |

Not an array question: `height: null` alone shows it just as loudly, which is
why it is not ticket 14's to fix.

- [ ] A verdict on whether the marker belongs in a dynamic style's body
- [ ] Either the marker, or a recorded reason not to emit one
- [ ] `modules-1266-an-array-of-nulls-in-a-dynamic-style` reads `identical`, or
      its note says why it still does not
