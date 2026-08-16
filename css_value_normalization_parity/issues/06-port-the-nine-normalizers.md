# 06 — Port the nine value normalizers

**What to build:** The nine small transformations the reference compiler applies
to a parsed value, ported one-to-one and run in its exact order over the token
list from ticket 03.

In order: unclosed-function detection, unclosed-string detection, whitespace,
timings, zero dimensions, leading zero, quotes, camel-case value conversion,
and — only when the font-size-to-rem option is enabled — font-size conversion
appended last.

**The order is load-bearing and must be preserved.** Timings runs before leading
zero specifically so that a millisecond value converted to seconds is then
stripped of its leading zero. Reordering them silently changes output.

The decisive property of this set is what it does *not* contain. No normalizer
understands hex colors, letter case, quote characters, or exponent notation, so
none of them can alter those — which is exactly why the divergences in the
parent issue close by construction in ticket 07 rather than by correction.

Port verbatim. Each normalizer gets its own file named after its upstream
counterpart, and one module holds the ordered list and the fold over it,
mirroring the upstream entry point. Resist making them more idiomatic: their
value is that a maintainer can diff them against upstream by eye at the next
release, and anything gained by restructuring is lost the first time upstream
changes and nobody can tell whether the difference is intentional.

Nothing is wired into the compiler in this ticket.

**Blocked by:** 02 — JavaScript-semantics float parsing utility; 03 — Vendored
value parser port.

**Status:** resolved

- [x] All nine are ported and run in the upstream order, with the font-size
      conversion appended only when its option is enabled
- [x] File names correspond one-to-one with their upstream counterparts
- [x] The whitespace normalizer collapses runs of whitespace in place and
      normalizes separator spacing without moving any space to a different
      position in the value
- [x] The zero-dimension normalizer decides whether a token sits inside a
      function by comparing source offsets, matching upstream, rather than by
      tracking visitor state
- [x] The custom-property exemption in the zero-dimension normalizer is
      preserved
- [x] Numeric decisions route through the utility from ticket 02, and numeric
      output through the existing JavaScript-spelling utility
- [x] Unclosed functions and unclosed strings raise the established errors, with
      the same messages as today
- [x] Behavioural deltas beyond the six reported divergences are accepted, not
      papered over — camel-case conversion applying to every top-level word
      token, and leading-zero stripping applying wherever upstream applies it,
      are expected consequences of porting faithfully
- [x] Where a delta looks like an upstream defect rather than a difference,
      upstream behaviour is still adopted and the deviation is called out in the
      commit message
- [x] Verified against the harness at the value level; the normalizers are not
      individually pinned by unit tests that would lock their internals

## Comments

Ported and landed. Every expectation in the new test module is a string the
reference compiler actually produced, taken from a run of its own normalizers
over `postcss-value-parser@4.2.0`, not written by hand.

Deviations from a literal transcription, each forced and each behaviour-neutral:

- The whitespace normalizer's importance handler removes a node from the list
  the walk is iterating, which Rust cannot do from inside a borrow. The removals
  are planned against the untouched list, then applied. The plan reproduces all
  three of the original's quirks: the index tested belongs to the node's own
  sibling list while the list edited is always the top level; a removal shortens
  the list without shortening the walk, so an annotation that is not the last
  token reads past the end; and a removal is skipped when the preceding
  top-level node is not a space.
- The two upstream crashes — an empty node list, and that overrun — are
  reproduced as rejections with local messages rather than an imitation of a
  JavaScript runtime error string.
- The unclosed-function report quotes the rule it came from, which is what this
  compiler's message has always carried and the reference implementation's has
  not.

Upstream defects adopted rather than corrected, because hash parity outranks
local correctness here: `msTransform` dashifies to `ms-transform` rather than
`-ms-transform`; an all-capitals value gains a leading dash (`ABC` becomes
`-abc`); the zero-dimension window closes at the *first* function's end, so
`translate(0px) translate(0em)` keeps the unit in one call and drops it in the
other.

The nine are not pinned individually. The seam is the fold's output, which is
what the class name hashes.
