# 07 — `borderTop` is emitted here, dropped upstream, rejected by design

Status: needs-triage
Phase: Phase 2

**What to decide, then build:** what a `border` shorthand does.

Found while comparing the reporter's output. It is **not** an `sx` bug — the
conditional-style pipeline is fine. `sx={[styles.base, flag && styles.alt]}`
with `alt: { color: "red" }` folds identically in both compilers. The
divergence is the declaration `borderTop: "none"` in the reporter's `alternate`
style.

Three different behaviours for one input:

| | `borderTop: "none"` | `border: "none"` | `borderInline: "1px solid red"` |
| --- | --- | --- | --- |
| upstream source intent | throws | throws | throws |
| `@stylexjs/babel-plugin` 0.19.0 actual | silently drops | silently drops | silently drops |
| this compiler | `.x76ihet{border-top:none}` | silently drops | `.xtt68lu{border-inline:...}` |

`preprocess-rules/property-specificity.js:51` is explicit about the intent:

```js
borderTop: (_rawValue) => {
  throw new Error(
    'borderTop is not supported. Use borderTopWidth, borderTopStyle and borderTopColor instead.',
  );
},
```

The same holds for `border`, `borderInline`, `borderBlock` and the per-side
variants. Yet 0.19.0 emits no rule and no error, so the throw is being
swallowed somewhere between `flatten-raw-style-obj.js` and the caller — that is
an upstream defect worth reporting.

**This compiler is inconsistent with itself**, which is the part that is
unambiguously wrong regardless of what upstream does: `border` drops silently
while `borderTop` and `borderInline` emit a shorthand rule. A shorthand that
reaches the stylesheet defeats the specificity model
`property-specificity` exists to enforce — a later `borderTopWidth` cannot
reliably override `border-top`.

**Needs a decision before any code**, because every option is a breaking change
for someone:

- **Reject**, matching upstream's stated intent and its error text. Correct, and
  it fails builds that compile today.
- **Drop silently**, matching upstream's observed 0.19.0 behaviour. Byte parity
  with the reference implementation, but it discards a style the author wrote
  and neither compiler warns.
- **Keep emitting**, and accept the divergence.

Whichever is chosen, the three properties must agree with each other. Verify
the default `styleResolution` in play before concluding — the
`legacy-expand-shorthands` mode expands these rather than rejecting them, and
the answer may differ per mode.
