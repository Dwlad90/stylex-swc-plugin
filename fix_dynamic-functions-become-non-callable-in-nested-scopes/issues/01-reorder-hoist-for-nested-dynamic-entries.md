# 01 — Reorder the hoist so a nested dynamic entry stays callable

**What to build:** A dynamic `stylex.create` entry declared inside a nested
scope compiles to an arrow function, as it does at module scope. The caller
applies the dynamic rewrite to the compiled object first and hoists the result
once. The helper no longer hoists on its own and loses its program-level
parameter. The hoist of static fragments inside the helper stays.

Snapshot tests in the dynamic styles suite cover a TypeScript namespace, a
function body, an IIFE, and a block scope. Each case has a static sibling
entry. One dev variant with runtime injection proves the injection calls anchor
where the reference compiler puts them. Every snapshot is checked against the
Babel plugin from `node_modules` before it is accepted.

The harvested parity corpus is regenerated in the same commit, so the
rs-compiler pretest stays green.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] The namespace case from the issue compiles `color` to an arrow function
      inside the hoisted `_styles` declaration.
- [ ] Function, IIFE, and block scopes produce the same shape, with the static
      sibling unchanged.
- [ ] The runtime-injection dev variant matches the reference output.
- [ ] Module-scope dynamic styles are unchanged: every existing snapshot passes.
- [ ] Harvested corpus regenerated; `pnpm test:crates:workspace` passes.
- [ ] One conventional commit: `fix(transform): ...`.
