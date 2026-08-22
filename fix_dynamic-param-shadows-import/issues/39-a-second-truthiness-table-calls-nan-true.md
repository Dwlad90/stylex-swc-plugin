# 39 — A second truthiness table calls `NaN` true

Status: `resolved`
Blocked by: None

**What was found:** A ternary whose test is `NaN` chooses the wrong branch. The
value is falsy, both compilers agree it is a number, and this compiler takes the
consequent anyway.

```js
export const styles = stylex.create({
  named:    { height: NaN ? '1px' : '2px' },
  computed: { height: (0 / 0) ? '1px' : '2px' },
});
```

| | Babel 0.19.0 | here |
| --- | --- | --- |
| `NaN ? '1px' : '2px'` | `height:2px` | `height:1px` |
| `(0 / 0) ? '1px' : '2px'` | `height:2px` | `height:1px` |
| `NaN \|\| '2px'` | `height:2px` | `height:2px` |

The third row is the interesting one: a logical operator gets it right where a
ternary does not, on the same value.

## The mechanism

There are two truthiness tables.

`coercions::to_js_boolean` is the one the logical operators read, and it is
correct — it names the trap in a comment beside the arm:

```rust
// Both zeroes and `NaN` are the falsy numbers. `-0.0 != 0.0` is already
// `false`, but `NaN != 0.0` is `true` — every comparison against `NaN` is
// false *except* the inequality, so it has to be named rather than left to
// fall out of the arithmetic.
Expr::Lit(Lit::Num(num)) => Some(num.value != 0.0 && !num.value.is_nan()),
```

`convert_expr_to_bool` in `shared/utils/ast/convertors.rs:316` is the one the
ternary reads, and it is the same table written again without that half:

```rust
Lit::Num(n) => n.value != 0.0,
```

So this is the Repeated Switches shape rather than an arithmetic slip: the
second copy was written before the coercion crate existed and has drifted.

## What else the second copy gets wrong

Reading it beside the first turns up more than the `NaN` row, none of it
measured yet:

- `UnaryOp::Minus | Plus | Tilde` all answer `!convert_expr_to_bool(arg)`, so
  `-1 ? a : b` reads `-1` as falsy. Only `Bang` is a negation; the other three
  are arithmetic and should read the operand's *value*.
- `UnaryOp::TypeOf => true` is right by accident — every `typeof` result is a
  non-empty string — but it is stated rather than derived.
- The `_` arm of the literal match aborts with `stylex_unimplemented!` where the
  coercion refuses, so a big integer or a regular expression in a ternary test
  ends the build rather than deopting. That is the split
  `adr/0002` is about.

## The shape a fix would take

Delete the second table and have `convert_expr_to_bool`'s callers read
`coercions::to_js_boolean`, which already answers every arm above. The two
signatures differ — the coercion is a total function over an already-evaluated
expression returning `Option<bool>`, and this one takes the state and the
function map so it can resolve an identifier first — so the seam is "resolve,
then coerce" rather than a straight substitution.

- [x] Each row of the second table is measured against upstream before it is
      deleted, the unary ones especially
- [x] The `Option<bool>` refusal reaches the ternary as a deopt rather than as
      an abort
- [x] Corpus rows for the `NaN` ternary and for a negative-number ternary
- [x] `globals_as_style_values`'s row is kept and its comment re-written — the
      snapshot never moved, because the `NaN` arm was corrected ahead of this
      ticket

## What was done

`convert_expr_to_bool` is deleted. `nodes/conditional_expression` and the `!`
arm of `nodes/unary_expression` read `evaluate_result_to_js_boolean`, the
existing coercion bridge the logical operators already read, so there is one
truthiness table and one bridge over it.

The seam turned out to be wider than "resolve, then coerce". Both askers took
the truthiness of an *expression*, and the evaluator holds values in shapes that
have no expression form -- an array is its own vector, a fold is a function map.
Each stands for an object and every object is truthy, so requiring an expression
refused the build on `[] ? a : b`, on an arrow, on `undefined`, on `void 0` and
on a fold. The bridge answers all of them, so the askers now read the evaluated
*value* and every one of those rows agrees with upstream.

### The unary rows

Measured before deleting, and none of them was reachable. `Minus`, `Plus` and
`Tilde` in the second table answered `!convert_expr_to_bool(arg)`, which reads
`-1` as falsy -- but `nodes/unary_expression` folds those three to a number
through `evaluate_unary_numeric` before any truthiness question is asked, so the
arms only ever saw a value that had already been folded. `-1 ? a : b`,
`~0 ? a : b` and `~(-1) ? a : b` agreed with upstream before the change and
agree after it. The four unit tests that asserted the wrong reading were
asserting an unreachable arm; they are gone, and the reachable behaviour is
pinned end to end in `truthiness_table::the_falsy_numbers_reached_by_arithmetic`.

`TypeOf => true` is likewise unreachable: `typeof x` folds to a string first.

### The abort

The `_` arm's `stylex_unimplemented!` is gone with the table, and the coercion's
`None` reaches both askers as a deopt. It changes no verdict, because upstream
aborts on the same inputs: a big integer or a regular expression in a ternary
test stops both builds with `Unsupported expression: BigIntLiteral` /
`RegExpLiteral`, refused by the evaluator before the truthiness question is
reached.

### Where it is pinned

`transform_stylex_create_test::truthiness_table`, nine cases, every one measured
against `@stylexjs/babel-plugin@0.19.0` and byte-identical to it including class
names. Corpus rows `modules-1266-a-ternary-reads-the-one-truthiness-table` and
`modules-1266-a-value-with-no-expression-form-as-a-ternary-test`.
`modules-1266-a-folded-function-map-read-as-a-condition` moved from
`acceptance-divergent` to `identical`, and
`invalid_values::a_static_fold_read_as_a_condition_is_refused` is gone -- that
input folds now.
