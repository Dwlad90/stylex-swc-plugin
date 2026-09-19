# 11 — Bring the state crate to the coverage gate

**What to build:** The `stylex-state` crate that ticket 08 extracted measures
41.92% of regions and 40.50% of lines against its own tests. The workspace
coverage gate demands zero uncovered lines and zero uncovered regions from every
crate that has a `src/lib.rs` and tests. Write the tests that close the gap.

The shortfall is not new code going untested. It is a boundary revealing what
was already true: the state manager was covered *transitively*, by compiling
whole files through the transform, and the transform is itself exempt from the
gate. Extracting the state manager made that coverage stop counting for it.

`stylex-state` is on the coverage exemption list, so CI is green in the
meantime. The exemption is the holding position, not the answer: the code is
worth direct tests, and this ticket removes the exemption when they exist.

**Where the gap is**, from `pnpm run test:coverage` in the crate:

| File | Uncovered regions | Uncovered lines | Cover |
| --- | --- | --- | --- |
| `state_manager.rs` | 1373 | 963 | 38.92% |
| `common.rs` | 114 | 67 | 71.28% |
| `evaluate_result_value.rs` | 77 | 53 | 31.86% |
| `theme_ref.rs` | 65 | 58 | 46.28% |
| `flat_compiled_styles_value.rs` | 58 | 46 | 0.00% |
| `evaluate_result.rs` | 30 | 30 | 0.00% |
| `state.rs` | 9 | 12 | 0.00% |
| `functions.rs` | 6 | 5 | 0.00% |

The five small files are mostly constructors, accessors and the `StyleqValue`
implementation, and are reachable directly. `state_manager.rs` is the real work
and is best taken a method group at a time.

Note that the coverage tool keeps only the best-covered instantiation of a
generic, so a generic helper can read as fully covered while one instantiation
is untested.

The `stylex_state` exemption also shelters the crate's `resolution` module,
which was a crate on the gate before it was folded in and is still at 100%.
Removing the exemption must keep it there.

**Blocked by:** None — the state crate exists as of `a1baab79e`.

**Status:** done

- [x] `stylex-state` reports zero uncovered lines and zero uncovered regions.
- [x] `stylex_state` is removed from all three exemption lists:
      `package.json`, `scripts/coverage-missing.sh` and
      `scripts/packages/test/coverage.sh` -- and from a fourth the ticket does
      not name, `EXCLUDED` in `scripts/git/crate-coverage-runner.test.mjs`.
- [x] Tests cover regular and irregular inputs, and the edge cases each method
      states, rather than only the path that makes the number go up.
- [x] The full workspace suite stays green.

## Comments

**2026-09-02, ticket 25.** Re-measured on the tip of
`feat_split-transform-crate`: 43.71% of regions, 45.00% of functions and 42.38%
of lines, with 1486 unexercised regions across 5 files. The headline figures
above were taken at ticket 08 and the file table with them; both are stale by a
few points but the shape is unchanged. The exclusion stays until this ticket
lands, and `guidelines/STRUCTURE.md` now names this ticket as its remover.

**2026-09-06, closed.** 316 tests in the crate, and every file at 100% of
regions, functions and lines. `resolution` stayed at 100%, as the row required.
The exemption is off all three lists and out of `guidelines/STRUCTURE.md`.

Nine test modules were added, one per question the state answers, rather than
more cases in `state_manager_test.rs`: `import_kind`, `import_queries`,
`binding_queries`, `jsx_spread`, `call_index`, `style_injection`,
`file_and_options`, `functions` and `flat_compiled_styles_value`, with
`theme_ref_test` and `evaluate_result_value_test` extended. Every one tests
through the public surface, so the private helpers below -- `add_file_extension`,
`build_atom_inject_item`, `decl_init_hashes`, the three `add_inject_*` writers --
are reached through `register_styles`, `register_atom_styles`,
`import_path_resolver` and `flush_pending_insertions` rather than opened up.

Two fixtures were added beside the two that existed:
`fixtures/package_json_with_name/{app.js,vars.stylex.js}`, so the CommonJs
resolver has a real import to resolve.

**Seven branches in `state_manager.rs` could not be reached, and were removed
rather than excluded.** Each is provably total, and the gate is what made that
visible:

- `metadatas.is_empty()` in `register_styles` and `register_atom_styles` --
  `MetaData::convert_from_injected_styles_map` maps one metadata per rule, so it
  answers empty exactly when the style map does, which the line above already
  returned on.
- `needs_runtime_injection` in `register_styles` -- it tested for
  `InjectableStyleKind::Regular | Const`, which is the whole enum.
- The `Some(idents)` arm of `inject_import_inserted.take()` in
  `setup_injection_imports` -- the early return above answers for every state
  that holds them.
- Two `Path::to_str()` guards in `get_canonical_file_path` -- both paths are
  built from `&str`, so the relative path is text.
- The bounded `iter.next()` guard in `flush_pending_insertions` -- `import_end`
  is an index into the list being drained.
- The `?` in `specifier_at` and the re-lookup in `import_binding` -- the filter
  above `import_binding`'s `min` has already read the specifier at that pair.
- The empty-suffix arm of `matches_file_suffix` -- an empty suffix is matched by
  the `ends_with` on the line above.
- The absent-bucket and absent-entry arms of `release_member` -- the only callers
  hand back a callee taken out of the call map, and every member that map holds
  was bucketed when its call was recorded.

`set_jsx_spread_replacement` was rewritten the same way and for the same reason:
the loop it held fell through to `false` after reading a bucket that had no
matching entry, which the record cannot produce.

Two of those rewrites also do less work, counted rather than timed.
`matches_file_suffix` joined the suffix and the extension with a `format!` per
extension per ask -- up to eight heap allocations on every import a module makes;
it now strips the extension and asks the rest, and allocates none. `release_member`
made two passes over the bucket, a search and a shift; it now makes one. Neither
sits on a path the benches measure, so neither was benched: under
`guidelines/PERFORMANCE.md` only a paired release comparison decides a number,
and cross-run noise is far wider than either change.

`release_member` reads the bucket through `entry(..).or_default()` rather than
`get_mut`, which costs a second hash on the pass that empties it. That is the
price of a total function: a `get_mut` leaves a "no such bucket" arm that nothing
can reach and the gate therefore cannot pass. The bucket is always there, so the
insert `or_default` would make is never paid.

**2026-09-06, the fourth list.** The ticket names three exemption lists and there
are four: `scripts/git/crate-coverage-runner.test.mjs` holds its own copy, which
it asserts the `case` in `scripts/packages/test/coverage.sh` agrees with. Nothing
compares the four, so taking `stylex_state` off three of them left the suite
green and failed in the pre-push hook instead. The three comments that claimed
"three lists" now say four and name the fourth, and `guidelines/STRUCTURE.md`
records that nothing compares them.

Removing the row also emptied a test beside it. `a crate whose name only contains
an excluded name is measured` used `stylex-state-index` against `stylex-state`,
which proves nothing once the second is off the list; it now uses
`stylex-transform-index`.

Ticket [46](./46-compare-the-coverage-exclusion-lists.md) adds the guard that
compares the four, so the next row to move fails where it is edited.
