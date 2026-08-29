# 03 — Mutation disqualifies the binding, not the method

**What to build:** Sorting or pushing a list the author wrote out produces a
declaration, as it does under the reference compiler.

```js
transitionProperty: ['b','a'].sort().join(',')   // must fold to a,b
zIndex: ['a','b'].push('c')                      // must fold to 3
transitionProperty: 'b,a'.split(',').sort().join(',')  // must fold to a,b
```

All three are refused today, and all three fold under the reference compiler.
The refusal was added on the reasoning that matching it would carry mutation
into an otherwise pure evaluation. Measured, that reasoning does not hold: the
reference compiler does not refuse mutating methods at all. It folds them on
any receiver not reachable by name, and instead disqualifies the **binding** —
its mutation test walks a binding's references with no position check, so a
binding a mutating method touches anywhere is dead for the whole file in both
directions, including reads that come before the mutation.

So the engine only ever mutates a temporary nothing can name afterwards, which
is unobservable. The rule that belongs here already exists in the reference
resolution chain; this ticket removes the wrong rule and proves the right one
already matches.

Note for whoever picks this up: the commit that added the refusal is on
`develop`. This reverses it deliberately, with the measurement above as the
reason, and the commit message should say so.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The fold no longer refuses a method because it mutates its receiver
- [x] The three shapes above fold to the same declaration text and the same
      class name as the reference compiler
- [x] A binding a mutating method touches stops folding, and so does a read
      that appears *before* the mutation
- [x] A reassigned binding stops folding, by the separate rule that already
      covers reassignment
- [x] A mutating call on an intermediate value — the result of another call —
      still folds, because no binding is involved
- [x] The corpus entries that pinned the mutating divergence are re-measured
      and now record agreement

## Answer

The `is_mutating_array_method` refusal is gone from the fold guard. Measured
against `@stylexjs/babel-plugin` 0.19.0, every claim in the ticket holds:

| Input | Reference compiler | Here, now |
| --- | --- | --- |
| `['b','a'].sort().join(',')` | `.x1iq4t92{transition-property:a,b}` | identical |
| `['a','b'].push('c')` | `.xzkaem6{z-index:3}` | identical |
| `'b,a'.split(',').sort().join(',')` | `.x1iq4t92{transition-property:a,b}` | identical |
| `['a','b'].reverse().join('-')` | `.x1y9cpk8{content:"b-a"}` | identical |
| `const a=['b','a']; a.sort();` then `a.join(',')` | refuses | refuses |
| the same read placed *above* the `a.sort()` | refuses | refuses |
| `let a=…; a=['c'];` then `a.join(',')` | refuses | refuses |

So the binding rule that replaces the method rule already existed and already
matched; nothing was added to binding resolution. The mutating call on an
intermediate value folds because no binding is involved, which is the third row
above.

The harness was run against `@stylexjs/babel-plugin` 0.19.0 on a rebuilt
`dist/`: 1123 subjects, **0 changed, 0 unexpected**.
`modules-06-mutating-array-method` in the parity corpus flips from
`acceptance-divergent` to plain agreement, and a new row,
`modules-mutated-binding-read-through-a-method-call`, records the half that
still refuses — the read placed above the mutation, which is the position a
disagreement would be least visible in.

Transform-level cases are in
`crates/stylex-transform/tests/transform_stylex_create_test/mutating_methods_and_bindings.rs`,
including the control that an untouched binding beside them still folds. Note
what the four binding cases pin: reaching the mutated binding *through a method
call* refuses with `Unsupported expression: CallExpression` on **both**
compilers, because the receiver's own refusal does not travel out of the
receiver's evaluation. The same binding read without a call names the rule on
both, and that case is pinned beside them so the four are not read as pinning
the wrong sentence.
