# A TypeScript module reads an unreferenced import as a type

**Status:** accepted

The type-stripping pass runs before the StyleX visitor, and in a TypeScript
module it removes an import specifier nothing references _as a value_. So a
dynamic-style parameter that shadows such an import shadows nothing, and a shape
the reference implementation refuses compiles here.

```ts
import { create, keyframes } from '@stylexjs/stylex';

export const styles = create({ dyn: keyframes => ({ height: keyframes }) });
```

| file           | `@stylexjs/babel-plugin` 0.19.0 | here                                |
| -------------- | ------------------------------- | ----------------------------------- |
| `page.js`      | `Invalid pseudo or at-rule.`    | `Invalid pseudo or at-rule.`        |
| `page.ts/.tsx` | `Invalid pseudo or at-rule.`    | `.x16ye13r{height:var(--x-height)}` |

**This is intended.** The `.js` half was fixed — `verbatim_module_syntax` is on
for JavaScript input, so a specifier nothing references survives, which is what
Babel and esbuild both do and what makes the shadowing visible. The `.ts` half
stays as it is.

## Why the gap is not closed

**Eliding is TypeScript's rule, not this compiler's.** A specifier with no value
reference may name a type, and a type has no module to import at runtime. `tsc`
removes it; so does every other TypeScript toolchain. Upstream reads the name
only because Babel merges visitors per node and runs plugins ahead of presets,
so the StyleX plugin sees the import before `@babel/preset-typescript` removes
it. That is plugin ordering rather than a considered TypeScript semantics, which
is what makes this a decision rather than an obvious defect.

**The mechanism that would close it is unsafe, and its hazard is the common
case.** Two orderings were ruled out while the `.js` half was being fixed:
running the StyleX visitor before the strip breaks five tests across two
packages, because the strip then runs on StyleX's _output_ and elides every
import StyleX has just made unreferenced; and handing the visitor a pre-strip
snapshot leaves the napi pipeline registering imports from the snapshot while
`cargo test` registers them off the AST, which rebuilds the split that hid the
original defect.

The third candidate — re-inserting the elided specifiers of a **StyleX import
source only** back into the AST after the strip and before the visitor — keeps
one source of truth, and fails on what it re-inserts. `@stylexjs/stylex` exports
`StyleXStyles`, `StaticStyles`, `Theme`, `VarGroup`, `CSSProperties` and
`Keyframes` among a dozen more as `export type`, absent from the runtime module.
Written without the `type` keyword, which is how it is written in practice:

```ts
import { StyleXStyles } from '@stylexjs/stylex';
```

that specifier is elided by inference, is indistinguishable at the AST from the
`keyframes` above, and re-inserting it emits an import of a binding the runtime
module does not export — a link error under ESM, `undefined` under CJS interop.
Verified against the built compiler: the specifier above is elided from a `.ts`
module and kept in a `.js` one, and `importElision.spec.ts` pins both.

A re-elide step after the visitor would remove what the re-insertion added, and
that is a second mechanism guarding the first, added to serve the second reason
below — which argues the other way anyway.

**Closing it turns working builds into failing ones.** `.tsx` is most of the
StyleX written anywhere. Making it refuse means every module whose dynamic-style
parameter happens to share a name with a StyleX API export or a theme import
stops compiling — to reproduce an upstream answer that is the less defensible of
the two. A parameter is a parameter; `height: keyframes` is an ordinary dynamic
value, and `var(--x-height)` is the right reading of it. Upstream's refusal is
the accident here.

## What it does not cost

**No class name moves.** The divergence changes only _which programs compile_,
never the bytes of one that compiles under both. That is the compatibility
contract, and it is intact: a module that compiles on both sides compiles to the
same CSS and the same class names.

**The `.js` half agrees byte for byte.** Sixteen inputs were measured through
the napi pipeline across the family — `keyframes`, `firstThatWorks`,
`positionTry`, a bare `when`, an alias, a non-ASCII alias, the namespace import,
and every hostile shape beside the fold — and the `.js` half matches upstream on
all of them.

## Consequences

**It is documented where an author will look**, not only in a test:
`stylex-rs-compiler`'s README carries it under _Deliberate divergences from
`@stylexjs/babel-plugin`_, beside the four rejections that go the other way.
That section previously said "four values that upstream accepts are rejected
here"; this is the one that upstream rejects and is accepted here, and it is
labelled as such rather than folded into the table.

**It is pinned as intended rather than as measured.**
`__test__/importElision.spec.ts`, under _a TypeScript module keeps the elision_,
asserts that the reported module still compiles in every TypeScript extension,
that an unreferenced specifier is still elided, and — the case that carries the
reason — that a type-only-by-inference specifier is still elided. A future
reader who wants to close the gap has to make that last case fail, which is the
right thing to have to argue with.

**Reopening needs a new reason.** An upstream change that stops reading a
stripped import, or a StyleX release that exports its types as values, would
each dissolve half of this. A fresh reading of the `.js`/`.ts` asymmetry would
not.
