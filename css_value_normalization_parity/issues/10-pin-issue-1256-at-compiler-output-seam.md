# 10 — Pin issue #1256 at the compiler-output seam

**What to build:** Permanent regression tests for all six divergences reported
in the parent issue, asserted where the contract actually lives.

The other tests in this effort assert normalized declaration text at the value
normalization entry point. That is the right seam for the bulk of coverage, but
it cannot see the thing the parent issue is actually about: the **class name**.
The class name is a hash of the canonical declaration, and it is the
compatibility contract between this compiler and the reference one. A test that
checks the declaration text but not the hash would miss a defect in hashing
itself.

So these six go through the full transform and assert class names and rule text
from the emitted style metadata, matching the reference compiler exactly.

All six are pinned, including the two that were already fixed before this effort
began. Those two were closed by restoration passes that ticket 07 deletes; if
the port regresses them, that must fail loudly here.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** resolved

- [x] All six reported cases are covered, each asserting both class name and
      rule text from the transform's style metadata
- [x] Expected class names and rule text come from the reference compiler via
      the harness, never hand-written
- [x] The whitespace case covers all six of its sub-inputs, including the
      gradient with percentage color stops
- [x] The math-function spacing case covers all three of its sub-inputs
- [x] The hex case covers both the standalone colors and the one inside a
      gradient
- [x] The already-fixed cases — transform function capitalization and plain
      decimal spelling of large numbers — are pinned alongside the rest, so a
      regression introduced by deleting their restoration passes cannot pass
      silently
- [x] Tests are placed with the existing value normalization transform coverage,
      following its established shape rather than introducing a new one
- [x] Each test names the reported case it pins, so a future failure is
      traceable to the report without archaeology

## Answer

Six snapshot tests in
`crates/stylex-transform/tests/transform_value_normalization_test/css_value_normalization.rs`,
one per reported symptom, named `issue_1256_*`. Each carries every sub-input the
report gives for its symptom, so the file holds all 14 illustrating values.

The styles are exported, which puts both halves of the contract in one snapshot:
the injected rule text, and the compiled object mapping each namespace to its
class name — the object markup actually reads. A few pre-existing tests in this
file export too, for their own reasons; what marks these six is the `issue_1256_`
prefix and the measurement quoted above each.

**Expectations are measured, not written.** Every test quotes the class name and
rule `@stylexjs/babel-plugin@0.19.0` produces for the same source, read from a
harness run over the `reported` corpus set. Reports are gitignored, so the
command is the artifact:

```sh
pnpm run --filter=@stylexswc/rs-compiler build
pnpm run --filter=@stylexswc/rs-compiler parity -- --set reported
```

Each quoted rule was compared against the generated snapshots: all 14 match byte
for byte, class name included.

## Beyond the checklist

`class_name_edge_cases.rs` alongside it pins 39 further edge shapes at the same
seam — escapes, non-ASCII, `url()` bodies carrying CSS syntax, comments,
malformed-but-accepted values, vendor prefixes, nesting, extreme numbers, custom
properties, letter case, separators, importance — each likewise quoted from a
harness measurement. `class_name_rejections.rs` holds five `stylex_test_panic!`
cases for the deliberate rejections (brace, second declaration, depth guard,
unprefixed custom property) — the opposite contract, kept in its own module so
neither is read as the other. 52 distinct harness-measured rules are pinned
across the two parity files: 14 reported, 38 edge.

The harvest corpus was regenerated for the new test values, and a full run over
all 744 declarations reports **0 value-normalization divergences**. The harness
collapses duplicates across corpus sets — 67 of 811 entries here — so a value
that lands in both a hand-written set and the harvest is measured once.
