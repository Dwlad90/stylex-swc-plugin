# 07 — Draft the upstream defect report

**What to build:** One issue body for `facebook/stylex`, ready to post, covering
the two defects this effort found in the reference implementation's
logical-expression evaluation. Not code.

Both defects sit in the same block and share one root cause — a **value**
confused with its **truthiness** — so they belong in one issue a maintainer can
fix in a single edit, not two.

1. **The nullish guard tests truthiness where it means nullishness.** The
   guard is, in effect, `leftConfident && !!(left ?? rightConfident)`. When the
   left side is falsy but not nullish — `0`, `false`, `''` — the `??` yields
   that falsy value, `!!` turns it to `false`, and the expression falls through
   to an unconditional deopt. So a statically-resolvable `x ?? 5` fails to
   compile when `x` is `0`, and `` `${x ?? 'red'}` `` fails when `x` is `''`,
   even though both sides are confident. The guard appears to have been meant as
   `left != null || rightConfident`.

2. **`&&` with a falsy confident left side crashes rather than diagnosing.**
   The branch correctly returns `left`, but a later consumer does not expect
   that value and dies with a bare `Cannot read properties of undefined
   (reading 'type')` — no code frame naming the offending property, which for an
   author inside a large style object is the difference between a two-minute fix
   and an afternoon.

Include a minimal self-contained module per defect and the observed output, in
the shape the upstream repository asks for. State the version the behaviour was
observed on. Keep the report about the reference implementation's own behaviour
— it stands on its own, and this port is not the subject.

## Posting is a separate, explicit act

The repo's tracker is local; publishing anything publicly is deliberate. Draft
the text, show it, and stop. Posting happens under the maintainer's own account
only on an explicit go-ahead in the same breath — never as a side effect of this
ticket being picked up. Record the resulting issue number here once it exists.

**Blocked by:** 03.

**Status:** ready-for-human

- [x] One drafted issue body covering both defects, with the shared root cause
      stated
- [x] A minimal self-contained reproduction per defect, with observed versus
      expected output
- [x] The observed version is stated
- [x] The report stands on the reference implementation's own behaviour, without
      requiring the reader to know about this port
- [x] The draft has been shown for review and **not** posted
- [ ] After an explicit go-ahead: posted, and the issue number recorded in this
      file

## Comments

The draft is at [`../upstream-defect-report.md`](../upstream-defect-report.md).
It carries the issue body, then a short section marked as excluded from that
body saying which of the two defects this port reproduces and which it declines
to — so the parity decision is visible to a reader of the draft without the
upstream reader having to know this port exists.

Nothing has been posted. The last box stays open until there is an explicit
go-ahead.
