# 07 — Create the evaluator crate and seed it with the dependency-free leaves

**What to build:** Stand up the crate that will hold the general JavaScript
evaluator, and move into it the parts that can travel alone: the growable stack,
the binding lookup and the evaluation helpers all have **zero** internal
dependencies, and the evaluation cache depends only on the growable stack.

Splitting this from the bulk move that follows keeps that move purely mechanical
and gives the new crate a compiling, gated existence before ~9.5k lines land in
it. Roughly 1.8k lines move here.

The **deopt** helper deliberately stays behind for now: it reads the code-frame
builder, so it travels with the core in the next ticket.

**Blocked by:** 05 — Extract the diagnostics crate; 06 — Rename the nested-config crate.

**Status:** ready-for-agent

- [ ] The crate exists under the name freed by the rename and compiles.
- [ ] The growable stack, binding lookup, helpers and evaluation cache live in it.
- [ ] Nothing moved in this ticket depends on anything still in the transform.
- [ ] The transform reaches these through the new crate path, with no facade left behind.
- [ ] The unit tests covering the moved code move with it.
- [ ] The crate reaches zero uncovered lines and zero uncovered regions, or ships a temporary exclusion that ticket 08 removes.
- [ ] The crate has a `CONTEXT.md` and a context-map row.
- [ ] The crate is placed in the documented layer list.
- [ ] Benches diffed against the baseline; no regression outside noise.
