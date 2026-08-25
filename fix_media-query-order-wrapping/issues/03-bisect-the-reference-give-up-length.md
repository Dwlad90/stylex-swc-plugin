# 03 — Find the ladder length at which the reference implementation gives up

**What to build:** A recorded number and the evidence behind it: the breakpoint
ladder length at which the reference implementation stops merging ranges and
emits the authored queries unmerged, together with the observed output on either
side of that length. The reference implementation guards its own exponential
expansion by catching the call-stack error its recursion raises and returning
the input rules, so this length is the boundary of its merging behaviour.
Ticket 08 needs it, because an equivalent bound in this compiler is a number we
choose and must be placed no earlier than the reference implementation’s own.

Measurement only — nothing in this repository changes.

**Blocked by:** None — can start immediately.

**Status:** done, with two criteria answered rather than met — see
`../evidence/give-up-length.md`

- [ ] The give-up length is stated as a number, with the ladder shape used to
      find it. **Cannot be met as written — answered instead.** There is no such
      number: the reference implementation never gives up merging. The ladder
      shape is recorded, and ticket 08's premise changes as a result. Left
      unchecked deliberately, because a ticked box would read as a number having
      been found.
- [ ] The reference implementation's output is recorded for a ladder just below
      and just above that length. **Cannot be met as written — answered
      instead.** There is no give-up length to straddle, so the output is
      recorded either side of the first length at which a retained contradiction
      appears at all: 4 and 5 rungs.
- [x] The reference implementation's resolved version is recorded beside the
      number.
- [x] It is stated whether the reference implementation degrades or fails at and
      beyond that length, since the two call for different behaviour from us.
- [x] Nothing in this repository is modified.
