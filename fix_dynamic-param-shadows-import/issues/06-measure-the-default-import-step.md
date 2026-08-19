# 06 — Measure the default-import step, then mirror it or rule it out

Status: `ready-for-agent`
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
- [ ] If they diverge: the step lands, the constant is revived, corpus entry
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
