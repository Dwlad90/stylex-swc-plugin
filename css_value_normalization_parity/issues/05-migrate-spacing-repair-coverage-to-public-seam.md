# 05 — Migrate spacing-repair coverage to the public seam

**What to build:** The same treatment as ticket 04, for the other body of
implementation-coupled tests: those asserting against the hand-rolled whitespace
repair pass and the helpers around it.

This coverage is the more valuable of the two. It is where years of individually
reported defects accumulated — a function result immediately followed by a unit,
URL bodies containing characters that look like CSS syntax, comments inside
values, adjacent quoted strings, non-ASCII content, percentage followed by a
number. Each of those assertions exists because someone hit the bug. The
implementation they guard is deleted in ticket 07; the knowledge must not be.

Re-express every case against the public value normalization entry point,
asserting current behaviour, annotated with the harness verdict from ticket 01.
Green before the swap and after it.

**Blocked by:** 01 — Babel differential parity harness.

**Status:** resolved

- [x] Every input covered by the existing spacing-repair tests is covered by a
      case at the public normalization entry point
- [x] Expectations assert current behaviour, so the suite is green before any
      pipeline change
- [x] Each case records whether it matches the reference compiler or is a known
      divergence scheduled to change, sourced from the harness
- [x] The regressions preserved explicitly include: a function result directly
      followed by a unit with no space inserted, URL bodies copied verbatim,
      comments passed through intact, adjacent strings kept separate, non-ASCII
      content preserved, and the leading-zero treatment of negative decimals —
      **except "comments passed through intact", which describes the repair pass
      and not the compiler.** Codegen drops a comment before the pass is
      reached, so what the entry point can assert is that comments disappear.
      Recorded that way in `drops_comments_from_the_value`, and argued for in
      the module header; see "Four places where the seam and the pass disagree"
      below
- [x] The value-extraction and rule-structure helpers used only by these tests
      are covered through the public entry point too, so their removal in ticket
      09 loses nothing
- [x] No new assertion references the repair pass or any helper ticket 07
      removes
- [x] The original test files are left in place for now — deletion is ticket 09

## Answer

`crates/stylex-css/src/css/tests/spacing_repair_parity_test.rs` — 42 tests over
180 distinct declarations, all reaching the compiler through
`normalize_css_property_value` and nothing else. The three source bodies it
replaces are
`crates/stylex-css/src/css/normalizers/tests/whitespace_normalizer.rs` (57
inputs), `crates/stylex-css/src/tests/whitespace_normalizer_tests.rs` (60), and
`restore_negative_leading_zero_test` at the bottom of
`crates/stylex-css/src/css/normalizers/tests/base.rs` (18). All three are left
in place; deletion is ticket 09.

**The `Case` machinery is now shared.** `Reference`, `Case`, `same`, `diverges`
and the `check` runner moved from `value_normalization_parity_test` into
`css/tests/support.rs`, which already held the option objects and
`panic_message`. Both parity modules import them, so the self-policing
"a case claiming a divergence must really diverge" assertion is one
implementation rather than two. `support.rs` also gained two things that were
being written out longhand: `rejects`, which replaces three copies of the same
catch-and-match-the-message loop, and `unchanged(property, value)` — the case
whose expectation *is* its input.

`unchanged` is the majority shape by a wide margin, because most of what either
module asserts is that a value is not rewritten: 94 of 101 cases here and 33 of
54 in the sibling were spelling the same string out twice, which is 127 pairs of
literals free to drift apart. A case now says "unchanged" once, as itself. The
module carries no duplicated declaration at all after the change — 180 case-table
calls, 180 distinct declarations.

**A seventh harvest shape.** `rejects("width", &["*(", …], …)` is a property
followed by a slice of values, which none of the six existing extractors could
see — so moving the rejection loops onto it would have dropped six inputs out
of the corpus, including the four whose `both reject` verdict this module cites.
`harvest-corpus.ts` now reads that shape, stopping at the `]` so the diagnostic
argument after the slice is not harvested as a value. `unchanged` joined the
call names shape 1 already reads, for the same reason — a constructor the
harvest cannot see is 127 declarations quietly leaving the corpus. Both are
unit-tested in `parity/__tests__/harvest.test.ts`, under "shape 7" and "shape 6"
respectively. Shape 7 also removes two junk entries the old inline loops had
been contributing (`color: unclosed function`), which is why the `both reject`
and `acceptance divergent` buckets are cleaner than the previous run's.

**What the verdicts say, read as a whole.** The reference compiler does not
repair spacing, because it never damages it — it returns the value the author
wrote. So almost every space this pass inserts is a divergence, and the cases
that agree are the ones where it inserts nothing: the 45-unit table behind issue
#927, `url()` bodies, string contents, subtraction operators, already-spaced
values, non-ASCII text. The one real exception is `/`, which the reference
compiler also spaces — and spaces differently at the start of a value
(` / 7` against `/ 7`), which is a divergence of its own.

That inversion is why so many migrated cases carry `diverges`: not because the
migration changed anything, but because the repair pass is a divergence
generator by construction, and this is the first time each of its rules has been
verdicted individually.

**Every case is machine-checked against the harness.** The expectations were not
written by eye: a validator parsed the module's `unchanged`/`same`/`diverges`
call sites with the harness's own `scanRustLiterals`, looked each
`(property, value)` pair up in `--json` output, and compared the recorded `expected` against
`entries[].rust.declarations` and the recorded upstream spelling against
`entries[].babel.declarations`. It reports clean apart from the two comment-only
cases noted below.

**Not migrated, deliberately.** Four inputs have no seam equivalent, because
their subject is a helper's behaviour on a string no stage of the compiler
produces: `is_css_unit("")`, and `extract_css_value`'s doubled-brace,
triple-brace, no-brace and no-colon rule literals. So is the `Cow` allocation
contract at the bottom of `restore_negative_leading_zero_test` — borrow when
nothing changes, own when something does — which is an efficiency property of a
function the entry point does not expose. All of it guards code ticket 09
deletes.

The extractor's *observable* contract does survive: a value keeps a `;` inside
a data URL, a `:` inside an `http://` URL, nested parens and quoted braces, all
covered by `copies_url_bodies_verbatim` and
`balances_nested_parens_inside_a_url_body`.

**Four places where the seam and the pass disagree, and the seam wins.**

- **Comments are dropped, not preserved.** The pass copies a comment through
  verbatim — it must, since spacing a `/*` would produce `/ *` — but SWC's
  codegen removes it first, so the observable behaviour is that comments
  disappear. `drops_comments_from_the_value` records that.
- **Degenerate inputs are rejected before the repair runs.** `)(`, `*(`,
  `url(it's-fine.png)` and `url(a/*b.png)` reach the structural guard, not the
  pass. The first two are `both reject`; the two URL bodies are
  `acceptance divergent`, because the guard reads the value as text and does not
  know `url()` from any other function.
- **An empty value is `acceptance divergent`.** This compiler normalizes `""`
  and `"   "` to empty; the reference compiler rejects the declaration. There is
  no reference spelling to compare, so it is asserted directly.
- **A comment-only value is `structurally divergent`.** `width: /* a */`
  normalizes to empty here and this compiler then drops the declaration, so the
  two emit a different *number* of declarations. Kept in the case table with the
  reference declaration text as the upstream spelling, since that is what a
  pipeline change would have to start producing.

**One finding, filed rather than fixed here.**

- **An escaped quote inside a string breaks out of the rule** (ticket 15).
  `fontFamily: "a\"b#c"` normalizes to `a"b#c}` — SWC's codegen emits the string
  *without* its quotes, extraction then fails to see the generated rule's
  closing brace as a terminator, and the `}` lands in the declaration. That is
  the exact failure the structural guard exists to prevent; the guard reads the
  author's value, which is well-formed, and the brace is manufactured downstream
  of it. The single-quoted spelling fails more quietly: `'a\'b#c'` becomes
  `"ab#c"`, with the character silently deleted. This is the only divergence in
  the module that is also a correctness bug.

**Two old test modules now say so.** The headers of
`css/normalizers/tests/whitespace_normalizer.rs` and
`tests/whitespace_normalizer_tests.rs` name their successor and say they are
kept only until the pass they address is deleted, so nobody adds a case to the
module that is on its way out.

**Corpus.** No hand-written entries were needed — the harvester's shape 6 reads
`same`/`diverges` call sites, so the new module's inputs entered the corpus on
their own. The corpus is now 777 declarations: 545 identical, 174 divergent, 17
structurally divergent, 22 acceptance divergent, 19 rejected by both.

**Two properties had to be swapped to get a verdict at all.** `background` and
`border` are shorthands that both compilers expand away, so a case written on
one produces no declaration to compare and the harness reports a vacuous
`identical`. The url cases moved to `backgroundImage` and the `border` cases to
`boxShadow` and `color`. Worth knowing before writing a case on a shorthand: a
verdict with empty `declarations` on both sides is not evidence of anything.
`parity/README.md` now says so, and the validator described above fails on it.

**The long data URL stays whole.** Its two literals ran to 187 characters, over
the line budget, and neither a `const` nor `concat!` could fix that without
taking the case out of the corpus — the harvest reads literals, not bindings.
Rust line continuations do both: still one literal to the scanner, which already
decodes `\`-newline the way rustc does, and verified to produce the same 177
bytes as the copy in the module it was migrated from.

**Glossary.** `crates/stylex-css/CONTEXT.md` gains **Spacing repair**,
**Reference verdict** and **Reference compiler**. The first two were being used
as settled vocabulary across two modules and a README without ever having been
defined; the third pins the term the modules use for `@stylexjs/babel-plugin`.
