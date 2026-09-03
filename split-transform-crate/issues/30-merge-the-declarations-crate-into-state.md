# 30 — Merge the declarations crate back into the state crate

**What to build:** The declarations crate holds 211 lines of source against
1,399 lines of tests, and two of its lookups are one-line pass-throughs to
state-manager methods. Its own documentation argues it belongs to neither of
its two consumers, but never considers the third option: a module inside the
crate both consumers already depend on. Nothing in the state crate depends on
it, so no cycle forces the boundary — it is a layer that costs a crate and
buys nothing.

Fold it back in as a resolution module of the state crate, and remove the
scaffolding, the context-map row and the layer entry that a crate of its own
required.

**Blocked by:** 29 — the downcast question decides what the resolution
module's signatures look like, so settling it first avoids doing this twice.

**Status:** resolved

- [x] The declarations crate's contents live as a module of the state crate
- [x] Its dependents import from the new path, and no facade is left behind
- [x] The crate's scaffolding, context-map row and layer entry are removed
- [x] Coverage does not regress in the receiving crate
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

## Comments

The merge landed as described. `resolution/lookup.rs` and
`resolution/convertors.rs` are a module of the state crate, their 60 tests run
in the state crate's own binary, and 16 `use` statements across 14 files in the
evaluator and the transform read the new path. The crate directory and its six
non-source files -- `Cargo.toml`, `package.json`, `taplo.toml`, `turbo.json`,
`README.md`, `CONTEXT.md` -- are gone, with the `pnpm` catalog entry, the two
`package.json` rows, the context-map row and layer 6; layers 7 to 9 moved down
one, which `the_documented_ladder_matches_the_manifests` confirms against the
manifests.

The merge added no dependency to the receiving crate: every crate the module
imports was already a state dependency, which is the other half of why no cycle
forced the boundary.

Coverage did not regress, but the *gate* changed and that is worth saying
plainly. The module is still at 100% -- 128 regions, 0 missed, measured with
`llvm-cov` on the state crate -- yet the declarations crate was **on** the
coverage gate and `stylex_state` carries a temporary exclusion. So the code's
coverage travelled and its gate did not. The exclusion row in
`guidelines/STRUCTURE.md` now says so, and names keeping the module at 100% as
work for `11-cover-the-state-crate`.

Two pass-throughs were deliberately left in place: `get_import_by_ident` and
`get_var_decl_from` still forward to `import_binding` and `declaration_of`.
Removing them is right -- they are now forwards to methods in the same crate --
but it means re-homing 370 lines of tests that overlap the state manager's own,
which would have buried the merge in an unrelated diff. Filed as
`36-drop-the-two-pass-through-lookups`.

Two corrections to this ticket's own framing, for whoever reads it next.

The **blocker on 29 did not hold**. The stated reason was that "the downcast
question decides what the resolution module's signatures look like", but 29
touched this code in exactly one line, and that line is a test helper:
`dummy_fn` took `&mut dyn StyleOptions` and now takes `&mut StateManager`. No
production signature in `lookup.rs` or `convertors.rs` ever named
`StyleOptions`. The merge would have been safe before 29.

One **`spec.md` figure moved**, and one that looks like it should have did not.
User story 5 counts the excluded surface as "31,744 across six" crates. The
crate count is still six: the declarations crate was never on the exclusion
list, so folding it in removed nothing from that list. The line total did move,
because the module now sits inside a crate that is excluded -- measured at
31,928 by the story's own rule (every `src/**/*.rs` less `tests?|benches?|
examples`), so +184 against the stated baseline. The story's prediction of
"15,835 lines across four crates" once tickets 11 and 15 land is unaffected:
the module lands inside `stylex_state`, which ticket 11 takes off the list
anyway. Flagged rather than edited, because one number in a measured set should
be re-measured with the rest.
