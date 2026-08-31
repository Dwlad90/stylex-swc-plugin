# 03 — Move the three exported macros to the macro crate

**What to build:** The transform crate exports three macros whose bodies name
hard-coded paths rooted at the defining crate, and those paths point at three
different future destinations: one into the diagnostics code, one into the
evaluator, and one into *both* the AST convertors (which stay) and the evaluator
(which leaves) inside a single expansion.

Because an exported macro publishes at its defining crate's root, leaving them
in place would force the evaluator crate to depend back on the transform — a
cycle Cargo rejects outright. The first breakage lands at the diagnostics
extraction, not the evaluator one, so this must be cleared before either.

Move all three to the macro crate, which the context map already describes as
the error and panic vocabulary every crate raises failures through. That crate
sits at a low layer and cannot name types in the upper layers, so each macro
takes the function it calls **as a parameter** — macros expand at the call site,
so the caller supplies the path. Same injection principle as everywhere else in
this work.

These macros are Rust-only machinery with no counterpart, so the parity
constraint permits changing their shape. This is the one ticket in the sequence
where code shape genuinely changes rather than merely moving, which is why it is
isolated: a failure here stops the work early and cheaply.

**Blocked by:** None — can start immediately.

**Status:** ready-for-human

- [x] All three macros live in the macro crate.
- [x] No macro body names a path in a layer above the macro crate.
- [x] Each call site supplies the function the macro invokes.
- [x] The macro crate gains no new dependency.
- [x] Behaviour at every expansion site is identical — the emitted code is the same.
- [x] Test files change import lines only; no assertion, input or fixture is touched.
- [x] Debug workspace build and test green.
- [x] Coverage gate still passes.

## Verification

The three macros are `crates/stylex-macros/src/evaluation_macros.rs`. Each one
now names only its own `$` parameters, and the leading arguments are the
functions it calls:

- `deopt_unsupported!(deopt, ...)` — 75 call sites.
- `expr_to_str_or_deopt!(convert_expr_to_str, deopt, ...)` — 1 call site.
- `stylex_panic_with_context!(wrap_in_paren_ref,
  build_code_frame_error_and_panic, ...)` — 2 call sites.

`stylex_panic_with_context!` takes **two** functions, not one. Its body wrapped
the expression through `stylex_ast::ast::factories::wrap_in_paren_ref` — an
absolute path into layer 5 — as well as calling the reporter, and the ticket
forbids a body naming any layer above the macro crate. Both are injected.

Every injected path resolves to what the old body hard-coded. `deopt` and
`convert_expr_to_str` are already in scope at every site: `deopt` through the
`super::super::*` glob off `evaluate/mod.rs`, `convert_expr_to_str` through
that file's own import. Only `wrap_in_paren_ref` and
`build_code_frame_error_and_panic` needed adding to `evaluate/mod.rs`.

`stylex-macros/Cargo.toml` is untouched: a macro body expands at the call site,
so the caller's crate carries the dependency.

### Results

- `cargo check`, `cargo clippy --all-targets` and `cargo test`, all
  `--workspace --all-features`: green, debug profile, 8119 passed, 0 failed.
  Seven of those are new; nothing was removed, because the deleted module held
  no tests.
- `pnpm run test:coverage:workspace`: 100% on lines, functions and regions.
- `cargo fmt --all --check` and `pnpm format:check`: clean.
- `cargo doc -p stylex_macros`: no warnings.

### Notes for later tickets

- **The macro crate is coverage-gated, the transform crate is not.** Moving a
  macro moves it from an excluded crate to an included one. The new tests
  expand each macro over stubs so the gate has something to measure; expansion
  itself emits no region in the defining crate.
- **A `macro_rules!` intra-doc link is order-dependent.** A macro referring
  forward to one defined below it in the same file does not resolve by bare
  name. Write `[`name!`](crate::name)`.
- **ADR 0002 lives in the transform crate and describes macros that no longer
  do.** It gained a paragraph on the injection and lost the claim that the
  macros sit "in the same file" as the evaluator. Ticket 08 should decide
  whether the ADR travels with the evaluator.
