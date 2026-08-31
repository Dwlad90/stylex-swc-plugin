# 12 — Extract the declarations crate

**What to build:** The crate that [ticket 08](./08-move-evaluator-core.md)'s
research turned out to need before the evaluator can move. Ticket 08 landed
`stylex-state`; this lands the layer above it, and
[ticket 13](./13-move-evaluator-core.md) then moves the evaluator onto both.

`utils/ast/convertors.rs` straddles the crate boundary, but only half of it is
in the cycle. `expr_to_num` calls `binary_expr_to_num_or_str`, which calls
`expr_to_num` back, and `transform_bin_expr_to_number` calls `evaluate_cached`:

| In the cycle — stays for ticket 13 | Pure — goes to the new crate |
| ---------------------------------- | ---------------------------- |
| `expr_to_num`                      | `convert_expr_to_str`        |
| `ident_to_number`                  | `convert_ident_to_expr`      |
| `convert_unary_to_num`             | `handle_tpl_to_expression`   |
| `transform_bin_expr_to_number`     | `convert_lit_to_raw_value`   |
| `expr_tpl_to_string`               | `ident_to_string`            |

Cut the module along that line. The pure five call nothing but each other and
four _declaration lookup_ helpers in `stylex_state::common`
(`get_var_decl_by_ident`, `get_import_by_ident`, `get_var_decl_from`,
`get_var_decl_parts_by_ident`). Those nine functions are one idea -- resolve a
name or expression against the declarations the state recorded -- and both the
transform and the evaluator consume it.

Create `stylex-declarations` for the nine, directly above `stylex-state`.
Roughly 300 source lines. `stylex-state` sheds its readers and keeps the state,
its value types and its writers.

`convertors_tests.rs` (1,823 lines, 67 tests) splits the same way: the pure
half's tests come here, the cycle half's stay for ticket 13. Around 27 of the
111 tests in `common_tests.rs` come with the lookups.

- [ ] The nine functions live in `stylex-declarations`; nothing else does.
- [ ] `stylex-state` no longer holds a declaration lookup.
- [ ] The crate has `Cargo.toml`, `package.json` with the Cargo deps mirrored, a
      catalog entry, `turbo.json`, `taplo.toml`, `README.md`, `CONTEXT.md`, a
      context-map row and a layer-list position.
- [ ] **It reaches the coverage gate with no exemption.** Unlike `stylex-state`,
      its tests travel with it. Expect a small gap:
      `convert_lit_to_raw_value` has no direct test today.
- [ ] No re-export facade in the transform or in `stylex-state`.
- [ ] Benches A/B'd against `develop` on one machine in one session; no
      regression outside the layout floor. A new crate shifts function placement
      under `-C lto -C codegen-units=1` even where no executable line changed.
- [ ] `CONTEXT.md` defines the vocabulary that arrived, and no term is left
      defined in two crates.
- [ ] The full workspace suite is green, with `pnpm format:check`, `lint:check`,
      `lint:shell`, `typecheck` and `test`.

## Decisions already taken

Do not re-litigate these; they were settled before this ticket was written.

1. **Cut the convertors cycle, do not carry it.** Moving the module whole would
   put five state-only conversion functions in a crate named for evaluation, to
   satisfy a cycle only the other five are in.
2. **The pure half gets its own crate, not a corner of `stylex-state`.** The
   seam between "what the state records" and "what a name resolves to" already
   exists; `common.rs` was hiding it.
3. **The crate is `stylex-declarations`.** `stylex-bindings` was rejected
   because _binding write_ stays on the state manager, and a glossary term whose
   definition spans two crates is what the context map exists to prevent.
   `stylex-resolve` was rejected for colliding with `stylex-path-resolver`.
4. **No coverage exemption for this crate.** Its tests travel with it, so a
   shortfall is a signal worth seeing rather than pre-empting.
5. **This is its own ticket, not a commit inside the evaluator move.** A new
   crate boundary and a 10.5k-line relocation in one reviewable unit cannot be
   reviewed, and bench movement could not be attributed between them.
6. **Benches: A/B against `develop`, the base branch.** The `pre-split`
   criterion baseline was destroyed outside this work and is four commits stale;
   ticket 07 established that an A/B on one machine in one session is the
   stricter test. Expect an LTO-layout floor around +4% -- ticket 07 measured
   +3.65% on a bench whose crate was byte-identical between legs.

**Tooling.** Ticket 08 was executed mechanically with three scripts kept at
[](../tools/README.md): a use-tree expander (a literal path rewrite
misses nested brace groups and reports a false clean), a compiler-driven
visibility narrower, and an import re-nester that corrupts comments and globs --
read its caveats before running it.

**Blocked by:** None — `stylex-state` exists as of `a1baab79e`.

**Status:** ready-for-agent
