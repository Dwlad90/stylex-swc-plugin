# 23 — Fix the parity harvest

**What to build:** The parity harvester walks a hard-coded list of two crates
for Rust test sources, pulls CSS declarations out of them into a committed
corpus, and that corpus generates a committed fixture in another crate. The
crate split moved 39 Rust test files out of one of those two crates into six
new ones, and the list was never widened. It costs nothing today only because
the few declarations that moved are duplicated elsewhere — but any value added
under those six crates now silently never reaches the corpus or the generated
fixture. Nothing asserts the list: every existing harvester test builds a
synthetic tree, so the list itself is untested and undocumented.

Separately, the harvester treats any camelCase key as a CSS property, so
`stylex.env` selector keys are harvested as declarations. Four such false rows
sit in the corpus today and flow into the generated fixture.

Both halves regenerate the same committed artifacts, so they cannot land
independently — do them together and regenerate once.

Ticket 21 has since regenerated the corpus once, for a reason unrelated to
either half: three `origin` line numbers had fallen behind a documentation
commit, and the restored gate fails on that. Ids and row order did not move,
so `cases.rs` was untouched and nothing here is done. Regenerate once more
when both halves land.

**Blocked by:** 21

**Status:** ready-for-agent

- [ ] The harvester scans all six new crates
- [ ] Env selector keys are not harvested as CSS properties
- [ ] The corpus and generated cases are regenerated once: every added entry
      is explained, the four known false entries are gone, and no legitimate
      entry was lost
- [ ] A test asserts that every crate holding a harvestable declaration is in
      the scan list, and fails when one is removed
- [ ] Tests cover both the env branch and env select shapes
- [ ] The scan list is documented where a reader will find it
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [ ] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own
