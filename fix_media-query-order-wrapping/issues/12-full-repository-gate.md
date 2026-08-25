# 12 — Full repository gate

**What to build:** Proof that the whole repository is green, not just the parts
this work touched. Typecheck, format check, both linters including the shell
pass, the type-aware lint that only reports after a build, the Rust suites, the
JS suites, and the parity harness.

The JS suites exercise the built artifact rather than the Rust sources, so a
rebuild precedes them or they measure the previous compiler. The parity harness
prints the versions it resolved before anything else; that line belongs in the
evidence, because the reference implementation is held by the lockfile rather
than by an exact range and moves under a dependency update without anything in
the parity directory changing.

Anything skipped or failing is stated rather than omitted.

**Blocked by:** 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11. Ticket 11 is
drafted rather than filed, by the maintainer's decision; nothing it would
change is code, so the gate is not waiting on it.

**Status:** done — see `../evidence/gate.md`

- [x] Typecheck, format check, both linters, and the type-aware lint pass.
- [x] The Rust suites pass.
- [x] The JS suites pass against a build made after the last Rust change.
- [x] The parity harness passes, and the versions it resolved are recorded.
- [x] Any step skipped, or any failure accepted, is named explicitly with its
      reason.
