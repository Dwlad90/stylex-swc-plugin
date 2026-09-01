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

- [x] The nine functions live in `stylex-declarations`; nothing else does.
- [x] `stylex-state` no longer holds a declaration lookup.
- [x] The crate has `Cargo.toml`, `package.json` with the Cargo deps mirrored, a
      catalog entry, `turbo.json`, `taplo.toml`, `README.md`, `CONTEXT.md`, a
      context-map row and a layer-list position.
- [x] **It reaches the coverage gate with no exemption.** Unlike `stylex-state`,
      its tests travel with it. Expect a small gap:
      `convert_lit_to_raw_value` has no direct test today.
- [x] No re-export facade in the transform or in `stylex-state`.
- [x] Benches A/B'd against `develop` on one machine in one session; no
      regression outside the layout floor. A new crate shifts function placement
      under `-C lto -C codegen-units=1` even where no executable line changed.
      Also A/B'd against the parent commit, which is what actually attributes
      this ticket -- see the bench record for why, and for the target-directory
      trap that invalidated a first attempt.
- [x] `CONTEXT.md` defines the vocabulary that arrived, and no term is left
      defined in two crates.
- [x] The full workspace suite is green, with `pnpm format:check`, `lint:check`,
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

**Status:** resolved

## What landed

`stylex-declarations` at layer 10, 229 source lines. The four declaration
lookups left `stylex-state/src/common.rs`; the five pure convertors left
`crates/stylex-transform/src/shared/utils/ast/convertors.rs`. The cycle half of
that module stayed for ticket 13, cut on the line the table above draws.

Two `StateManager` methods the lookups read -- `import_binding` and
`declaration_of` -- widened from `pub(crate)` to `pub`. That is the narrowest
visibility a cross-crate call has.

**Tests.** 27 of the 111 in `common_tests.rs` moved, and 17 of the 67 in the
transform's `convertors_tests.rs`, both verbatim. 16 more were written to close
the gap the ticket predicted: `convert_lit_to_raw_value` and
`get_var_decl_parts_by_ident` had no direct test, and `handle_tpl_to_expression`
had no case for a declaration without an initializer. The crate reports **100%**
lines, regions and functions, with no exemption in `coverage.sh` or in the root
`test:coverage:workspace` list.

**Benches.** Recorded in [`../bench/ticket-12.md`](../bench/ticket-12.md).

## Left for later, deliberately

- `convert_ident_to_expr` has no caller anywhere outside its own tests, and had
  none before this move either. It is pre-existing dead code rather than
  something this ticket created, so it is left alone. Ticket 14 already owns the
  question of what to do with an uncalled `pub` function.
- `convertors_tests.rs` carries six copies of the same `make_var_declarator`
  helper. They arrived that way and moved verbatim. Hoisting one copy means
  editing six moved test modules, which would cost the property that makes this
  diff cheap to review: only import lines changed.
