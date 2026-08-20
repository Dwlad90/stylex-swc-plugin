# Issue #1269 — metadata dedup: not a bug

Issue: https://github.com/Dwlad90/stylex-swc-plugin/issues/1269
Verdict: **planned behaviour — close as *not planned***
Upstream reference: `~/Projects/Facebook/stylex` @ `5f51b244` (v0.19.0)

## Repro

```js
import * as stylex from '@stylexjs/stylex';
export const alignStyles = stylex.create({
  right: { justifyContent: 'flex-end' },
});
export const styles = stylex.create({
  base: { justifyContent: 'flex-end' },
});
```

Both compilers run with `dev: false`, `unstable_moduleResolution: { type: 'commonJS' }`.

| | `metadata.stylex` entries | emitted code |
| --- | --- | --- |
| Babel 0.19.0 | 2 identical tuples | identical |
| rs-compiler | 1 | identical |

## Evidence

### Why Babel emits two

`packages/@stylexjs/babel-plugin/src/utils/state-manager.js:726` — `addStyle` is an
unconditional `this.metadata.stylex.push(style)`. No dedup anywhere on that path.
The duplicate is a side effect of an append-only array, not a designed signal.

### Why it provably cannot matter

Upstream's own CSS assembler dedupes it right back —
`packages/@stylexjs/babel-plugin/src/index.js:673`:

```js
const collectedCSS = Array.from(new Map(group.map(([a, b]) => [a, b])).values())
```

Keyed by class name. Our packages call that *same* upstream `processStylexRules`:

- `packages/plugin-shared/src/plugin-core.ts:358`
- `packages/postcss-plugin/src/bundler.ts:105`

So the CSS is byte-identical by construction, not by coincidence.

### Runtime-injection parity is intact

The case that *would* be a bug. With `dev: true, runtimeInjection: true` we emit
**both** `_inject2({ ltr: ".x13a6bvl{justify-content:flex-end}", priority: 3000 })`
calls, exactly like Babel.

Our metadata dedup (`crates/stylex-transform/src/shared/structures/state_manager.rs:1353`,
`IndexSet<MetaData>` keyed on class name + style + priority) is whole-file; the
inject-side dedup is per-declarator. Nothing observable at runtime diverges.

The duplicate tuples are byte-identical, so no information is lost — there is nothing
a consumer could recover from the second entry.

## Ready-to-paste closing comment

Thanks for the precise repro — this is expected behaviour, not a bug, and we're going
to keep the deduplicated metadata.

**Why Babel emits two entries.** `StateManager.addStyle` in the Babel plugin is an
unconditional `this.metadata.stylex.push(style)`
([`state-manager.js:726`](https://github.com/facebook/stylex/blob/v0.19.0/packages/%40stylexjs/babel-plugin/src/utils/state-manager.js#L726)).
There's no dedup on that path, so a second `create()` producing the same declaration
appends a second, byte-identical tuple. The duplicate is a side effect of an
append-only array, not a signal that carries information.

**Why it can't affect output.** Babel's own CSS assembler removes it again before
emitting anything. In `processStylexRules`
([`index.js:673`](https://github.com/facebook/stylex/blob/v0.19.0/packages/%40stylexjs/babel-plugin/src/index.js#L673)):

```js
Array.from(new Map(group.map(([a, b]) => [a, b])).values())
```

Rules are keyed by class name, so duplicates collapse. Our bundler integrations call
that same upstream `processStylexRules`, which is why both compilers emit identical
stylesheets — by construction, not coincidence.

**Runtime injection is at parity.** With `dev: true, runtimeInjection: true` we emit
both `_inject2(...)` calls, exactly like Babel. Our metadata dedup is whole-file; the
injection-site dedup is per-declaration. Nothing observable at runtime differs.

**What we're deliberately choosing.** We accumulate rules in an insertion-ordered set
keyed on `(className, style, priority)`. Since the class name is a hash of the
declaration itself, equal keys imply equal rules — a dropped entry is always an exact
copy of one we kept. That gives us a metadata array that is already canonical:
`length` means "distinct rules in this file", which is the number that's actually
useful for caching and incremental rebuilds.

The one real cost is the one you identified: a snapshot test that diffs
`metadata.stylex` arrays across the two compilers will show a length difference. If
you're writing such a test, compare the deduplicated sets — that comparison holds for
both compilers and is stable across this class of Babel-internal change.

Closing as *not planned*. If you hit a concrete consumer that genuinely needs the
duplicate entry, please reopen with the case and we'll revisit.

## Unrelated finding (needs its own issue)

`crates/stylex-types/src/structures/meta_data.rs:31-39` — `MetaData`'s hand-written
`Hash` calls `hash_f64(self.priority)` and discards the returned `u64` without feeding
it into the hasher, so `priority` never participates in the hash. Correctness is safe
(`Eq` still compares it), but it's a dead line and a needless bucket collision. The
`impl` is annotated `#[cfg_attr(coverage_nightly, coverage(off))]`, so coverage would
never have flagged it.
