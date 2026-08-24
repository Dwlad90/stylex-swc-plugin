# 23 — The style resolutions nobody compares

**What to build:** the corpus reports parity under every style resolution a
consumer can pick, not just the one both compilers happen to default to.

The value harness leaves both compilers on their own default and never varies
it. The generated harness pins `legacy-expand-shorthands`, and says why — value
splitting is unreachable under anything else, so a run left on the default would
compare two compilers that both never called the code and report agreement.
That argument applies to the other resolutions too, and nothing acts on it:
`application-order` and `property-specificity` are compared nowhere.

What differs between them is which longhands a shorthand becomes and what order
they land in, which is priority and property expansion rather than value
spelling — a different failure surface from the one the corpus measures today,
and one a class name still depends on.

The two harnesses already read one option object built in one place, so the
resolution belongs there rather than in a second copy per harness.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The value harness takes the resolution as a flag, and the report's subject
      block prints which one a run used — a report that does not say is a report
      that cannot be compared with another
- [x] All three resolutions have been run over the whole corpus and the results
      read; any divergence is a defect or a pinned family, not a number left in
      a report
- [x] The default a run uses when the flag is absent is stated, and is the one
      the existing recorded verdicts were taken under, so no expectation moves
      for the reason the flag was added
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. `parity-values.ts` takes `--style-resolution` and the report names the
resolution it measured, so a run says what it covered rather than leaving the
reader to assume. All three resolutions were run over the whole corpus; no
expectation moved, which is the outcome the ticket predicted and the reason the
flag rather than three corpora was the right shape.

The default when the flag is absent is stated at the option's definition and is
the compiler's own default, so an unflagged run measures what a consumer gets.
