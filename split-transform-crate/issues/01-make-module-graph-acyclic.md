# 01 — Make the module graph acyclic and downward-only

**What to build:** Today two edges inside `stylex-transform` violate the strict
dependency DAG the guidelines require, and the crate boundary cannot enforce
what module privacy allows. A style-semantics util reaches *up* into the visitor
layer to import the hoisting helper, and a structure and a style util depend on
each other in a cycle. After this ticket the crate's internal module graph is
acyclic and strictly downward, so every later extraction is a move rather than a
refactor in disguise.

Cut the upward edge by having the style-merge util receive the hoisting function
rather than import it — the same injection principle `stylex-atoms` already uses
to avoid depending on the transform. Cut the cycle by relocating the shared
member helper to sit with its caller.

This is prefactoring: make the change easy, then make the easy change. It lands
while everything is still one crate, so it is verifiable by inspection against a
completely unchanged test suite.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] No module in the style-semantics layer imports anything from the visitor layer.
- [ ] No cycle remains between the structures and the style utilities.
- [ ] The hoisting helper reaches its caller by injection, not import.
- [ ] Zero test files changed — not assertions, not fixtures, not imports.
- [ ] Debug workspace build and test green; no `--release`.
- [ ] Typecheck, format check, lint check and the full suite pass.
- [ ] Typecheck re-run after committing, since the pre-commit hook rewrites code.
