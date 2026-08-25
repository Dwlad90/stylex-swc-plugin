# 06 — The declaration loss, asserted red

**What to build:** A failing test describing what an author actually gets from
the reference implementation when two entries of one conditional value map
canonicalize to the same query text: one of the two declarations is absent from
the output entirely. This is only observable once ticket 05 lands, because
collisions are what retained contradictory branches produce.

Assert it at the end-to-end seam, where the missing declaration is visible in
emitted CSS and in the rule count, and in the parity corpus, so that the rule
count is compared against the reference implementation rather than against our
own belief about it. Expectations come from a run of the reference
implementation.

Commit red, separately from ticket 07, so the history shows this as a second
cause rather than as fallout from ticket 05.

**Blocked by:** 05.

**Status:** done, with one criterion answered rather than met — see
`../evidence/collision.md`

- [x] A conditional value map whose entries canonicalize to one query text is
      asserted at the end-to-end seam and in the parity corpus.
- [x] The expected rule count and emitted text are quoted from a reference
      implementation run, with the version recorded.
- [ ] Both fail before ticket 07, and the failure shows this compiler keeping a
      declaration the reference implementation drops. **Cannot be met as
      written — answered instead.** Neither can fail: the loss is already
      reproduced here, one layer below the transform, by the consumer that
      writes its output into an insertion-ordered map. 4032 ordered value maps
      over an alphabet built for collisions find no input the two compilers
      disagree on, and the one shape a search cannot reach is ruled out by
      argument in the evidence file.
- [x] The test states which of the two colliding declarations survives and at
      which position, since both are part of the contract.
