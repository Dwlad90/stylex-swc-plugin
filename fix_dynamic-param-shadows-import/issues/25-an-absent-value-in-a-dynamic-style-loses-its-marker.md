# 25 — An absent value in a dynamic style loses its marker

Status: `resolved`
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

- [x] A verdict on whether the marker belongs in a dynamic style's body
- [x] Either the marker, or a recorded reason not to emit one
- [x] `modules-1266-an-array-of-nulls-in-a-dynamic-style` reads `identical`, or
      its note says why it still does not

## Answer

The marker belongs there, and it was never a marker: it is what the dynamic
split already spells for a property with nothing to say. Landed as one commit.

### The mechanism was one guard

`apply_dynamic_style_functions` splits a dynamic style's compiled object into a
static half and a conditional half by reading each property's class list. It
guarded that walk with `if !class_list.is_empty()`, so a property whose value
carries no class name was skipped and its key never reached either half. The
guard also made the `expr_list.is_empty()` fallback three lines below it dead
code -- and that fallback already spelled the empty string, which is upstream's
answer. Removing the guard is the whole change; nothing new is constructed.

Upstream has no such guard, and reaches `''` the same way: `classList` is `[]`
for a value that is not a string literal, `exprList` stays empty, and
`joined` falls to `t.stringLiteral('')`. So the empty string is a consequence of
the split rather than a second spelling of absence beside the static half's
`null` -- which is why the two halves disagree about how an absence looks and
both are right.

### What was measured

`@stylexjs/babel-plugin` 0.19.0 under the parity harness's configuration. Every
row the ticket named, and the neighbours that decide whether the change is as
narrow as it claims:

| input, inside `create({ dyn: (h) => ({ … }) })` | before | after |
| --- | --- | --- |
| `height: null` | key dropped, `(h)=>({})` | `_temp = { kZKoxP: "" }`, as upstream |
| `height: ['', '']` | key dropped | as upstream |
| `height: [null, null]` | key dropped | as upstream |
| `height: null, color: 'red'` | only the colour key | both keys, as upstream |
| `height: null, width: h` | two array members | three, as upstream |
| `height: { ':hover': null }` | key dropped | as upstream |
| `margin: null`, application-order | key dropped | nine longhand keys, as upstream |
| `borderTop: null` | no key | no key -- the property table refuses first |
| `(h) => ({})` | `(h)=>({})` | unchanged |

The last two are the boundary. An empty body has no property to keep, and a
shorthand the specificity table refuses expands to nothing before its value is
read, so neither grows a key -- inventing one there would emit a key the source
does not describe.

### Verification

`cargo test --workspace --all-features` 0 failed, `cargo clippy --workspace
--all-features --all-targets` clean, `cargo fmt` clean, `pnpm typecheck &&
pnpm format:check && pnpm lint:check && pnpm test` green. `parity` 0 changed
verdicts over 1009 subjects, with the two rows that recorded this divergence
now reading `identical` and five rows added for the shapes above.

### One snapshot moved, and it is the reported one

`dynamic_param_shadowing_edges::a_shadowing_param_with_nothing_to_emit` covered
an empty body and a `null` body in one case, on the assumption that both leave
nothing behind. They do not, and now the snapshot shows the parting: the empty
body keeps its bare object and the `null` body grows `_temp`. Its prose says so
and points at `absent_style_values`, which owns the rule.
