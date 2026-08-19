# 05 — Deopt a shadowed `undefined` / `NaN` / `Infinity`

Status: `ready-for-agent`
Blocked by: 04

**What to build:** A dynamic style parameter named `NaN`, `Infinity` or
`undefined` becomes an ordinary dynamic parameter, instead of failing the build.

```js
export const styles = stylex.create({ a: (NaN) => ({ width: NaN }) });
```

The reference implementation compiles this to `width: var(--x-width)` plus the
`@property` rule. We answer `Only static values are allowed inside of create()
call.`, because the three global names are returned as themselves without first
asking whether anything in scope shadows them — so the parameter is emitted as a
static value, and CSS generation rejects it downstream.

The reference implementation asks about the binding first: shadowed by one, it
refuses; unshadowed, it answers the global. Mirror that in step 7 of the chain.
The refusal message is already written and commented out beside the other
evaluation errors; revive it.

The refusal is what makes the value fall through to the inline-style path, which
is where the dynamic parameter comes from. That is the whole behaviour change —
there is no new emit path.

Carries this branch's snapshot churn, because a refusal that used to be an
answer changes what a few existing snapshots record.

- [ ] The example compiles to the reference implementation's rules
- [ ] An *unshadowed* `NaN` / `Infinity` / `undefined` in a style value still
      answers the global
- [ ] Corpus entry with the verdict it is known to read
- [ ] Snapshot in the dynamic-styles tests, beside the existing theme-import
      dynamic case
- [ ] Snapshot churn is in this commit, not smuggled into another
