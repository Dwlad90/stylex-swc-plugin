# 02 — Reproduce both failures at HEAD and build the parity harness

**What to build:** A maintainer can run one command and see both reported
failures happen in this worktree, at the current commit, rather than in a
published release — and can see, for every `sx` case, what the reference
implementation produces for the same input.

The harness generates the compiled input the way the reporter did, with the
tool that produced it, rather than approximating it by hand; that tool is
already a development dependency of the Vite plugin package.

It shows two things at HEAD: a compiled call written with the object shorthand
comes back still carrying a native prop, and a module that uses the prop with
no import is refused by the plugin before the compiler is reached.

It also produces the parity reference the later tickets are checked against.
For raw markup the comparison is direct — same source, same configured prop
name, outputs must be identical. For a compiled call the comparison is
indirect and deliberately so: the reference implementation has no compiled-call
path and returns such input unchanged, so asserting output equality there would
assert the defect. Each compiled case is instead paired with the raw markup it
was compiled from, and the check is that our compiled output carries the same
props call, the same resolved binding and the same injected import as the
reference output for that markup, differing only in the call shape.

The harness lives in this tracker directory, which has no installed modules of
its own, so it resolves what it needs through a package that has them. Keeping
it here also keeps any mention of another implementation out of the repository.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] Running the harness at HEAD shows the compiled shorthand untransformed
- [ ] Running the harness at HEAD shows the plugin refusing a module that uses
      the prop and imports nothing
- [ ] The compiled input is generated, not hand-written
- [ ] For each raw-markup case the harness prints both outputs and whether they
      are identical
- [ ] For each compiled case the harness prints the reference output for the
      paired raw markup
- [ ] Nothing is added to the repository; the harness is confined to the
      tracker directory
