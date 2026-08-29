# 32 — A refusal falls through rather than failing the build

**What to build:** A shape the fold cannot answer for is handed back to the
dispatch below, which answers for it as it always did — rather than being turned
into a deopt that fails a module the merge base compiled.

**Two places it stopped falling through.**

*An argument with no expression form.* `evaluate_callback_args` deopts
`ARGUMENT_NOT_EXPRESSION` when an argument fails `binds_a_parameter`. The merge
base left the parameter unbound and folded the body, and so does upstream:

```js
const first = (fn) => 'red';
color: first(() => 1)      // upstream and merge base: "red". here: a failed build
```

Theme-reference arguments now binding is the improvement this arm was written
for; the regression is the arm's `else`.

*A rule decided before candidacy.* `admit_call` answers `Rule` for the locale,
numeric-receiver and amplification checks before `admit_value(obj)` has said
whether the receiver resolves at all. A call that is not a candidate therefore
reports a rule and stops, instead of declining and letting the dispatch below
try. Two of the three are syntax-only and belong first — that is spec story 37,
*"the guard's cheap refusals run before any binding is resolved"* — but the
amplification check is not, and `review-02-03-04.md` P7 records this exact shape
as fixed: *"candidacy is decided first"*. It was re-split with no note at the
site saying why.

**These are one ticket because they are one question**: what a fold owes the code
underneath it when it declines. The answer the spec gives is that a shape the
bridge does not carry belongs to the dispatch below and the dispatch answers for
it. Both sites currently answer for it themselves, by failing.

## While the file is open

`evaluate` in `nodes/call_expression.rs` is one function of five hundred and
seventy-eight lines holding two structurally identical `FunctionConfigType`
cascades and a repeated `match result_fn` — the same switch on the same type,
twice, in a file this effort rewrote almost entirely. Both fixes above land in
that function, so the member-callee arm comes out here rather than in a ticket of
its own. `_maybe_function` is read twice despite its underscore, and is renamed
for what it holds.

**Blocked by:** 26.

**Status:** resolved

- [x] `first(() => 1)` folds to `red` again, and every argument shape the merge
      base bound still binds
- [x] Candidacy is decided before any rule that needs a resolved value; the two
      syntax-only checks stay first, and the site says which is which and why
- [x] A refusal inside a dynamic style function still leaves the call for the
      runtime — spec story 28, which this must not disturb
- [x] The member-callee arm is its own function, the two `FunctionConfigType`
      cascades are one, and `_maybe_function` is named for what it holds
- [x] Each behavioural shape has a fixture that failed on this branch and folds
      after; the extraction changes no expected value anywhere

## How it was resolved

**The unbindable argument no longer refuses on its own.** An argument with no
expression form binds nothing and the parameter is left unbound, which is what
the language does with an argument nobody passed — so `((fn) => 'red')` applied
to an arrow folds, as it does upstream and on the merge base.

Removing the refusal outright was measured first and gave back the internal note
ticket 22's arm existed to replace: an unfolded body reached the style value as
syntax and `transform_bin_expr_to_number` reported `Left expression is not a
number: Identifier`. So the callback now says whether its body folded —
`EvaluationCallback` answers `Option<Expr>`, and `None` is how it declines. The
caller names the refusal, and `binds_a_parameter` decides which of two sentences
it is: an argument that bound nothing is what an author can change, and a body
that answered nothing with every argument bound leaves only the body to name
(`FUNCTION_BODY_WITHOUT_VALUE`, new).

**The amplification bound moved behind candidacy** in `Walk::admit_call`, with the
site saying that the locale and numeric-receiver checks are syntax and stay in
front. Measured: `nope.repeat(3)` reported a character ceiling where
`nope.padStart(4, '0')` on the same receiver already declined; the three now agree
on the resolution's own sentence. `amplification.rs` no longer claims an ordering
it does not have — a call receiver is left unread because a length is bounded per
link, not because of when the reading happens.

**`evaluate` in `nodes/call_expression.rs`** went from 578 lines to 258. The
member-callee arm is `member_callee`, answering a three-armed `MemberCallee`; the
`Object.keys` static is `global_static_callee`; the two `FunctionConfigType`
cascades are `applied_entry`; the two `match result_fn` copies are `map_method`;
`_maybe_function` is `named`.

Fixtures: `transform_stylex_create_test/refusals_that_fall_through.rs` (22 cases,
measured against `@stylexjs/babel-plugin@0.19.0`) and
`evaluate/tests/fall_through_tests.rs` for the reasons at the evaluator's own
seam. One divergence is written down rather than closed: a *call* as the receiver
of a `repeat` stays unread, so upstream folds `label(() => 1).repeat(3)` where
this compiler names the ceiling.

## Review

Both axes ran. What changed as a result:

- **The `named callback` glossary entry was a wrong definition** and is rewritten:
  it still said a function argument is "refused, naming the call", which is the
  behaviour this ticket removes.
- The nested-`Option` lookup shape is a named `MapEntry`, and `map_method` takes
  the lookup rather than a `bool` flag.
- The three test harnesses answer the callback's `Option` with a `match`, not an
  `unwrap_or_else(|| panic!(..))` — `guidelines/stack/RUST.md` forbids the second
  spelling without qualification.
- The comment at the reordered bound no longer claims the check reads *only*
  resolved values: a written-out count and receiver are syntax, and the cost of
  the new order is one walked receiver on a build that fails either way. Said at
  the site.
- The story-28 fixture asserted `contains("var(--")`, which passes on almost any
  dynamic output. It now also asserts the declined call survives into the runtime
  function as the printer spells it.

Two findings answered rather than changed:

- **`label(() => 1).repeat(3)` refuses where upstream folds `ababab`.** Measured
  in a worktree at the merge point: the same sentence before the reordering and
  after. The walk *admits* a call receiver and refuses on the length it will not
  read, so this is spec story 24's ceiling and not a candidacy question. Pinned
  with that reasoning at the fixture.
- **`apply_own_arrow` reads the arguments without knowing the arity.** True, and
  as fine as it gets from that position: the callback is a closure, so a parameter
  count is not available and an argument nobody has a parameter for reads like one
  that does. A body failing for its own reason beside an argument with no form is
  therefore named for the argument — a true sentence, not the most useful one.
  The limit is stated at the site and pinned by
  `a_body_failing_for_its_own_reason_beside_an_unbound_argument_names_the_argument`.

`stylex_transform` is excluded from the enforced coverage run, so the new
`Unnamed` arms are not gated by it; each is a pre-existing path now named rather
than implicit.

Red before green, measured in a worktree at the merge point: 12 of the 23
transform fixtures fail there and pass here.
