# 47 — A dead operand is printed deeper than it is walked

**What is wrong:** The stack the fold claims for the print and the parse is
sized from `maxEvaluationDepth`, which bounds the guard's *walk*. The printed
source is not bounded by it. An operand a short circuit never reaches is printed
whole — the engine decides the short circuit itself — so SWC's printer and the
engine's parser both descend through nesting the guard never spent a level on.

**Measured, on the shipped default ceiling of 32.** `['a', false && [[…'x'…]]]
.join('-')` folds at 200 levels of dead nesting and aborts the process with
`SIGBUS` at 300. Not a diagnostic, not a refusal: the failure mode the whole
ceiling exists to prevent, reached by an input the ceiling does not look at.

**It is not new.** The claim has been `ceiling × 2 × 64 KiB` since it was
introduced, and the arithmetic is unchanged by 37 — 37 only moved where it is
made. What 37 added is the name for the factor: `UNWALKED_NESTING`, whose doc
says plainly that it is a margin and not a bound, and the case in
`short_circuited_walk_tests.rs` that says why 200 is the number written there.

**Why the obvious fixes are not obviously right.**

- *Spend the ceiling on the dead operand anyway.* Deletes the behaviour
  `a_dead_operand_deeper_than_the_ceiling_is_never_entered` pins, which folds
  input the reference implementation folds.
- *Print only the operand that runs.* The guard already knows which one that is.
  It changes what the engine is handed, so every rule that reads the printed
  source has to be re-argued.
- *Claim from the text rather than from the ceiling.* Sound and cheap to state —
  the deepest run of unclosed brackets in the printed source — but a string
  literal full of brackets over-claims wildly, so it needs a lexer rather than a
  scan.

**Blocked by:** none.

**Status:** resolved

- [x] A dead operand nested past what the claim covers refuses or folds — never
      aborts
- [x] The number in `short_circuited_walk_tests.rs` stops being a number the
      claim happens to cover

## What was done

The third option, made sound: the claim is taken from the text, but from the
*tree* rather than from the printed characters — so a string full of brackets
claims the one level it is, and no lexer is needed.

`nesting_of` in `growable_stack.rs` counts the descent the printer and the
parser each make, at the three node kinds that nest without bound: an
expression, a statement and a binding pattern. Everything between two of them is
a fixed number of frames. Expressions alone were not enough, and the gap was
found in review rather than in a build: a dead callback whose body is four
hundred nested `if` statements reads as three levels and aborted the process.
The count asks for room at every level as every walk this compiler owns does, so
measuring a deep tree cannot itself overflow.

The guard calls it at the two places it declines to enter an operand — the dead
side of `&&`/`||`/`??` and the untaken arm of `?:` — and records the deepest
reach on the reader. `fold` claims for the deeper of that and the ceiling.

`UNWALKED_NESTING` is gone: the margin it stood for is now measured, so
`LARGEST_CLAIM` halves to 512 MiB and the assertion follows. `DEEPEST_CARRIED`
and `carriable` name the limit in the module that owns the claim, rather than
leaving the caller to compare against a constant from another crate.

Text nesting past that limit is refused by `nesting_too_deep_to_carry`, a
sentence of its own rather than the depth ceiling's: that one counts fold levels
and a source level is not always one of them, so quoting text nesting through
its wording would have told an author to shorten something they had not written.

**Measured against the reference implementation.** It folds 200 and 300 levels,
both conditional arms, the nested-statement and nested-pattern shapes to the
same values this compiler now does, and dies with `Maximum call stack size
exceeded` at 2000 and on a 400-level pattern.

Covered by `a_dead_operand_deeper_than_the_ceiling_is_never_entered`,
`a_dead_arm_deeper_than_the_ceiling_is_never_entered`,
`a_dead_operand_inside_a_dead_operand_is_measured_with_it`,
`a_dead_operand_of_nested_statements_is_measured_too`,
`a_dead_operand_of_nested_patterns_is_measured_too`,
`brackets_written_inside_a_dead_string_claim_nothing`,
`a_live_operand_deeper_than_the_ceiling_still_refuses` and
`a_dead_operand_past_the_largest_claim_refuses`; nine unit cases on `nesting_of`
in `growable_stack_tests.rs`; a message-shape case in `evaluation_errors_test.rs`;
and two whole modules —
`a_dead_operand_nested_far_past_the_default_ceiling_still_folds` and
`a_module_nested_past_the_largest_claim_refuses_rather_than_aborting`. ADR 0004,
the two glossary entries and the `MAX_EVALUATION_DEPTH_LIMIT` doc record the
change.
