# A refusal and a broken invariant are separate constructs

**Status:** accepted

The evaluator has two failures, and only one of them is a bug. Each gets its own
construct, and the two read differently at every call site.

`deopt_unsupported!` is the ordinary one: the author wrote something with no
compile-time value, so the expression falls to the runtime. Answering "I cannot
fold this" is part of the contract — it is the counterpart of the reference
implementation's terminal
`deopt(path, state, errMsgs.UNSUPPORTED_EXPRESSION(path.node.type))`. An operand
of `&&`, `||` or `??` is evaluated speculatively under a forked confidence
precisely so that it may fail.

`stylex_panic_with_context!` is the other: the evaluator contradicted something
it had itself just established. Aborting is right there, because continuing
would emit CSS derived from reasoning known to be wrong.

The signatures keep them apart without a reader following the message text. The
refusal takes the `EvaluationState` it records itself on; the panicking one takes
the `StateManager` it builds a code frame from.

## Considered options

**One construct with a flag or a documented convention.** This is what the code
had, and it is what produced the defect the split fixes: an unsupported input
shape aborted the build from inside a speculative operand, over an expression
that was never going to be folded. A convention that both failures are spelled
the same way and told apart by their message gives a reviewer nothing to see.

**Return an error type rather than expanding to `return None`.** The honest
shape, and rejected on blast radius rather than on merit: the evaluator's arms
answer `Option<EvaluateResultValue>` throughout, and threading a result type
through them is a change to every fold in the crate for no change in behaviour.
The macro hiding a `return` follows `expr_to_str_or_deopt!` beside it, which is
the existing convention in the same file.

## Consequences

**A refusal is not silent.** The reason lands on the evaluation state, and a
deopt that reaches a position requiring a static value — inside
`stylex.create()`, say — is reported there with a code frame built from it.

**Adding a panic to the evaluator is now a claim.** It says the invariant broken
is one this code established, and that no author input can reach it. Anything an
author can write is a refusal instead.
