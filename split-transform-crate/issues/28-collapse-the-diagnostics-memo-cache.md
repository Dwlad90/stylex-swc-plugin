# 28 — Collapse the diagnostics memo cache into the diagnostics crate

**What to build:** The diagnostics crate inverts its dependency on the state
manager through a nine-method trait, which is the right idea in the wrong
shape. Four of the nine methods are diagnostics' own memoisation cache, which
happens to be stored in the state manager — so the trait publishes cache
internals instead of asking a question. The state manager already forwards
those four to an internal cache sub-struct, so the implementation is nine
bodies that each call an inherent method of the same name. Its own doc comment
concedes the trap that creates: renaming one of the nine inherent methods
turns its body into unbounded recursion rather than a compile error. The
trait's rationale comment is also stale — it says the transform implements the
trait, and the transform does not.

Move the cache to the crate that owns it. The state manager holds it as a
field, the trait shrinks to the few genuine questions diagnostics must ask of
compilation state, the nine forwarders disappear, and one accessor stops
returning a nested option where a borrowed string would do.

**Blocked by:** 24 — the trait needs coverage against the real implementation
before it is reshaped, so the refactor happens under test cover.

**Status:** resolved

- [x] The memoisation cache is owned by the diagnostics crate and held as a
      field by compilation state
- [x] The trait exposes only the questions diagnostics genuinely must ask
- [x] No method body is a same-named self-delegation, so a rename is a compile
      error rather than unbounded recursion
- [x] The seen-module accessor returns a borrowed string rather than nested
      options
- [x] The trait's rationale comment names the crate that actually implements it
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

**Resolved by:** `516787ecf` — the memo is a `DiagnosticMemo` in
`stylex-diagnostics`, held as a field by `StateManager`, which never reads it.
The trait keeps six questions: the filename, the memoized module, its key span
index, and the memo.

**Worth recording:** a qualified `StateManager::get_filename(self)` in a trait
body is *not* enough to satisfy the third criterion. The path resolves to the
inherent method first but falls back to the trait method in scope, so a rename
still gives a recursion *warning* where a compile error belongs. Every body
reaches a field instead.

The twelve cases the memo owns live in `crates/stylex-diagnostics/src/tests/`,
and the state crate keeps one case for what it still answers -- that the manager
hands back one memo rather than a fresh one per question.
