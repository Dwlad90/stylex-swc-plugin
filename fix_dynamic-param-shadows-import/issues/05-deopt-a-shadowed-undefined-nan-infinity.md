# 05 — Deopt a shadowed `undefined` / `NaN` / `Infinity`

Status: `resolved`
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

- [x] The example compiles to the reference implementation's rules
- [x] An *unshadowed* `NaN` / `Infinity` / `undefined` in a style value still
      answers the global
- [x] Corpus entry with the verdict it is known to read
- [x] Snapshot in the dynamic-styles tests, beside the existing theme-import
      dynamic case
- [x] ~~Snapshot churn is in this commit~~ — there is none; see below

## Comments

Landed as one commit. `resolve_reference`'s step 7 asks
`StateManager::declares_binding` before answering, so a binding refuses with the
revived `UNINITIALIZED_CONST` and only an unbound name answers the global.

Measured against `@stylexjs/babel-plugin` 0.19.0 — every shape the ticket is
about now agrees:

| input | before | after |
| --- | --- | --- |
| `(NaN) => ({ width: NaN })` | refused `Only static values…` | `width:var(--x-width)` + `@property`, as upstream |
| the same with `Infinity` / `undefined` | refused | as upstream |
| `(NaN) => ({ width: NaN + 1 })` | folded to `width:NaNpx` | dynamic var, as upstream |
| `const NaN = '5px'` then read | emitted `width:5px` | both refuse, same text |

Verification: `cargo test --workspace --all-features` 0 failed over 27 binaries,
`cargo clippy --workspace --all-features --all-targets` clean, `cargo fmt`
clean, `pnpm typecheck && pnpm format:check && pnpm lint:check && pnpm
lint:shell && pnpm lint:type-aware && pnpm test` green, `parity` 0 changed
verdicts over 865 subjects (modules set 56 → 62).

### The predicted snapshot churn does not exist

The ticket expected "a refusal that used to be an answer changes what a few
existing snapshots record". Nothing changed: every existing `NaN` / `Infinity`
in the suite (`transform_stylex_create_test/global_builtin_calls.rs:47,354`) is
*unbound*, so it still folds to the global. The step only moved inputs where
something in scope had taken the name over, and the suite had none. Recorded
because the absence is evidence the step is as narrow as it claims, not a step
that was skipped.

### The step needed a question the chain could not ask

Steps 1, 3, 4 and 8 all resolve through the declaration list or the import
table. The binding this step is about is a dynamic style's *parameter*, which
appears in neither — so the pre-scan now records every binding the module
declares as `StateManager::declared_bindings`, keyed by full SWC `Id`.

`Id`-keying is what makes it scope-aware without the scope tree the spec ruled
out: the resolver runs ahead of this pass in both entry points (`lib.rs:305` and
`transform::mod`'s `resolve_factory`), so a parameter carries its own context
and the global `NaN` beside a `function f(NaN)` matches nothing. Pinned by
`a_global_beside_an_unrelated_binding_of_its_name_still_folds_to_itself` and by
corpus `modules-1266-a-global-beside-an-unrelated-binding-of-its-name`.

### Two narrownesses, both written down at the step

- **TypeScript's binding forms are not collected.** `enum`, `namespace` and
  `import x = require()` have no visitor. Harmless in production —
  `typescript_strip` runs before this pass and lowers all three to `var` /
  `const` — but this crate's own test transform runs the resolver without the
  strip, so the gap is real there. Pinned as absent by
  `records_nothing_for_typescript_only_binding_forms`, so a change to the strip's
  position reads as a failing test.
- **The set is the host module's.** A name bound inside an *imported* file
  carries a context this pre-scan never saw, so it misses and the global stands
  where upstream would evaluate in that file's own scope. Fails safe, and
  closing it means evaluating imported files in their own right, which this
  compiler does not do at all yet — the same gap ticket 06 is about.

### Left for their own tickets

- `width: NaN` with nothing bound still diverges: upstream emits `width:NaNpx`,
  this compiler refuses the non-numeric identifier as a style value. It is a
  CSS-value question, not a resolution one, and the globals step now answers the
  global exactly so that it stays visible. Recorded as
  `modules-1266-the-unshadowed-globals-in-a-style-value`, `acceptance
  divergent`.
- A computed key that refuses now reports this step's reason where upstream
  reports `Only static values are allowed inside of a create() call.` Both
  refuse; upstream swallows an obj-key reason and this compiler propagates it,
  which is a message-propagation difference older than this step.

### Review

Both axes of `/code-review` ran against the commit. Standards found the
`CONTEXT.md` chain entry claiming one absent step where two are absent (steps 2
and 6), an unanchored ADR glossary link, a third spelling of the three-name test
— now one exported predicate, `is_global_spelled_as_an_identifier`, in
`stylex-js::coercions`, which already owned the set privately — `bindings` /
`binds` not matching the glossary term, and one paragraph copied four times. All
fixed. Spec found no scope creep, confirmed the step matches
`evaluate-path.js:670-683` including the refuse-before-reading-the-initializer
part, and raised the two narrownesses above.

Two of its findings were re-examined and answered differently than first
applied, both because the evidence beat the reasoning:

- **The ADR's glossary link is unanchored.** It was anchored on review, on the
  grounds that an anchor is more useful. It is not: the glossary's terms are
  bold paragraphs rather than headings, so the fragment resolves nowhere, and
  the sibling `adr/0001` already links `../../CONTEXT.md` plain. Anchored links
  are right *inside* `CONTEXT.md`, where four pre-date this work, and wrong
  across files.
- **Step 7 deopts on the reference, not on the binding's declaration**, where
  upstream passes `binding.path`. Recorded as required rather than as merely
  consistent with the chain: `deopt`'s path becomes `state.deopt_path`, which is
  the expression emitted *for the runtime*, and for a dynamic style that is the
  inline style's value — `(NaN)` in the snapshot. The declaration there would
  emit the wrong expression. Upstream's `binding.path` is a diagnostic location,
  which is a different job.

A DRY pass over the tests followed, since the review's own Repeated Switches
finding applied to them too: `FOLDED_GLOBALS` replaces five literal spellings of
the three names, `assert_folded_to_the_global` replaces two copies of the
same assertion, one `ModuleState::bound_in_an_unrelated_scope` replaces a test
that hand-rolled the state the builder exists to assemble, and `collect_ts` /
`assert_ts_binds` reuse the collector-test helpers instead of a second walk.
Asserting exact sets rather than "the name is absent" turned up one fact worth
having: `namespace NaN { … }` records nothing for the namespace and `a` for the
`const` inside it.
