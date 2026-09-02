# 35 — Harvest only what is a declaration

**What to build:** `parity/harvest-corpus.ts` reads a Rust string literal as a
CSS declaration whenever it looks like one, and it cannot tell where the
literal was going. Five entries in the corpus today are not CSS:

| Entry | What it really is |
| --- | --- |
| `width: limit 64, found 65` | an assertion message in `value_normalization_parity_test` |
| `width: limit 64, found 5000` | the same, one case below |
| `width: , ` and `boxShadow: , ` | the separator argument of a `join(", ")` |
| `s: 0.25rem` | a JavaScript object key inside an embedded test source |

Each one costs twice. It is a subject both compilers are run against on every
pull request, which is wasted work; and it is a row a reader has to dismiss by
hand before the report says anything, which is what makes a report stop being
read.

This is the same defect as the ternary colon the harvester used to read as a
declaration — the fix there was one shape, and the shape list is now four. So
the ticket is not a fifth guard: it is to give the harvester the one thing it
lacks, which is where the literal was going. A literal that reaches an
`assert!`, a `panic!`, a format argument or a `join` is not a declaration
whatever it spells, and a `key: value` pair inside an embedded JavaScript
source is a JavaScript property rather than a CSS one unless the object is a
style namespace.

Found while closing ticket 24, which asserts 111 corpus values directly and
had to exclude these five because a case over one of them would state nothing
about CSS.

**Status:** needs-triage

- [ ] The harvester drops a literal whose use is an assertion, a panic, a
      format argument or a separator, and keeps every declaration it reads
      today — the corpus loses exactly the five entries above and gains
      nothing
- [ ] A `key: value` pair in an embedded JavaScript source is harvested only
      where the enclosing object is a style namespace, so a property named
      after no CSS property does not become one
- [ ] The harvester's own suite covers each dropped shape, and fails when the
      guard is taken away
- [ ] The corpus is regenerated and the entry count accounted for in the
      commit message
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
