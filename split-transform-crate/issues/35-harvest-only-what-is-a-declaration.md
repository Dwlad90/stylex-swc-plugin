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

**Status:** done

- [x] The harvester drops a literal whose use is an assertion, a panic, a
      format argument or a separator, and keeps every declaration it reads
      today — the corpus loses exactly the five entries above and gains
      nothing
- [x] A `key: value` pair in an embedded JavaScript source is harvested only
      where the enclosing object is a style namespace, so a property named
      after no CSS property does not become one
- [x] The harvester's own suite covers each dropped shape, and fails when the
      guard is taken away
- [x] The corpus is regenerated and the entry count accounted for in the
      commit message
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## What it turned out to be

The list of five was written against an older corpus. `s: 0.25rem` had already
gone with ticket 34, which bounded shape 5 to the `stylex.create` call, and
twenty-one more of the same kinds had arrived since. Asking
where a literal was going drops all of them, and the corpus goes from 823 to
798 with nothing gained:

| Kind | Entries | Example |
| --- | --- | --- |
| A needle searched for in output | 11 | `width: limit 64, found 65` |
| A branch of a ternary | 5 | `black: white`, read off `isDark ? 'black' : 'white'` |
| An assertion or panic message | 2 | `height: expected \`{value}\` to be rejected` |
| A separator handed to `join` | 2 | `boxShadow: , ` |
| A key that names no property | 3 | `e21: a`, read out of `1e21: 'a'` |
| A key of an object handed to a function | 1 | `toString: notfn` |
| A rejection pair read inside out | 1 | `calc("a: width` |

Three things came out of it that the ticket did not ask for and each is one
line of the same answer:

- Two helpers in the suites are named `rejects` and take their arguments in
  opposite orders. Shape 7 now requires the slice, so the reversed one is not
  read as a table.
- A key a comment sits in front of was being lost, since a comment is not a
  comma. The key-position test reads the nearest *code* character instead.
- `format!` is deliberately not on the list. It builds both messages and
  values, and the message it builds reaches an assertion, so the chain catches
  it there. That is what keeps `boxShadow: 0px 0px {n}px #000`.

One real subject went with the junk. `width: calc(1px` reached the corpus only
as the needle of `contains("* { width: calc(1px }")`; the value is a genuine
input of `normalize_value("calc(1px", "width", …)`, whose argument order no
shape reads. The unclosed-function family keeps `width: calc(1px);height:2px`
and `color: red;calc(1px`, so the family is still covered; harvesting that
helper would *add* entries and belongs to its own ticket.

## Cost

The harvest is unchanged at 0.38-0.42 s, of which 0.16 s is the Node start.
The new pass over a fixture costs about 7 ms of the 170 ms harvest, and the
callee walk about 2 ms.

The callee walk is bounded to 512 characters, the way every other backward
read here is bounded. A statement has no bound of its own, and a case table is
one statement holding hundreds of rows, so an unbounded walk read the whole
statement once per row: a 2000-row table took 1035 ms rather than 21 ms. The
window leaves the corpus byte-identical, and the furthest a disqualifying
callee sits behind a literal in the suites today is about 100 characters.

Not fixed, and not from this change: `isTableInput` finds the first literal
after an opening bracket by scanning from the head of the file's literal list,
so a test block costs the square of the literals in it. It does not bite the
corpus as it stands.

## What the review changed

Six holes, none of which moved the corpus but each of which would have let the
junk back in on the next ordinary test edit:

- Masking blanks string literals only, so a `matches('(')` counted a
  parenthesis that closes nothing and a `calc(` or a `;` in prose ended or
  redirected the walk. The walk now blanks comments and character literals
  inside its own window.
- A `;` or a brace written in a comment ended the walk, which brought
  `width: limit 64, found 65` straight back.
- One space defeated the callee read, so `String ({ toString: 'notfn' })` was
  read as a style object; and closing that let `return({ … })` read as a call,
  which would have dropped a real style object. Reserved words are named.
- `true` and `false` were patched by spelling. The shape is the `[` after the
  closing brace: an object the fixture indexes is a lookup table. That also
  drops `{ color: 'red', width: 'blue' }[flag]`, which the two spellings never
  reached.
- Shape 7 matched one spelling of the slice, so a `vec![…]` would have lost a
  whole rejection table in silence. The `[` is now found structurally.
- The doc claimed an argument list never crosses a brace. A closure body does,
  and that the walk stops there is deliberate: the suites wrap the call under
  test in `catch_unwind(AssertUnwindSafe(|| { … }))`. The limit is stated.

## The three that were left

Each was reported as pre-existing, and each was one ordinary test edit away
from undoing the guards above. All three are closed.

**The mask blanked string literals only.** Its own contract says bracket
matching runs over it, and a `matches('(')` or a `calc(` written in prose
carries a bracket that closes nothing. The scanner already stepped over
comments and character literals to find the literals at all, so it now reports
where they are and one masker blanks all three. The callee walk gave up sixty
lines of window handling that asked the same question locally. Two scanner
bugs came out with it: `'\''` was read as ending at the escaped quote, and the
plain form of a character literal was tested before the narrower escaped one.

**A test block was bounded by its values.** `testBlocks` matched braces over
raw source, and `"* { color: red { }"` is an authored input here, so a block
closed early on one value and ran to the end of the file on another. Two
blocks reached line 987, which is how 23 values belonging to other tests were
harvested under `width`. Bounding the block over the mask lost 11 degenerate
values with them, because the wrong property was the only reason they were in
the corpus at all — so shape 2 now reads the property off any of the six calls
that take a declaration, and `refuses_with("color", value, …)` gives them
back under `color`. The corpus went 798 to 799: 23 misattributed pairs out, 24
correctly attributed in, including the whole `url()` body family and the
rule-breaking values, which no block had ever reached.

**Two readers walked a list from its head.** `isTableInput` and
`literalsBetween` cost the literals in the whole file per question. One binary
search answers both, and a table of 8000 rows went from 271 ms to 66 ms. The
widened property reading then made a second cost visible — each block searched
the file for each of six names — so the call sites are found once per file and
shared with shape 1, which was already finding them. The harvest is 0.35
seconds, against 0.40 before any of this.
