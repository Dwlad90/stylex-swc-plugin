# 22 — The stripped specifier the fold never sees

Status: `resolved`
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

- [x] The three orderings are measured against the whole suite
- [x] The corpus row stops reading `acceptance-divergent`
- [x] The theme-import shapes 01 measured are re-measured under the choice

## Resolution

**The third ordering**, narrowed one step further: the type-stripping pass keeps
every import specifier when the input names a JavaScript file, and keeps
TypeScript's elision otherwise. `crates/stylex-rs-compiler/src/lib.rs` swaps
`typescript::strip` for `typescript(Config { verbatim_module_syntax, .. })` and
answers that flag from the filename's extension.

The line is not "the reported input is a `.js` file". It is that eliding a
specifier with no value reference is *TypeScript's* rule and only TypeScript's:
a binding nothing references as a value may name a type, and a type has no
module to import at runtime. JavaScript has no type-only imports, so in a `.js`
module the same specifier is a value import the author wrote, and removing it
changes what the module means. Babel and esbuild both keep it. Running the rule
over a file that is not TypeScript was the defect; the shadowed StyleX name was
what made it visible.

The narrowing is the extension list. The ticket said "non-`.ts`"; the code says
`js`, `jsx`, `mjs`, `cjs` and nothing else, matched without regard to case. An
extension no toolchain agrees on — or none at all — keeps the elision, which is
the conservative half: an elision only ever removes.

`verbatim_module_syntax` turns off less than its name suggests. Measured, not
assumed: an `import type` statement, an inline `type` specifier, a type
annotation, an interface, a type alias, an `export type` and an as-expression
are all still stripped from a `.js` module, and an enum and a namespace still
become their runtime objects. Only *inference* — "nothing references this, so it
must have been a type" — is what stops.

### What the three orderings measured

| ordering | reported shape | whole JS suite | parity |
| --- | --- | --- | --- |
| visitor before the strip | fixed, every extension | **5 failures** in 2 packages | one row, the target |
| pre-collected imports | fixed, every extension | green | one row, the target |
| `verbatim_module_syntax` for JavaScript | fixed for `.js`/`.jsx`/`.mjs`/`.cjs` | green | one row, the target |

All three fix the reported module and none of them moves any other corpus row.
The choice was made on the other two columns.

**Upstream's order fails on its own output.** Running the visitor first means
the strip runs *after* StyleX has consumed the names, so every import StyleX
just made unreferenced is elided from the emitted module. That drops
`import stylex from "@stylexjs/stylex"` — a snapshot in `__test__/index.spec.ts`
caught it — and, worse, drops `import { Container } from './container.stylex'`
once the theme constant has been folded into CSS. That import is the dependency
edge `@stylexswc/unplugin` walks to resolve an imported `defineConsts` at-rule
before transforming placeholder CSS, so four of its Vite tests stopped calling
`transformCss` at all. Verified directly: under that ordering the emitted module
for a `defineConsts` consumer carries no imports whatsoever.

**Pre-collected imports rebuild the seam this ticket is about.** It works, and
the whole suite is green. What it costs is that the shipping pipeline would then
register imports from a snapshot the napi crate hands over, while `cargo test`
registers them by walking the AST — so every Rust test in `stylex-transform`
would stop exercising the path that ships. That is exactly the condition that
hid this defect: the third row of the table at the top of this ticket is
`cargo test` answering a question the pipeline never asked. A fix whose
mechanism recreates its own hiding place is the wrong fix, even when it is
green.

### What it leaves open

A TypeScript input still compiles the reported module where the reference
implementation refuses, and that is now a stated position rather than an
accident: under TypeScript the specifier really is dead, `tsc` really does
remove it, and upstream only reads it first because Babel merges visitors and
runs plugins ahead of presets. Preserving it for a `.ts` input to match would
mean emitting an import of a module that may hold nothing at runtime.

It is the majority of real StyleX code all the same, so it is filed rather than
waved through — [24](./24-the-typescript-half-of-the-stripped-specifier.md) —
and pinned meanwhile by the `a TypeScript module keeps the elision` block of
`__test__/importElision.spec.ts`, so closing it later reads as a change instead
of as noise.

### Where the guard lives

No Rust test can reach this seam — that is the finding. So the boundary spec
`crates/stylex-rs-compiler/__test__/importElision.spec.ts` asks only what the
boundary decides: which specifiers survive the strip, and what the emitted
module carries as a result. 31 cases.

That the fold *refuses*, and what it refuses beside hostile CSS, is
`stylex-transform`'s question, and `guidelines/coding/TESTING.md` says to keep
such a question under `cargo nextest` rather than pay the boundary for it.
Nine more hostile shapes went there instead, beside the ones 16 already pinned:
a vendor-prefixed property, an at-rule with no condition, an empty condition
key, a media query holding a stray brace, and a NUL, a zero-width space, a
right-to-left override and an astral scalar in a condition key. Each reads
`Invalid pseudo or at-rule.`, byte-identical to the reference implementation,
and so — measured through both compilers — do 1, 3, 8 and 200 nested
pseudo-classes, a 20 000-token value beside the fold, and a module carrying the
shape a thousand times.

What stayed at the boundary is the extension axis, the aliased and non-ASCII
local names the strip's usage analysis has to match per binding, one entry of
each of the fold's two materializations, and the facts only an emitted module
can carry: that a JavaScript module keeps an import nothing references, that a
TypeScript one still elides it, that a side-effect import is untouched, and that
every explicitly type-only form — `import type`, an inline `type` specifier, an
annotation, an interface, a type alias, an `export type`, an as-expression — is
still stripped from a JavaScript module while an enum and a namespace still
become their runtime objects.

Two shapes there answer differently and are pinned as such: `defaultMarker`,
which is
[21](./21-a-shadowed-default-marker-param-reports-an-internal-shape.md), and an
unpaired surrogate in a condition key, which reads `String value contains
invalid UTF-8 encoding.` because it cannot cross the napi boundary at all — a
divergence that belongs to the boundary, not to this seam.

A default or namespace import cannot reach this seam at all, which is worth
saying because the first draft of the spec tested both: `stylex.create(...)`
references the binding, so nothing elides it. Only a named specifier can have a
shadowing parameter as its only occurrence.

The extension decision is a named function with its own Rust unit tests, which
*can* run under `cargo test`: case folding, `.d.ts`, a dotfile, a name that is
only an extension, multiple extensions, a directory that spells one, non-ASCII
and lookalike extensions, an empty path, and a 10 000-segment one.

### A fifth `{ fn }` entry, and it agrees

Reading the reference implementation's registration rather than trusting the
list turned up `unstable_conditional`, which `index.js:7287` spells as
`{ fn: conditionalIdentity }` exactly as `keyframes` is spelled. It is
registered from `stylexConditionalImport` rather than for every create call,
though, so — like `types` — nothing folds and the parameter stands. Both
compilers compile it. Pinned as a second guard beside `types` in
`a_dynamic_param_shadowing_a_named_unstable_conditional_import_still_compiles`,
because a fold that fired on the whole family by name would break it.

### 01's accidental sparing is gone, and nothing needed it

[01](./01-pin-why-an-unused-import-spares-the-shadowed-parameter.md) recorded
that a dynamic style whose parameter shadows a named theme import compiled *by
accident*: the import was elided, so the name match had nothing to match. For a
JavaScript input the import now survives and the match runs — and the module
still compiles, because 02's binding-aware lookup refuses to match a parameter
against an import's binding. Measured on both of 01's shapes, named and default,
with the import kept in the emitted module and the verdict unchanged;
`modules-1266-shadowed-param-beside-an-unrelated-static-prop` and
`modules-1266-default-theme-import-shadowed-by-a-dynamic-param` both still read
`identical`. Their notes said the elision was what spared them, which is no
longer true for a `.js` subject and is corrected; the same goes for
`modules-1266-import-unreferenced-elsewhere`. Both halves — the module compiles
*and* the import is still there — are asserted at the boundary now, so a
regression to the name match cannot hide behind the elision a second time.

The whole corpus reports `changed 0`, the workspace 6373 Rust tests, and the
node suite 64 of 64 tasks.
