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

**Status:** ready-for-agent

- [ ] The spec fails on the binding built from `develop` and passes on the
      binding built from this branch.
- [ ] Namespace and IIFE cases are executed, not only compiled.
- [ ] `pnpm typecheck`, `pnpm lint:all`, `pnpm lint:type-aware`,
      `pnpm format:check`, `pnpm test` and `pnpm test:crates:workspace` pass.
- [ ] `/code-review` run over the branch; findings addressed.
- [ ] Spec acceptance criteria in `spec.md` ticked.
- [ ] One conventional commit: `test(rs-compiler): ...`.
