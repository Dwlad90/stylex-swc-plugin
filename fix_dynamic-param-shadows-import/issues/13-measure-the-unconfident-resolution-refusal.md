# 13 — Measure the refusal at the tail of the import step

Status: `resolved`
Blocked by: 06 — the constant it would use is revived there, and the gap this
ticket owns is the comment 06 left at the site.

**What to build:** A verdict on the *second* place the reference implementation
gives `IMPORT_FILE_EVAL_ERROR`, and then either the refusal or a recorded reason
not to give it.

Step 2 of the chain mirrors upstream's default-specifier refusal
(`evaluate-path.js` 0.19.0 line 652-654). Upstream reaches the same constant a
second time, at the *tail* of step 1 (line 6360 of the bundled `lib/index.js`):

```js
if (state.confident) {
  ...
  return returnValue;
} else {
  deopt(binding.path, state, IMPORT_FILE_EVAL_ERROR);
}
```

A resolution that came back unconfident is, to upstream, an imported file it
could not fold. This compiler falls out of the `if state.confident` block
silently and keeps whichever refusal the resolution already recorded — so the
reference either reports a different message or, if nothing was recorded,
continues down the chain to a step upstream never reaches.

Which of those two it is has not been measured, and that is the whole ticket.
Reaching the branch at all needs an import whose path *resolves* and whose
`evaluate_theme_ref` then leaves the evaluation unconfident, so the first job is
finding an input that gets there — if none exists, the absence is the verdict and
gets recorded at the site as one.

- [x] An input that reaches the unconfident branch, or a recorded finding that
      none does — none does, and the reason is structural
- [x] ~~Both compilers measured on it~~ — no input to measure
- [x] ~~If they diverge~~ — unreachable, so it cannot
- [x] If they agree: the comment at the site says what was measured, replacing
      the one that says the measurement is open

## Comments

**Filed while landing 06.** Found in code review of the default-import step: the
constant it revives has two upstream call sites and 06 mirrors one. Noted at the
site rather than implemented, because the chain's rule is that an absent step is
measured before it is ruled out — see
`crates/stylex-transform/docs/adr/0003-one-ordered-chain-resolves-a-reference.md`.

## Answer

**Unreachable, structurally — no refusal to add.** Answered by reading both sides
rather than by running an input, because the finding is that no input exists.

Upstream reaches its second `IMPORT_FILE_EVAL_ERROR` only out of
`evaluateImportedFile` (`lib/index.js` 0.19.0 lines 6120-6148), which parses the
imported module, folds the named export out of it with `evaluateCached`, and
refuses when that fold comes back unconfident. It is selected by the *other* arm
of upstream's path resolver:

```js
const returnValue = type === 'themeNameRef'
  ? evaluateThemeRef(value, importedName, state)
  : evaluateImportedFile(value, importedName, state, bindingPath);
```

This compiler has only the first arm. `ImportPathResolution` is
`Resolved { path } | Unresolved` — there is no third state that means "resolved,
now go and evaluate it" — and `evaluate_theme_ref` takes `&StateManager` and
returns a `ThemeRef`, so it cannot clear `state.confident` on the way past. No
resolution reached from step 1 can leave the evaluation unconfident, so the
branch has nothing to answer.

Recorded at the site in `binding.rs` as an absent step with its reason, which is
what the chain's ADR asks of one. It becomes reachable the day this compiler
evaluates an imported file in its own right — the same missing capability the
globals step's cross-file gap waits on, so whoever adds it inherits both.
