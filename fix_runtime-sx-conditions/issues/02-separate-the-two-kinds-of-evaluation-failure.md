# 02 — Separate an unsupported shape from a broken invariant

Status: `resolved`
Phase: Phase 1

**What to build:** The evaluator stops aborting builds from inside evaluations
that are allowed to fail.

Two unrelated failures share one construct today:

- The evaluator does not fold this input shape. That is an ordinary answer, and
  the reference implementation gives it by deopting —
  `deopt(path, state, errMsgs.UNSUPPORTED_EXPRESSION(path.node.type))` at
  `evaluate-path.js:1055`.
- The evaluator's own reasoning broke. That is a bug, and aborting is right.

Both are `stylex_panic_with_context!`, which is why the first kind ended up in a
position that requires the second kind never to happen. See `../spec.md` for
how `1322be8c1` made it reachable.

Introduce a distinct construct for the first kind — a `deopt_unsupported!` or
equivalent — so the two read differently at every call site. A doc note would
not have prevented this; the convention is exactly what failed. A type that
cannot express "abort" would be better still, but that is a rewrite of the
evaluator's signature surface and does not belong in a regression fix.

Then audit every `stylex_panic*` site under
`crates/stylex-transform/src/shared/utils/js/evaluate/` — roughly fifteen —
and classify each. After `1322be8c1` any of them is reachable from any
`&&` / `||` / `??` operand, so leaving the four call-expression arms fixed and
the rest alone just changes which input reports the next issue.

The four arms that fire on the reported input, all in
`nodes/call_expression.rs`: the array-method arm, the `Expr::Array` method arm,
the string-method arm, and the `_` catch-all for expression types.

Sites that turn out to be genuine invariant violations stay panicking. Record
the classification in the review, not just the diff — the judgement is the
deliverable, and a wrong call in either direction is invisible afterwards.

Do not revert `nodes/logical_expression.rs`. It matches
`evaluate-path.js:829-831`, which evaluates the right operand unconditionally
so it can name which side deopted.

## Comments

### The audit, and the classification it produced

`deopt_unsupported!` lives beside `stylex_panic_with_context!` in
`shared/utils/macros/evaluation.rs`. The two take *different state*, which is
what makes them read differently at every call site without a reviewer having
to follow the message text: this one takes the `EvaluationState` it records the
refusal on, the panicking one takes the `StateManager` it builds a code frame
from.

The site count was larger than the ticket's estimate. `evaluate/` held 144
panic-family invocations, not fifteen — the fifteen were the ones whose message
said "not yet supported", and every one of the rest was reachable from the same
`&&` operand.

**Deopted — an unsupported input shape (129 sites).** The criterion applied: can
this fire on input a user can write, where the evaluator has not itself already
established the fact being contradicted? Grouped by file:

- `nodes/call_expression.rs` — the four arms the reported input reaches (array
  method, `Expr::Array` method, string method, expression-kind catch-all), the
  `Math` / `Object` built-in dispatch and their argument shapes, every
  `FunctionType` that cannot be applied in the position it was reached in, and
  the `Object.fromEntries` entry shapes.
- `nodes/member_expression.rs` — every property lookup that cannot be read: a
  non-numeric array index, a key with no string form, a spread that leaves the
  object's own keys unknown, a receiver the member path reads no properties
  from, and an unconfigured or absent `stylex.env` entry. The `props.iter()
  .find()` was rewritten as a loop: a predicate has no way to say "refuse", so
  the closure had to abort, which is the failure this split exists to remove.
- `nodes/object_expression.rs` — a spread with no object form, a getter/setter,
  a value with no expression form.
- `nodes/unary_expression.rs`, `nodes/conditional_expression.rs` — an operand or
  a test with no compile-time reading.
- `nodes/identifier.rs`, `mod.rs` — an index-map binding, a tagged template, a
  computed key that folded to a value with no expression form.
- `helpers.rs` — the three helpers whose return type could not express a
  refusal (`evaluate_result_vec_to_array_expr`, the two
  `normalize_js_object_method_*`, `args_to_numbers`) now return `Option`, and
  their callers deopt.

One site outside `evaluate/` had to move with them:
`utils/ast/convertors.rs::expr_to_num` returns a `Result` and aborted inside it
anyway, so `Math.abs({})` and `-{}` aborted no matter what the caller did. It
now reports through the `Result` it already had.

**Kept panicking — a broken invariant (15 sites).** Each contradicts something
established immediately above it, or something the grammar guarantees:

| site | why it is an invariant |
| --- | --- |
| `mod.rs` `Expr::Paren` | `normalize_expr` unwraps parens before dispatch |
| `mod.rs` identifier not resolvable | guarded by `normalized_path.is_ident()` |
| `mod.rs` import specifier | the ident was found *by* `get_import_by_ident` |
| `object_expression.rs` key must be present | every `PropName` arm answers `Some`; the computed one deopts |
| `binary_expression.rs` `unwrap_or_panic!` | the chain ends in `Ok(Null)`, so the `Result` cannot be `Err` |
| `binary_expression.rs` operand with no value | documented at the site; pinned by `an_operand_with_no_value_refuses_rather_than_aborting` |
| `sequence_expression.rs` empty sequence | the grammar requires two operands |
| 8 × `stylex_unreachable!` in `call_expression.rs` | each guarded by an `is_*` check on the line above |

### Two behaviour changes that are not the panic/deopt split

Both were found by the new tests and are called out because a reviewer should
agree with them separately:

1. **An array hole in a method receiver now refuses rather than aborting** —
   but it refuses rather than folding. `[, 1].join('-')` is `"-1"` in
   JavaScript and the evaluator's array representation carries no `undefined`,
   so answering would have written `"1"` into the stylesheet. A wrong value is
   worse than a declaration that falls to the runtime.
2. **`"abc".charCodeAt(10)` refuses** where it aborted. JavaScript answers
   `NaN`; folding to that is the prototype-surface work in issue 06, not this.

### Found on the way, not fixed here

`nodes/array_expression.rs` drops array holes with `.flatten()`, so an array
that reaches a method through a binding answers `['a', 1]` for `[, 'a', 1]`.
That makes `[, 1].join('-')` fold to `"1"` where the reference implementation
gives `"-1"`, and it would make `.length` answer `1` instead of `2`. It changes
emitted CSS, so it belongs with issue 01 rather than with a regression fix.

### Tests

- `evaluate/tests/unsupported_shape_tests.rs` — the reported input, its three
  logical-operand positions, and a sweep that puts every unfoldable shape in
  each of them. Paired throughout with the folds each refusal must not have
  broken, so "refuse everything" cannot pass.
- `transform_stylex_create_test/logical_operators.rs` — the reporter's `sx`
  shape end to end, and the per-operator property behind it.
- Three existing tests asserted the *old* aborting behaviour and now assert the
  refusal; `tests/evaluation/evaluation_module_transform.rs` renders a deopted
  expression unchanged instead of panicking, which is what the compiler does.

### Review, and what it caught

Both review axes independently found the same defect, which is recorded here
because it is the exact failure mode this ticket exists to prevent and it was
introduced *by* the fix.

`normalize_js_object_method_array_arg` answers the object form of an
`Object.keys`/`values`/`entries` receiver. Its arms already skipped an element
they could not read, so converting the one arm that used to abort into a skip
looked locally consistent — and made `Object.keys([x => x])` fold to `[]` where
JavaScript has one own key. A refusal turned into a *silently shorter value*:
the build passes, and the stylesheet gets something the source never described.

Fixed by giving the receiver three answers rather than two, because "no own
keys" and "cannot be read" are both spelled by an absent object and mean
opposite things:

```rust
pub(super) enum ObjectMethodReceiver {
  Object(ObjectLit),
  NoOwnKeys,   // `Object.keys(5)` is `[]` — folds
  Unreadable,  // an element with no expression form — refuses
}
```

The three call sites shared one `or_else` chain that had to agree on this and
was edited separately, which is the shape of the original bug; they now go
through one `normalize_object_method_receiver`. Note this also changes the
pre-existing `_ => continue` arm to refuse — a silent drop that predates the
split, in the same wrong-value class.

Pinned by `an_unreadable_receiver_element_refuses_rather_than_shortening_the_list`,
paired with `a_readable_object_method_receiver_still_folds` so the two absent
cases cannot be collapsed again.

Also from the review, both judgement calls rather than defects:

- `EvaluateResult::refused(deopt, reason)` replaces the six-field bail-out
  record spelled three times in `evaluate_obj_key`.
- The doc on `evaluate_result_vec_to_array_expr` claimed "every caller refuses
  the value" while one caller dropped it. The claim is now true and says why a
  shorter array is not an acceptable answer.

Declined: a `DeoptReason` newtype for the macro's reason argument. `deopt` takes
`&str` throughout the evaluator, and changing its signature surface is the
rewrite issue 02 explicitly rules out ("a type that cannot express \"abort\"
would be better still, but that is a rewrite of the evaluator's signature
surface and does not belong in a regression fix").
