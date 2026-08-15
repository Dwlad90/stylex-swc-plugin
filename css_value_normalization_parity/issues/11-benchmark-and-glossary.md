# 11 — Benchmark the new pipeline and update the glossary

**What to build:** Evidence for the performance claim, and a glossary that
describes the code as it now is.

**Benchmark.** The parent spec argues the replacement is cheaper: per
declaration, the old path built a synthetic rule string, ran a full CSS parse,
walked the result with a mutating visitor, ran code generation, then made five
separate full-string scans to extract and repair the result — against one value
parse, nine cheap token walks, and one emit. That is an argument, not a
measurement. A memoizing cache sits in front of normalization, so repetitive
files already skip much of the old cost and the end-to-end gain is bounded.
Measure it rather than asserting it, and report the honest number even if it is
smaller than the structural argument suggests.

Scoped to measurement. Not added as a continuous-integration gate.

**Glossary.** In this crate, "normalizer" currently means a mutating visitor
over a CSS stylesheet. After this effort it means a small transformation over a
token list, one of nine, named after its upstream counterpart. A reader coming
to the crate glossary would be actively misled. Terms that no longer exist come
out. The value scanner does **not** go in: it has a crate of its own now, and
its glossary with it, so what belongs here is only the pointer that a reader
following "normalizer" needs. That its behaviour is reproduced quirk-for-quirk
rather than designed is stated there — it is the fact most likely to be
"corrected" by a well-meaning future contributor, and it belongs beside the
code it describes.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** ready-for-agent

- [ ] A criterion benchmark exercises the public value normalization entry point
      over a representative corpus, placed alongside the crate's existing
      benchmark
- [ ] The corpus spans the shapes that matter: plain keywords, dimensions,
      functions, nested functions, gradients, and long shorthands
- [ ] Before and after numbers are captured and recorded on this ticket, run on
      the same machine
- [ ] The effect of the memoizing cache is stated, so the number is not read as
      a whole-compile speedup it is not
- [ ] The benchmark is not wired into continuous integration
- [ ] The crate glossary defines "normalizer" as it now works, and drops terms
      that no longer exist
- [ ] The glossary records that the value parser is a faithful port of a
      third-party library and that idiomatizing it would defeat its purpose
- [ ] The glossary index is updated if its entry for this crate changed
