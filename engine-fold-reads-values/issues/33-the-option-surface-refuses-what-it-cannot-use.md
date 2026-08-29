# 33 — The option surface refuses what it cannot use

**What to build:** A project that configures a limit is told what it got, and a
value that is not a count is refused where it is written rather than silently
becoming something else.

**Four gaps, all on options this effort added or changed.**

*The evaluation depth changed twice, silently.* The hard cap went from `1 << 20`
to `8 * 1024` — a hundred and twenty-eight fold — and is now re-clamped on every
read. A project configuring `50000` gets `8192` and is not told. The **units**
changed too: this branch's own test moves `arithmetic(315)` to `arithmetic(159)`,
so the same configured number buys roughly half the nesting it used to. Both are
defensible, and the cap is what makes the stack claim finite — at the maximum it
is exactly one gibibyte, which is the number the cap was chosen against. Neither
change is additive, and `maxEvaluationDepth` is an existing public option.

*The character ceiling tightened six and a half fold*, from 65,536 on the merge
base to the new default. Right for hand-written styles, which is what the default
is sized for and says so. A generated token module is the case that could sit
between the two numbers and compile before and not after — worth measuring
against the corpus before shipping, and worth naming in the same upgrade note as
the depth, since a reader hitting one will want to know about the other.

*A ceiling accepts what is not a count.* Non-integer, `NaN`, `Infinity`, negative
and string values are unpinned at the NAPI boundary for all three ceilings. What
a `NaN` ceiling does today is undefined by any test.

*`maxFoldedEntries` is the thin sibling.* `maxFoldedCharacters` has an
invalid-input test, a past-the-limit clamp test and an option-beats-env test.
`maxFoldedEntries` has none of the three. `Ceiling::clamped` has no behavioural
test at all — deleting it and returning the bare `usize` passes the whole suite.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] A configured depth above the cap produces a diagnostic naming the cap,
      rather than being clamped in silence
- [x] The depth cap, the depth unit change and the character-ceiling tightening
      are recorded together where a user upgrading will read them, as a breaking
      note rather than a README default
- [x] The character-ceiling change is measured against the corpus, so the claim
      that nothing real sits between 65,536 and the new default is a measurement
- [x] All three ceilings refuse a non-integer, `NaN`, `Infinity`, a negative and
      a string, at the boundary and with a sentence naming the option
- [x] `maxFoldedEntries` carries the three tests `maxFoldedCharacters` has
- [x] `Ceiling::clamped` has a test that fails when it is removed

## Answer

**One rule, at the options boundary, for all three ceilings.** A configured value
is a whole number between `1` and that ceiling's cap or it is refused by name.
The three arrive as `ConfiguredCeiling` -- an `f64` or a note that what came was
not a number at all -- rather than as `i64`, which read `1.5` as `1`, `NaN` and
both infinities as `0`, and answered a string with a message about a conversion.
`as_ceiling` in `crates/stylex-rs-compiler/src/structs/mod.rs` is the single
place it is decided.

The refusal was made uniform across the three rather than special-cased to the
depth, which is what the first box asked for. A per-option divergence would be
arbitrary: the reason a clamped `50000` is worse than a refused one is the same
reason for all three, and `maxFoldedCharacters: 2 ** 40` was clamped in the same
silence. The one test that pinned that clamp is now a refusal test.

The **environment variables are deliberately not held to this rule**. They are
shared by every build on a machine, so a mistyped one still falls back to the
default rather than failing a build. That split is now stated in the glossary and
in the README.

**The corpus measurement.** Ran the value harness twice, once at the new default
of `10000` and once at `STYLEX_MAX_FOLDED_ENTRIES=65536`, the fixed number on the
merge base. Three of 1,191 declarations change verdict:

| row | source |
| --- | --- |
| `modules-18-a-length-a-call-declares` | `Array(20000).fill(0).length` |
| `modules-18-a-declared-length-that-never-crosses` | `String(Array(10001))` |
| `modules-06-amplified-array-length` | `'x'.repeat(20000).split('').length` |

All three are rows written to demonstrate the entry ceiling. Nothing that reads
as a real declaration sits between the two numbers.

To re-run it, from `crates/stylex-rs-compiler`:

```sh
node --import tsx/esm ./parity/parity-values.ts --json /tmp/at-default.json
STYLEX_MAX_FOLDED_ENTRIES=65536 \
  node --import tsx/esm ./parity/parity-values.ts --json /tmp/at-65536.json
```

Then compare the two reports' `entries[].verdict` pairwise by index.

**The refusal is not the whole surface.** It is the NAPI options boundary only.
A Rust caller building `StyleXOptionsParams` itself, and the environment
variables, both still resolve through `Ceiling` as before -- neither has a
written line to name. The `resolve_from` doc and the glossary say that rather
than the stronger claim they said on the first pass.

**A correction to the ticket's third paragraph.** The six-and-a-half-fold
tightening is `maxFoldedEntries` (65,536 to 10,000), not the character ceiling --
`maxFoldedCharacters` is new on this branch and has nothing to tighten from.

**A correction to the ticket's first paragraph.** The depth unit did not change
for the evaluator's own descent: a bare tower of arithmetic folds at 317 levels
on this branch exactly as on the merge base. What halved is an expression handed
to a method call, which the fold's guard now walks counting syntax nodes rather
than evaluation levels -- `Array(arithmetic(315)).length` on the merge base
against `arithmetic(159)` here. The upgrade note says that rather than the
stronger claim.

**Where it landed.**

- `crates/stylex-rs-compiler/src/enums/mod.rs` -- `ConfiguredCeiling`
- `crates/stylex-rs-compiler/src/structs/mod.rs` -- `as_ceiling`, the rule
- `crates/stylex-rs-compiler/src/tests/structs_tests.rs` -- the rule, per ceiling
- `crates/stylex-rs-compiler/__test__/index.spec.ts` -- the five spellings across
  the boundary, per ceiling, plus the three `maxFoldedEntries` was missing
- `crates/stylex-transform/.../tests/state_manager_test.rs` -- the `clamped`
  wiring, verified by dropping the call and watching it fail
- `crates/stylex-rs-compiler/README.md` -- an `## Upgrading` section, since the
  repo has no changelog
- `crates/stylex-structures/CONTEXT.md` -- the option/environment split
