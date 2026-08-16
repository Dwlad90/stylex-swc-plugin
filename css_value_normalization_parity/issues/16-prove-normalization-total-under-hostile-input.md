# 16 — Prove normalization total under hostile input

**What to build:** A measured claim that the public value-normalization seam
answers *every* input — with a declaration or with a StyleX diagnostic — and
never with a panic nobody wrote, a stack overflow, or an abort.

## Why this is not already covered

The parser underneath has this proof.
`no_arrangement_of_the_characters_it_branches_on_can_crash_it` in
`crates/postcss-value-parser/src/tests/properties.rs` runs every four-character
string drawn from the alphabet the scanner branches on — brackets, quotes,
escapes, comment markers, separators — and asserts the scan completes. That is
a claim about every input, not about the ones somebody thought to write down.

Nothing above the parser has it. Between `normalize_css_property_value` and the
scan sit a structural pre-scan, six guards, and nine passes, and every one of
them indexes into text an author controls: the passes slice token values, split
a dimension into number and unit, re-spell a number through a float parse and a
JavaScript-shaped printer, and read a quote character off the front of a string.
Each is a place where a byte index can land off a character boundary or past an
end. The existing coverage at this seam is a case table — nearly a hundred
tests, every expectation harness-measured — and a case table proves the cases in
it. It cannot prove the absence of an input that takes the process down, which
is the failure mode ticket 14 already found once, in this same layer, from a
shape no table happened to name.

The distinction that makes this testable: **a rejection is not a crash.** This
compiler raises a rejection as a panic carrying one of the `LINT_*` diagnostics
in `crates/stylex-constants/src/constants/messages.rs`, and the compiler catches
it and reports the file. So the claim is not "never panics" — it is "every panic
is one of ours." An index-out-of-bounds, a `None` unwrapped, or a slice off a
character boundary is a different thing entirely: it escapes as a message the
author cannot act on, and the two are indistinguishable to a test that only
checks `is_err()`.

## Scope

Totality only, at the seam that already exists. This does not add a seam, does
not test the nine passes individually, and does not assert a single new
expectation about *which* declaration a value produces — the case tables and the
harness own that, and duplicating it here would re-create the
implementation-coupled coverage this effort spent five tickets removing.

Where an input the sweep reaches turns out to produce a *wrong* declaration
rather than a crash, that is a divergence, and it belongs in the harness corpus
and a case table — not here.

**Blocked by:** None. The seam and its diagnostics are in place; 07 settled what
sits behind it.

**Status:** resolved

## Acceptance

- [x] An exhaustive sweep runs every string of the scanner's branching alphabet
      up to a length the suite can afford, through the public normalization
      entry point, and asserts each call either returns or raises a diagnostic
      from the known `LINT_*` set — never an unrecognized panic
- [x] The sweep names the panic it saw when it fails, so a regression reports
      the offending input and the message rather than only that something broke
- [x] The alphabet covers what the passes branch on as well as what the scanner
      does: a digit, a decimal point, a sign, a unit letter, `%`, `!`, and the
      structural guard's `{`, `}` and `;`
- [x] Multi-byte input is swept too — the alphabet is ASCII, and a byte index
      that is safe among ASCII is exactly where a character-boundary panic
      hides. Combining marks, an astral-plane code point, a right-to-left mark,
      and a non-breaking space, each placed at a boundary a pass slices on
- [x] The sweep runs against every property class that takes a different path
      through the passes: a plain longhand, a duration, `transitionProperty`
      (dashifies), a custom property (exempt from zero canonicalization), and
      `fontSize` with the rem option enabled
- [x] Boundary numbers are swept: the largest and smallest finite double, a
      value that overflows to infinity, one that underflows to zero, an
      exponent with no digits, a lone sign, a lone decimal point, and a number
      long enough that its digits exceed a double's precision
- [x] Degenerate structures are asserted, each to what it should actually do:
      nesting past `MAX_VALUE_NESTING_DEPTH` rejects, balanced and unbalanced,
      while a megabyte-long value and fifty thousand sibling tokens normalize —
      neither length nor token count is bounded, and a compiler that refused
      them would be refusing valid CSS
- [x] The suite's runtime cost is stated on this ticket, and the sweep length
      chosen so the crate's tests stay usable rather than to maximize the number
- [x] Every diagnostic constant reachable from this seam is reached by at least
      one input in the sweep, so the known set is a measured list rather than an
      aspirational one
- [x] No expectation in this ticket is written by eye; anything asserting a
      declaration's text cites a harness run

## Answer

`crates/stylex-css/src/css/tests/totality_test.rs`, ten tests, roughly a
hundred and forty thousand calls through `normalize_css_property_value`.

**Nothing crashed.** Every sweep passed on the first run. That is the headline
and it is a negative result: the seam was already total over everything reached,
and this change is the measurement rather than a fix.

**Runtime.** Roughly 1.1s wall for all ten, run in parallel. The `stylex_css`
crate went from 978 tests to 992 and from 1.07s to about 1.1s, so the sweep is
effectively free at the crate level. Lengths were chosen against that budget:
the twenty-one character alphabet exhaustively to three (9,723 values), and a
ten-character subset — what nests and what quotes — to four (10,000 values),
each run under all five property classes. The real ceiling is not time but
output: no panic hook is installed, because replacing it would silence
libtest's capture for every test running in parallel, so every rejection the
sweep provokes prints.

**Two things the sweep found, neither of them a crash.**

_Four arrangements take two normalization runs to settle rather than one._
`()/` gains a trailing space on the second run; `00)`, `00\` and `00*` each
lose a trailing character on it. All four are fixed points from the second run
on — the movement is a one-off, not a ratchet. (An earlier reading of this
called `()/` unbounded growth; asserting the property rather than a pinned
literal is what caught that, and the test now measures convergence.)

Nothing normalizes twice today, so what a declaration gets is the first run's
spelling — and the harness says the reference compiler produces exactly that
spelling for all four. Pinned three ways: which arrangements move, and how they
converge, in `totality_test`; the first run's declaration text in
`value_normalization_parity_test`; and the class name in
`class_name_edge_cases`. The second run's text is pinned nowhere as a literal,
deliberately: normalization is applied once, so no reference compiler ever
produced it and there would be nothing to measure it against.

_One diagnostic had no dedicated assertion._ `LINT_UNCLOSED_COMMENT` was only
ever reached incidentally; the reachability test would have passed with the
guard collapsed into the rule-breaking one beside it. Now asserted by name,
with a companion test that a *closed* comment is not taken for an unclosed one.

**Measured against `@stylexjs/babel-plugin@0.19.0.`** Twenty-two hostile values
added to the harness's `edge` corpus — the four non-settling arrangements,
eleven number spellings at the edges of a double, and seven multi-byte
characters at slicing boundaries. All twenty-two report `identical`.
`parity/results/` is gitignored, so the record is this ticket and the corpus;
reproduce with:

```sh
pnpm run --filter=@stylexswc/rs-compiler build
pnpm run --filter=@stylexswc/rs-compiler parity -- --set edge
```

Four of the numbers are not returned as
written, and all four agree with upstream: `1e-324px` and `-0px` both read back
as `0px` and *keep* their unit, `0.px` reads as `0` and loses it, and
`0.12345678901234567890123456789` comes back rounded to `.12345678901234568`.
What separates the two that keep the unit from the one that drops it is that
the rule reads the authored spelling rather than the quantity.

The corpus was re-harvested as part of this, which the harness's `--check` had
been failing since an earlier commit moved the depth-limit rejection into its
own module. One value lost its harvest source in that move; it is still covered
by hand as `edge-nesting-past-the-depth-limit`.

A full run over all three corpus sets — 768 declarations — reports **`divergent
0`**. The remaining non-`identical` verdicts are the pre-existing acceptance
and structural ones this compiler takes deliberately.

**Deliberately not asserted.** Which declaration a swept value produces. The
sweeps check only the shape of the answer — a string, or a known diagnostic —
so that no unmeasured expectation enters the repo through this door. Everything
above that names a spelling names a harness run for it.

**Why all twenty-two are pinned at the class-name seam too**, not only at the
normalization seam. The parent spec reserves the issue seam for the six
reported cases, on the grounds that what #1256 is about is the class hash. That
reasoning applies here for the same reason: a hostile value that reaches a
stylesheet reaches it as a hash, and the declaration text is only the input to
it. Added at maintainer request, and cheap — three snapshots, no new machinery.
The prose is not duplicated across the two seams; the class-name module states
the class names and points at the normalization module for why each value is
interesting.

**Not in scope, and left open:** the passes are still never permuted, so
pass-ordering is only protected where the composed output happens to differ.
Nothing here changes that.
