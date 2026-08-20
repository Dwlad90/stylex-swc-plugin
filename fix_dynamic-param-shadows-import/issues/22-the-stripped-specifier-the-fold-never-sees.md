# 22 — The stripped specifier the fold never sees

Status: `needs-triage`
Blocked by: None

**What was found:** `typescript_strip` runs ahead of the transform, and it elides
an import specifier with no value reference. A dynamic style's parameter is not a
reference to the binding it shadows — so when the parameter is the specifier's
only occurrence in the module, the specifier is gone before anything could
register it, and every question about what the name means answers *nothing was
imported*.

```js
import { create, keyframes } from '@stylexjs/stylex';
export const styles = create({ dyn: (keyframes) => ({ height: keyframes }) });
```

| | |
| --- | --- |
| Babel 0.19.0 | `Invalid pseudo or at-rule.` |
| rs-compiler | `.x16ye13r{height:var(--x-height)}` plus an `@property` rule |
| `cargo test`, same source | `Invalid pseudo or at-rule.` |

The third row is the whole finding. The Rust suite runs the resolver but not
`typescript_strip`, so it sees the specifier and refuses — the fix from
[16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md) is correct
and reachable. The napi pipeline
(`crates/stylex-rs-compiler/src/lib.rs`) applies
`resolver → typescript_strip → the stylex visitor`, and the middle pass removes
the input the last one needed.

This is the mechanism
[01](./01-pin-why-an-unused-import-spares-the-shadowed-parameter.md) pinned,
seen from the other side. There it explained why an unused import *spared* a
shadowed parameter, which was the benign direction — a module that compiled
where it should have compiled. Here the same elision costs a refusal, which is
the direction that ships CSS the reference implementation refuses.

Eliding the specifier is faithful TypeScript: a binding with no value reference
may be a type, and TS removes it. What is not faithful is that the reference
implementation never strips before it reads, so the two compilers are asked
different questions. Any fix is a pipeline-ordering decision, not a change at
the identifier seam:

- run the stylex visitor before `typescript_strip`, which is upstream's order and
  the widest blast radius of the three;
- collect the module's imports before stripping and hand them to the visitor,
  which keeps the order and splits one source of truth in two;
- pass `verbatim_module_syntax` for a non-`.ts` input, which is the narrowest and
  answers only for `.js` and `.jsx` files — the reported input among them.

Measure the suites under each before choosing. The corpus row is
`modules-1266-param-shadows-a-named-import-referenced-nowhere-else`
(`acceptance-divergent`); the shape that keeps the specifier alive is
`modules-1266-param-shadows-a-named-function-map-import`, which reads
`both-reject` and is what proves the seam itself is right.

- [ ] The three orderings are measured against the whole suite
- [ ] The corpus row stops reading `acceptance-divergent`
- [ ] The theme-import shapes 01 measured are re-measured under the choice
