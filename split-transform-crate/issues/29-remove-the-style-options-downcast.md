# 29 — Remove the style-options downcast escape hatch

**What to build:** Two separate traits stand in for the state manager to avoid
a dependency cycle, and one of them is routinely defeated. A public helper
downcasts a style-options handle straight back to the concrete state manager,
and ten call sites across two crates use it. The trait it defeats has exactly
one implementer and is immediately downcast, with a comment observing that
every handle the compiler builds is that one type — so the abstraction buys
nothing and the helper is a public item whose only purpose is to cross a
boundary and cancel it.

Pick one inversion and commit to it: either widen the options trait so it
answers what those ten call sites actually need, or let them take the state
manager directly and drop the trait from that path. This is a design-level
change to the crate graph — write the approach down and get it agreed before
editing, and keep the hottest path free of new indirection.

**Blocked by:** 21

**Status:** ready-for-agent

- [ ] Either the options trait answers what the call sites need, or they take
      compilation state directly and the trait leaves that path
- [ ] The downcast helper is gone, and nothing re-introduces an equivalent
- [ ] No new dynamic dispatch lands on the evaluation path
- [ ] Benches confirm the fold is unchanged
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [ ] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own
