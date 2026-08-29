# 12 — A receiver reached through a binding gets none of the prototype surface

Status: needs-triage
Phase: Deferred

**Filed from:** 06, which closed the gap for a receiver written as a literal and
left this half open. 06's own "unify the two array dispatch arms" instruction is
the first half of it.

**What to build:** A method call folds the same way whether its receiver is
written into the expression or reached through a binding.

06 put the engine in front of a call whose receiver *carries its own value*, so
the guard reads syntax. `const V = ["a","b"]` is not syntax the guard can carry:
the value exists, but only after the evaluator has resolved the identifier, by
which point the call has already fallen through to the two method tables 06 was
meant to make unnecessary.

Measured against `dist/` at 06's completion, all through a `const` binding in
the same module:

| written                            | result                                            |
| ---------------------------------- | ------------------------------------------------- |
| `V.join(", ")`                     | folds                                             |
| `V.map(x => x + "px").join(" ")`   | `The method 'join' is not yet supported`          |
| `V.at(0)`                          | `The array method 'at' is not yet supported`      |
| `V.includes("a")`                  | `The array method 'includes' is not yet supported`|
| `V.slice(1).join("")`              | `Unsupported expression: CallExpression`          |
| `S.startsWith("doc")`              | `The method 'startsWith' is not yet supported`    |
| `S.toUpperCase()`                  | `The method 'toUpperCase' is not yet supported`   |

The reference implementation folds every one of them, so each is an acceptance
divergence this compiler now carries in the *narrower* case only. The second row
is the exact example 06 cited when it asked for the unification, so that bullet
is unfinished rather than moot: the engine bypasses the divergent tables when
the receiver is a literal and cannot when it is not.

## Why 06 did not simply unify them

Not a one-line change, which is worth recording so the next attempt does not
start by assuming it is. The two arms in `nodes/call_expression.rs` disagree
about the *shape* of `context`, not only about which method names they accept:

- The `EvaluateResultValue::Vec` arm sets `context` to the flat list of element
  values, and accepts `map`, `filter` and `join`.
- The `Expr::Array` arm wraps that list in one more level —
  `vec![EvaluateResultValue::Vec(receiver)]` — and accepts only `map` and
  `filter`, routing `join` into a deopt.

`evaluate_join` iterates its context calling `as_expr()` on each entry, so it
needs the flat shape; `evaluate_map` and `evaluate_filter` read
`context.first()` expecting the nested one. Accepting `join` in the second arm
without reconciling the shapes turns a deopt into a panic. So unifying means
agreeing one shape across `evaluate_map`, `evaluate_filter` and `evaluate_join`
and both arms — which is the same "two tables that must agree and are edited
separately" complaint one level down, and it deserves its own change rather than
riding along with 06.

## Two ways to build it, and the one worth measuring first

1. **Unify the arms and widen `ArrayJS`.** Reconcile the context shape, then add
   the missing names. This fixes arrays and leaves strings where they are:
   `StringJS` carries `concat` and `charCodeAt` and nothing else, so a bound
   string still gets almost nothing. It also rebuilds by hand exactly what the
   engine already answers, which is the argument 05 settled against.
2. **Fold on the resolved receiver.** Once the evaluator has resolved the
   receiver to a literal, hand *that* to the engine — the same fold 06 built,
   re-entered with the resolved value substituted for the identifier. Arrays,
   strings and objects all land at once, chains keep working because each link
   resolves to a literal, and both method tables plus `ArrayJS`, `StringJS` and
   the two arms become dead code to delete.

Option 2 is the one to price. The seam it needs is the point where
`call_expression::evaluate` already knows the receiver's value, so the question
is whether the fold can be re-entered there without evaluating the receiver
twice — and whether every guard boundary 06 states still applies to a receiver
the source did not spell, the nesting bound especially, since a resolved value
can be far larger than the expression that named it.

Do not start it without answering that last point: a bound array of ten thousand
elements substituted into printed source is the memory hazard 06 bounded, minus
the bound.
