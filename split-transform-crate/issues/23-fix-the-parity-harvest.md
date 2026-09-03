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

**Status:** resolved

- [x] The harvester scans all six new crates — and every other crate, because
      the list was removed rather than widened
- [x] Env selector keys are not harvested as CSS properties
- [x] The corpus and generated cases are regenerated once: every added entry
      is explained, the four known false entries are gone, and no legitimate
      entry was lost
- [x] A test asserts that every crate holding a harvestable declaration is in
      the scan list, and fails when one is removed — there is no list to
      assert, so the test asserts the stronger property instead: a declaration
      under a crate nothing names is harvested
- [x] Tests cover both the env branch and env select shapes
- [x] The scan list is documented where a reader will find it — the scan
      surface is in `parity/README.md`, `guidelines/STRUCTURE.md` and
      `guidelines/SCRIPTS.md`
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

## Answer

### The list is gone, not widened

The scan named `stylex-css` and `stylex-transform`. Widening it to name the new
crates would leave the same defect in place for the next split, so the walk now
starts at `crates/` and reads the crate names off the tree. Nothing has to be
kept in step, and the test that guards it harvests a declaration out of a crate
name that appears nowhere in the repository.

Scanning everything opened one loop the list had closed by accident:
`postcss-value-parser`'s `cases.rs` is generated *from* this corpus and spells
its inputs as CSS rules, so it harvested one of its own rows back in. Sources
with `@generated` in their header comment are now skipped, which is the marker
that file already carried. Only the header counts, so a value further down that
spells the marker does not exempt a real test file.

### The env objects are opaque, so none of them are read

The false rows came from the object a `stylex.env` function is called with.
Which keys there are CSS properties is decided by the environment function the
test supplies: `select({ primary: 'red' }, 'primary')` names branches and
`colors({ color: 'yellow' })` names properties, and the source gives the
harvester no way to tell them apart. So the object handed directly to the call
is skipped whole. A branch body sits one brace deeper and is an ordinary style
object, so its declarations still reach the corpus.

The brace depth is answered by one forward pass with a stack that skips quoted
text, and only for a fixture that mentions `stylex.env.` at all.

### What moved

828 declarations to 825. Three of the four false rows are gone
(`alternate: 16px`, `alternate: blue`, `secondary: blue`). Two entries changed
origin only: `color: blue` now comes from `stylex-state-index`, which is one of
the crates the list had stopped covering, and `color: yellow` from a fixture
that is not an env one. `cases.rs` moved two rows, both of them reorderings.

### What the brace scan does not read

The scan skips quoted text, so a brace in a value cannot open a block. It does
not skip comments. A JavaScript comment holding an unbalanced brace inside a
fixture would put every key after it under the wrong object. No fixture in the
repository has one — the guard runs on the 24 literals that mention
`stylex.env.`, and none of them carries a comment at all — so the case is left
open rather than answered with code nothing exercises. The corpus check reports
it if that changes.

### Out of scope, found while measuring

*The fourth false row survives on its own merits.* `primary: red` is still in
the corpus, harvested from `const color = { primary: 'red' };` in
`logical_operators.rs` — a plain JavaScript object beside a `stylex.create`
call, not an env argument. Shape 5 reads the whole fixture literal rather than
the `stylex.create` object inside it, so any object in a fixture contributes its
keys. That is a wider false-positive class than this ticket, and closing it
means bounding shape 5 to the call. Filed as 34.

*The widened walk costs 130 ms.* The scan reads 879 sources rather than 414,
and `parity:harvest:check` goes from 0.23 s to 0.36 s. The cost is literal
scanning and masking, not disk. Making the mask lazy would win back about
50 ms, at the price of a memoized getter and a guard in each of the four
extractors. Not taken: the check runs in front of a vitest suite that takes
seconds, so the complexity buys nothing a reader would thank us for.

*The harvest check can be cached away.* `@stylexswc/rs-compiler#test` declares
no Turbo input outside its own package, so a Rust test edit in another crate
does not invalidate it and the cached task replays without running its
`pretest`. This predates the ticket. The wiring suite in
`scripts/git/generated-fixtures.test.mjs` does not catch it because it only
looks at `generate:*` scripts. Filed as 33.
