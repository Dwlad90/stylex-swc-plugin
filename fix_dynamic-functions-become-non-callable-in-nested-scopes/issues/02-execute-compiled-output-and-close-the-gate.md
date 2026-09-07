# 02 — Execute the compiled output in the shipped binding and close the gate

**What to build:** A vitest spec in the rs-compiler package compiles the
namespace and IIFE cases with the shipped native binding, runs the output in a
child Node process, and asserts on what the issue reported: the dynamic entry
is callable, two different arguments give two different results, and the
static sibling is still a plain object. The binding is rebuilt before the spec
runs, because the spec exercises `dist/*.node` and not the Rust sources.

With both commits in place, the whole gate runs and a code review of the branch
against the spec is completed, with findings fixed.

**Blocked by:** 01 — Reorder the hoist so a nested dynamic entry stays callable.

**Status:** resolved

- [x] The spec fails on the binding built from `develop` and passes on the
      binding built from this branch.
- [x] Namespace and IIFE cases are executed, not only compiled.
- [x] `pnpm typecheck`, `pnpm lint:all`, `pnpm lint:type-aware`,
      `pnpm format:check`, `pnpm test` and `pnpm test:crates:workspace` pass.
- [x] `/code-review` run over the branch; findings addressed.
- [x] Spec acceptance criteria in `spec.md` ticked.
- [x] One conventional commit: `test(rs-compiler): ...` (dc229267c).

## Comments

- 2026-09-07: Done in dc229267c. The suite failed on the binding built before
  2789cf914 with `TypeError: styles.color is not a function` in all three
  tests, and passes on the binding rebuilt from this branch.
- 2026-09-07: `@stylexjs/stylex` is not resolvable from the rs-compiler
  package, so the child runs the compiled module with a `stylex.props` stub
  that returns its arguments. The dynamic call is an argument expression, so
  the stub cannot hide the reported failure. SWC lowers the compiled
  TypeScript to CommonJS first, because a namespace is not erasable syntax.
- 2026-09-07: Review notes not acted on: the outcome check after
  `runNodeScript` now exists in three specs with different wording, and a
  shared `runNodeScriptOrThrow` in `nodeScript.ts` would end the drift. The
  reported failure is a catchable `TypeError`, so an in-process run would
  also do; the child process stays because this ticket asks for it.
- 2026-09-07: A third case with many dynamic entries beside static siblings
  in one function body guards the per-entry rewrite. The count is only
  "many"; no threshold is known.
- 2026-09-08: The outcome check is shared now: `runNodeScriptOrThrow` in
  `nodeScript.ts` (ca89d1313). The env value suite keeps its own assertion
  on purpose.
