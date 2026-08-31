# 06 — Rename the nested-config crate

**What to build:** A crate currently holds the name `stylex-evaluator` while
containing no evaluation at all — its entire surface is flattening the nested
config objects that `defineVars` and its siblings accept. It was created whole
for the nested-API work; nothing was ever moved into it. The mismatch is severe
enough that the context map carries an explicit note steering readers away from
it and back to the transform for the general evaluator.

Rename it to describe what it does, freeing the name for the code that earns it
in the following tickets, and delete the note the mismatch made necessary.

This is a pure rename: no logic changes, no files move between crates.

**Blocked by:** None — can start immediately.

**Status:** ready-for-human

- [x] The crate is renamed in its manifest and on disk.
- [x] Every dependent manifest points at the new location.
- [x] The context-map row is renamed and the redirect note is deleted.
- [x] The crate's own `CONTEXT.md` heading matches the new name.
- [x] Its position in the documented layer list is updated.
- [x] No behaviour changes; the suite is green with no test edits beyond import lines.
- [x] The old name is free for use.
- [x] Lockfile regenerated and committed.

---

## Outcome

The rename landed as specified. `crates/stylex-evaluator` is now
`crates/stylex-nested-config`, the Cargo package is `stylex_nested_config` and
the npm package is `@stylexswc/nested-config`. Every `.rs` hunk is an import
path, an identifier or a string literal; all four sources are byte-identical at
100% similarity, and no file crossed a crate boundary.

Layer 8 keeps its number and is now named "Nested config and inline syntax" in
`guidelines/STRUCTURE.md`; the 19 crate READMEs that draw the DAG name its
subgraph "Nested Config". The redirect note the wrong name made necessary is
gone from `CONTEXT-MAP.md`, and the transform row there already says where the
general JS evaluator lives.

Green on `cargo check`, `cargo clippy --all-targets`, `cargo test` (29 test
binaries) and `cargo fmt --check`, each with `--workspace --all-features`, and
on `pnpm format:check`, `pnpm lint:check`, `pnpm typecheck` and `pnpm test`
(91 tasks). The harvested parity corpus and the value-parser fixture did not
drift, so ticket 10 has nothing to regenerate on account of this move.

### The crate is still mixed-domain

`src/common.rs` holds `evaluate_bin_expr` and `resolve_node_package_path`. The
flatteners in `src/nested.rs` use neither -- both callers are in
`stylex-transform`. The old name covered both concerns loosely; the new one
covers only the flattening, so the mismatch `guidelines/STRUCTURE.md` forbids
("each crate owns exactly one concern") is now visible rather than hidden.

Moving them is out of scope here -- this ticket forbids files crossing crate
boundaries -- so the README and `CONTEXT.md` say plainly that the two helpers
sit here and await a home. `evaluate_bin_expr` belongs with the evaluator
ticket 08 creates; `resolve_node_package_path` belongs with
`stylex-path-resolver`. Decide both in ticket 10.

### The documented dependency edges are wrong, and were wrong before

Every crate README draws `stylex_nested_config` depending on `constants`, `js`,
`path-resolver` and `types`. `Cargo.toml` depends on `stylex_ast`,
`stylex_enums`, `stylex_macros` and `stylex_structures`. Separately,
`crates/stylex-css/package.json` declares the crate while
`crates/stylex-css/Cargo.toml` does not depend on it at all, so the
`stylex_css --> stylex_nested_config` edge is fiction.

The rename propagated these faithfully rather than correcting them, because 19
hand-maintained copies of one graph is a defect of its own and fixing it is not
a rename. Ticket 10 renumbers the same layer list -- correct the edges there,
and consider generating the graph from the manifests instead.
