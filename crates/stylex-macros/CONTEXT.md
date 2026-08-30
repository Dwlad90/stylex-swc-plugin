# stylex-macros

`macro_rules!` macros for errors, panics, collection literals and the two ways
an evaluation stops. Every user-facing failure in the compiler is spelled
through one of these, which is what makes the `[StyleX]` prefix universal.

## Language

**StyleX panic**:
`stylex_panic!` and its siblings (`stylex_unimplemented!`,
`stylex_panic_with_file!`) — a panic whose message is prefixed `[StyleX]` so a
compiler bug is distinguishable from a panic in SWC or a dependency.
_Avoid_: assert, abort, hard error

**StyleX error**:
A `StyleXError` — the recoverable counterpart, carrying a message plus optional
file, key path and source location. Built by the `stylex_err` /
`stylex_err_with_file` functions or raised by `stylex_bail!` / `stylex_anyhow!`.
Errors propagate; panics do not.
_Avoid_: diagnostic, compile error

**Key path**:
The chain of object keys leading to the offending value, carried on a
`StyleXError` so a message can name `colors.primary` rather than the whole
`stylex.create` call.
_Avoid_: breadcrumb, path, trace

**Confident collection**:
`collect_confident!` — pushes an evaluation result's value into a collection
while it stays confident, and returns `None` from the calling function at the
first result that is not. The evaluator sense of _confident_ is defined in
[stylex-transform](../stylex-transform/CONTEXT.md).
_Avoid_: try_collect, safe collect

**Refusal macro**:
`deopt_unsupported!` and `expr_to_str_or_deopt!` — each records a deopt on the
evaluation state and returns `None` from the calling function. A broken
invariant is `stylex_panic_with_context!` instead, which builds a code frame
and panics. The two are told apart by their state argument; why they are
separate constructs is
[ADR 0002](../stylex-transform/docs/adr/0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md).
The evaluator senses of _deopt_ and _confident_ are defined in
[stylex-transform](../stylex-transform/CONTEXT.md).
_Avoid_: bail, early return, unsupported error
