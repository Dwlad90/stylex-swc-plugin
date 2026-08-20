# 06 — Measure the default-import step, then mirror it or rule it out

Status: `resolved`
Blocked by: 04

**What to build:** A verdict on step 2 of the chain — either the step, or a
comment saying why there is no step.

The reference implementation refuses a reference that resolves to a *default*
import specifier, with a distinct message. We treat one as a theme reference
like any other. Our message constant for it is commented out alongside the two
that 03 and 05 revive — but unlike those two, there is no measured divergence
behind it yet.

So measure first. Put a default import of a theme file through both compilers
and compare. If the outputs differ, mirror the step and revive the constant. If
they agree, leave the step out and record *at the site* that the difference is
deliberate and what was measured — an absent step with no explanation is what
invites the next reader to add it speculatively.

Either outcome is a complete ticket. The deliverable is the verdict, not the
code.

- [x] Both compilers measured on a default theme import, result recorded
- [x] If they diverge: the step lands, the constant is revived, corpus entry
      added with the verdict it reads
- [ ] ~~If they agree~~ — they do not; this branch is closed

## Comments

**Measured while implementing 02 — they diverge, so the step lands.**

```js
import * as stylex from '@stylexjs/stylex';
import tokens from 'tokens.stylex.js';
export const styles = stylex.create({ wrapper: { color: tokens.color } });
```

| | verdict |
| --- | --- |
| `@stylexjs/babel-plugin` 0.19.0 | refuses: *There was an error when attempting to evaluate the imported file…* |
| rs-compiler | accepts, emitting `.x…{color:var(--xe7srj8)}` |

Measured with no shadowing anywhere in the module, so the divergence is about the
import kind and nothing else. Adding a dynamic parameter that shadows the default
binding does not change either side's answer.

So step 2 of the chain exists, `IMPORT_FILE_EVAL_ERROR` gets revived, and this
ticket's "leave a comment saying the absence is deliberate" branch is dead.

One thing to fix while landing it: `dynamic_param_shadows_a_default_theme_import`
in `crates/stylex-transform/tests/transform_stylex_create_test/dynamic_styles.rs`
currently snapshots the accepting behaviour, with a comment saying it is not a
parity claim. This ticket rewrites that snapshot.

## Answer

The step landed. `IMPORT_FILE_EVAL_ERROR` is revived in
`stylex-constants::constants::evaluation_errors`, and step 2 of the chain refuses
a reference whose binding is a default import specifier.

Where the step sits: nested inside step 1's block rather than after it, because
the two read one lookup and ask one question — which *specifier* binds this
reference. Upstream's step 1 guard is
`!bindingPath.isImportDefaultSpecifier() && !bindingPath.isImportNamespaceSpecifier()
&& bindingPath.isImportSpecifier()`, so a default specifier never reaches the
theme-reference resolution there either; the order is upstream's however the two
are nested. The specifier is what the question is asked of, not the declaration,
because `import tokens, { colors } from 'colors.stylex.js'` has to refuse
`tokens` and still resolve `colors`.

Two things the measurement did not predict:

- The refusal is **not** gated on `disable_imports`, because upstream gates only
  the resolution inside step 1 on `state.functions.disableImports` and reaches
  its default-import refusal either way.
- Upstream does not `return` from the refusal; the reference falls through the
  rest of the chain and deopts a second time with `UNDEFINED_CONST`. The first
  deopt wins on both sides, so the fall-through is unobservable and this returns.

Corpus entries added, all three reading their recorded verdict against
`@stylexjs/babel-plugin` 0.19.0:

| entry | verdict |
| --- | --- |
| `modules-1266-default-theme-import` | `both reject` (was `acceptance divergent`) |
| `modules-1266-default-theme-import-shadowed-by-a-dynamic-param` | `identical` |
| `modules-1266-a-named-theme-import-beside-a-default-one` | `identical` |

Two existing tests changed, both into agreement:

- `dynamic_param_shadows_a_default_theme_import` no longer reads the import
  outside the dynamic style, so it still guards the `Default` arm of the binding
  match — and guards it harder than the accepting snapshot did, since a
  regression to a name match now refuses instead of emitting different CSS. Its
  unshadowed half moved to `validation_stylex_create_test::invalid_values` as the
  refusal.
- `invalid_value_non_static_import_default` in
  `validation_stylex_create_test::style_declarations` refused for the *path*
  before and refuses for the *specifier* now, which is upstream's reason for the
  same input.

Found in code review and filed rather than fixed: `IMPORT_FILE_EVAL_ERROR` has a
second upstream call site, at the tail of step 1, where a resolution that comes
back unconfident is refused as an imported file upstream could not fold. This
chain falls through silently there. Noted at the site and owned by
[13](./13-measure-the-unconfident-resolution-refusal.md) — mirroring it here
would be adding a step on the strength of how the two implementations look, which
is the thing the chain's ADR forbids.

Two more corpus entries came out of the same review, both measured rather than
reasoned about, and both `both reject`: `modules-1266-default-import-of-a-non-theme-file`
and `modules-1266-a-call-through-a-default-import` — the second being the shape
`invalid_value_non_static_import_default` asserts, whose message this change
rewrote.

Out of scope but touched: `packages/design-system/README.md` documented
`import tokens from '@stylexswc/design-system/tokens.stylex'`, a default import
of a file with no default export. It never worked; it is now a hard refusal, so
the one-word fix to a named import went with this change.
