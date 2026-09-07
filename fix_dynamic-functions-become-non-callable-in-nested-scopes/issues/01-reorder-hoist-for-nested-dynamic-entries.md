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

**Status:** resolved

- [x] The namespace case from the issue compiles `color` to an arrow function
      inside the hoisted `_styles` declaration.
- [x] Function, IIFE, and block scopes produce the same shape, with the static
      sibling unchanged.
- [x] The runtime-injection dev variant matches the reference output, apart from
      the `base` debug line number, which is a pre-existing lookup issue (see
      Comments).
- [x] Module-scope dynamic styles are unchanged: every existing snapshot passes.
- [x] Harvested corpus regenerated; `pnpm test:crates:workspace` passes.
- [x] One conventional commit: `fix(stylex_transform): ...` (2789cf914). The scope
      is the Cargo package name, as `guidelines/git/CONVENTIONS.md` asks.

## Comments

- 2026-09-07: Done in 2789cf914. The four production snapshots are identical to
  the reference compiler output. The dev snapshot has the same declarations in
  the same order: the `_temp` static fragment, the `_inject2` calls, then
  `_styles`, then the namespace.
- 2026-09-07: Pre-existing, out of scope: in dev mode the `$$css` debug line
  for the static sibling `base` inside a namespace reads line 7 where the
  reference reads line 5. The unfixed source gives the same number, and the
  same input at module scope gives the correct line. The nested-scope key
  lookup in the debug-data path needs its own ticket.
- 2026-09-07: Pre-existing, out of scope: `register_styles` recomputes the
  stable hash of `ast` and `fallback_ast` once per metadata inside its loop
  (`stylex-state/src/state_manager.rs`, `add_style_to_inject`). Both hashes
  are loop-invariant. Cheap follow-up, not part of this fix.
- 2026-09-07: The harvest check reports the corpus up to date: the new tests
  add no declaration the corpus did not hold, so no corpus file changed.
- 2026-09-08: Both follow-ups are done on this branch. The debug line
  number is fixed in 4bcfa0ec0 and c00bb72d1: the code frame quotes the
  authored text (disk first under `useRealFileForSource`, then the given
  text) and no longer a module printed back out; the test harness hands the
  transform its source map, and the fixture outputs were regenerated to the
  authored lines. The loop-invariant hashes are hoisted in 37dca4ce6. The
  `enableDevClassNames` default now follows `dev` as the reference does
  (197f446bd), so the dev snapshot needs no explicit flag any more.
