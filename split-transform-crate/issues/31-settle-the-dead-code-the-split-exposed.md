# 31 — Settle the dead code the split exposed

**What to build:** A dead-code audit of the whole workspace, run against the
base branch for comparison, found the refactor itself remarkably clean: it
*removed* three dead-code allowances and four unused-import allowances and
added none, and there are no dead modules, no orphan non-test source files, no
dead enum variants and no unused crate dependencies in anything it touched. Out
of 1,863 public definitions cross-referenced against workspace-wide usage, two
items are newly dead and owed by this branch.

**The first is an over-widened visibility.** A declaration-framing helper in
the diagnostics crate was crate-private on the base branch and the split
widened it to public. It has no production caller anywhere in the workspace;
its only callers are two test files, one of them in a different crate. So the
item is public *solely so a cross-crate test can reach it*. The deadness
itself is pre-existing — it had no production caller before either — but the
public visibility is new, and it is the second concrete instance of the
uncalled public function that ticket 12 raised and left open. Note that a
library crate's dead-code lint never fires on a public item, so nothing will
ever warn about this.

**The second is an orphaned script target.** The shared script target that
every crate's `test` script used to invoke is now called by nothing: the base
branch had fourteen manifests pointing at it, and this branch has none, because
they were all rewritten to echo a skip line. Its fate depends on what ticket 21
decides — restoring the test scripts gives it a caller again, while keeping the
skip means it should go.

**Alongside the two, record what is deliberately not being fixed.** The audit
found seventeen pre-existing dead items, and the precedent set by ticket 12 is
that pre-existing deadness is recorded rather than fixed opportunistically.
Recording it is still worth doing, so the next reader does not re-derive the
list. Two parts of that inventory are worth calling out: eight items whose
leading-underscore names hide them from the dead-code lint entirely, which is a
blind spot rather than a decision; and four standing dead-code allowances, none
added by this branch, which the audit has already explained — one is
load-bearing because the only calls to the method it covers form a cycle the
compiler reads as dead, one covers a trait method with three implementations
and no caller at all, and two sit on public items where the lint cannot fire
anyway, making them vestigial no-ops.

**Blocked by:** 21 — the orphaned script target's fate follows from the
test-gate decision. The declaration-framing item overlaps ticket 28's
reshaping of the same crate's public surface, so whichever of the two lands
second should check the other's outcome rather than assume it.

**Status:** ready-for-agent

- [ ] The declaration-framing helper is dealt with: either deleted along with
      its two test call sites, or narrowed back to crate-private with the
      other crate's two assertions moved in beside it
- [ ] No public item in the six new crates is public solely to satisfy a test
      in another crate
- [ ] The orphaned script target is deleted, or given a caller, matching
      whatever ticket 21 settled
- [ ] The pre-existing dead-code inventory is recorded rather than fixed:
      five uncalled public functions and the eight underscore-named items
- [ ] The four standing dead-code allowances carry the audit's answers, so a
      reader can tell the load-bearing one from the two no-ops without
      re-deriving it
- [ ] Whether leading-underscore naming should keep hiding items from the
      dead-code lint is settled one way or the other — it is a lint blind spot
      the repo may not have chosen deliberately
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
