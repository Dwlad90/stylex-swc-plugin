# 20 — A string doubled by the evaluator rather than by a call

**What to build:** The character ceiling reaches a string the evaluator grows,
not only one a fold is asked to build.

```js
const a0 = 'x'.repeat(100000);
const a1 = a0 + a0;
const a2 = a1 + a1;
// … eight more lines …
```

Measured on this compiler: `a10.length` folds to **102400000** in three
seconds. Nothing refused it. `maxFoldedCharacters` is a hundred times smaller
than the value that came out, and it never looked: every `+` is a binary
expression the evaluator answers itself, and `bound_value_too_large` is read
only where a value crosses into a fold.

The chain is bounded, but by the wrong number. `maxEvaluationDepth` refuses the
fifteenth doubling, so the multiplier tops out near `2 ** 14`; with a base at
the character ceiling that is about **1.6e10** code units, which is tens of
gigabytes and not a diagnostic. A depth budget bounds how far the evaluator
recurses and was never a claim about how large a value gets.

**Not the same fix as 18.** 18 put the guard in front of calls whose result
length an argument declares, and the guard is asked about calls. Nothing in this
chain is a call — `a0 + a0` is an operator, answered by the evaluator's own
binary-expression path, one line at a time and each line innocent. So the bound
belongs where the concatenation happens, which is a different subsystem and a
different reviewer's file. Recorded separately for that reason, which is the
decision 18's second residual asked whoever took it to make.

Where to put it is part of the work. The concatenation path is the obvious
place, and the question to answer first is whether the ceiling belongs on each
`+` or on what a binding may *hold* — the second is the stronger claim, since it
would also reach a template literal and a `concat`, and it is the one that says
what a name is allowed to be worth rather than what one operator may do.

**Found while building 18**, in the same sweep that asked whether an input could
still allocate past a ceiling. Not a regression: nothing in 12 or 18 changed the
evaluator's arithmetic.

**Blocked by:** none.

**Status:** resolved

- [x] `const b = a + a` past the character ceiling refuses, wherever the chain
      is written
- [x] The bound is the ceiling a project already sets, not a second number
- [x] Whether it lives on the operator or on what a binding may hold is decided
      and written down, with a template literal and `concat` measured against
      the same question
- [x] A case measures the wall clock, so a refusal that arrives on time is told
      from one that arrives after the allocation
- [x] The corpus records whichever spellings upstream folds and this compiler
      refuses — upstream folds the ten-doubling chain above to `400000px` at a
      smaller base

## Answer

**The bound sits on the growth, not on what a binding may hold.** Three
measurements decide it against the ticket's own preference:

1. An inline `(a0 + a0).length` allocates exactly as much as the chain does and
   no binding holds the result, so a bound on bindings would let the same string
   through when it is written one way. The ticket's own first criterion —
   *wherever the chain is written* — is what rules the stronger-sounding claim
   out.
2. The growth is where the memory is spent, so refusing there refuses *before*
   the next doubling allocates rather than after it.
3. A long string a binding merely holds is one allocation the author asked for.
   `const a = 'x'.repeat(400000)` folds, is read through a template and folds
   again. What turns a typo into gigabytes is compounding, and only the growth
   site sees it.

**`concat` and `repeat` were measured against the same question and needed
nothing.** Both are calls, so both already carry a bound: `a0.concat(a0)` past
the ceiling refuses with `Folded string is too large…` on the way back out of the
fold, and `a0.repeat(2)` refuses with `Cannot bound the string 'repeat' would
build.` before the fold runs.

`+` and an interpolation are therefore the two *expressions* the evaluator grows
a string with, and both now grow it through `GrownString`, which owns the buffer
as well as the count so neither site can append without being measured. What that
does **not** reach is a third grower found while checking this claim: an array's
own `ToString` renders every element and joins them inside
`stylex_js::coercions`, so the buffer counts a string that is already built. The
refusal is right and the number is right, and it arrives after 4.4 seconds on a
two-hundred-element array of long strings. That is 20's own complaint one layer
down, in a shared coercion in another crate, and it is filed as issue 23 on the
terms 18 used to file this one — with the measurement, so nobody re-derives it.

One helper carries the bound for both:
`evaluate/helpers.rs::grow_string_within_ceiling`, reading
`maxFoldedCharacters` through `StateManager::character_ceiling`, so no second
number was introduced. It counts UTF-16 code units, which is what every other
reading of that ceiling spends, and takes its refusal path as a closure so the
happy path clones no syntax.

**Measured**, default ceiling, `@stylexjs/babel-plugin` 0.19.0:

| Input | Reference compiler | Here, before | Here, now |
| --- | --- | --- | --- |
| the ten-doubling chain, base 100000 | `102400000px`, ~1s | `102400000px`, 3.05s | refuses, 0.11s |
| `const a1 = a0 + a0` at base 600000 | `1200000px` | `1200000px` | refuses |
| `(a0 + a0).length` at base 600000 | `1200000px` | `1200000px` | refuses |
| `` `${a0}${a0}`.length `` at base 600000 | `1200000px` | `1200000px` | refuses |
| the same chain under `maxFoldedCharacters: 4000000` | `1200000px` | — | `1200000px` |
| `a0.concat(a0).length` | `1200000px` | refuses (fold) | unchanged |
| `a0.repeat(2).length` | `1200000px` | refuses (arithmetic) | unchanged |
| `('x'.repeat(1000000) + '').length` | `1000000px` | `1000000px` | `1000000px` |
| `('x'.repeat(999999) + 'xx').length` | `1000001px` | `1000001px` | refuses |
| twenty doublings from base 1 | folds | depth budget | depth budget |
| `` `${a}` `` over 200 × 900000 | `180000199px`, <1s | `180000199px` | refuses, 4.4s — issue 23 |

The wall clock is the third criterion: 0.11s against 3.05s, because the refusal
lands at the fourth doubling with eight hundred thousand characters allocated and
nothing more.

Two edges are pinned deliberately. The ceiling is a length that **folds** — a
million characters exactly is the fold and a million and one the refusal — and
the length is counted in code units, so a pair of astral characters is four and
not the two scalars or the eight bytes they spell as. A ceiling of zero is read as
unset and the default answers, the reading every other ceiling gives it.

Transform cases are in
`crates/stylex-transform/tests/transform_stylex_create_test/grown_string_ceiling.rs`
(12 tests), the diagnostic's own in `stylex-constants`. Two corpus rows record
the divergence — `modules-20-a-string-the-evaluator-doubles`, which carries the
reported ten-doubling chain at a base where upstream's `1000448px` is only just
past the ceiling, and `modules-20-a-string-a-template-grows`. Both are
`configuration: maxFoldedCharacters`, so they print under *Configured ceilings*
rather than as divergences a reader has to act on. Harness: 1178 subjects, **0 changed, 0
unexpected**.

Three documents were made wrong by leaving them alone and were corrected with the
change rather than left for issue 16: the `Allocation ceilings` glossary entry,
the `maxFoldedCharacters` section of the compiler README, and the two ceiling
accessors' own doc comments, which counted the sites that spend each number.
