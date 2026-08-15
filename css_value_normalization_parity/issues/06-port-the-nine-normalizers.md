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

**Status:** ready-for-agent

- [ ] All nine are ported and run in the upstream order, with the font-size
      conversion appended only when its option is enabled
- [ ] File names correspond one-to-one with their upstream counterparts
- [ ] The whitespace normalizer collapses runs of whitespace in place and
      normalizes separator spacing without moving any space to a different
      position in the value
- [ ] The zero-dimension normalizer decides whether a token sits inside a
      function by comparing source offsets, matching upstream, rather than by
      tracking visitor state
- [ ] The custom-property exemption in the zero-dimension normalizer is
      preserved
- [ ] Numeric decisions route through the utility from ticket 02, and numeric
      output through the existing JavaScript-spelling utility
- [ ] Unclosed functions and unclosed strings raise the established errors, with
      the same messages as today
- [ ] Behavioural deltas beyond the six reported divergences are accepted, not
      papered over — camel-case conversion applying to every top-level word
      token, and leading-zero stripping applying wherever upstream applies it,
      are expected consequences of porting faithfully
- [ ] Where a delta looks like an upstream defect rather than a difference,
      upstream behaviour is still adopted and the deviation is called out in the
      commit message
- [ ] Verified against the harness at the value level; the normalizers are not
      individually pinned by unit tests that would lock their internals
