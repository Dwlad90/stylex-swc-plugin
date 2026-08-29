# 29 — The bridge counts once and copies once

**What to build:** The values coming back from the engine are counted against one
running total, the way the values going in already are — and the values going in
are walked once rather than twice.

## The outward budget

`Outward` carries `method`, `depth` and `ceilings` — and no total. Every check is
per value: `keys.len() > entries`, `length > entries`, `string.len() > characters`.
`Inward` and `Transport` do accumulate. So a graph whose nodes each sit at the
ceiling converts to `entries ^ depth` AST nodes:

```js
const x = [/* 10 000 elements — exactly the ceiling, which uses `>` */];
content: x.map(() => x).length
```

Ten thousand references to one array on the engine's side — cheap, correctly so,
because the engine aliases. Ten to the eighth `Expr` boxes on this side, roughly
nineteen gigabytes, because the outward walk copies. The refusal never fires: no
single value is over the line.

The fix is the shape already in use. `Inward::count` and `Inward::text` thread a
total and refuse when it passes; `Outward` needs the same, counted as values are
produced rather than as they are inspected. The reason the ceilings exist at all
is that the engine bounds loops, recursion and stack but not allocation — and
this is allocation on the Rust side of the bridge, which the engine's own bounds
could never have seen.

## The inward walk copies twice

A carried name is walked twice and allocated twice: `EvaluateResultValue` into
`Carried`, then `Carried` into `JsValue`. The two-pass shape exists for a good
reason — the bounds have to be checked before an engine is built, so nothing
oversized reaches it — but the first pass only needs to *measure*, and it
allocates a full intermediate instead. A measuring walk that allocates nothing,
followed by a single conversion straight to `JsValue`, keeps the ordering and
halves the copying. It belongs here because it is the same accounting question in
the opposite direction.

## Three things the outward walk decides that nothing observes

- **Own-key ordering.** `PropertyKey::Index` and `order_own_keys` are reached by
  no fold-path test; `folded_object_values.rs` folds string-keyed objects only.
  Integer-first ordering settles declaration order between rules of equal
  specificity, so a regression there changes which rule wins and nothing fails.
- **The outward depth ceiling.** `Outward::descend` is never reached — every
  depth refusal fires on the inward guard walk first. A fold whose *result* is
  deeper than its input reaches it, and nothing proves it answers.
- **The symbol-keyed arm**, which has a string-shape unit test and no module.

**Blocked by:** 26.

**Status:** resolved

- [x] `Outward` carries a running total of entries and characters, counted as
      values are produced
- [x] The aliasing shape above refuses, naming `maxFoldedEntries`, without
      allocating its way there first — measured, not argued
- [x] A fold that legitimately sits under the ceiling still folds, so the total
      is not double-counting an aliased value the AST really does need twice
- [x] The inward walk measures without allocating, then converts once; the bounds
      are still decided before an engine exists
- [x] Integer-first own-key ordering is pinned through a fold, not only through
      the object evaluation
- [x] `Outward::descend`, the `PropertyKey::Index` arm and the symbol-keyed arm
      each have a module that reaches them, or are marked unreachable with the
      invariant named

## What landed

**One total per direction.** `Totals` is now shared vocabulary in `engine_fold`:
two running counts and the ceilings they are counted against. `Transport` holds
one and `Outward` holds one, because each direction allocates what the other does
not. Counting answers the ceiling it passed rather than a bare `true`, so the
sentence a caller writes names the number that counted rather than reaching for
it again.

The outward walk counts an array's length before it reads an element, an object's
key count before it reads a property, and every string and **key** as it is
produced. The keys were the gap the review found: the total's own doc said "every
string and key" and only the inward half was counting them, so an answer of few
enormous keys passed a ceiling the module claimed to enforce.

**`Carried` is gone.** A value is measured where it was measured before — in
`bind`, before anything is printed and before there is an engine — and the value
the evaluator already answered is what is kept, built into the engine's own values
once, in `arguments`. The two walks are one traversal under a `Carriage` trait, so
which shapes the bridge carries cannot come to differ between measuring them and
building them; `Measure`'s values are the unit type, so a list of them never
reaches the allocator.

Deleting `Carried` took the StyleX namespace object with it: it was a
`Carried::Object` of functions, and is now `Crossing::Namespace`, assembled where
the other engine values are. That shape is not something the ticket asked for — it
is what the deletion cost.

## Three readings worth recording

**"Naming `maxFoldedEntries`"** is read as *the entry ceiling is what fires*, not
as the option string appearing in the sentence. No refusal in this compiler names
an option; `array_length_too_large` and its siblings name the limit, which is the
half an author can act on, and one sentence spelling the option would be the odd
one out.

**Two of the three "nothing observes" bullets were already stale.**
`an_object_result_carries_the_own_key_order_the_language_gives_it` folds integer
keys, so `PropertyKey::Index` and `order_own_keys` are both reached; and
`a_value_the_engine_nested_past_the_bound_refuses_rather_than_overflowing_a_stack`
reaches `Outward::descend`, since the input it refuses is a handful of levels deep
and the answer is forty. What was genuinely unobserved is subtler and is what the
new test pins: the engine already answers its own keys integer-first, so
`order_own_keys` can never *disagree* with it, and every ordering case that
existed would pass with a naive numeric sort in its place. A key that reads as a
number and is not an array index — `"01"`, `"4294967295"` — is what tells the two
apart.

**The symbol-keyed arm is marked unreachable with the invariant named**, and the
invariant is now pinned rather than asserted: `Symbol` is not one of the globals
the guard admits, in a callback body as much as anywhere else, and a computed key
is refused by its shape.

## Where it is proved

`folded_answer_totals.rs` at the transform seam — five cases, each folding output
measured against `@stylexjs/babel-plugin` 0.19.0, and each refusal measured against
it too. Upstream has no such ceiling, so it folds all five and then refuses four of
them where the value lands; the modules agree on the outcome and differ on the
sentence.

`engine_fold_tests.rs` carries the three unit cases: the aliased array refusing at
the shipped ceiling and finishing, the non-canonical numeric key keeping its place,
and the symbol invariant.
