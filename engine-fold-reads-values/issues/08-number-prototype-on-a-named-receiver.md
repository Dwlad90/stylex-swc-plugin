# 08 — `Number.prototype` on a named receiver

**What to build:** Number methods become reachable at all, without losing a
refusal that has to survive.

```js
const n = 255;
export const styles = stylex.create({ x: { content: n.toString(16) } });
```

There is no number method table, so this fails in every position today. The
transport from 05 makes the named case work almost for free — which is
precisely why this is a ticket rather than a line in 05's acceptance. The
interaction that needs proving is the one that could be lost by accident:

```js
(1.5).toFixed(1)   // must KEEP failing, in both compilers
```

The reference compiler applies the method without a receiver on a numeric
literal and throws, so folding it here would emit a declaration for a module
that build rejects. A number a fold *produced* is a different shape and folds
in both, as is a negated literal, which is not a literal at all.

**Blocked by:** 05.

**Status:** resolved

- [x] Number methods fold on a named receiver, matching the reference
      compiler's declaration text and class name
- [x] A method call on a numeric literal written into the source is still
      refused, and the corpus records that both compilers reject it
- [x] A number a fold produced is a valid receiver
- [x] A negated numeric literal is a valid receiver

## Answer

The surface was measured method by method against `@stylexjs/babel-plugin`
0.19.0 and agrees byte for byte — every class name and every rule text — so the
mechanism 07 forced really did cover this, and the four checkboxes needed
proving rather than building. One defect fell out of the measuring, and it is
the only production change here.

### The one thing that was wrong: a declined number call did not name its rule

`const n = 255; n.toFixed(undefined)` folds upstream and refuses here, which is
a generic `undefined`-as-an-argument gap and not this ticket's. What *was* this
ticket's is the sentence it refused with: `Unsupported expression:
NumericLiteral`, which tells an author only that they wrote a number.

The dispatch behind the fold keeps a list of receivers whose prototypes the fold
owns whole, and a call arriving there on one of them is named for the rule that
declined it rather than for the receiver's node kind. The list was a string and
an array. 07 made a number and a boolean carryable receivers whose prototypes
fold through the engine exactly as a string's does, and never added them — so
the two primitives it admitted were the two that lost ticket 02's contract.
Both are on the list now.

Two things came with it. The early bail that answered `Unsupported expression:
BooleanLiteral` for a boolean receiver *written out* is gone: it predates the
fold, and all it did afterwards was give a written boolean a worse sentence than
a named one, which is position deciding the answer again. And
`n.toFixed(window.x)` now reads `Referenced constant is not defined.` — the
reference compiler's own sentence for that input, reached only once the receiver
stopped answering first.

Removing that bail changes the sentence and not the verdict, which was worth
measuring rather than reasoning about: the bail sat behind the fold, so it was
only ever reached once the fold had *declined*, and a call the fold accepts
never saw it. Measured both ways with the production file reverted and restored,
`true.toString()`, `(true).toString()`, `false.valueOf()` and
`true.toString().toUpperCase()` fold to byte-identical declarations before and
after. The first and last are pinned in
`a_declined_number_call_names_the_rule_that_declined_it` beside the refusals, so
the pair reads as the one claim it is.

### What was measured

Pinned in `number_prototype_receivers.rs`, sixteen tests:

- The five folding methods of `Number.prototype`, each with an argument and,
  where the language allows one, without — `toPrecision()` is `toString()` and
  `toPrecision(4)` is not. Plus the three `Object.prototype` methods a number
  inherits, which no number method table would have listed.
- Receivers that hold a number without being one: a negated literal, a unary
  plus, a double negation, arithmetic, an element, a property, a nested
  property, a name of a name, a `let` nothing writes to, a callback parameter,
  `NaN` and `Infinity`, a shadow of the `Number` global, and a name spelled
  `toFixed`.
- The written-literal refusal in ten positions — int, float, exponent, hex,
  numeric separator, receiver of a longer chain, argument to another fold,
  inside a callback, inside a fallback array, under two conditions.
- The numeric edges, where the two number-to-string paths could part company:
  `-0`, `1e21`, `1e-7`, `MAX_VALUE`, `MIN_VALUE`, `MAX_SAFE_INTEGER` at radix
  36, `0.1 + 0.2` to twenty digits, an integer literal past `f64` precision,
  and an overflow to `Infinity`.
- The ceilings, which are the language's rather than this compiler's: a number
  is sixty-four fixed bits, so the only thing that grows is the digit count an
  argument asks for, and the language caps every one of those at a hundred. A
  number receiver has no amplifying method at all.
- The arguments the language itself refuses — radix 1, 37 and -16, digits -1
  and 101, precision 0 and 101 — which throw in both compilers.
- Scale and chains: two hundred named numbers converted one by one, a chain
  crossing number to string to array to string to number, and four sweeps that
  walk a whole argument range inside a callback.
- The boundaries, each with the reference compiler's behaviour recorded beside
  it: `toLocaleString` refused for the locale data the engine does not carry,
  `constructor`/`call`/`apply` refused as reads that escape onto the function
  graph, and `BigInt`, a reassigned binding and an amplifying call on a folded
  receiver keeping their own rules.

### One stale claim removed

`named_string_receivers::a_template_hole_is_coerced_the_way_the_reference_compiler_coerces_it`
carried a comment saying a *named* template hole holding a number refuses here
where upstream folds it, and named this ticket as the owner of that gap. It was
written before 07 and stopped being true with it. The named hole is a case in
that test now rather than a sentence about a gap: `const n = 1e21;` in
`` `${n}px`.trim() `` folds to `1e+21px`, the same declaration the literal hole
builds — which it has to, since two spellings of one value that hashed
differently would be two class names.

## Comments

### From 07

Most of the mechanism landed there, forced rather than chosen. A static reads its
arguments as ordinary expressions, and `Math.round(BASE / Math.pow(SCALE, 3) / 0.16)`
— the `global-tokens` fixture, a fluid type scale — has a binding at every
operand. Carrying a number as an *element* while refusing one as a *name* would
have been position deciding the answer again, which is exactly what 07 removed.

So `is_a_carryable_receiver` admits numbers and booleans now, and
`const n = 5; n.toFixed(1)` folds to `5.0`, matching the reference compiler.
Pinned as
`named_array_receivers::a_name_holding_a_number_is_a_receiver_and_a_written_one_is_not`,
which also re-proves the refusal this ticket exists to protect:
`(5).toFixed(1)` still refuses, in `receiver_is_a_written_number`, because that
rule reads how the receiver was *written* and a name is not a literal.

What is left here is the proving rather than the mechanism: the surface measured
method by method against the reference compiler, the corpus row saying both
compilers reject a written literal, and the two receivers that are not literals
at all — a number a fold produced, and a negated literal.

### For 14

Three rows, all measured.

**The `Number.prototype` surface folds and wants a row.** Five methods on a
named receiver, agreeing with the reference compiler on class name and rule
text. There is no row for it at all today.

**`toLocaleString` on a *number* is the locale category's fourth reason, and it
is the one that matters.** `modules-06-locale-string-on-an-object` records the
object cost of the exclusion; on a number the method genuinely formats —
`(1234.5).toLocaleString('de-DE')` is `1.234,5` in the language — and upstream
answers it from the host's own locale, so upstream's own answer is
machine-dependent. That is the reason the parent ticket's fourth bullet names,
and no row carries it yet.

**Two number-receiver divergences are agreement-not-wanted rather than gaps.**
`n.constructor('5')` and `n.toString.call(n)` both fold upstream and are refused
here by the escaping-property rule — the same boundary that keeps
`''.constructor.constructor('return 1')()` out of the compiler. Pinned as
`number_prototype_receivers::a_read_that_escapes_a_number_is_refused`.

### For whoever owns the argument surface

`undefined` written as an argument refuses on every receiver —
`'abc'.slice(undefined)`, `['a','b'].join(undefined)`, `n.toFixed(undefined)` —
and the reference compiler folds all three. It is not number-specific and not a
receiver question: the guard resolves the name, gets nothing carryable back, and
the call stops being a candidate. Whoever widens what a name may hold should
pick it up. The sentence it refuses with is correct now, which is all this
ticket owed it.
