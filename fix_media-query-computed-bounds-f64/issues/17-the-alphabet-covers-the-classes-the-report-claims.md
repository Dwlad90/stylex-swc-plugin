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

**Status:** done

- [x] Each candidate above is either added or dismissed with the reason it
      cannot produce a new part shape
- [x] Any class added is justified by the shape it reaches, not by the count it
      adds
- [x] The run stays inside a few minutes, or the cost of the growth is stated
- [x] No divergence the audit surfaces is left unrecorded — a defect gets a
      ticket, a deliberate refusal joins a pinned family
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Outcome

Three classes added, two candidates dismissed with the argument recorded in the
`FRAGMENTS` documentation:

- **added** `comment, empty` (`/**/`) — the other way to reach an empty part, by
  a different scan: this comment closes, so the empty part need not be the last.
- **added** `separator as a part` (`;`) — a separator standing as a part rather
  than between two, which no joiner can reach. Neither compiler refuses it, so
  the split stays observable.
- **added** `five space-separated parts` — every other class is one part, so a
  pair was at most two and nothing could reach the fifth side an expansion
  discards or the sixth a fold carries.
- **dismissed** *importance annotation not at the end* — already generated: the
  pairs are ordered, so `!important 1px` puts it ahead of a part, which is the
  part shape the fold reads.
- **dismissed** *brace as a fragment* — and not on the criterion this ticket
  named. It *can* produce a part shape no existing class produces; that shape is
  simply unobservable, because a part that is a `{` or `}` is refused before any
  expansion is emitted, so every row would report a refusal already pinned and
  none would say where the value was cut. Its sibling the semicolon stayed for
  exactly the reason this one did not: neither compiler refuses it.
- **dismissed** *unicode range beside a signed number* — already generated, as
  `U+0-7F` joined to `-1px` by the `+` joiner, which is the ambiguous spelling.

Cost: 129,744 → 153,624 subjects, 76s → 84s. No divergence went unrecorded — the
audit surfaced no new refusal, and the one shape the generated corpus reached
that the curated one did not is now pinned as the `first refusal to fire` family
and tracked for closure in ticket 20.
