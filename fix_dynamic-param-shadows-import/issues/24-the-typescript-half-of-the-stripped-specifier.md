# 24 — The TypeScript half of the stripped specifier

Status: `needs-triage`
Blocked by: None

**What was found:** [22](./22-the-stripped-specifier-the-fold-never-sees.md)
stopped the type-stripping pass from eliding import specifiers in a JavaScript
module, which is where eliding them was simply wrong. A TypeScript module keeps
the elision, so the reported shape still compiles there.

```ts
import { create, keyframes } from '@stylexjs/stylex';
export const styles = create({ dyn: (keyframes) => ({ height: keyframes }) });
```

| | |
| --- | --- |
| Babel 0.19.0 | `Invalid pseudo or at-rule.` |
| rs-compiler, `page.js` | `Invalid pseudo or at-rule.` |
| rs-compiler, `page.ts` | `.x16ye13r{height:var(--x-height)}` plus an `@property` rule |

Measured across the family, all through the napi pipeline with a `.ts`
filename: `keyframes`, `firstThatWorks`, `positionTry`, a bare `when`, an alias,
a non-ASCII alias, the namespace import, and every hostile shape beside the fold
— sixteen inputs where the `.js` half now agrees byte for byte with upstream and
the `.ts` half compiles. Pinned as it stands in
`crates/stylex-rs-compiler/__test__/importElision.spec.ts`, under
`a TypeScript module keeps the elision`.

## Why 22 stopped where it did

Eliding a specifier nothing references as a value is TypeScript's own rule: the
binding may name a type, and a type has no module to import at runtime. `tsc`
does it, and it is why a blanket `verbatim_module_syntax` is not available —
preserving such a specifier makes the emitted module import a file that may hold
nothing at all.

Upstream reads the name only because Babel merges visitors per node and runs
plugins ahead of presets, so the StyleX plugin sees the import before
`@babel/preset-typescript` removes it. That is plugin ordering, not a considered
TypeScript semantics — which is what makes this a *decision* rather than an
obvious defect, and why it is filed rather than fixed in passing.

Against that: the two answers now depend on the filename for byte-identical
JavaScript, and `.tsx` is most of the StyleX written anywhere. A build that
fails on `page.js` and succeeds on `page.ts` is a surprising thing to ship, even
when each half is defensible on its own.

## What was already ruled out

- **Run the StyleX visitor before the strip.** Upstream's order. Fixes every
  extension, and breaks five tests across two packages: the strip then runs on
  StyleX's *output* and elides every import StyleX has just made unreferenced,
  including the `./x.stylex` edge `@stylexswc/unplugin` walks to resolve an
  imported `defineConsts` at-rule. Evidence in 22.
- **Hand the visitor a pre-strip snapshot of the imports.** Fixes every
  extension and the whole suite is green, but the pipeline would then register
  imports from the snapshot while `cargo test` registers them off the AST —
  rebuilding the split that hid this defect in the first place. Evidence in 22.

## The shape a fix would have to take

Neither ruled-out option is disqualified by *what* it achieves, only by how. A
third mechanism would have to keep one source of truth and touch only the
specifiers StyleX needs. The candidate not yet measured is re-inserting the
elided specifiers of a **StyleX import source only** back into the AST after the
strip and before the visitor: one source of truth (the AST), and the runtime
hazard is bounded, because a StyleX import source is a real module. Its cost is
that a `.ts` module which keeps its StyleX import — `types` is the one that does
— would emit a specifier `tsc` had removed, which is a lint warning at best and
wrong emit at worst.

Measure that before choosing, the way 22 measured its three.

- [ ] The re-insertion mechanism is measured against the whole suite
- [ ] A decision is recorded: close the gap, or record the `.ts` answer as
      intended and say so in the compiler's docs rather than only in a test
- [ ] `__test__/importElision.spec.ts::a TypeScript module keeps the elision`
      says whichever it turns out to be
