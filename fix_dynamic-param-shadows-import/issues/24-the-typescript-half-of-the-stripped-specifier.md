# 24 — The TypeScript half of the stripped specifier

Status: `wontfix`
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
- [x] A decision is recorded: close the gap, or record the `.ts` answer as
      intended and say so in the compiler's docs rather than only in a test
- [x] `__test__/importElision.spec.ts::a TypeScript module keeps the elision`
      says whichever it turns out to be

## Answer

The `.ts` answer is recorded as intended, in the compiler's docs rather than
only in a test. The re-insertion mechanism was costed rather than built, because
costing it turned up a hazard that disqualifies it before a suite run could say
anything, and a second reason that argues against closing the gap at all.

### The first box is left unticked

It asked for the mechanism to be *measured against the whole suite*, and it was
not: it was costed, and the costing disqualified it before a suite run could say
anything. That is a good reason to stop and not a reason to call the box done,
so it stays `[ ]`. The decision the second box asks for is genuinely recorded,
and it does not depend on the suite run -- the second reason below stands on its
own even if the mechanism were made safe.

### The mechanism fails on what it re-inserts

Re-inserting the elided specifiers of a StyleX import source keeps one source of
truth, which is what recommended it over the two options 22 ruled out. It also
re-inserts the *type-only-by-inference* ones, and those are the common case
rather than an edge:

```ts
import { StyleXStyles } from '@stylexjs/stylex';
```

`@stylexjs/stylex` 0.19.0 exports `StyleXStyles`, `StaticStyles`,
`StaticStylesWithout`, `StyleXStylesWithout`, `StyleXClassNameFor`, `StyleXVar`,
`StyleXArray`, `Theme`, `VarGroup`, `PositionTry`, `CSSProperties`,
`CompiledStyles`, `InlineStyles`, `Keyframes`, `MapNamespaces` and `Types` under
`export type` — confirmed absent from `lib/es/stylex.js`. Written without the
`type` keyword, which is how it is written in practice, such a specifier is
elided by inference and is **indistinguishable at the AST from the `keyframes`
the ticket is about**. Re-inserting it emits an import of a binding the runtime
module does not export: a link error under ESM, `undefined` under CJS interop.

Verified against the built `dist/*.node`, not reasoned:

| input | file | result |
| --- | --- | --- |
| `import { StyleXStyles } from '@stylexjs/stylex'` | `.ts` | elided |
| the same line | `.js` | kept |
| the reported shape | `.ts` | `x16ye13r`, `--x-height` |

The first row is the shape the re-insertion would put back. Now pinned as
`importElision.spec.ts::a type-only-by-inference specifier is still elided`, so
a future reader who wants the mechanism has to make that case fail first.

A re-elide pass after the visitor — removing exactly what the re-insertion added
and nothing else — would repair it, and it is a second mechanism guarding the
first. It was not built, because of the reason below.

### Closing the gap costs more than the gap does

The ticket weighed "a build that fails on `page.js` and succeeds on `page.ts` is
a surprising thing to ship". True, and the direction of the surprise matters:
`.ts` is the half that **succeeds**. Closing the gap means making it fail.

`.tsx` is most of the StyleX written anywhere, so that is every module whose
dynamic-style parameter happens to share a name with a StyleX API export or a
theme import — and it would be to reproduce an upstream answer that is the less
defensible of the two. A parameter is a parameter; `height: keyframes` is an
ordinary dynamic value and `var(--x-height)` is the right reading of it.
Upstream refuses only because Babel runs plugins ahead of presets, so its StyleX
plugin sees an import `@babel/preset-typescript` is about to remove. That is
plugin ordering, and this ticket already said so.

And the compatibility contract is intact either way: the divergence changes only
*which programs compile*, never the bytes of one that compiles under both. No
class name moves.

### Where it is written down

- `crates/stylex-transform/docs/adr/0007-a-typescript-module-reads-an-unreferenced-import-as-a-type.md`
  — the decision, the ruled-out mechanisms, and what would reopen it.
- `crates/stylex-rs-compiler/README.md`, under *Deliberate divergences from
  `@stylexjs/babel-plugin`* — where a person surprised by it will actually look.
  That section listed four values upstream accepts and this compiler rejects;
  this is the one that goes the other way, and it is labelled as such rather
  than folded into the table.
- `__test__/importElision.spec.ts::a TypeScript module keeps the elision` now
  says *intended* rather than *measured*, and carries both reasons in its
  comment plus the type-only case that makes the first one concrete.

### What would reopen it

An upstream change that stops reading a stripped import, or a StyleX release
that exports its types as values — either dissolves half of this. A fresh
reading of the `.js`/`.ts` asymmetry would not.
