# 15 — Bring the evaluator crate to the coverage gate

**What to build:** The `stylex-evaluator` crate that ticket 13 filled measures
66.76% of regions and 70.70% of lines against its own tests. The workspace
coverage gate demands zero uncovered lines and zero uncovered regions from every
crate that has a `src/lib.rs` and tests. Write the tests that close the gap.

The shortfall is not new code going untested. It is the same boundary effect
[ticket 11](./11-cover-the-state-crate.md) records for the state crate: the
evaluator was covered _transitively_, by compiling whole files through the
transform's integration suite under `crates/stylex-transform/tests/`, and the
transform is itself exempt from the gate. Moving the evaluator out made that
coverage stop counting for it. Ticket 13 moved all 569 unit tests down with the
code, which is what the 66.76% is; the missing third has no unit test anywhere.

`stylex-evaluator` is on the coverage exemption list, so CI is green in the
meantime. The exemption is the holding position, not the answer.

**Where the gap is**, from `pnpm run test:coverage` in the crate:

| File                                    | Uncovered regions | Cover          |
| --------------------------------------- | ----------------- | -------------- |
| `evaluate/nodes/call_expression.rs`     | 464               | 31.16%         |
| `evaluate/nodes/member_expression.rs`   | 282               | 49.10%         |
| `evaluate/engine_fold/transport.rs`     | 268               | 39.09%         |
| `evaluate/engine_fold/guard.rs`         | 201               | 81.54%         |
| `evaluate/helpers.rs`                   | 183               | 61.06%         |
| `evaluate/engine_fold/amplification.rs` | 135               | 74.77%         |
| `evaluate/mod.rs`                       | 125               | 66.22%         |
| `evaluate/engine_stylex_functions.rs`   | 102               | 16.39%         |
| `evaluate/nodes/global_conversion.rs`   | 92                | 17.12%         |
| `evaluate/nodes/object_expression.rs`   | 82                | 70.61%         |
| `evaluate/nodes/arrow_function.rs`      | 73                | 32.41%         |
| `evaluate/nodes/binary_expression.rs`   | 59                | 81.27%         |
| `evaluate/nodes/unary_expression.rs`    | 49                | 67.33%         |
| `evaluate/engine_fold/outward.rs`       | 37                | 82.04%         |
| `evaluate/engine_fold/theme.rs`         | 37                | 74.83%         |
| `convertors.rs`                         | 29                | 86.94%         |
| `evaluate/binding.rs`                   | 28                | 84.53%         |
| the remaining twelve files              | 63                | 68.42%--97.92% |

Three groups, in the order they are worth taking:

1. **The small whole-file gaps** -- `evaluate_result.rs` (the `refused`
   constructor) and `nodes/typescript_expression.rs` both read 0.00% and are a
   handful of lines each. `nodes/global_conversion.rs` and
   `engine_stylex_functions.rs` are under 20% and are each one concern.
2. **The two big node handlers** -- `call_expression.rs` and
   `member_expression.rs` carry 746 uncovered regions between them and are best
   taken one callee shape and one lookup shape at a time. Every shape is already
   reachable through `evaluate_source`, so no new scaffolding is needed.
3. **`transport.rs`** -- 268 regions at 39%. The carriage of a value in and of
   an engine value back is exercised end to end by the transform's suite and
   almost not at all directly.

Note that the coverage tool keeps only the best-covered instantiation of a
generic, so a generic helper can read as fully covered while one instantiation
is untested.

**Blocked by:** None — the evaluator crate is filled as of ticket 13.

**Status:** done

- [x] `stylex-evaluator` reports zero uncovered lines and zero uncovered
      regions.
- [x] `stylex_evaluator` is removed from all **five** exemption lists, not the
      three this criterion first named: `package.json`,
      `scripts/coverage-missing.sh`, `scripts/packages/test/coverage.sh`,
      `EXCLUDED` in `scripts/git/crate-coverage-runner.test.mjs`, and the row
      under "Excluded from Coverage" in `guidelines/STRUCTURE.md`.
      `scripts/git/coverage-exclusions.test.mjs` compares all five and names the
      one that disagrees, so removing four of them fails that test.
- [x] Tests cover regular and irregular inputs, and the edge cases each function
      states, rather than only the path that makes the number go up.
- [x] The full workspace suite stays green.

## Comments

**2026-09-02, ticket 25.** Re-measured on the tip of
`feat_split-transform-crate`: 66.86% of regions, 75.22% of functions and 70.93%
of lines, with 2347 unexercised regions across 28 files. Ticket 24 added two
test files here and moved the region figure by 0.1 of a point, which is the
point that ticket made: the holes it closed were behavioural, not lines. The
exclusion stays until this ticket lands, and `guidelines/STRUCTURE.md` now names
this ticket as its remover.

**2026-09-07, first instalment.** Twelve test modules land, and the crate moves
from 66.85% of regions -- re-measured here, where the ticket body says 66.76%
and the 2026-09-02 comment 66.86% -- to 78.70%. 2347 unexercised regions down
to 1507, and 28 files down to 25. Four files reach 100% that did not before:

| File                                  | Before | After   |
| ------------------------------------- | ------ | ------- |
| `evaluate/engine_stylex_functions.rs` | 16.39% | 95.08%  |
| `nodes/typescript_expression.rs`      | 0.00%  | 100.00% |
| `nodes/identifier.rs`                 | 34.78% | 100.00% |
| `nodes/optional_chain.rs`             | 79.59% | 100.00% |
| `nodes/arrow_function.rs`             | 32.41% | 90.74%  |
| `nodes/call_expression.rs`            | 31.16% | 70.03%  |
| `nodes/member_expression.rs`          | 49.10% | 72.02%  |
| `nodes/global_conversion.rs`          | 17.12% | 63.96%  |
| `nodes/template_literal.rs`           | 77.27% | 84.85%  |
| `nodes/unary_expression.rs`           | 67.33% | 76.00%  |
| `engine_fold/transport.rs`            | 39.09% | 58.18%  |
| `engine_fold/guard.rs`                | 81.54% | 84.48%  |
| `evaluate/helpers.rs`                 | 61.06% | 68.94%  |
| `evaluate/mod.rs`                     | 66.22% | 75.95%  |

Three routes had to be opened before any of it could be reached, and each is a
shape no source alone can write:

- The TypeScript grammar. `parse_ts_expr` in the scaffolding, because the six
  type-only expressions have no other spelling and the suites read ES.
- The compiler's own function fold. `evaluated_against` takes a function map the
  case built, which is the one way to reach the conversions, the callee kinds
  and the member reads still written out in Rust -- the engine declines every
  call over a value with no JavaScript form, and that decline is the route.
- The module's imports. `evaluated_in_a_state` records the import a collector
  would have recorded, which is what decides the name `firstThatWorks` is
  reachable under.

Four assertions were written to say only that something folded, and each was
made to say what it folded to before this landed. Three of the four were then
wrong: a member read off the namespace answers the entry rather than the object
form of it, an arrow the binding cannot apply refuses under its own node kind
rather than under the call's, and `~fn` is `-1` rather than `NaN` because
`ToInt32` reads `NaN` as zero. Each is the reference implementation's answer;
none would have been caught by the weaker assertion.

**What is left.** `engine_fold/transport.rs` (58.18%), `nodes/global_conversion.rs`
(63.96%), `nodes/conditional_expression.rs` (68.42%), `evaluate/helpers.rs`
(68.94%), `nodes/call_expression.rs` (70.03%), `nodes/object_expression.rs`
(70.61%), `nodes/member_expression.rs` (72.02%),
`engine_fold/amplification.rs` (74.77%), `engine_fold/theme.rs` (74.83%),
`evaluate/mod.rs` (75.95%) and `nodes/unary_expression.rs` (76.00%) hold most of
the remainder. `amplification.rs`, `theme.rs`, `object_expression.rs` and
`binary_expression.rs` are unchanged from the figures above because nothing here
touched them.

Several arms are provably unreachable and want the treatment
[ticket 11](./11-cover-the-state-crate.md) gave the state crate -- removal
rather than a test. Found so far, each measured rather than guessed:

- `nodes/conditional_expression.rs` refuses twice for a test that folded to
  nothing while confident. `evaluate_cached` deopts before it answers `None`,
  and the memo stores no confident `None`, so neither arm can be entered.
- `nodes/template_literal.rs` carries a `raw` parameter that is `false` at every
  call site. The `TaggedTpl` arm that would pass `true` is commented out.
- `nodes/member_expression.rs` refuses in `read_fold_member` for a value with no
  object form. Both call sites pass a fold, which always has one.
- `nodes/global_conversion.rs` answers each conversion's empty case. A call with
  no argument never declines in the engine, so none of the four is reached, and
  the same holds for its `NoNumberForm` arm: a function coerces to `NaN`.
- `nodes/unary_expression.rs` refuses for an operand that folded to nothing
  while confident, for the same reason the conditional does.

Each removal is a change to shipped code and is worth its own reading, so they
are recorded here rather than made alongside the tests.

**2026-09-07, second instalment.** 89.67% of regions to 91.58%, and 725
unexercised regions to 591. Four files reach the gate that had not:
`stylex_first_that_works.rs`, plus the three the first instalment left one or
two regions short.

| File                         | Before | After   |
| ---------------------------- | ------ | ------- |
| `nodes/unary_expression.rs`  | 65.49% | 88.03%  |
| `nodes/global_conversion.rs` | 69.44% | 91.67%  |
| `engine_fold/theme.rs`       | 75.51% | 89.12%  |
| `nodes/object_expression.rs` | 83.87% | 89.96%  |
| `nodes/member_expression.rs` | 81.95% | 85.74%  |
| `engine_stylex_functions.rs` | 95.08% | 98.36%  |
| `evaluate/mod.rs`            | 88.92% | 90.81%  |
| `evaluate/helpers.rs`        | 88.09% | 89.79%  |
| `nodes/template_literal.rs`  | 89.09% | 94.55%  |
| `stylex_first_that_works.rs` | 98.69% | 100.00% |
| `convertors.rs`              | 98.20% | 99.10%  |

Two routes are new here, and both are a shape no source alone can write. A
**character ceiling the case names itself**: the guard carries a bound value
against `maxFoldedCharacters`, so a case that lowers it makes the engine decline
and the Rust path answer -- which is the only way to reach the ceiling refusals
in the string conversion, the numeric conversion, the template literal and the
unary operators. And a **value read back out of a folded object**: an array or
an arrow written straight down folds to the evaluator's own form, so the arms
that read one as the literal it was written as are reached only through
`({ a: [1, 2] }).a`.

Three assertions were written to say only that something refused, and each was
made to name its sentence before this landed. One was then wrong: a
concatenation over a value with no string was reading the operand walk's refusal
rather than the coercion's, because a name resolves through the state's own
function map and the case had put its map only beside it. The other two hold.

**A shipped bug came out of it**, and is commit
`fix(evaluator): read a style function through its parentheses`. Reaching the
callback arm of the object value match needed a value the arm could not read,
and a parenthesized arrow turned out to be one: parentheses are a node in this
tree and none in the reference implementation's, so
`root: ((color) => ({ ... }))` refused where the same function without them
folded. A transform case written to prove it found the second half -- the create
argument's own dispatch matched the bare node too, and stopped the build with
`A StyleX namespace must be an object.` Both read through the parentheses now,
and the transform case compiles to the same output as the case above it.

**A wider class the same reading turns up**, recorded rather than fixed, and
worth a ticket of its own:
[47](./47-read-every-style-shape-through-its-parentheses.md).

**What is left**, and the shape of it has changed. The eleven files below hold
563 of the 591:

| File                           | Cover  | Regions left |
| ------------------------------ | ------ | ------------ |
| `nodes/call_expression.rs`     | 86.44% | 91           |
| `engine_fold/amplification.rs` | 83.18% | 90           |
| `engine_fold/guard.rs`         | 92.75% | 79           |
| `nodes/member_expression.rs`   | 85.74% | 79           |
| `evaluate/helpers.rs`          | 89.79% | 48           |
| `engine_fold/outward.rs`       | 82.04% | 37           |
| `evaluate/mod.rs`              | 90.81% | 34           |
| `engine_fold/transport.rs`     | 93.64% | 28           |
| `nodes/object_expression.rs`   | 89.96% | 28           |
| `nodes/unary_expression.rs`    | 88.03% | 17           |
| `engine_fold/theme.rs`         | 89.12% | 16           |

**The dead arms this instalment measured.** Each was probed rather than guessed,
and none is a test that can be written -- they want the treatment
[ticket 11](./11-cover-the-state-crate.md) gave the state crate. Added to the
five the first instalment recorded:

- `nodes/unary_expression.rs`. `type_of`'s `Expr::Fn` and `Expr::Class` arms and
  its `_ => None`, and so the `typeof` refusal and the `!` refusal that read
  them. No evaluated value holds a function expression or a class expression:
  the dispatch refuses both. `nodes/global_conversion.rs` already writes the
  reason down for its own `ToObject` refusal -- every expression the evaluator
  answers with is a literal, an array, an object, an arrow or a global spelled
  as a name.
- `nodes/global_conversion.rs`. The three empty-argument answers, `String()`,
  `Number()` and `Object()`, and the `Object` refusal. A call with no argument
  declines nothing, so the engine answers it and the Rust path is never entered.
  The `Array` length refusal beside them _is_ reachable, through
  `Array(String(sx).length)`, and is now covered.
- `nodes/member_expression.rs`. `fold_entry_value`'s spread arm; `refuse_lookup`'s
  `Length` and `Missing` arms, which both callers answer before calling; the
  `Expr::Array` receiver's spread and hole refusals and the `Expr::Object`
  receiver's spread and method refusals, since the evaluator builds those values
  and they carry none of the four; the `Expr::Member` and non-`undefined`
  `Expr::Ident` receiver arms; and the `parent_is_call_expr` branch, which needs
  the transform's member-call registry and so cannot be entered from this crate
  at all.
- `nodes/object_expression.rs`. The holey-array guard; the two readings for a key
  that is `None`, where every key arm answers or returns; and the "no
  compile-time value while confident" refusal, since `evaluate_with_functions`
  clears confidence whenever it answers nothing.
- `evaluate/mod.rs`. The two `stylex_panic_with_context!` guards -- a
  parenthesized expression is normalized before the dispatch reaches them.
- `engine_fold/theme.rs`. `unbuilt` and its two callers, which guard a constant
  script; and `derive`'s missing-argument reading of a flag, since the key it
  reads first sits at a later index and so cannot be present without them.
- `evaluate/helpers.rs`. Every `EvaluateResultValue::Null` arm, each already
  documented in place as unreachable.

**2026-09-07, third instalment.** 91.58% of regions to 93.26%, and 591
unexercised regions to 464. Three files reach the gate:
`nodes/member_expression.rs` (from 85.74%), `nodes/object_expression.rs` (from
89.96%) and `nodes/unary_expression.rs` (from 88.03%). Eighteen of the crate's
thirty-six files are now at the gate.

Most of this is removal rather than tests, and the removals rest on one reading:
**an array or an object a fold hands back is one the evaluator wrote**, so it
carries no hole, no spread, no getter and no shorthand. Every producer was
enumerated to say so, the `env` option's napi path included, which writes
`spread: None` per element and key-value props alone. The member read had been
testing for all four on every lookup and copying each property to do it; it now
reads the value as it is, and a deep copy of one property per property scanned
is gone with the checks.

**The correction that matters, and it overturns the second instalment.** That
comment recorded `EvaluateResultValue::Null` as unreachable and listed the
bridge arms that answer it among the dead. That is wrong, and the route is the
memo:

- A `+` whose left side is another `+` folds that side through `folded_once`.
- `fold_binary_expr` answers `Null` on an `anyhow` error, and neither error
  calls `deopt` -- so the fold answers _nothing while the walk is still
  confident_.
- `memoized_fold` stores that as `resolved: true, value: None`, and every later
  read of the same subtree is handed it back with confidence intact.

So `(() => 1) + 1 + 2` leaves the memo answering nothing for `(() => 1) + 1`,
and `[(() => 1) + 1]` then folds -- confidently -- to an array holding
`EvaluateResultValue::Null`. **`evaluate_cached` can answer `None` while the
state is confident**, which is the invariant three of this instalment's first
attempts had assumed away. Each was reverted:

- `nodes/member_expression.rs` had replaced a computed key's guard with `?`.
  That turned `Property not found` into `Unsupported expression:
MemberExpression` for a reachable input.
- `nodes/object_expression.rs` had merged a property value's two guards. That
  turned `Value of key 'k' has no compile-time value, but got BinaryExpression`
  into `k > unknown error`.
- `nodes/unary_expression.rs` had answered `!absent` as `true` and
  `typeof absent` as `"undefined"`. Both would have been confidently wrong: the
  value means _nothing resolved it_, not _it is not there_, which is exactly
  what `evaluate/helpers.rs` says when it refuses one.

All three are guards again, and all three are now covered rather than dead --
`UNRESOLVED_MEMO_WARM` and `evaluated_after` in the scaffolding are what a case
warms the memo with. The one place the reading did settle is `typeof`: it now
gives the sentence `!` gives for the same value, since both refuse an operand
with no reading at all rather than a shape either could name.

**What is left**, 464 regions across eighteen files. The five that hold 366 of
them:

| File                           | Cover  | Regions left |
| ------------------------------ | ------ | ------------ |
| `engine_fold/amplification.rs` | 83.18% | 90           |
| `nodes/call_expression.rs`     | 86.53% | 90           |
| `engine_fold/guard.rs`         | 92.75% | 79           |
| `evaluate/helpers.rs`          | 90.21% | 46           |
| `engine_fold/outward.rs`       | 82.04% | 37           |

The memo route above is the tool the next instalment wants: `evaluate/helpers.rs`
holds the three `Null` arms this comment used to call dead, and each is now
reachable through it.

**2026-09-07, fourth instalment.** One file, `engine_fold/amplification.rs`,
which the third instalment's table listed first at 83.18% with 90 unexercised
regions. It moves to 98.27% and 9, and to **zero uncovered lines**. The crate
moves 93.26% to 94.48% of regions, and 464 unexercised regions to 379.

| File                           | Before | After  | Regions left |
| ------------------------------ | ------ | ------ | ------------ |
| `engine_fold/amplification.rs` | 83.18% | 98.27% | 9            |

Twenty-three cases land in one new suite, `tests/amplification_reading_tests.rs`.
Its subject is the reading _under_ the guard's arithmetic: before the guard can
multiply anything it has to see a count, a width and a magnitude, and each of
those is seen a different way.

**One route had to be opened, and it is a shape no source alone can write.** An
array written straight down folds to the evaluator's own list, so the arm that
reads a receiver as the _literal it was written as_ is reached only through a
value that carries one — `({ a: [1, 2] }).a`, where the member read answers the
property's own node. That one shape reaches the written-length count, both
written-element widths and the written magnitude, which is most of the file's
remainder. It is the same route the second instalment opened for the member
read, used here for the guard.

**Five assertions were written to say only that something folded or refused, and
each was made to name its answer before this landed. Four of the five were then
wrong**, and each of the four is the reference implementation's answer:

- `Array.from(({ a: 1 }).a, () => 'ab'.repeat(3))` refuses under the
  _callback's_ sentence, not the count's. The count was read; what could not be
  read is how many times the body runs.
- A count leaf below zero refuses under the _count's_ sentence, not the
  callback's — no count is read at all, so the callback rule is never reached.
- `'ab'.repeat(-1)` is not refused by the guard. `-1` is a unary expression, so
  the count resolves through `ToNumber` and truncates to zero, a repeat of none
  is admitted, and what the author reads is the language's own
  `RangeError: String.prototype.repeat: count must be non-negative`. The count
  the guard rejects for being negative is one it has to do _arithmetic_ with,
  which is only reachable inside a callback.
- `Array.from({ length: -1 })` folds to the empty array rather than refusing,
  because that is what the language answers for a length it will not accept.

**The removals, and the one reading they rest on.** All 21 regions the tests
could not reach were dead by a single invariant, and it is the one the third
instalment settled: **an array or an object a fold resolves is one the evaluator
wrote.** `evaluate_result_vec_to_array_expr` is the only producer of an
`Expr::Array` this guard can be handed, and it writes one present element per
item and no spread; `object_expression.rs` rebuilds every key as an identifier.
So:

- `measured_receiver`'s spread scan over the written elements is gone. It was a
  third full pass over every array-literal receiver, guarding a shape that
  cannot arrive.
- `rendered_element` and `number_written_as` are gone, replaced by one
  `WrittenArray` the two units read a receiver through. Their hole and spread
  arms were the dead ones, and the count now comes off the same value the
  elements do rather than off a separate read of the slot list.
- `is_a_length_key` reads one spelling rather than three, because `length` is a
  valid identifier and every factory here spells such a key as one. The quoted
  spelling is still a case, and a case for a key that is _not_ `length` is what
  fails the day the reading widens.
- `declared_length_of`'s property walk is one match rather than two nested ones,
  so the spread arm no caller can enter is gone.

The deleted comment on `rendered_element` had argued the other way — that a dead
arm's reading is what a branch would be admitting on the day the rule is
relaxed. That argument is answered rather than dropped: the reading now lives on
the term "Evaluator-written array" in `CONTEXT.md`, and `WrittenArray` says
plainly that nothing re-checks the invariant and that an array arriving with a
hole or a spread is a fault in whichever producer built it. Recorded here
because it reverses a written decision.

**The nine regions left**, each measured rather than guessed. All nine are a `?`
that never short-circuits, and none is a line — the file reads 100% of lines.

- `count_bound`, twice. `to_js_number` of a written literal, and of the
  expression a named count resolved to. Every literal the guard reaches has a
  compile-time number: a BigInt and a regular expression are the only two
  without one, and the dispatch refuses both before the guard sees them. Every
  _value_ without one is a function, which has no expression form and fails the
  first `?` instead — which is covered.
- `numeric_bound`'s resolved leaf, for the same reason as the second above.
- `bounds_of`, twice — in `numeric_bound` and in `receiver_length`. A name the
  scope binds always has a bounds entry, because binding and measuring happen
  together.
- `Repeats::counted` on the array rule. `Unmeasured` reaches the string rule and
  is covered there; whether it can reach the array rule is not yet probed.
- `Depth::descend`, twice, in `rendered_characters` and `rendered_expr`. A
  receiver nested past what is left of the ceiling. Reachable in principle and
  not yet reached: two attempts refused at the evaluator's own nesting ceiling
  before the guard measured anything, so the case wants the call nested _inside_
  levels rather than the receiver nested deeply.

**What the review corrected, and it is worth its own line.** The first reading
of the invariant was written as "the evaluator rebuilds every key as an
identifier". That is **false** of this reader's input: a spread hands its
source's props over as written, and the function-fold and `env` bridges build
theirs with `create_key_value_prop`, so a key like `"max-width"` does reach
`declared_length_of` as a `PropName::Str`. The narrowing is safe all the same,
for a narrower reason: `length` is a valid identifier, so
`convert_string_to_prop_name` spells that one key as an `Ident` whatever wrote
it. The comment says the narrower thing now. The arrays half of the invariant
was checked the same way and holds -- an author's array literal folds to the
list arm, one carrying a hole or a spread refuses first, and the `env` bridge
writes present elements only -- but it has two producers rather than the one the
first reading named.

**The performance reading**, since this touches a guard that runs on every call
the fold considers. Passes over the elements are unchanged at two, the spread
pre-scan removed a third that ran unconditionally, `impl Iterator` monomorphises
into the same `try_fold` the closures did, and the two narrowed matches each drop
a branch. No allocation and no clone on either side. `EngineFoldRoundTrip`'s
`callback` leg is the only bench whose receiver reaches the changed arm, and it
trips at about 1.11x against 16--34% cross-run noise, so a few nanoseconds over a
four-element array is not something it can resolve.

**2026-09-07, fourth instalment, review follow-up.** Eight findings from the
review of the two commits above, each read and then fixed or answered. No
behaviour changed and the figures are unmoved -- `amplification.rs` at 98.29% of
regions and 100% of lines, the crate at 94.48%, 838 crate cases and 8814
workspace cases green.

**The invariant now has one home.** It was written out three times and two
copies had already come to disagree about how many producers it holds of. It is
a domain term, so it went to the crate's `CONTEXT.md` as **Evaluator-written
array**, beside **Written slot**, which is the other value class and the thing
readers were confusing it with. `evaluate_result_vec_to_array_expr` states the
guarantee it upholds, the two readers point at the term, and nothing restates
the whole contract any more.

**Two claims of mine were false and are corrected.** Both are the same defect,
one file apart:

- `nodes/member_expression.rs` said `evaluate_result_vec_to_array_expr` "is the
  one thing that builds it". The `env` option's napi bridge builds one too. This
  was pre-existing, from the third instalment, and the fourth's own comment had
  started pointing at it while stating the corrected version -- so the
  cross-reference led to the stale wording.
- The producer's own doc then said `CONTEXT.md` "is where the term and its
  readers are listed". The entry lists producers, not readers. It also named a
  count of readers that nothing keeps true. Both clauses are gone.

**`WrittenArray` replaces `written_elements`.** The count came from
`elems.len()` while the widths came from a separate call over `elems`, at three
sites; the two agree only under the invariant, which the doc concedes nothing
re-checks. They now come off one value, so pairing a count with a _different_
array's elements is unrepresentable. It is a `Copy` newtype over the slice and
both methods take it by value, so neither reading holds the other up. What it
does not do is enforce the invariant -- `slots()` is still `len()` -- and the
doc says so rather than claiming otherwise.

**One test was coverage rather than a guard, and now it is both.**
`a_key_that_is_not_length_declares_nothing` read
`Array.from({ length: 2, 0: 'x' }).length` as 2, which passes on `length: 2`
alone. Rewriting it to `{ 0: 'x' }` reached the false branch but still pinned
nothing: `is_a_length_key` only feeds `Declared`, which gates a _refusal_, so
the fold answers the same number either way. The count is now two million,
past the entry ceiling, and that is what tells the two readings apart -- a key
wrongly read as `length` declares two million elements and is refused, where the
same count under a key that is not `length` declares nothing and folds. Proved
by widening `is_a_length_key` to match every `PropName` and watching the case
fail, then restoring it. A smaller count pins nothing, and the case says so.

**Register and mechanics.** A doc line had reached 117 characters, which
`cargo fmt` cannot catch here because `wrap_comments` is nightly-only and the
check warns and exits zero. Markdown bold had been written inside a `//`
comment, where nothing renders it. The two long docs are cut roughly in half:
what is kept is the contract, the counter-example and the case that guards it,
and what went is the argument about them. `is_a_length_key` names its guarding
case again -- the review was right that dropping it lost the one load-bearing
line in that paragraph.

**Answered rather than changed.** The `See [written slot](#written-slot)` anchor
resolves to a bold term rather than a heading, which is true of all sixteen
anchors already in that file, so it follows the local convention. And the
`elems.len()` in `nodes/member_expression.rs` stays a plain read: that file
indexes a slot where this one iterates, so it wants a different reading, and it
is already at the gate.

**2026-09-07, fifth instalment.** One file, `nodes/call_expression.rs`, which
the third instalment's table listed first at 86.53% with 90 unexercised regions.
It reaches **100% of regions and 100% of lines**. The crate moves 94.48% to
95.84% of regions, and 379 unexercised regions to 284. Twenty-one of the
thirty-six files are at the gate, from twenty.

| File                       | Before | After   | Regions left |
| -------------------------- | ------ | ------- | ------------ |
| `nodes/call_expression.rs` | 86.53% | 100.00% | 0            |

Eleven cases land: seven in one new suite,
`tests/declined_call_receiver_tests.rs`, whose subject is how the dispatch below
the engine reads a receiver, and four beside the suites that already own the
callee shapes.

**Two routes had to be opened.** Both are a shape no source alone can write.

- **The memo, warmed through the parentheses the receiver carries.** A receiver
  with no value at all while the walk stays confident is the memo's answer, and
  the memo is keyed by the subtree as written -- a parenthesised expression is a
  node in this tree, so `(() => 1) + 1 + 2` warms nothing that
  `((() => 1) + 1).foo()` reads. `((() => 1) + 1) + 2` does, and that one warm
  reaches both the named and the computed reading of an absent receiver.
- **A property beside the receiver's own.** An object whose method the dispatch
  is to read has to reach the dispatch at all, which means the engine must
  decline the call -- so the case gives the object a second property holding one
  of the compiler's own functions, which has no JavaScript form. The method
  under the called key then answers, and it answers with a value rather than a
  refusal.

**A quoted key is the other new reading**, and it is one class of value rather
than one arm. An object the evaluator wrote spells its own keys as identifiers,
but two things build a key from a _name_ instead -- `function_fold_to_object`,
from an entry name, and the `env` option's napi bridge, from a JavaScript
property name -- and both go through `create_key_value_prop`, which quotes a
name no identifier can spell. So `{ ...sx }` over a namespace holding `a-b` is
the one way a property of an evaluated object arrives quoted, and it reaches
both readers of such a key: the method lookup and the type function's entry
walk.

**Four assertions were written to say only that something refused, and each was
made to name its sentence before this landed. Two of the four were then wrong**,
and both had been passing for a reason the case did not name:

- `/re/.test(x)` reads `Unsupported expression: RegExpLiteral` from the
  _literal_ and not from the call. The dispatch folds no regular expression in
  any position, so the receiver never has a value for a method lookup to read,
  and the arm written for a regex receiver had never been entered. The case says
  where the sentence comes from now.
- A spread argument to a _method_ never reaches this dispatch at all. The engine
  walks the arguments of every call it looks at and refuses a spread there,
  whether or not it goes on to read the method -- proved by instrumenting the
  argument walk, where `` `ab`.nope(...['a']) `` scores no hits. An applied
  arrow is the one callee whose spread the dispatch does see, and that is the
  case that stayed.

**The removals, and the readings they rest on.** All fourteen arms the tests
could not reach were dead, each probed rather than argued:

- **A question asked twice.** `object.is_ident()` then `object.as_ident()`, and
  the same for the property at two levels. Read as the name it is, and the arm
  for the answer already ruled out is gone -- with it the one
  `stylex_unreachable!` in the file.
- **A refusal with no reason.** `deopt` is the only thing in the crate that
  clears confidence, and it writes the reason as it does so, so
  `reason.is_some()` and `!confident` say the same thing. The receiver's refusal
  is raised off the reason now, and the `UNDEFINED_CONST` arm for a refusal
  carrying none is gone.
- **A receiver carried as an entries map.** Nothing in this crate constructs
  `EvaluateResultValue::Map` -- `stylex-transform`'s
  `evaluate_stylex_create_arg` is the only producer, and it answers one rather
  than storing it anywhere the evaluator reads. So the named arm, the computed
  key's `as_map` read and `map_method` itself were three readings of a value no
  receiver can hold. The computed reading is now the refusal it always gave.
- **The own-keys question's argument.** The no-argument and the spread guards
  both stand where the engine has already answered: a call with no argument
  holds no value the engine could decline over, so the language's own
  `TypeError` is what an author reads, and a spread is refused where it is
  written. Probed five ways, a shadowed `Object` included. What is left reads
  the first plain argument and names no callee without one -- and
  `a_static_the_language_throws_on_refuses_with_what_it_threw` now names the
  thrown sentence, which is what tells that removal from a regression.
- **An evaluated object's properties.** The type function's spread and
  non-key-value guards, and the same two inside the object receiver's method
  find. This is the reading the third and fourth instalments settled, stated for
  objects.
- **A short-circuit only a spread can take.** `evaluate_func_call_args` answers
  `None` for one thing, a spread argument, and no spread reaches the declined
  method path -- so the `?` there never short-circuited. Dropping it changes
  nothing even if it were reached: a refusal already in the state is not written
  over, so the sentence is the same either way.

**The invariant has one home**, as the fourth instalment's did. It went to
`CONTEXT.md` as **Evaluator-written object**, beside **Evaluator-written
array**, and `written_key_values` is the one reader both positions ask through.
What the term promises is the _kind_ of every property; what it deliberately
does not promise is the spelling of a key -- the half the fourth instalment's
review had to correct for arrays, and the half that is load-bearing here, since
a quoted key is a case rather than a fault.

**The performance reading.** The object receiver's method lookup read the whole
object out through `get_key_values_from_object` first, to then find one property
in it. That helper allocates a vector and makes two deep clones of every
property -- one to expand a shorthand, one to keep the pair -- before a key is
compared. It scans in place now and clones the one value it found, so a receiver
of N properties costs one clone rather than 2N and a vector. The deleted `Map`
arms drop their own share: each built a fresh key expression, one of them a
`String` too, only to look one up. All of it is on the path of every declined
method call written over an object. Nothing else moved -- the argument walk is
unchanged and the two narrowed matches each drop a branch -- and no bench here
reaches the changed arm, since `evaluate_bench`'s fixtures write no declined
method call on an object.

**What the review corrected.** Three things, none of them behaviour. The
written-object walk had been inlined at both readers, which is the shape the
fourth instalment's review already called out once; it is `written_key_values`
now. The own-keys receiver work sat inside a closure that buried it, and is
`own_keys_callee`. And two comments described the code they replaced rather than
the code that is there -- along with a `CONTEXT.md` link written as a rustdoc
path inside a `//` comment, where nothing renders it, which is the same register
finding the fourth instalment recorded.

**2026-09-07, fifth instalment, review follow-up.** Nine findings from three
reviews of the two commits above, each read and then fixed or answered. No
behaviour changed and the figures are unmoved -- `call_expression.rs` at 100% of
regions and lines, the crate at 95.84%, 847 crate cases and the workspace suite
green.

**Two claims of mine were stated one scope too wide.** Both are the same defect,
and it is the one the fourth instalment's review recorded against its own
predecessor:

- The comment justifying the removed `UNDEFINED_CONST` arm said `deopt` is the
  one thing in the crate that clears confidence. That is false of
  `EvaluateResult` as a type. `EvaluateResult::refused` builds a refusal that
  carries no reason, and the transform calls it that way twice; a forked state
  in `logical_expression` keeps its parent's reason while holding confidence.
  The removal is still right, for the narrower reason the comment now gives:
  `evaluate_with_functions` hands the receiver a fresh state, and this reader
  sees no other producer.
- The `written_key_values` doc said a quoted key is the half "both readers here
  do have to answer for". Only the type function answers for one. The method
  lookup passes one over and could not do otherwise, which is the finding below.

**A reader's rule turned out to be unpinnable, and that is worth writing down.**
`a_key_the_receiver_spells_with_quotes_...` had a doc naming a mechanism the
case did not test. Trying to strengthen it established something better: **no
input can tell the method lookup's key reading apart.** The name it compares
against comes from a dot, so it is always an identifier, and a quoted key can
never equal one -- reading the key as an identifier and comparing its text give
the same answer for every input. Proved by mutation: widening the match to any
identifier key still passes. What the case _does_ pin is the shared walk, proved
the same way -- making the walk yield nothing fails it. So the case is renamed
for the one thing it pins, and the reading is documented as a cheap way to get a
comparable name rather than a rule.

**One finding was a reachable behaviour change, and it was not.** A review read
`is_valid_callee` as matching a name alone and concluded that a module-shadowed
`Object` with no argument reaches the removed guard, changing
`Cannot fold 'keys' at compile time.` into the terminal refusal. A second review
carried it
further, to an _unresolvable_ `var Object`. Neither is reachable, and the second
is the more interesting miss: the receiver's global is checked before the
argument walk, so a declarator shadows and the engine **refuses** rather than
declines. Probed across four initialised declarator forms, an uninitialised one,
all three own-keys spellings and a non-question static -- twelve inputs, none of
which declines. Only a decline reaches this dispatch.

The comment now says that rather than asserting the conclusion, and it names
where each of the two outcomes is pinned: the engine's own `TypeError` in
`a_static_the_language_throws_on_refuses_with_what_it_threw`, widened here to
all three spellings, and the shadowed receiver in `shadowed_names_tests`, which
already owns that seam.

**Register and mechanics.** A doc line was ungrammatical. The glossary entry and
the helper doc had come to list different things, and neither matched the code
-- an assignment pattern is refused, and a shorthand is _expanded_ rather than
refused, which is why it never arrives. Both now use the vocabulary
`object_expression` already uses for the same set. The new suite hand-rolled a
namespace map the scaffolding already builds, and keyed a second copy on a
literal where a constant holds the same string.

**Answered rather than changed.** The mixed dash style in `call_expression.rs`
is pre-existing -- seven such lines predate this change and the new doc sits
among them, so normalising would mean editing untouched code. And
`let _ = evaluate_func_call_args(...)` stays: the `match` that would
"handle all cases"
adds an arm no input can enter, and this crate's gate rejects an uncovered
region. The call is made for its effect on the state, and its answer is
genuinely unused.

**2026-09-07, sixth instalment.** One file, `engine_fold/guard.rs`, which the
third instalment's table listed at 92.75% with 79 unexercised regions and which
re-measures here at 93.02% with 76. It moves to **99.72% of regions and 99.53%
of lines**, and 3 regions. The crate moves 95.84% to 96.92% of regions, and 284
unexercised regions to 210.

| File                   | Before | After  | Regions left |
| ---------------------- | ------ | ------ | ------------ |
| `engine_fold/guard.rs` | 93.02% | 99.72% | 3            |

Nineteen cases land: seventeen in one new suite,
`tests/guarded_walk_tests.rs`, whose subject is what the walk reads before the
engine sees anything, and two beside the suite that already owns which name
reaches `firstThatWorks`.

**The route that opened most of it is the depth ceiling itself.** The guard
refuses a walk that descends past `maxEvaluationDepth`, and every `?` on that
walk -- nineteen of the seventy-six regions -- is a decline no _shape_ can
raise: a pattern declines only for depth, and so does a block. So a case names
the ceiling it is about, writes the smallest shape that crosses it, and puts a
plain callback beside it at the same ceiling. That last half is what makes the
case say something: without it a refusal proves only that the ceiling is low.
Measured rather than reasoned, one ceiling at a time -- an array pattern, an
object key-value, an object rest, an array rest and a defaulted pattern each
cost a different number of levels, and a block costs two where the statement
inside it costs one.

**The other route is the statements a callback body may hold.** `return` was the
only one any suite wrote, so the four arms beside it -- a `var` declaration, a
nested block, an `if` with and without an alternative, and a stray semicolon --
had never been entered. Each folds to the value it computes, and the `if` is
written as three folds with three different answers rather than as three
refusals: a case that only proved the shape was admitted would pass on an engine
that ran the wrong branch.

**Every case names the value or the sentence it expects rather than only that
something folded**, which is what the fourth and fifth instalments each found to
be the difference between a case and a coverage line. The one answer worth
recording is `['a'].map(x => { return; }).join('')`, which is the empty string
and not `undefined`: a `return` with no argument answers `undefined`, and
`Array.prototype.join` writes an absent element as nothing. That is the
reference implementation's answer, and the suite next door already pins the
same rule for `['a', undefined, 'b'].join('-')`.

**The removals, and the readings they rest on.** Six of the seventy-six were
dead, each probed rather than argued:

- **A question asked twice, three times over.** `PropOrSpread::Prop` then
  `PropOrSpread::Spread`, where the type has two variants. `Expr::Ident` then
  `is_theme_ref_base`, which is that same question spelled as a predicate. And
  `MemberProp::PrivateName`, which is written only inside a class body -- the
  member walk now reads the one property shape that is a value, a computed key,
  and passes the other two over.
- **A rule asked of the wrong thing.** `a_global_written_as_a_value` unwrapped
  parentheses and asked for a name. Its one caller stands in the walk's
  `Expr::Ident` arm, and the walk unwraps a parenthesis before it dispatches, so
  both readings were of a name it already held. It takes the name now, and reads
  the two constant sets a value position claims rather than the callee
  predicate, which wanted an expression to be asked of. The rule that
  parentheses change nothing is pinned in source instead --
  `['a'].concat(((String)))` refuses with the same sentence as `String` alone.
- **A refusal for a declaration that is always there.** `admit_a_named_function`
  refused a name it could not find an initialiser for. It is entered only where
  the evaluator answered a _callback_ for the name, which it can only do by
  reading that name's declaration. The declaration is read once in the caller
  now, beside the reading that decides whether a name resolving to nothing was a
  function -- both start from the same place, and a callback with no declaration
  falls to the second, which finds no function either and hands the call back.
- **A kind carrying a name three ways.** `Admitted` was an enum whose three
  variants all carried an `Atom`, and only one of them is ever told from the
  others. The name is a field now and the kind is beside it, so the reader that
  wants the name no longer matches on a kind it does not care about -- and the
  `Named` arm, which the outermost call can never reach because
  `admit_a_named_call` declines there, is gone with the match.

**The three regions left**, each measured rather than guessed, and each a shape
no input can build:

- `Scope::bounds_of`'s `Scope::Module` answer. Both callers ask
  `Scope::binds` first, and a name some scope binds is found in that scope --
  so the recursion never reaches the module. Reachable if the two questions
  become one, which is a change in `amplification.rs` and wants its own reading.
- `Bindings::nested_pattern`'s `Pat::Expr | Pat::Invalid` arm. `[o.a] = …`
  assigns through a member and an invalid pattern is a parse failure; the parser
  writes neither where a parameter or a declarator binds, so the arm exists for
  the type. It cannot be removed -- the match would stop being exhaustive.
- `admit_an_applied_global`'s `read(…)?`. Reading a plain property off the
  engine's global object throws nothing.

**The performance reading**, since the walk runs in front of every call the fold
considers. Nothing allocates that did not, and four readings get cheaper: the
name a call was admitted under is a field read where it was a match; the member
property walk and the object property walk each drop a branch; the dotted theme
read drops one `matches!` per chain; and a bare name written as a value no longer
walks a parenthesis chain that never had a link, nor matches an expression it
already holds the name of. The declaration behind a name is read for a name
holding a **function** alone, and only after the transport is asked whether it
already carries one -- so a name that resolved to nothing, and a name whose value
the bridge cannot carry, own nothing and read the binding once, as before.

**What the review corrected, and it is the reason the paragraph above reads as it
does.** The first shape of the moved declaration read put it in a tuple beside
the resolved value, which evaluates before either arm is chosen: every name that
resolved to nothing and every name holding an uncarryable value then deep-cloned
its whole initialiser and dropped it unread, and a name written twice cloned
twice because the clone had moved in front of the transport's own question. Both
are on the walk's most-taken path. The read is gated on the answer being a
function now, the transport is asked first, and `admit_a_named_function` takes
the declaration the caller already owns.

**Three further findings, each answered by sharing what had been written twice.**
Two removals had traded one dead region for a comment that nothing checks, which
is the drift the docs beside them had warned against:

- The value-position rule stopped reading the callee set through the callee
  rule's own predicate. `is_valid_callee` is two functions now, one taking an
  expression and one taking a name, and both rules read the set through the
  second. This is the one change outside the evaluator crate --
  `stylex-js/src/helpers.rs`, with a case of its own that asserts the two
  readings agree.
- The dotted theme read stopped calling `is_theme_ref_base` and named it in a
  comment instead, so widening that rule would have left the guard behind. It is
  `theme_ref_base` now, which answers the base's _name_, and `is_theme_ref_base`
  is that answer's presence -- one reading, and the guard reads the name off it.
- `Admitted` had one field behind a getter and one bare, which said nothing.
  Both are read the same way.

**Four cases said less than their own docs claimed**, and each now names what it
was already reading: the amplification sentence a name a block declares refuses
under; the engine's own `not a callable function` where a callback parameter
shadows one of the compiler's functions; `Referenced constant is not defined.`
where such a call's argument is handed back outside a callback, which had been
written as an inequality against the sentence it is _not_; and, for a chain read
off a group, the three variables three depths of the same path answer, where the
case had only asked that each start with `var(--x`.

**2026-09-07, seventh instalment.** One file, `evaluate/helpers.rs`, which the
third instalment's table listed at 90.21% with 46 unexercised regions and which
re-measures here at 40. It reaches **zero uncovered regions and zero uncovered
lines**. The crate moves 96.92% to 97.44% of regions, and 209 unexercised
regions -- 210 by the sixth instalment's count -- to 169.

| File                  | Regions left before | Regions left after |
| --------------------- | ------------------- | ------------------ |
| `evaluate/helpers.rs` | 40                  | 0                  |

Eleven cases land: nine beside the suite that already owns the own-keys walk,
and two in the file's own suite, which is where a coercion's classification
table is read.

**The subject is one reader with three receivers nothing asked it about.**
`Object.keys`, `Object.values` and `Object.entries` reach this compiler's own
walk only where the _engine_ declined the call, and the walk then reads the
receiver three ways: out of the array literal it was written as, out of the
evaluated value, and out of the compiler's own function fold. Most of the hole
was the second and the third.

**Two routes had to be opened.** Both are a shape no source alone can write.

- **The memo, behind a declined call.** A value that resolved to nothing while
  the walk stayed confident is the memo's answer, and only a declined call
  reaches this reader -- so a case needs both doors at once, which
  `evaluated_after_against` in the scaffolding is. That one shape reaches every
  reading of an element with no value: at the top of an array, inside a nested
  one, and one level below that, where the three answer differently.
- **A third level of nesting.** An array of an array of an array is the one
  input that reads a nested array _out of_ another nested array, which is most
  of the nested reader's remainder.

**Every case names the count or the key it expects.** Three name the key rather
than the count alone, because dropping one element of two leaves one either way,
so a count would pass on a walk that kept the wrong one.

**Where this walk parts from the reference implementation**, measured against it
one shape at a time with `.scratch/engine-fold-reads-values/babel.mjs`. An array
literal read from the syntax is read past anything the walk cannot fold -- a
hole, an element that refused, an element that resolved to nothing -- where the
reference implementation refuses the declaration; and an element with no
compile-time form goes the other way, refusing here and folding there. Two
readings do agree and are now pinned as agreeing: a three-level array answers
one key holding all three levels, and a function receiver answers the empty
list. The parting has a ticket of its own,
[48](./48-read-an-array-receiver-as-the-reference-does.md), because one half of
it writes a _shorter array_ than the source describes rather than only refusing
or folding where the other compiler does not.

**The removals, and the readings they rest on.** Each was probed by making the
arm panic and running the whole workspace suite, the transform's file corpus
included:

- **A question the object walk already answered.** `own_keys` refused a spread
  and refused a property that is not a key and a value. Both stand behind an
  [evaluator-written object](../../../crates/stylex-evaluator/CONTEXT.md#evaluator-written-object):
  every receiver it reads is an evaluated value or is built from one, and an
  object literal an author wrote is evaluated before any of them sees it. The
  walk reads `written_key_values` now, which the fifth instalment wrote for the
  two readers in `call_expression.rs` and which has moved beside the value class
  it reads. One reader for three positions.
  `a_property_that_is_not_a_value_refuses_the_receiver` had been passing on that
  sentence coming from the _object walk_, which is where a getter is refused,
  and its doc says so now.
- **An absent value the caller has already passed over.**
  `normalize_js_object_method_nested_vector_arg` checked its entries for the
  absent value. Its one caller passes over a nested array holding one before it
  calls, so the check could not be entered.
- **A binding asked of the wrong thing.** `get_binding` took an expression, to
  match one arm of and hand the name to `declaration_of`. Both callers stand in
  the guard's `Expr::Ident` arm and already hold the name, so the arm for
  everything else could not be entered and the function was a pass-through. It
  is gone, `initializer_of` takes the name, and `the_module_declares_a_function`
  drops the parameter it was being passed the same name twice under. This is the
  reading the sixth instalment gave `a_global_written_as_a_value`, applied to
  the other two questions on that walk.

**One removal was wrong and the review caught it, which is worth its own line.**
The array-literal reader's own absent-value arm was removed on the claim that
"an evaluation answers `None` for a value that is not there rather than the
absent-value variant". That is one scope too wide, and it is the same defect the
third instalment reverted three times. An **index read** answers the variant
itself: it clones the slot it found out of the array it read, and an array holds
the absent value where an element folded to nothing. So
`Object.keys([, [(() => 1) + 1][0]])` folded to no keys before the removal and
refused with `A style array value can only contain strings or numbers.` after
it. Probed, reverted, and now covered by
`an_element_read_out_of_an_array_as_nothing_carries_no_key`.

**The `defineVars` group's string is pinned against the reference
implementation** rather than against this compiler recomputing it: the group
answers `xop34xu` for `vars.stylex.js` and `vars`, which is what
`getVarGroupHash` answers for the same two names, read out of that compiler. Its
_object_ coercion is still an object, which is the one place the two bridges
answer differently on purpose -- the language reads `String(group)` through the
`toString` trap and `typeof group` through the kind.

**The performance reading**, since the guard runs in front of every call the
fold considers. Nothing on that walk allocates that did not, and a name's
declaration is now looked up without the match that asked whether the expression
was a name, at both sites. In the own-keys walk the property loop drops two
guards per property, and the nested reader drops one `matches!` per entry. No
new allocation and no new clone anywhere. The one cost is that `own_keys` no
longer stops at the first property it cannot read, so it walks the whole object;
the property it would have stopped at cannot arrive, and keeping the bail-out
would mean keeping a second copy of the walk.

**The one thing zero does not yet mean.** `helpers.rs` is at zero in the merged
view `scripts/coverage-missing.sh` reports, and llvm-cov's own
`--show-missing-lines` lists the file nowhere. llvm-cov's _summary_ still counts
5 regions and 2 lines against it, and all seven are per-instantiation gaps in
`write_string_of`, which is generic over its sink and has 65 instantiations of
which 62 are covered. The gate reads the summary, so this crate cannot come off
the exemption list on merged zeroes alone. Recorded because it is the first file
here to reach merged zero and find the number still above it, and whoever closes
this ticket has to answer for it.

**What is left**, 169 regions across fourteen files. The five that hold 130 of
them:

| File                       | Cover  | Regions left |
| -------------------------- | ------ | ------------ |
| `engine_fold/outward.rs`   | 82.04% | 37           |
| `evaluate/mod.rs`          | 90.81% | 34           |
| `engine_fold/transport.rs` | 93.64% | 28           |
| `engine_fold/theme.rs`     | 89.12% | 16           |
| `engine_fold/engine.rs`    | 89.58% | 15           |

**2026-09-07, eighth instalment.** One file, `engine_fold/outward.rs`, which the
seventh instalment's table listed first at 82.04% with 37 unexercised regions.
It moves to 94.92% of regions and 12. The crate moves 97.44% to 97.86% of
regions, and 169 unexercised regions to 146. `engine_fold/engine.rs` moves with
it, 89.58% to 91.67%, because a throw raised while an answer is carried out is
its route too.

| File                     | Before | After  | Regions left |
| ------------------------ | ------ | ------ | ------------ |
| `engine_fold/outward.rs` | 82.04% | 94.92% | 12           |

Nine cases land: seven in one new suite, `tests/folded_answer_tests.rs`, whose
subject is the two allocation ceilings on the way back, and two beside the suite
that already owns a theme group inside the engine.

**The subject is a bound nothing on the way in can stand for.** The guard bounds
what one written call may be _asked_ to build and the transport bounds what a
resolved name carries in. Neither sees `['abcd', 'efgh'].map((s) => s)`, which
names no value at all: both strings are the engine's own, and the engine aliases
where this side copies. So the counting is here, and only the way in had cases.

**Three readings had no case anywhere**, and each is a shape a plain source does
not reach:

- **A running total.** Four characters each and seven allowed. A check per value
  waves both through; the total refuses the pair, and the same pair at eight
  folds. That pairing is what says the number being read is the sum.
- **A key.** The only string of `Object.fromEntries([['abcdefgh', 1]])` is its
  key, since a number costs no characters -- so a ceiling the key alone passes
  is what tells the key's counter from the value's, which write the same
  sentence.
- **A getter that runs while the answer is carried out.** `Object.create` is the
  one way to write one: the call never reads the object it builds, so the throw
  is raised by the read on the way back and nowhere else.

**A theme group inside an answer had a case that never reached this file.** It
joined inside the engine, so what crossed back was one string the language
wrote. The two new cases carry the array and the object out whole, which is what
reaches the group reading here -- the one exotic object a fold can produce.

**Every case names the value or the sentence it expects.** No assertion here was
wrong before it landed, which is the first instalment of the five that has read
that way. What the ceiling cases did need is the companion inside the bound: a
refusal at every size says nothing about the ceiling, and each of the five names
the size that still folds.

**Measured against the reference implementation**, one shape at a time through
`.scratch/engine-fold-reads-values/babel.mjs`. `'abcdefgh'.toUpperCase()`,
`Object.fromEntries`, `Object.groupBy`, `Object.create` with a descriptor and
`['red'].map((s) => s)` all fold there and fold to the same values here. The
ceilings themselves have no opposite number: the other compiler bounds none of
this, which is the whole reason they exist.

**The removals, and the readings they rest on.** Each was probed by making the
arm panic and running the whole workspace suite, the transform's file corpus
included -- 8863 cases, no hit:

- **One refusal written twice.** A value with no object at all and an object of
  a kind this side writes no expression for answered the same sentence for the
  same reason, in two places. The three kinds are one match now -- an array, a
  plain object, and everything else -- and a symbol or a big integer falls
  through to the refusal it always reached. Neither can arrive: the guard
  admits no spelling that produces one, probed through a getter as well as at
  the top of an expression.
- **One entry read written twice.** An element of an array and a property of an
  object are the same reading, and had drifted into two. Both go through one
  reader now, which is what makes the getter's throw one call rather than two --
  and the property no longer clones the key it has just built a name from.

**The performance reading**, since this recurses over every element and property
of every answer. `as_property_value` moved rather than copied: `as_expr` takes a
reference and clones, so a folded object of N properties made N deep copies of
trees it was about to drop. It now moves the expression it already owns and
leaves only an array to the shared rebuild, which is what keeps a folded array
element and an evaluated one under one rule. The key clone above goes with it,
and nothing new allocates.

**The twelve regions left**, all of them, read off the per-function export
rather than counted off the annotated text -- llvm-cov writes two regions for a
multi-line `return`, which is where a count taken by eye loses two:

- Four `?` that never short-circuit, one each: `own_property_keys`, the array's
  `length` read, and the two reads behind a theme group. An ordinary object's
  own keys cannot throw, an array's `length` is a data property, and a group
  answers its own two keys through the compiler's own traps.
- One more: `as_property_value` at its call site, which fails only for the value
  below.
- Two for `length_of`'s refusal for a length that is not a count. Asked of an
  array exotic object alone, whose `length` the language keeps inside `u32`. It
  is not deleted the way ticket 11's arms were, because there is no total
  reading of the property to put in its place; the doc says so now.
- Two for the symbol key. It cannot be deleted either -- the match would stop
  being exhaustive -- and the value it needs cannot be built, which is the same
  standing the guard's `Pat::Expr | Pat::Invalid` arm has.
- Three for the closure `as_property_value` writes its refusal in.
  `Outward::value` answers one of the two shapes `as_expr` reads, so the `None`
  is unreachable by construction. It stays because the alternative is a second
  copy of the array conversion here, which is the drift the shared one exists to
  prevent.

So this file does not reach the gate, and two of the three that hold it back are
a refusal with no total reading behind it rather than an arm a rewrite removes.
Whoever closes this ticket has to answer for them, as the seventh instalment's
per-instantiation gap has to be answered for.

**What is left**, 146 regions across thirteen files. The five that hold 102 of
them:

| File                       | Cover  | Regions left |
| -------------------------- | ------ | ------------ |
| `evaluate/mod.rs`          | 90.81% | 34           |
| `engine_fold/transport.rs` | 93.64% | 28           |
| `engine_fold/theme.rs`     | 89.12% | 16           |
| `engine_fold/engine.rs`    | 91.67% | 12           |
| `engine_fold/outward.rs`   | 94.92% | 12           |

**2026-09-07, ninth instalment.** One file, `evaluate/mod.rs`, which the eighth
instalment's table listed first at 90.81% with 34 unexercised regions. It
reaches **100.00% of regions, functions and lines** — the first file in this
crate to reach the gate on llvm-cov's own summary as well as on the merged view,
which is the answer the seventh instalment's `helpers.rs` note asked for. The
crate moves 97.86% to 98.36% of regions and 98.42% to 98.81% of lines, and 141
unexercised regions across fourteen files to 107 across thirteen. (141 rather
than the 146 the eighth instalment recorded: re-measured on the tip before the
work started.)

| File              | Before | After   | Regions left |
| ----------------- | ------ | ------- | ------------ |
| `evaluate/mod.rs` | 90.81% | 100.00% | 0            |

Thirteen cases land, 886 to 899: six in one new suite,
`tests/evaluated_array_form_tests.rs`, whose subject is the array literal an
evaluated list is written as; three beside the suite that already owns the
object a folded function map stands for; four beside the suite that already
owns what an object key names.

**The subject of the new suite is a list with no literal form.** An evaluated
array has two spellings, and `evaluate_result_vec_to_array_expr` is the one
place that turns the first into the second. Its whitelist — an element value is
an array, an object, a literal or a name, and nothing else — had no case
anywhere, nor did the recursion that carries a nested list's refusal up. Both
are asked of the function directly as well as through source, because the
transform above and the `env` option's napi bridge hand it lists this crate did
not build: the whitelist is a contract of the function rather than of any one
source shape. The source route is `{ a: [[() => 1]] }` and `{ ...[[() => 1]] }`
— a style value and a spread operand are the two positions that read a folded
list through its literal form.

**Three readings had no case anywhere**, and each is a shape one written call
does not reach:

- **The three map entries that are not a plain function.** The `env` option's
  object, a map nested inside a map, and a compiled style map all stand for an
  object of their own keys, and only the plain function and the marker map had a
  case. Of the three, **only the `env` object is live**: the transform registers
  it as an entry of the namespace's own fold, beside the functions. The other
  two are registered under a name of their own and never as an entry of a map,
  so no written source reaches those two arms — they keep the match exhaustive,
  and each case says what its arm answers if the surface ever nests. This is
  what the shipped doc on the nested map already says, and the two cases now
  agree with it.
- **A key written as a big-integer literal.** `1n` names the property `1`, and
  it is the one key spelling with no second way to write it.
- **A computed key that names no string.** A boolean, `null` and an object all
  refuse with `Key is not a string`, which is a sentence about the key rather
  than about the value.

**Measured against the reference implementation**, one key at a time through
`@stylexjs/babel-plugin`. `{ 1n: 'red' }` declares `1: red` in both, and
`{ color: 'red', 2n: 'blue' }` orders the integer key first in both — the order
the language gives own keys, which one case now pins. A nested array holding a
callback refuses in both, with the same sentence: `A style array value can only
contain strings or numbers.` Note that a nested array of plain _values_ folds
here and is refused one stage later, where the reference implementation refuses
it during evaluation; the end answer is the same refusal, so the pair case says
what the evaluator answers rather than what `create` accepts.

**A divergence the comparison found, and it is not fixed here.** A computed key
that is not a string names a property there and refuses here, and `[1 > 2]`
names two _different_ properties — `0` here, `false` there, with no error either
side. Nine measured rows are in
[ticket 49](./49-name-a-computed-key-the-way-the-language-does.md), which owns
the decision: the coercion decides a CSS property name, so it does not belong in
a coverage instalment.

**Three removals, and the readings they rest on.** Every one is provably total
by reading rather than by probing, which is why none of them needed a panic
probe:

- **A guard against a shape the line above removes.** `Expr::Paren` had an arm
  that aborted the build. The scrutinee is `normalize_expr(path)`, which unwraps
  parentheses in a `while` loop, so no arm below it can be handed one. The arm
  is gone and the invariant is written where the loop is called.
- **A question asked twice to get one answer.** `is_ident()` and then
  `as_ident()`, with an abort for the second answer the first had already ruled
  out. One `as_ident()` in a let-chain now, and the message it aborted with is
  gone with it.
- **A kind read twice.** The fallback arm called `get_expr_node_kind` once for
  the warning and once for the sentence the author reads. Read once into a
  local, which is also why the warning's argument had no case: `warn!` does not
  evaluate its arguments unless a logger asked for that level, and the tests
  install none above `Error`. Its sentence now matches the other warning in the
  crate rather than carrying a grammar error of its own.

**The performance reading**, since `_evaluate` runs once per node of every
expression the compiler folds. Two readings, and the first is smaller than it
first looked: `warn!` reads its argument only when a logger asked for that
level, so hoisting the kind saves a call on a refusal logged at `warn` and
leaves the count unchanged when nothing is logging — which is also why the
region had no case. What it does not do is add one, and hoisting is what makes
the read eager and therefore countable. The second is unconditional: one fewer
match on the node kind on every name the module resolves, and the walk that
reaches `resolve_reference` is every unqualified identifier in every style.
`get_expr_node_kind` answers a `&'static str`, so nothing here allocates.
Nothing new clones either.

**What is left**, 107 regions across thirteen files. The five that hold 77 of
them:

| File                         | Cover  | Regions left |
| ---------------------------- | ------ | ------------ |
| `engine_fold/transport.rs`   | 93.64% | 28           |
| `engine_fold/theme.rs`       | 89.12% | 16           |
| `engine_fold/outward.rs`     | 94.92% | 12           |
| `engine_fold/engine.rs`      | 91.67% | 12           |
| `nodes/global_conversion.rs` | 91.67% | 9            |

**What the review changed, since it is not all test code.** Four findings, all
applied:

- The assertion that a source folded to a list was written twice — once in the
  new suite, once in `array_hole_tests.rs`, each with its own copy of the
  confident check. `folds_to_a_list` in `tests/source_evaluation.rs` is the one
  reading now, and both suites read it. This is the rule that file's own module
  doc states.
- `Key is not a string` was an inline literal in `evaluate_obj_key` and a second
  inline literal in the case asserting it. It is `KEY_IS_NOT_A_STRING` in
  `stylex-constants` now, which is how every other message in that function is
  written.
- Three claims in this record were wrong or measured on a basis it did not
  state: the two arms that are not live, the remainder counted two ways, and the
  reading of the hoisted call. All three are corrected above.
- One case doc said "three" over a loop of five spellings, and one comment said
  the second call ran on every refusal. Both corrected, and the comments went
  through a Simplified Technical English pass.

The review also asked for the `[1 > 2]` divergence to be pinned rather than only
recorded, which is
`a_comparison_read_as_a_key_names_the_number_it_folded_through` — the
thirteenth case. It asserts the wrong answer on purpose, and says so, so that
[ticket 49](./49-name-a-computed-key-the-way-the-language-does.md) changes it
visibly.

**2026-09-08, tenth instalment.** One file, `engine_fold/transport.rs`, which the
ninth instalment's table listed first at 93.64% with 28 unexercised regions. It
reaches **100.00% of regions, functions and lines** — the second file in this
crate to reach the gate on llvm-cov's own summary. The crate moves 97.49% to
98.06% of regions and 98.66% to 98.97% of lines, and 113 unexercised regions
across fifteen files to 87 across twelve. (113 rather than the 107 the ninth
instalment recorded: re-measured on the tip before the work started.)

| File                       | Before | After   | Regions left |
| -------------------------- | ------ | ------- | ------------ |
| `engine_fold/transport.rs` | 91.13% | 100.00% | 0            |

Seventeen cases land, 899 to 916: twelve in one new suite,
`engine_fold/tests/carried_shape_tests.rs`, whose subject is the carriage itself;
two beside the suite that already owns a carried value; one beside the suite that
already owns the StyleX function the engine may call; and two more in the new
suite after the review.

**The subject of the new suite is a walk wider than any source.** Every value
that crosses today is one the evaluator answered, and the evaluator rebuilds
every object it answers with identifier keys — `create_ident_key_value_prop`, in
`nodes/object_expression.rs` and in the three helpers beside it. So the reader's
other two key spellings, a hole, a spread, an accessor, a computed key and a
value of the compiler's own have no source at all to be written in, and the
coverage figure said as much: the suite that folds `{ 'a-b': 1 }` through a
module binding left `PropName::Str` unexercised, because what crossed was the
rebuild rather than the source. The cases ask the walk directly, under the
measuring carriage where the shape is what is being read and under the building
carriage where the value it comes to is.

**Three routes had to be opened**, and each is a shape a folded stylesheet cannot
show:

- The measuring carriage on its own. `measured_under` builds a `Measure` over a
  stated ceiling and a stated depth, which is what lets a case name the bound it
  is about instead of arranging an expression large enough to meet it.
- The transport with a value put into it. `carry` records a crossing without
  measuring it, so a case can hand the building walk a value no guard would have
  resolved — a `defineVars` group, in the one case that is about the build
  refusing.
- A builder the case wrote. The group builder is a `JsFunction` the caller
  passes, so a builder that throws is the route to the one refusal the building
  walk has left.

**Three readings had no case anywhere:**

- **A name read twice, and a function called twice.** Both are one crossing, and
  both early returns were unexercised. The module's own function had a case
  already (`guarded_walk_tests`); the value and the two StyleX spellings did not.
- **A string no Rust `str` can hold.** `'\uD800b'` crosses as the two code units
  it is written from. The reading that carries it was unexercised, which means
  nothing had shown the bridge is UTF-16 on this side rather than UTF-8.
- **A key past a bound.** A key is text the engine holds, so it counts against
  the character ceiling like a value. No case had said so.

**Measured against the reference implementation**, one shape at a time through
`@stylexjs/babel-plugin`. `carried.concat(carried).join('-')` over `['a','b']`,
`'\uD800b'` read as a code unit and a length, two `stylex.firstThatWorks` calls
in one array, `Object.keys` over `{ ab: 1, 'a-b': 2, 2: 3 }`, `{ 1e21: 1 }` and
the repeated key `{ a: 1, b: 2, a: 3 }` all fold there and fold to the same
values here — `a-b-a-b`, `d8002`, `var(--a, blue)|var(--b, red)`, `2|ab|a-b`,
`1e+21` and `a|b:3`. The last two are the language's own answers about own keys:
an integer key comes first, and a repeated key keeps the place of its first
writing and the value of its last.

**One removal, and the reading it rests on.** `Carriage::property` returned a
`Result` because the building half defined each property through
`create_data_property_or_throw`, which runs the language's own
`[[DefineOwnProperty]]` step and can therefore throw. Both objects this module
builds are fresh, ordinary and never handed to JavaScript before they are
finished, and every property it writes is writable and configurable — so every
question that step asks has one answer here. The property is written into the
object's own properties instead. Boa lands both spellings in the same
`insert_with_slot`, so a new key and a repeated key behave exactly as before,
which the two new cases pin. Four regions go with the `Result`: the `?` on the
property, the `?` on the namespace object, the `?` inside the definition, and
the definition's own error mapping.

**The performance reading**, since this runs once per property of every object
that crosses and once per function of the StyleX namespace. The direct write
drops, per property, a shape lookup for the existing key, an extensibility
vtable call, a property-key clone, the `&mut Context` borrow the step needed, and
the closure and error mapping around it. Nothing new allocates and nothing new
clones; the descriptor the builder writes is the descriptor the step derived.

**The two regions the eighth instalment's `as_property_value` note asked about**
are not here — they are in `outward.rs`, which this instalment does not touch.

**What is left**, 87 regions across twelve files. The five that hold 52 of them:

| File                           | Cover  | Regions left |
| ------------------------------ | ------ | ------------ |
| `engine_fold/theme.rs`         | 83.15% | 15           |
| `engine_fold/outward.rs`       | 92.62% | 11           |
| `engine_fold/engine.rs`        | 89.69% | 10           |
| `engine_fold/amplification.rs` | 97.64% | 8            |
| `nodes/global_conversion.rs`   | 89.87% | 8            |

**What the review changed, since it is not all test code.** Six findings, all
applied:

- The doc under the new definition said the key is always new, which is false —
  `{ a: 1, a: 2 }` writes the same key twice — and its own next paragraph said
  so. The grounds are now the true ones: every property written here is writable
  and configurable, so a repeated key replaces the value in the slot the first
  writing made.
- Nothing tested the repeated key, which is the one question the two definition
  steps could have answered differently, and nothing tested the four attributes
  the descriptor now spells out by hand. Two cases each.
- The reading of an engine value was written twice — once in the new suite, once
  in `var_group_tests.rs`. `answered_by` in `engine_fold/tests/engine_reads.rs`
  is the one reading now, and both suites read it.
- The new case about a function crossing once had a name one word from the
  existing case about a _module's_ function crossing once. It says `stylex` in
  its name now, and its doc names the third route.
- The suite compiled the real group builder for every built case, though no case
  built a group, and the depth comment counted containers where the budget counts
  descents. Both corrected.

**2026-09-08, eleventh instalment.** One file, `engine_fold/theme.rs`, which the
tenth instalment's table listed first at 83.15% with 15 unexercised regions. It
reaches **100.00% of regions, functions and lines** — the third file in this
crate to reach the gate on llvm-cov's own summary. The crate moves 98.06% to
98.39% of regions and 98.97% to 99.21% of lines, and 87 unexercised regions
across twelve files to 72 across eleven.

| File                   | Before | After   | Regions left |
| ---------------------- | ------ | ------- | ------------ |
| `engine_fold/theme.rs` | 83.15% | 100.00% | 0            |

Nine cases land, 916 to 925, all in the suite that already owns the group.

**The whole of the remainder was one shape: a step that cannot fail from the
source that reaches it.** `theme.rs` has three of them, and each wanted a
different answer:

- **A source that is a constant.** `compile_var_group` builds the traps in four
  steps — the text parses, to a function, which is called once, and has to
  answer a function — and each step has a refusal. None can fire for
  `var_group_traps()`, so the source is a parameter now: `compile_traps` takes
  it, `compile_var_group` passes the one that is shipped, and a case hands in a
  source that fails the step it is about. `unbuilt` and the closure that reads a
  throw are reached with them, which is ten of the fifteen regions.
- **A value that answers with a throw.** `is_a_var_group` and `var_group_text`
  each read one key off a value, and each carries the refusal for a read that
  threw. No group this compiler builds can throw, so the route is a value it did
  not build: an object with a getter that throws. Two regions.
- **An arm that cannot be entered at all.** `derive` read its five arguments one
  index at a time, and answered `false` for a flag that was not there. It reads
  the key at index 4 before either flag, so a list short enough to miss a flag
  had already thrown — the arm was dead. It is gone, and the read is one slice
  pattern over named values, which is also what says the flags are there
  whenever the key is. Three regions, one covered and two removed.

**Measured against the reference implementation**, through
`@stylexjs/babel-plugin` 0.19.0 under all four spellings of the two naming
options. A group's own hash is `xop34xu` there and here; a member is
`var(--x1ineb92)` under three of the four and `var(--primary-x1ineb92)` under
debug with readable class names, which is the pair this crate carries across the
bridge as two booleans.

**The performance reading.** `compile_traps` is cold — `compile_var_group` runs
once inside `Engine::new`, which one thread reaches once, so the split costs a
call on a path that parses a source and builds a realm. `derive` is not: it runs
once per member read off a group inside a fold. The slice pattern asks the
length once where five `get` calls asked it five times and built five `Option`s,
and one closure goes with the dead arm. Nothing new allocates and nothing new
clones; the three strings the derivation hands the naming are the three it
handed before.

**What is left**, 72 regions across eleven files. The five that hold 44 of them:

| File                           | Cover  | Regions left |
| ------------------------------ | ------ | ------------ |
| `engine_fold/outward.rs`       | 92.62% | 11           |
| `engine_fold/engine.rs`        | 89.69% | 10           |
| `nodes/member_expression.rs`   | 97.48% | 8            |
| `nodes/global_conversion.rs`   | 89.87% | 8            |
| `engine_fold/amplification.rs` | 97.64% | 8            |

**What the review changed, since it is not all test code.** Six findings, all
applied:

- The naming was asserted as a table of all four option pairs, which is the table
  `theme_ref_test` in the state crate already owns and states it owns. It reads
  the one pair this module is about now — the readable spelling, which is the
  only answer that shows both booleans crossed.
- Four cases wrote the same three-arm match on a decline by hand, in the
  instalment that moved a fifth into the shared reading.
  `assert_refused_saying` sits beside `assert_refused_by_rule` in
  `engine_fold/tests/engine_reads.rs` now, and both read one `assert_refusal`.
- Two cases about the derivation asserted only that something threw, and one
  about a plain object only that it refused. Each names its sentence now, and the
  derivation's sentence is `READ_WITHOUT_AN_IDENTITY` rather than a literal
  written twice.
- `compile_traps`'s doc claimed a generality that does not exist — there is one
  source and one caller. It gives the true grounds now: the four refusals are
  unreachable from the shipped source, so the source is a parameter.
- `engine_reads.rs` gained a helper its module doc did not describe. The doc says
  what the file holds.
- Three Boa realms were built for a function that ignores its context, and the
  loop variable was called `absent` for a value that is present.

One claim in the performance review is wrong and is recorded here so it is not
read back as fact: it says a call with fewer than five arguments used to derive a
name with the flags defaulting to `false`. It did not. The old code read the key
at index 4 before either flag, so any shorter list already threw, and the slice
pattern preserves that exactly.

**2026-09-08, twelfth instalment.** One file, `engine_fold/outward.rs`, which the
eleventh instalment's table listed first at 92.62% with 11 unexercised regions.
It reaches **100.00% of regions, functions and lines** — the fourth file in this
crate to reach the gate on llvm-cov's own summary. The crate moves 98.39% to
98.64% of regions and 99.21% to 99.38% of lines, and 72 unexercised regions
across eleven files to 61 across thirteen.

| File                     | Before | After   | Regions left |
| ------------------------ | ------ | ------- | ------------ |
| `engine_fold/outward.rs` | 92.62% | 100.00% | 0            |

Fifteen cases land, 925 to 939 — fourteen in one new suite,
`engine_fold/tests/built_answer_tests.rs`, whose subject is the walk out itself,
and one beside the suite that already owns the amplification reading.

**The remainder was two shapes again, and the second one wanted a type rather
than a test.**

- **A step that cannot fail from the answer that reaches it.** Four of them: the
  own-key list of a plain object, an array's `length`, and the two reads a theme
  group answers. Every one is the language's own read, so every one can throw,
  and no answer a shipped fold produces makes one throw — an ordinary object
  always lists its keys and an array's `length` is a data property inside `u32`.
  The route is a value the walk was handed rather than one it grew: a proxy whose
  traps throw, and an object whose `length` is text or is below zero. Two cases
  ask the private walk directly, which is what the carriage suite one level up
  already does and for the same reason.
- **A conversion that had to answer for values that cannot arrive.** The object
  position read a folded value back through the evaluator's shared reader, which
  is partial over the evaluator's value — so it carried a refusal for a shape the
  walk cannot build, and the caller carried the `?` that refusal needed. The walk
  answers a type of its own now, `Built`, with exactly the two shapes it builds:
  a value and a list. Both readings off it are total — the evaluator's list at
  the top of an answer, the array literal under an object key — so the refusal
  and its `?` are gone rather than untested. Three regions.

**A symbol key is the one arm left that is answered rather than reached.** It
needs a `Symbol`, which the guard admits neither as a global nor as a computed
key, so no module reaches it; a case hands the walk an object carrying one.

**Measured against the reference implementation**, through
`@stylexjs/babel-plugin` 0.19.0. Three sources fold on both and agree to the
class name: `'ab'.repeat(count)` under `count = [3]` is `.x5ryvnc`, under
`count = []` is `.x14axycx`, and `[1, ['a', []]].flat(2).join(' ')` is
`.x1ax882u`. A folded object holding an array — the path the conversion changed
— has no Babel answer to compare against: `Object.entries` is not one of the
statics it folds.

**The performance reading.** The object position used to clone: a folded array
under a key was rebuilt by the shared reader, which copies every element and
every subtree of it, and then the original was dropped. It moves now. The list
position pays for that with one more list per array level — the walk builds a
`Built` list and reads it into the shape the position asks for — and `Built` is
the smaller of the two elements, so the build loop got cheaper by as much as the
read-out costs per element. The two readings ask for stack room per level, as
every walk across this bridge does: the walk's own growth is released when it
returns, and the tree can be as deep as the evaluation ceiling.

**What the benches can see of that is only the cost.** Every fold leg in
`perf_fixtures/engine-fold.js` answers a string except two that answer a
top-level two-element array, and none answers an object holding an array — so
the clone this removed is priced nowhere. A leg that answers one would be the
honest way to read it, and is left to the ticket that next touches that fixture.

**What is left**, 61 regions across thirteen files. The five that hold 39 of
them:

| File                           | Cover  | Regions left |
| ------------------------------ | ------ | ------------ |
| `engine_fold/engine.rs`        | 89.69% | 10           |
| `engine_fold/amplification.rs` | 97.64% | 8            |
| `nodes/member_expression.rs`   | 97.48% | 8            |
| `nodes/global_conversion.rs`   | 89.87% | 8            |
| `engine_fold/mod.rs`           | 95.27% | 7            |

Four of `amplification.rs`'s eight are the `?` around the two places a name is
resolved to a count, and each wants a name holding a value with no number at all
— a function is one. Left to the instalment that takes that file.

**What the review changed, since it is not all test code.** Two reviews, seven
findings, five applied and two recorded:

- The reading moved out of `mod.rs` was a duplicate. `evaluate_result_as_expr` in
  `evaluate/mod.rs` already answers the same question for the whole crate, so the
  copy is deleted and the two amplification sites call it.
- A case said a two-element list leaves the call unbounded. It does not: the list
  joins to `1,2`, which reads as `NaN`, and `NaN` counts as zero — so the call is
  bounded at nothing and folds to the empty string. The case reads all three
  lengths now and says what each one is.
- `Built::Value`'s doc did not say which expressions it may hold, and the
  allowlist the shared reader applied is gone with it. It names the four kinds,
  and says they are the set the evaluator's own reader admits.
- The doc claimed moves where one position now pays a list. It gives both halves.
- A cross-reference counted into another doc's prose by ordinal, and a loop built
  an engine per iteration.
- Recorded rather than applied: a folded object key is written through the
  identifier factory whatever the key spells, so `'a-b'` becomes an `IdentName`
  no identifier can spell. It is carried rather than printed — `create` over
  `Object.fromEntries([[':hover', ...]])` compiles to the same two rules as the
  reference implementation — so nothing observes it, and it is older than this
  work. A case now reads such a key so the spelling is at least written down.
- Recorded rather than applied: the bench gap above.

**2026-09-09, thirteenth instalment — the crate reaches the gate.**
`stylex-evaluator` reports **100.00% of regions, 100.00% of functions and
100.00% of lines** on llvm-cov's own summary. `pnpm run test:coverage:workspace`
reports the same over all 22,241 regions with the crate now measured, and the
crate is off all five exemption lists —
`scripts/git/coverage-exclusions.test.mjs` agrees.

| Measure   | Before | After   |
| --------- | ------ | ------- |
| Regions   | 98.64% | 100.00% |
| Functions | 98.94% | 100.00% |
| Lines     | 99.38% | 100.00% |

Sixty-one unexercised regions across thirteen files to none, and 939 cases to
983 — forty-four land, in seven new suites and six that already existed.

**The remainder was four shapes, and only the first of them wanted a test
alone.**

- **A message no build reads.** A `log` macro skips its arguments until a logger
  asks for that level, so eleven regions across `evaluate/mod.rs`,
  `nodes/binary_expression.rs` and `nodes/member_expression.rs` were the
  sentences an author reads beside a refusal, and nothing read them. The test
  binary already installed `pretty_env_logger` from a `ctor`, and `log` admits
  one logger per process — so `crate::tests::capturing_logger` **wraps** that
  one rather than replacing it: printing stays `RUST_LOG`'s decision, and a case
  keeps a copy of what its own thread wrote. The same helper `stylex_state` and
  `stylex_diagnostics` hold, with the wrapping added. Five cases read the
  messages back and each names the words it expects.
- **A step that cannot fail from what reaches it.** The shape the eleventh
  instalment named, and most of the rest. Where the thing that cannot fail is a
  _source_, it became a parameter, as `compile_traps` did: `Engine::started_on`
  takes the prelude and the traps, `Engine::new` passes the two that ship, and
  seven cases hand in one that fails the step it is about. `compile_var_group`
  went with it — it had one caller and its doc is now `compile_traps`'s. Where
  the thing that cannot fail is a _value_, the case built one the walk was
  handed rather than one it grew: an object whose `toString` throws for the
  fallback chain, a getter on `globalThis` for an applied global, a spread and
  an invalid parameter for the two patterns no source writes.
- **A reading no source reaches at all.** Each is asked directly, which is what
  the carriage and the walk-out suites already do. `Conversion::of` answers four
  empty argument lists no source produces — `String()` names an unshadowed
  global with nothing to keep on this side, so the engine folds it and the
  answer never comes back — and each of the four is a different value from the
  conversion of `undefined`, which is why they are kept. `count_written`,
  `count_resolved`, `number_resolved`, `rendered_characters` and `rendered_expr`
  are asked the same way.
- **An arm or an edge that cannot be entered.** Removal, which is the treatment
  [ticket 11](./11-cover-the-state-crate.md) gave the state crate. Five of
  them, each a change to shipped code and each read on its own below.

**What the removals changed.**

- `Scope::binds` and `Scope::bounds_of` were asked one after the other at both
  readers, and the second answer already carried the first — so `bounds_of`'s
  arm for the module scope could not be entered. They are one walk now,
  `Scope::bound`, answering `Option<Option<Bounds>>`: no scope binds the name,
  a scope binds it and nothing measured what it holds, or the bounds. The
  difference between the first two is what stops a callback parameter being
  read as the module name it shadows, so it stays visible. Two walks of the
  scope chain become one, per name read inside a callback.
- `LeftOperand::as_expr` answered `None` for a measured side, and a measured
  side is a string, so the `+` had already gone to the concatenation. The
  numeric path never held one. `evaluate_left_operand` is called only for `+`
  now — every other operator was paying for its `matches!(op, Add)` to say no —
  and the numeric path below holds a plain value by construction. `as_expr` and
  `is_string` are gone with it.
- `binding.rs` asked `state.confident` after resolving a theme import, and its
  own comment said the resolution cannot leave the evaluation unconfident. The
  question had one answer. The comment is the grounds now and the guard is
  gone.
- `try_fold` read the thread's engine with `?`, and the two sources an engine is
  built from are this compiler's own constants. It reads through the answer now
  rather than out of it.
- The conversion's argument walk answers nothing only for a spread, which the
  guard refuses before a conversion is reached — and a spread leaves the list
  shorter than the author wrote either way, which the next line already answers
  for. The two answers are read as one.

**One refusal is narrowed rather than removed**, and it is worth stating plainly
because the first attempt was wrong. `is_a_var_group` carries the throw a marker
read can answer with. The walk out **needs** it — a case in
`built_answer_tests.rs` hands the walk a proxy whose `get` throws and reads the
engine's own words back — so making the reading total broke that case. The
narrowing is at the other call site only: the fold asks the question of a value
the engine just built, where no group can throw, and the walk out reads the same
value a moment later and carries the throw properly. It reads a refusal there as
"not a group".

**Measured against the reference implementation.** `@stylexjs/babel-plugin`
0.19.0 through `pnpm run --filter=@stylexswc/rs-compiler parity`, over the
rebuilt addon: 942 rows identical, and the eight non-identical rows are the same
eight the tracker already records as expected. The transform's own 2,467
snapshots and the whole workspace suite are green.

**The performance reading**, from a review of its own. Nothing new allocates on
a fold, and nothing is cloned or moved that was not.

- The scope merge takes one walk of the chain where it took two, and the walk
  costs the same per level. It is read where the guard is sizing an amplifying
  call rather than per admitted call, so what it saves is small — and it
  compounds a little over a `+` or `*` subtree, which the reading recurses over.
- Every operator that is not `+` used to enter `evaluate_left_operand`, ask
  whether it was `+`, and wrap its answer into the wider of the two operand
  shapes for the next line to unwrap again. It calls the operand reading
  directly now, and `evaluate_left_operand` has one caller left.
- `var_group_traps()` builds its string once per engine, which is once per
  thread, exactly as the deleted `compile_var_group` did. It is built a little
  earlier than before — ahead of the realm rather than after it — which costs
  nothing on a path taken once.
- The engine now reaches the fold body through a `Result` rather than a `?`.
  The engine is already moved twice per fold, out of the thread's slot and back
  into it, and this is at most a third move of the same value in the same
  inlined scope, next to a print, a parse and an evaluation.
- The logger is `#[cfg(test)]`, and `pretty_env_logger` and `ctor` are
  dev-dependencies, so the open level cannot reach a shipped build or a bench —
  a bench compiles the library without `cfg(test)`. In the test binary the
  eleven message sites are all on refusal paths, and an argument that is not
  kept reaches a thread-local read and the wrapped logger's own filter rather
  than a `Debug` impl.

**2026-09-09, thirteenth instalment, review follow-up.** Two reviews of the
instalment above, one for correctness and one for performance, raised ten
findings between them. Nine are documents that said something the code does not
do. **One is a fold that answered wrongly**, and it is the reason this entry is
worth reading rather than skimming.

**A refusal the instalment above narrowed was a real regression.** The fold
asked `is_a_var_group` of the answer it had just built and read a throw as "not
a group", on the grounds that the walk out would meet the same throw a moment
later. It does not. The walk out reads **own** keys, so a marker on the
answer's _prototype_ is never read a second time. The instalment's own wording
said the refusing shape would need a proxy — and a proxy is neither an array
nor an ordinary object, so it goes to `exotic_value`, which asks the same
question with `?` and refuses properly. The shape that actually slipped through
is an ordinary object with an **inherited** `__IS_PROXY` accessor that throws.

A module reaches it. A callback body is not analysed — the engine parses it —
so a body may build what the walk would refuse written out, and a getter can
throw with no `throw` statement, which a body may not use. Measured through the
built addon:

```js
export const styles = stylex.create({
  x: {
    color: [vars.a]
      .map(() =>
        Object.create(
          Object.create(null, { __IS_PROXY: { get: () => null.x } })
        )
      )
      .at(0),
  },
});
```

folded to `{}`. The declaration was dropped, no CSS was emitted, and **nothing
was said** — the worst of the three outcomes, and the one this compiler refuses
in order to avoid. The `?` is back. The same source now refuses in the engine's
own words, `TypeError: cannot convert 'null' or 'undefined' to object`.

The tail is load-bearing and the case says so: `at(0)`, `find(() => true)` and
`pop()` leave the object itself as the answer, which is where the marker is
read. `map(...)[0]` and `flat()[0]` answer the _array_ holding it, so the
marker is read off the array, and those still fold — which is the existing
design rather than this finding.

**And the region was coverable the whole time.** With the `?` restored and one
case added, the crate still reports 100.00%. So the narrowing bought nothing it
was done for. That is the lesson to carry: the claim "no case can enter this"
has to be tested against a _source_, not argued from the call site, and this
instalment argued it from the call site three times.

**The fifth technique, and where it is legitimate.** The instalment names four
shapes. Three lines in it took a fifth: a step that cannot fail from what
reaches it, whose refusal is read _where it is asked_ rather than propagated
with `?`. `llvm-cov` charges `and_then`, `unwrap_or` and `unwrap_or_default` to
`library/core`, so the region stops counting against this crate where the `?`
was one here. One of the three was the regression above. The two that stand are
now documented as what they are, gate first:

- `engine_fold/mod.rs` — `Engine::new` builds from the prelude and trap source
  this compiler ships, and a failure still leaves through the closure's own
  `Result`. A step that _can_ fail is asked of `Engine::started_on`, which
  takes both sources so a case can hand in one that fails.
- `nodes/global_conversion.rs` — the walk answers nothing only for a spread,
  which the guard refuses first, and the empty list the fallback substitutes is
  caught by the length check on the next line. It refuses rather than folding,
  which is what makes the substitution safe.

The test is whether the substituted answer can fold something the source does
not describe. Where it can, this is laundering, and the group marker is what
that looks like.

**Two comments were false about a branch a module reaches every day.** The
conversion's length check was documented as "unreachable from any module an
author can write", on the grounds that an argument is dropped only after a
deopt and the confidence check above has already returned. There is no
confidence check above it. A probe on the branch counted **26** entries across
the crate's own suites, every one with the evaluation already unconfident — so
the branch is ordinary and its `deopt` is the no-op. A second probe, on
`evaluate_func_call_args` and over the evaluator's and the transform's suites
together, found **0** of 3,793 cases dropping an argument while confident. The
refusal is kept, and now says that rather than claiming unreachability.

**`count_written`'s doc drew its conclusion from the wrong half of its own
reasoning**, and the correction is not the one the first pass wrote. Its two
lines refuse for different reasons and **both** are reached. The `?` cannot
fire for the literal `Walk::count_bound` hands it, so every refusal it answers
arrives through `count_resolved` — that half is right. But `count_of` refuses a
count that is not finite, and a literal reaches that: `'x'.repeat(1e999)` and
`'x'.repeat('Infinity')` both read as infinity and both refuse, measured
through the addon. The first pass claimed every refusal here belonged to
`count_resolved`, which is false.

**The logger's own documents were wrong about the logger, in four places.**
`pretty_env_logger::formatted_builder()` reads no environment — only
`try_init`/`try_init_custom_env` call `parse_filters` — and `env_logger`'s
filter defaults to `Error` with no directives. So this binary has never
honoured `RUST_LOG`, before the wrapping or after it. Corrected rather than
made true: honouring it would change what an ordinary test run prints.

`install` runs from the `ctor`, so `log::max_level` is `Trace` for the whole
binary from the start — and a `log` macro gates on `max_level` alone. `log`'s
own `log_impl` calls `logger.log` and never `enabled`. So two more claims were
wrong: that `Log::enabled` is what the per-thread level answers from, and that
the two `enabled` answers decide where a built message goes. `log` decides:
`kept` says whether the thread keeps a copy, and the wrapped logger's filter
says whether it prints. `enabled` answers for `log_enabled!`, its one real
caller.

Three comments credited the assertions with covering those regions. The open
level is what runs the arguments; the assertions are what makes running them
mean something, and that is what they say now.

**The grounds for opening the level once were wrong too**, which is the same
mistake a third time and was caught by the performance review. They said the
harness runs cases in parallel. `pnpm test:crates:workspace:regular` is
`cargo nextest run`, which gives every case a process of its own, and this
crate sets `doctest = false` — so a narrower level would be safe there. The
reason that holds is written down instead: `install` runs from a `ctor`, so
there is no per-case moment to close the level in, and `cargo test` does put a
binary's cases on parallel threads of one process.

**One restructure left dead code behind.** `evaluate_left_operand` has had one
caller since the fifth change, inside the `BinaryOp::Add` arm, so its opening
`matches!(binary_expr.op, BinaryOp::Add)` was always true and its `reason`
parameter only ever received `LEFT_HAS_NO_VALUE`. Both are gone.

**Two cases said less than they knew, or said it about the wrong thing.**

- `an_argument_that_refuses_stops_the_conversion` asserted with `assert_ne!`
  which sentence the refusal is _not_. It names the one `own()` refuses with
  now. Note the limit the review pointed out: the guard and
  `evaluate_func_call_args` write the _same_ `SPREAD_ELEMENT` sentence, so the
  spread case cannot prove which of them refused. The ordering claim is true;
  that case is not its evidence.
- `a_spread_argument_stops_every_conversion` was filed under a conversion it
  never enters. A probe that panicked in the fallback proved it: the case
  passes, so `Conversion::evaluate` is not reached at all. Renamed
  `a_spread_argument_refuses_before_any_conversion`.

**The performance review found nothing to act on**, and confirmed the
containment the instalment asserted rather than taking it on trust. `ctor` and
`pretty_env_logger` are dev-dependencies, `src/lib.rs` gates the module with
`#[cfg(test)]`, and no bench installs a logger — so `log`'s level starts at
`Off` there and no message argument is built, which is _cheaper_ than the
pre-instalment test binary. The shipped addon installs `stylex_logs`' logger at
`Warn`; both `warn!` sites are free, one interpolating a `&'static str` the next
line needs anyway and the other taking no arguments. The two `debug!` sites that
format a whole object literal, `member_expression.rs:444` and `:451`, stay
unbuilt in a shipped build. The open level's cost to the suite is not
detectable: about 2 s of real work across 982 cases, the rest of the wall time
being one 4 s case and nextest's per-process startup.

The two code changes read as small improvements. Every operator that is not `+`
loses a call frame and a branch. The `+` path loses one discriminant test per
link of a chain and — because `reason: &str` is a fat pointer — the call drops
from seven register-class arguments to five, which removes a stack spill and
reload per link on SysV. Nothing else on these paths is a bottleneck this
repository's policy would let anyone act on: `Vec::default()` does not
allocate, `engine.var_group.clone()` is a `Gc` refcount bump a borrow could not
replace, and the `Expr` clone in `count_resolved` is pre-existing, sits behind a
full `evaluate_cached`, and has no measurement behind it.

**Measured against the reference implementation**, over an addon rebuilt with
the restored refusal. `@stylexjs/babel-plugin` 0.19.0 through
`pnpm run --filter=@stylexswc/rs-compiler parity`: 1,162 subjects, 910
identical, **`changed` 0** and **`unexpected` 0** — every verdict is the one the
tracker records, and the restored refusal moves none of them. The shape it
fixes is not in the corpus, which is why the case above lives in the crate's own
suite.

**The gates.** `cargo check`, `cargo clippy --all-targets` and
`cargo nextest run`, each `--workspace --all-features`: clean and **9,012**
cases green. `stylex-evaluator` reports **100.00% of regions, functions and
lines** over 4,472 regions. `pnpm typecheck`, `pnpm lint:check`,
`pnpm lint:type-aware` and `pnpm format:check` are clean, and
`scripts/git/coverage-exclusions.test.mjs` agrees the five lists match.

**2026-09-09, fourteenth instalment — the second review of the instalment
above.** Eleven review agents read the 30-commit branch against `develop`.
Nothing they raised blocked the ticket, and the acceptance criteria were met
before this entry. What follows is what the review changed, and it is worth
reading because **five folds answered a value the reference implementation does
not, and each reaches a stylesheet**.

**The probes came first, and one of them overturned the review's own reading.**
The three shapes the review said must be measured before anything changed were
measured through a rebuilt addon against `@stylexjs/babel-plugin` 0.19.0, with
a probe that now lives beside the parity harness as
`crates/stylex-rs-compiler/parity/probe.ts`. All three refuse here where the
reference implementation folds — and the refusal is the _array element_
refusing, not the node the finding named: `(() => 1) + 1` stops the walk before
the conditional or the unary operator is reached. So no source reaches the
substitution the finding asked about, and the memo-warm state the crate's own
cases build is its only route.

That looked like the finding was closed. It was not. Asked directly rather than
behind the `.trim()` the existing case wrapped it in, the conditional **folds**:

```
[(() => 1) + 1][0] ? "a" : "b"   ->   confident, "b"
```

`unwrap_or(false)` read a refusal as a benign answer and put the alternate arm
in the stylesheet. This is the `is_a_var_group` shape a third time, and the
`deopt` is back. The case that walks it asks the conditional directly now, and
sits beside `!`, `typeof` and `-` over the same value, which all refused all
along. The region is coverable, as the thirteenth follow-up found for the group
marker: the crate still reads 100.00%.

**The three `?`-for-guard sites keep the `?`.** A warmed memo was probed at
each, and every operand records its own refusal before the line is reached, so
no source reaches a confident absence there. What changed is the documents,
which claimed a reason that is not the reason.

**Seven readings were wrong, and every one was measured against the reference
implementation.** Four answered a wrong value, one stopped a debug build, and
two refused a comparison the language answers.

| Source                                 | Was                  | Is             | Reference      |
| -------------------------------------- | -------------------- | -------------- | -------------- |
| `-1 >>> 0`                             | `-1px`               | `4294967295px` | `4294967295px` |
| `-1 >>> 16`                            | `-1px`               | `65535px`      | `65535px`      |
| `4294967296 \| 0`                      | `2147483647px`       | `0`            | `0`            |
| `1 << 32`                              | a debug build panics | `1px`          | `1px`          |
| `(1 != '1') ? 'red' : 'blue'`          | `blue`               | `red`          | `red`          |
| `('a' == 'a') ? 'red' : 'blue'`        | refuses              | `red`          | `red`          |
| `(null == undefined) ? 'red' : 'blue'` | refuses              | `red`          | `red`          |

The bitwise half is one cause: a Rust cast saturates where `ToInt32` wraps, a
shift count was never masked to five bits, and `>>>` was written as the signed
`>>`. `to_int32` existed and `~` already used it. **A second implementation of
the same operators sits in `stylex-js`** (`operators::evaluate_bin_expr`, which
`flatten_raw_style_object` reaches) and was worse — 64-bit, and `-1 >>> 0`
answered `0` there. Both go through `to_int32`, the new `to_uint32` and the new
`to_shift_count` now.

The equality half is the other: the four operators compared two _numbers_,
because the numeric coercion ran before the operator was asked. They compare
two primitives now, and fall to the numeric reading only for a side that is not
one. Note the row that follows the reference implementation rather than the
language and says so: upstream writes `!=` as a strict comparison, so this does
too.

**Two more wrong answers, both smaller.** `Object.keys` of a string counted Rust
characters, so an astral character answered one key where the language answers
two, and every index after it came out shifted. It counts UTF-16 code units now,
and a receiver holding a lone surrogate is refused the way a spread of the same
string already refuses.

Be exact about what changed there, because the refusal is not the language's
answer either. The engine answers this call for an ordinary receiver and is
right — `Object.keys('\u{1F600}')` measures 2 in both compilers through the
addon. The reading written out in Rust is reached only past a declined fold,
which `sx.missing ?? '\u{1F600}'` is, and there a wrong key list became a
refusal rather than the two keys the reference implementation gives. A refusal
is the honest answer for a text no Rust string holds, and it is a parting from
the reference implementation all the same. Ticket 50 carries the row.

And the numeric fold of a binary expression reported `Left expression is not a
number` for a **right** side — both cases asserted only the substring `is not a
number`, so neither could see the label.

**Three shipped texts were misspelled**, and the third `#[should_panic]` gaining
an `expected` is what found two of them: `Value in not a number` and `Varable
... is not a number`. `Key is not a string` was a fragment with no full stop,
unlike every neighbour.

**The five removed guard rails stay removed, and the invariant behind them is
asserted where it is created.** Re-adding a reader-side check is what the gate
rejects, and it is what produced this shape. One case per producer instead —
the list-to-literal form, the object fold on both of its routes, and the `env`
option's NAPI-RS bridge, whose array is read _through_ a hole and compared
against the same value with no hole. `CONTEXT.md` names each case beside the
class it guards.

**The `unwrap_or` audit, extended from two sites to five**, which is what the
review asked for and what would have caught the conditional. The five are the
sites the review named. The substitutions added since are of the same kind and
each touches a diagnostic only, so the verdicts below hold for them too:

| Site                              | Substitution                           | Verdict                              |
| --------------------------------- | -------------------------------------- | ------------------------------------ |
| `engine_fold/mod.rs`              | `Engine::new()` through `and_then`     | sound                                |
| `nodes/global_conversion.rs`      | `unwrap_or_default()`                  | sound                                |
| `nodes/array_expression.rs`       | `deopt.as_ref().unwrap_or(&elem.expr)` | sound, and touches a diagnostic only |
| `stylex_first_that_works.rs`      | `?` to `and_then`                      | sound                                |
| `nodes/conditional_expression.rs` | `unwrap_or(false)`                     | **fails**, and is the fold above     |

The test the follow-up wrote is the right one: whether the substituted answer
can fold something the source does not describe. Applying it to all five is
what separates the four from the fifth.

**The policy the review asked to be decided once is written down**, in
`guidelines/stack/RUST.md` under "A step the gate cannot cover". Four answers,
ranked: prove the claim against a source and keep the guard; make the shape
unrepresentable; mark the step `#[cfg_attr(coverage_nightly, coverage(off))]`;
delete the guard and assert the invariant at the producer. Never reshape a
signature to move a region.

**Nine cases entered an arm without reading what it answered**, and each says
what now. Among them: the shift cases picked operands where the signed and the
unsigned readings coincide, and so locked the bug in; `-0` was asserted against
`0.0`, which cannot see a sign; five `#[should_panic]` attributes accepted any
panic; and two group resolutions matched `ThemeRef(_)`, which a group built
from the wrong name matches too.

**Four groups of shapes had no case anywhere.** `evaluate/cache.rs` had no test
file at all, and now has five: a subtree folded twice, a remembered refusal, the
sentence such a refusal answers with, and the depth-refusal reset that keeps a
shallow reading of the same subtree folding — the recorded bug where one deep
property decided a sibling's fold. A key written twice over all three routes
that reach it, and the empty string as a key an array index sorts ahead of. A
shorthand method, which every existing case reached through an accessor. And
`NaN`, either infinity and `-0`, as operands and as computed keys.

**The inward half of the transport bridge carried only containers.** A name
bound straight to a number, a boolean or either absent value is the other half,
and the boolean arm was entered by nothing at all — measured by making it panic
and running the suite, which is the technique this entry recommends for any
claim of the kind.

**One review finding was false and one was wrong.**
`pub mod stylex_first_that_works` was said to have no cross-crate consumer;
`stylex-transform` names it in five files. And a named function the walk cannot
admit was said to refuse by naming `unbound`; it names the call, `Cannot fold
'map' at compile time`, and the case says that.

**Not done, and why.**

- **`Built::List(Vec<EvaluateResultValue>)`**, the walk-out allocation the
  review proposes. It trades a total type for a partial one: the list would
  hold a value with no total conversion back to an expression, so `into_expr`
  would gain a refusal no case can enter — the shape this very review names as
  the thing to avoid. The saving is one `Vec` per array level, unmeasured, and
  no bench leg answers an object holding an array. Left for the bench leg the
  twelfth instalment deferred.
- **The object-key `String` per property.** Real, and per property per style
  object. Removing it wants an `Atom`-taking factory beside the `&str` one,
  which is new public surface for an unmeasured gain.
- **`to_memo` for a measured `+` chain** copies the accumulated text per link.
  The copy _is_ what the memo stores for that link, so the total is O(n^2) in
  data rather than in work. Removing it means not memoizing the intermediates,
  which is a behaviour change.
- **`is_a_valid_callee_name`** keeps its indirection: it has two callers and it
  is what keeps `VALID_CALLEES` private.

**Two records the instalment above owed.** `nodes/unary_expression.rs` swaps
`unsupported_expression(kind)` for `ILLEGAL_PROP_VALUE` on the `typeof` arm.
The arm is unreachable -- `mod.rs` refuses a big integer and a regular
expression before either becomes a value -- but it is a diagnostic text change
in shipped code and was in no entry. And `engine_fold/guard.rs` hands a
`Callback` with no readable declarator back as `NotACandidate` rather than
refusing with `unfoldable_function`, which is closer to the reference
implementation's `UNSUPPORTED_EXPRESSION('CallExpression')`.

**The two open debts the record flagged are closed by the run.** The gate
reports zero uncovered regions and zero uncovered lines over the workspace, and
`scripts/coverage-missing.sh` -- which merges the instantiations of a generic
before reporting -- agrees. So neither `write_string_of`'s per-instantiation
gap nor `length_of`'s two refusals is producing an uncovered region today.

**The systemic note is answered rather than actioned.** The review counts 149
`assert_deopts` call sites that discard the reason. `assert_deopts` already
fails a refusal that records no reason at all -- which is the class that
matters, since `stylex.create()` turns the reason into the build error -- and
which _sentence_ a refusal gives is a per-case question the suite already
answers with `assert_deopt_reason_contains` wherever the rule is the subject.
Converting the rest would assert a sentence the case is not about.

**Filed rather than fixed.** Ticket 50 covers the three member readings the
tests pin against the reference implementation — a string index, a key with no
string form, and the optional chain this compiler folds where upstream refuses.
Ticket 51 covers the five parity-corpus gaps, all outside this crate.

**The gates.** `cargo check`, `cargo clippy --all-targets` and
`cargo nextest run`, each `--workspace --all-features`: clean.

**2026-09-09, the operator-parity remediation.** The review of `develop..HEAD`
found five real divergences in the operator path, all now closed and all pinned
against the reference compiler through `parity/probe.ts` and eight new
`parity/corpus/modules.json` rows: the four relational operators compared two
strings as two numbers (`'10' < '9'` answered `blue` where the language answers
`red`); a string operand read its number through Rust's float grammar rather
than `StringToNumber`, so `'inf' * 1` was `Infinity` and `'0x10' * 1` refused;
`**` used IEEE `pow`, which answers `1` for `1 ** NaN` where the language
answers `NaN`; `+` decided addition against concatenation on unreduced operands,
so `({ valueOf: () => 2 }) + 1` wrote `[object Object]1` instead of `3`; and
`typeof` named a big integer `object`.

**Three earlier commit messages under-report what they removed.** Recorded here
rather than by rewriting the history:

- `078a7ee49` accounts for the four evaluator-written-value checks and "three
  more readings [that] each asked one question twice". It also removed two
  receiver arms that re-entered the fold — `Expr::Member(..)` and
  `Expr::Ident(..)`, now falling to the terminal `_ => deopt_unsupported!` —
  deleted `SPREAD_HIDES_OBJECT_KEYS` outright, and turned the array-index hole
  refusal into a value, where `develop` refused with `MEMBER_NOT_RESOLVED`.
- The `SPREAD_HIDES_OBJECT_KEYS` removal is sound, and was traced at the
  producer rather than argued from the call site: every receiver comes from
  `evaluate_cached`, the object node returns only `create_object_lit(...)`, the
  array node refuses a hole and a spread before the operand is evaluated, and
  `assert_written_form` asserts the shape at both producer routes. No path
  answers `undefined` where `develop` refused.
- The residual hardening the same review asked for is now in place: the
  amplification guard and the member index read both refuse a slot holding a
  hole or a spread instead of stepping over one, so neither rests on the
  producer contract any more.
