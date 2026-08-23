# 17 — The alphabet covers the classes the report claims

**What to build:** the shorthand fuzz's token classes cover the shapes an
expansion actually distinguishes, so that a class is added because it was
audited for rather than because a bug pointed at it.

The harness reports its alphabet precisely so a reader can weigh the claim
against the coverage. That contract was already broken once: the review of
ticket 14 found a fold defect reachable only by an *unterminated* comment,
which contributes an empty part, and the alphabet carried only a comment with
text in it. The class was added afterwards. One reactive addition is a miss;
a second would be a pattern.

Audit for the classes an expansion can tell apart and the alphabet does not
carry, at least:

- an importance annotation somewhere other than the end
- a brace or a semicolon as a *fragment* rather than only as a joiner
- a unicode range next to a signed number, where the `+` is ambiguous
- a part that is empty for a reason other than an unterminated comment
- a value whose parts outnumber the four a side-wise expansion reads

The deliverable is the audit and whatever it adds, not a target count. A class
crossed with everything else costs roughly 900 subjects per property, so the
judgement to record for each candidate is whether it can produce a part shape
no existing class produces.

**Blocked by:** None — can start immediately. Independent of 16, which changes
how rows are reported rather than which rows exist.

**Status:** ready-for-agent

- [ ] Each candidate above is either added or dismissed with the reason it
      cannot produce a new part shape
- [ ] Any class added is justified by the shape it reaches, not by the count it
      adds
- [ ] The run stays inside a few minutes, or the cost of the growth is stated
- [ ] No divergence the audit surfaces is left unrecorded — a defect gets a
      ticket, a deliberate refusal joins a pinned family
- [ ] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs
