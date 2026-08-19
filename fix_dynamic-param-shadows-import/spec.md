# Scope-aware identifier resolution in the evaluator

Status: ready-for-agent

Tracks GitHub issue
[#1266](https://github.com/Dwlad90/stylex-swc-plugin/issues/1266). Spec only --
no ticket files. Branch: `fix_dynamic-param-shadows-import`, cut from `develop`.

Reference: `~/Projects/Facebook/stylex` @ `@stylexjs/babel-plugin` 0.19.0,
`src/utils/evaluate-path.js`.

## Context

A dynamic style's parameter that shadows an imported binding fails the build:

```js
import { zIndex } from './vars/zIndex.stylex.js';
export const styles = stylex.create({
  wrapper: { zIndex: zIndex._10 },
  zIndex: (zIndex) => ({ zIndex }),
});
```

```
error: [StyleX] Style value must evaluate to a static expression.
```

`@stylexjs/babel-plugin` 0.19.0 compiles it to three rules. Reproduced on `HEAD`
(b69f28d94), and reproduced under the parity harness's own configuration —
`haste` resolution, one source string, no auxiliary theme file.

`crates/stylex-transform/src/shared/utils/common.rs:132` — `get_import_from`
matches an `ImportSpecifier::Named` on `local.sym == ident.sym`, while its
`Default` and `Namespace` siblings both use the ctxt-aware
`local.eq_ignore_span(ident)`. The resolver runs before the transform
(`crates/stylex-rs-compiler/src/lib.rs:305`), so an arrow parameter carries a
distinct `SyntaxContext` from the module-level import binding — which is why a
parameter shadowing a module-level `const` already resolves correctly
(`get_var_decl_from` uses `eq_ignore_span`) and a named import does not.

`evaluate` (`js/evaluate/mod.rs:432`) therefore resolves the *parameter* to the
import, returns a **confident** `EvaluateResultValue::ThemeRef`, and
`evaluate_partial_object_recursively` panics at
`evaluate_stylex_create_arg.rs:430` because a `ThemeRef` answers `None` to
`as_expr()`.

Upstream never faces the choice: `evaluate-path.js:596` resolves through
`path.scope.getBinding(name)`.

Auditing that against `evaluate-path.js:595-693` surfaced two further live
divergences, one of which emits CSS the reference compiler refuses:

| input | Babel 0.19.0 | rs-compiler HEAD |
| --- | --- | --- |
| `create({a:{color:c}})` then `const c='red'` | `Referenced value is used before declaration.` | **silently emits `.x…{color:red}`** |
| `create({a: (NaN) => ({width: NaN})})` | dynamic param → `var(--x-width)` | `Only static values are allowed inside of create() call.` |
| `create({dyn: (stylex) => ({height: stylex})})` | `Invalid pseudo or at-rule.` | `Style value must evaluate to a static expression.` |

Both missing target texts are already written and commented out at
`crates/stylex-constants/src/constants/evaluation_errors.rs:32,35`,
byte-identical to Babel's output.

## Decisions

- **`functions.identifiers` stays `Atom`-keyed and stays first.** Upstream is
  name-keyed too and consults it *before* scope (`evaluate-path.js:404-409`),
  injecting arrow parameters into that same map (`:391`) exactly as
  `nodes/arrow_function.rs:45` does. Making it `Id`-keyed would be a divergence.
- **`SyntaxContext`, not a scope tree.** The resolver already makes ctxt
  authoritative for shadowing; a hand-rolled scope tree would be a second source
  of truth for one question.
- **The step reorder is inert.** Once resolution is ctxt-aware, at most one of
  {import specifier, var declarator} can match a reference, so moving the
  initializer read after the import check changes no outcome. Risk sits entirely
  in the two revived steps.
- **`Map` materialization happens at the consumer.** `identifier.rs` must keep
  answering `FunctionConfigMap` for `stylex.when` as a callee.

## Work

Six commits, each green on its own.

### 1. `fix(stylex-transform)` — ctxt-aware named-import resolution

`shared/utils/common.rs:138` — the `Named` arm matches
`named_import.local.eq_ignore_span(ident)`, as its two siblings do. Fixes #1266
and case I (a parameter shadowing an *aliased* import, `{zIndex as zi}` + param
`zi`, which fails identically today).

### 2. `fix(stylex-transform)` — refuse a reference used before its declaration

Mirrors `evaluate-path.js:664-666` (`path.node.start < binding.path.node.end`)
in `js/evaluate/mod.rs`, ahead of the initializer read: compare `BytePos`,
revive `USED_BEFORE_DECLARATION`. **Skip the check when either span is
`DUMMY_SP`** — a synthesized node (e.g. from `expand_shorthand_prop`) has no
authored position, and skipping degrades to today's behaviour rather than
inventing a refusal.

Its own commit because it is a wrong-output bug independent of #1266, and a
bisect deserves to land on it.

### 3. `refactor(stylex-transform)` — one ordered chain in a new module

New `shared/utils/js/evaluate/binding.rs` owning "resolve this reference to its
binding", replacing the ad-hoc ordering in `js/evaluate/mod.rs:405-517`. One
function, steps in upstream's order, each carrying the `evaluate-path.js` line
range it mirrors:

1. import specifier → `ThemeRef` (`:598-649`)
2. default-import specifier → `IMPORT_FILE_EVAL_ERROR` (`:651-653`) — **measure
   first.** Its constant is also commented out (`evaluation_errors.rs:5`); we
   currently treat a default theme import as a theme ref. Mirror only if a
   measured divergence exists; otherwise leave a comment saying so.
3. `constantViolations` → `NON_CONSTANT` (`:655-657`)
4. `isMutated` → `NON_CONSTANT` (`:659-661`)
5. used-before-declaration → `USED_BEFORE_DECLARATION` (`:663-665`), from
   commit 2
6. `binding.hasValue` (`:667-669`) — deliberately absent; Babel sets `hasValue`
   only via `setValue`/`clearValue`, which this plugin never calls. Comment, no
   code.
7. `undefined` / `Infinity` / `NaN`: **deopt `UNINITIALIZED_CONST` when a
   binding exists**, else return the global (`:670-685`). This is what makes the
   `NaN`-parameter case match Babel: the value deopts and becomes an inline
   style instead of being emitted as a static `NaN`.
8. `path.resolve()` → the var-declarator initializer read, else
   `check_ident_declaration` (`:687-692`)

Steps 3 and 4 are spelled as two sequential probes even though
`StateManager::has_binding_write` answers both today
(`visit_mut_module.rs:140-200` records rebinding *and* member mutation, mutating
method receivers and `Object.assign` targets, `Id`-keyed, in one pass). Split
the collector's `binding_writes` into `binding_reassignments` and
`binding_mutations`, both filled by the same walk, so each chain step probes the
set upstream probes. Both deopt with `NON_CONSTANT`, so the split changes no
outcome — it costs one extra `FxHashSet` and buys a line-for-line mapping to
`evaluate-path.js`. The documented escape-into-a-call unsoundness carries over
verbatim.

`functions.identifiers` is consulted in `nodes/identifier.rs` *before* this
chain, untouched.

`check_ident_declaration` (`shared/utils/js/check_declaration.rs`) stays as-is —
its `Unsupported expression: FunctionDeclaration` / `ClassDeclaration` texts are
byte-identical to Babel's.

Carries the snapshot churn from step 7. Also lands:

- `crates/stylex-transform/docs/adr/0003-…` — the chain's order and why, beside
  0001/0002, which are decisions about this same evaluator
- the seam's name in `crates/stylex-transform/CONTEXT.md`

### 4. `fix(stylex-transform)` — delete the dead imported-name branch

`common.rs:139-147` matches an ident against an import's *imported* name, which
has no upstream counterpart. Its `ModuleExportName::Ident` arm is unreachable:
that ident carries the unresolved ctxt and never `eq_ignore_span`-matches a
reference. Had it ever matched, `js/evaluate/mod.rs:457` re-searches by *local*
name and `stylex_panic!("Could not resolve the import specifier")` — a panic
reachable only through this branch. Delete both arms; the suite is the check.

### 5. `fix(stylex-transform)` — a folded map rejected as a namespace

At the consumer — `evaluate_stylex_create_arg.rs:327` and `:430`, where
`as_expr()` answers `None` and we panic — materialize a
`FunctionConfigType::Map` / `EvaluateResultValue::FunctionConfigMap` as an
object literal carrying the map's keys, so it reaches `validate_namespace`
(`shared/utils/validators.rs:689`) and is refused by
`validate_conditional_styles` with `INVALID_PSEUDO_OR_AT_RULE` — Babel's text
for the same input.

This mirrors upstream exactly: `visitors/stylex-create.js:206` sets
`identifiers[stylex] = { …, when: stylexWhen }`, name-keyed and checked first,
so Babel folds the shadowing parameter to `{ when: … }` and
`validateConditionalStyles ← validateNamespace ← styleXCreateSet` rejects the
key `when`. The placeholder values are never emitted — validation panics on the
key first.

### 6. `test(rs-compiler)` — corpus, fixtures, snapshots, units

- **Corpus** (`crates/stylex-rs-compiler/parity/corpus/modules.json`, `expected`
  verdict recorded on each so a regression reports as a *changed verdict*):
  every case that diverges today — A (reported), I (aliased), J (prop order),
  B3–B6 (the "any other reference to the import, anywhere in the module"
  isolation), F (`stylex` parameter), used-before-declaration, `NaN` parameter —
  plus the agreeing cases as guards the reorder must not break: B, B2, a
  parameter shadowing a module-level `const`, a parameter named
  `firstThatWorks`, function/class declaration references, member mutation of a
  `const`.
- **Fixtures**: a `dynamic-param-shadows-import/` case under
  `crates/stylex-transform/tests/fixture/` — `input.stylex.js` + `output.js` +
  `output_prod.js`, per `tests/fixtures.rs`.
- **Snapshots**: `stylex_test!` for A and the `NaN` parameter in
  `tests/transform_stylex_create_test/dynamic_styles.rs`, alongside the existing
  theme-import dynamic test at `:78`.
- **Units**: the chain's resolution order, one test per step, in a `tests/`
  sibling of `binding.rs` per crate convention.

## Open item — closed by ticket 01

**Closed: a shadow of the scope-blind match, no second defect.** `typescript_strip`
(`stylex-rs-compiler/src/lib.rs:307`) elides an import whose binding has no value
reference, so the declaration never reaches `top_imports` and the name match has
nothing to match. Any other reference keeps it alive and the match fires. Under
`cargo test`, which runs the resolver but not `typescript_strip`, even the
dynamic-style-alone shape aborts — so the Rust suite reproduces #1266 directly.
Full evidence, including the two theories ruled out with measurements, in
`issues/01-pin-why-an-unused-import-spares-the-shadowed-parameter.md`.

The original statement of the open item, for context:

Case **B** — the dynamic style alone compiles correctly, and *any* other
reference to the imported binding anywhere in the module (not just in the same
`create()` call, and independent of prop order) turns it into the failure. I
ruled out the `evaluate_cached` `seen`-cache (prop order is irrelevant, so no
priming order explains it) and `disable_imports`
(`transform_stylex_create_call/runtime_function_map.rs:146` passes `false`).
Mechanism unpinned; one instrumented build settles it. Report the mechanism
before commit 1. If the seam fixes it as a side effect it needs no commit of its
own; if it turns out to be an independent defect it gets one.

## Verification

```sh
cargo test -p stylex_transform                 # units, snapshots, fixtures
pnpm run --filter=@stylexswc/rs-compiler build  # JS suites read dist/*.node
pnpm run --filter=@stylexswc/rs-compiler parity -- --set modules
pnpm typecheck && pnpm format:check && pnpm lint:check && pnpm test
```

End-to-end, both compilers on the reported input, comparing `metadata.stylex`:
the harness under
`/private/tmp/claude-503/…/scratchpad/repro1266/` (`run2.mjs`, `run8.mjs`,
`run9.mjs`) already runs all 14 cases side by side against
`@stylexjs/babel-plugin` 0.19.0 from `node_modules`; every row must read
`babel == swc`.
