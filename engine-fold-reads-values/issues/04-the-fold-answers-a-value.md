# 04 — The fold answers a value, not an AST node

**What to build:** A fold whose result is an object is usable, instead of
being thrown away.

The fold currently hands back a syntax node, and answers nothing at all when
the result is a plain object — so an expression that computes a nested value
object deopts even though the engine produced it correctly:

```js
color: [['default','red'],[':hover','blue']]
  .reduce((o, [k, v]) => ({ ...o, [k]: v }), {})
```

Answering the evaluator's own value type instead is the prefactor the rest of
this effort rests on. It is also what removes a disagreement already in the
code: one dispatch arm accepts `join` for a value the evaluation produced, and
another refuses it for the array a fold produced. The arms disagree about the
shape of the context rather than about the names, and one value type across
the bridge removes the disagreement rather than reconciling it. Ticket 06 is
where that becomes visible; this is where it becomes possible.

**Blocked by:** 02 — a value the bridge cannot carry has to be able to say so.

**Status:** resolved

- [x] The fold answers the evaluator's own value type rather than a syntax
      node, and every existing caller reads it unchanged
- [x] A plain object result crosses back and reaches the same places an object
      the author wrote would
- [x] An object result is bounded the way an array result already is, and the
      bound is stated in what it costs
- [x] Own-key order of a returned object matches the ordering the object
      evaluation already implements, asserted by a test rather than assumed
- [x] A function, a symbol, an undefined value and a big integer are still
      refused, each with its own reason
- [~] The existing fold tests pass, but **not** unchanged — the ticket's
      premise that no input changes shape was wrong, and four pinned inputs
      moved. See the Answer.

## Answer

`try_fold` answers `EvaluateResultValue` rather than an `Expr`. Each kind
crosses back as what the evaluator's own evaluation of that kind produces:

- a primitive as `Expr(Expr::Lit(…))`, unchanged;
- an array as `Vec(…)`, which is what an array literal evaluates to;
- a plain object as `Expr(Expr::Object(…))`, which is what an object literal
  evaluates to, built from the same `create_ident_key_value_prop` and ordered by
  the same `order_own_keys`.

Answering `Vec` is what collapses the two array dispatch arms. One accepted
`join` for a list the evaluation produced and the other refused it for the array
literal a fold produced, so `['opacity','color'].map(p => p).join(sep)` with a
bound `sep` died at the second link — the outer call falls to the older path,
which then met an `Expr::Array` it had no `join` for. It folds now, to upstream's
`.x1mz1wvm{transition-property:opacity,color}`.

The object half is the one the ticket named, and it is worth more than it looks:
a folded object reaches the condition positions a style value is mostly made of.
`['red'].reduce((o, v) => ({ default: v, ':hover': 'blue' }), {})` emits the same
two rules upstream emits, nesting included. A folded object whose keys are not
conditions is then refused by the position it landed in — `Invalid pseudo or
at-rule`, which is what upstream says too — rather than by the fold.

Own-key order is asserted rather than assumed: `{ b: 1, 2: 2, a: 3, 1: 4 }`
answers `1, 2, b, a`, measured against upstream through `Object.keys(…).join(',')`.

Bounds. An object is bounded by the same `MAX_FOLDED_ENTRIES` an array is, for
the same reason — one AST node per entry, which costs far more as a tree than it
did as a value in the engine — and the two array constants were merged into that
one number. The conversion also grew a nesting bound of its own: a loop inside
the engine can nest a value deeper than any expression the guard admits, and the
conversion recurses on the bare thread stack, so
`'x'.repeat(40).split('').reduce((a, c) => [a], [])` refuses with the depth
sentence instead of overflowing.

Refusals for what cannot cross: a function, a symbol, `undefined`, a BigInt, a
non-plain object, and an object carrying a symbol key each name their own kind.
Three of those are unreachable through today's guard — a symbol and a BigInt are
reachable only through a bare global — and the code says so rather than assuming
them away, since widening the guard is what the rest of this effort does.

`({ a: 1 }).valueOf()` and `({}).constructor()` folded from refusals to values as
a result; both fold upstream, so each was a divergence rather than a boundary.
Two parity corpus rows record the change:
`modules-folded-object-as-a-condition-object` and
`modules-folded-array-reaching-a-declaration`, both `identical`.

### Correction to the last checkbox

The ticket assumed no input changes shape. Four did, and their assertions were
edited rather than left standing. Each was measured against the reference
compiler first, and each turns out to be a divergence this closes rather than a
behaviour it breaks:

| Input | Was | Now | Reference compiler |
| --- | --- | --- | --- |
| `({ a: 1 }).valueOf()` | refused | folds to the object | folds |
| `({}).constructor()` | refused | folds to `{}` | folds |
| `(5).toFixed(2)` | `Unsupported expression: NumericLiteral` | names the rule | rejects, in its own words |
| `[1, 2].filter(1)` | `Unsupported expression: CallExpression` | the engine's `TypeError` | rejects, in its own words |

The first two are ticket 04's own subject — an object result that used to be
thrown away. The last two are ticket 02's: both compilers still reject, and only
the sentence moved, which the harness does not compare.
