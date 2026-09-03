# 36 — Drop the two pass-through lookups the merge co-located

**What to build:** `resolution::lookup::get_import_by_ident` and
`resolution::lookup::get_var_decl_from` are one-line forwards to
`StateManager::import_binding` and `StateManager::declaration_of`. Issue 30
cited them as the evidence that the declarations crate bought nothing, and the
merge has now put each forward in the same crate as the method it forwards to.
A module that re-exports its own crate's methods under a second name is the
`pub use` facade `guidelines/STRUCTURE.md` refuses, spelled as a function. The
two also disagree on argument order -- `(state, ident)` against
`(ident, state)` -- which only a wrapper can do.

Remove both, and let the five callers ask the state manager directly.

Then re-home their tests. `resolution_lookup_test.rs` holds 18 cases over
`get_import_by_ident` and 2 over `get_var_decl_from`; `state_manager_test.rs`
already holds 6 over `import_binding`. Both sets test one method, so they
belong in one place, and the 18 carry edge cases the 6 do not -- a string-named
specifier, a non-ASCII local name, an empty import declaration, one specifier
of a declaration matching without its siblings. Move rather than rewrite: no
case is dropped unless a reader can name the case that already covers it.

While re-homing those tests, settle one duplication the merge exposed but did
not create. `make_var_declarator` now exists three times in
`crates/stylex-state/src/tests/` -- `resolution_lookup_test.rs:19`,
`resolution_convertors_test.rs` and `state_writers_test.rs:16` -- with
`make_var_declarator_no_init` twice beside it. Across the old crate boundary
that was unavoidable; as siblings in one `tests/` tree it is not, and
`guidelines/stack/RUST.md` permits a test prelude for exactly this. The six
copies that sat inside `resolution_convertors_test.rs` were already collapsed
to one when the file moved.

This was left out of 30 on purpose. Every criterion of that ticket is met
without it, and folding a 370-line test move into the crate merge would have
made the merge itself hard to read.

**Blocked by:** 30

**Status:** resolved

- [x] Neither pass-through exists, and every caller asks the state manager
- [x] One test module covers `import_binding`, and one covers `declaration_of`
- [x] No case is lost: a dropped case names the case that covers it
- [x] One test prelude serves the crate's declarator builders
- [x] Coverage of the `resolution` module stays at 100%
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

## Outcome

Both pass-throughs are gone from `resolution::lookup`, and the five callers ask
`StateManager::import_binding` or `StateManager::declaration_of` directly. The
module now holds the two readers that can also answer from the function map;
`crates/stylex-state/CONTEXT.md` says where each of the four lives.

The 18 import cases and the 2 declarator cases moved into
`state_manager_test.rs`, beside the 6 that were already there, as `mod
import_binding` and `mod declaration_of`. 24 cases became 18: six were folded
into cases that assert the same thing with a miss beside the hit, and the module
doc names each covering case. Two additions beyond the move -- a positive
assertion on the aliased shadowing case, so its refusal cannot pass by
answering `None` to everything, and a ten-thousand-declarator case mirroring the
import one, so both indices are shown to answer from a key rather than a walk.

`tests/prelude.rs` now holds the declarator builders. It replaced four copies,
not three: `state_manager_test.rs` held the same builder under the name
`var_declarator`. `ident_at` and `ident` were also collapsed in that file, since
`ident_at(name, 0)` built exactly what `ident(name)` did.

Coverage of `resolution/lookup.rs` stays at 100% of regions, functions and
lines, and no line of either state-manager method is uncovered. A performance
review found the change neutral: both removed functions were plain forwards, and
dropping them relaxes a borrow that tied the ident's lifetime to the state's.

Left alone, as pre-existing and outside this ticket: two unresolved
`ModuleBindingsCollector` doc links in `state_manager.rs`.
