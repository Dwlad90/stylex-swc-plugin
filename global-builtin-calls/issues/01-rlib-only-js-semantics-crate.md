# 01 — Build the JS-semantics crate as an rlib only

**What to build:** The JS-semantics crate stops producing a dynamic library
nothing loads. No behaviour changes for anyone using the compiler; builds get
shorter by one pointless link step.

The crate declares both a dynamic library and an rlib. Nothing references the
dynamic one — no build script, no napi configuration, no packaging script, no
other crate. Reduce it to an rlib.

Scoped to this crate alone. Every crate in the workspace carries the same
declaration and only the compiler entry point needs it — one already has the
line commented out, so this conclusion has been reached here before. The
repo-wide cleanup is deliberately out of scope and wants its own branch.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] Confirmed nothing references the crate's dynamic library by name before
      changing it; reverted if anything does
- [x] The crate declares an rlib only
- [x] The workspace builds and the full Rust suite passes
- [x] The compiler artifact still builds and the JavaScript suite passes
      against it

## Answer

`crates/stylex-js/Cargo.toml` now declares `crate-type = ["rlib"]`. Commit
`4ba4c8c1b`.

A workspace-wide search for `stylex_js` / `libstylex_js` / `stylex-js` across
Rust, TOML, JSON, JS, TS, shell and CI files found only: the crate's own
manifests, the pnpm lockfile and workspace catalog entries, the
`@stylexswc/stylex-js` devDependency link in `crates/stylex-evaluator/package.json`
(a pnpm link, not a Cargo dependency — ticket 02 adds the Cargo one), the crate's
own test-file header comment, and unrelated `*_stylex_js` fixture and theme-hash
identifiers in the transform tests. Nothing loads the dynamic library. The crate
exports no `extern "C"` or `#[no_mangle]` items.

Verified: `cargo build --workspace` and `cargo test --workspace` clean (26 test
binaries, 0 failures); `target/debug` now holds `libstylex_js.rlib` with no
accompanying `.dylib`. The compiler artifact rebuilt and `pnpm test` passed all
63 tasks. `pnpm format:check`, `pnpm typecheck` and `pnpm lint:check` clean.
`pnpm lint:type-aware` not run — no TypeScript changed.

Note for ticket 02: because nothing in the release dependency graph depends on
this crate yet, it does not appear in `target/release` at all. Wiring it into the
transform crate is what first puts it in the compiler's build.
