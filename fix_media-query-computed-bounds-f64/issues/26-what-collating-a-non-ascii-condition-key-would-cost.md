# 26 — What collating a non-ASCII condition key would cost

**What to build:** a recorded decision about how this compiler should order a
condition key that is not printable ASCII, with the cost of each way of getting
there measured rather than guessed.

This is the only remaining divergence in the whole harness that costs a **class
name**, so it is the one place where deciding badly is expensive and deciding
slowly is cheap. Two nested pseudo keys, one carrying an accented letter, sort
one way here and the other way upstream, and the sorted path feeds the hash:
`x143q076` against `x1enrlzn` for the same source.

The cause is already written down where the comparator lives. Upstream sorts
pseudo keys with root collation, which places an accented letter beside its base
letter; this compiler implements that faithfully over printable ASCII and ranks
every byte outside it above the whole table. The comparator's own documentation
names the divergence and says what closing it needs — decomposition and the full
weight table, which is a collation dependency rather than a comparator.

What is not written down is the price. That is this ticket: what a real
collation crate costs in build time, binary size and dependency surface, against
what a generated table for the ranges an authored condition key plausibly uses
costs in maintenance and in the remainder it still leaves. Both numbers are
measurable today; neither has been measured.

The output is a decision someone can disagree with, not a preference. It says
which way, what it costs, what it leaves uncovered, and how the remainder stays
measured — because a remainder nobody measures is how this one lasted.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] Both options are costed against this repository rather than in general:
      build time, binary size, dependency surface and generated-table size are
      numbers taken from a build, not estimates
- [x] The set of code points an authored condition key can plausibly carry is
      argued for rather than asserted, since it is what decides whether a
      generated table has a defensible edge at all
- [x] The decision names what stays uncovered and how a reader will know it is
      still uncovered — a divergence left unnamed is the failure this whole
      mechanism exists to prevent
- [x] The finding is recorded where the next contributor meets the comparator,
      not only in this ticket

## Closing note

Delivered, and the finding is recorded as a decision rather than as prose:
`crates/stylex-css/docs/adr/0001-root-collation-orders-a-non-ascii-condition-key.md`.

Both options are costed against this repository. The dependency is +1 222 800
bytes on the `.node` addon (12.5%), six new crates of a 33-crate tree whose other
26 the workspace already carries, and 2.74s of CPU. The generated table is three
orders of magnitude smaller and does not work: measured against `localeCompare`
over 200 000 pairs per range it disagrees on 0.50% of Latin-1 Supplement and
9.95% once combining diacritics are in play, for three structural reasons --
secondary weights, completely ignorable characters, and expansions.

The set of code points a key can carry is argued rather than asserted: a quoted
attribute value is arbitrary text, so no generated range bounds it, which is what
settles the choice.

What stays uncovered is named -- upstream calls `localeCompare` bare, so a
Swedish or Danish build machine sorts `o-umlaut` after `z` where root sorts it
beside `o` -- and `fuzz-pseudo-order.ts` prints that remainder as a per-run
count rather than leaving it to memory.

The ADR is linked from the comparator it decides, so it is still what a
contributor meets there. It began as a doc comment on an empty module and was
moved once `docs/agents/domain.md`'s rule about where a context's docs live was
applied to it.
