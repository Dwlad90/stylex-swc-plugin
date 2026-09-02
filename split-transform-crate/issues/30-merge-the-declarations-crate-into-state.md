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

**Status:** ready-for-agent

- [ ] The declarations crate's contents live as a module of the state crate
- [ ] Its dependents import from the new path, and no facade is left behind
- [ ] The crate's scaffolding, context-map row and layer entry are removed
- [ ] Coverage does not regress in the receiving crate
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [ ] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own
