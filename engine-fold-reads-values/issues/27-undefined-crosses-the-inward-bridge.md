# 27 — `undefined` crosses the inward bridge

**What to build:** A named array or object holding `undefined` folds, so the
value a binding carries reads the same as the value written out.

```js
const a = ['a', undefined, 'b'];
content: a.join('-')      // upstream "a--b"; here, a refusal
```

**It is a fifth refusal nobody decided on.** The spec names four, each argued
and recorded in ADR 0008. This is not one of them — `Carried` simply has no
`undefined` arm, so `Inward::expr` falls to `NotACandidate` and the whole fold
declines. The shape is otherwise fully supported: the written-out literal
`['a',undefined,'b'].join('-')` folds today, `String(a)` and `` `${a}` `` fold
today, and `null` in the same position folds today. Only the named-binding path
is missing it, which is the exact gap the effort exists to close.

**Reached through more than `join`.** Every array method that renders an element
sees it, and so does `Object.keys` over an object with an `undefined` value.
`void 0` is the same value under a different spelling and must answer the same.

**Blocked by:** 26.

**Status:** resolved

- [x] `Carried` holds `undefined`, and both conversions — into the engine and
      back out — answer for it
- [x] `void 0` and the bare identifier reach the same value
- [x] An `undefined` nested inside an array inside a binding crosses too
- [x] `const o = {a: undefined}; Object.keys(o).join(',')` folds
- [x] The measured answers match `@stylexjs/babel-plugin` 0.19.0, and the corpus
      carries a row for the binding shape

## Answer

`Carried::Undefined`, reached from one arm in `Inward::expr` and built as
`JsValue::undefined()` in `to_js`. One arm answers both spellings, because the
evaluator has already turned `void 0` into the `undefined` identifier and hands
back the same node an author's bare name reaches it as. A binding that shadows
the name never arrives — the evaluator refuses a shadowed `undefined` ahead of
every reader of that shape, which is what makes matching the identifier safe
here, and both compilers refuse that input.

### The fifth refusal had a twin on the way out

The outward direction refused `undefined` too, under the same
`unfoldable_fold_result` sentence, and that was the same kind of omission: it is
not one of the four categories ADR 0008 argues for. Measured against
`@stylexjs/babel-plugin` 0.19.0, upstream folds every shape it covered —
`String("abc".at(99))`, `[1,2].at(99)`, `["a"].find(x => x === "z")`,
`` `${…}` `` over any of them — six of seven measured cases, and the seventh
(`content: "abc".at(99)` on its own) still refuses on both compilers, now with
the style-value sentence rather than a fold refusal. So `to_value` answers
`undefined` where it used to refuse, and the array loop's own copy of that
answer — written for holes — collapses into it. One arm rather than two, which
is the shape the array-hole comment was already arguing for.

### The guard above the bridge had the same hole

`Carried` carrying the value is not enough on its own: `rendered_expr` in
`amplification.rs` reads how wide the widest element of a receiver renders, and
it had arms for a string, a number, a boolean and `null` and none for this — so
an amplifying call over a binding holding one went on refusing after the bridge
could carry it. That is the ticket's *"reached through more than `join`"*
clause, and it is answered with one arm reading the width of the name.

The name is nine characters and a join renders the value as nothing, so the two
readings differ. The wider is the one a ceiling is safe to be told, and it is
already what the `null` arm beside it does — `null` renders as `"null"` there
and as nothing in a join.

### The scope this took, stated rather than assumed

Two changes here are outside the inward bridge the title names, and both are
recorded rather than folded in quietly. The **outward** refusal for `undefined`
and the **amplification** width are the same omission wearing two other hats:
neither is one of the four categories ADR 0008 argues for, both refused inputs
`@stylexjs/babel-plugin` 0.19.0 folds, and leaving either in place would have
made the inward arm reach a binding it still could not use. The measured effect
is a parity gain in every case that moved and a refusal that moved house in one
(`content: "abc".at(99)` alone, now refused by the style-value check on both
compilers rather than by the fold here). Four pre-existing tests were rewritten
to match, and the shared `unfoldable_fold_result` sentence lost `undefined` from
the kinds it lists and gained it in the kinds it says do fold.

### Where it is written down

`transform_stylex_create_test/undefined_in_a_named_value.rs`, thirty-eight
measured cases across both spellings, arrays, objects, nesting, the methods that
count an element rather than render it, the read-back coercions, the entry
ceiling at five thousand and past ten thousand, twenty levels of nesting, and
the two things that still refuse. `engine_fold_tests` carries the outward half.
`modules-27-undefined-inside-a-named-value` is the corpus row, and the harness
reports it **identical** with **unexpected 0**.
