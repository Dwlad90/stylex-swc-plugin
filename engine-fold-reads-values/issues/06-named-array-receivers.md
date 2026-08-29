# 06 — Named array receivers

**What to build:** The same for arrays, and the end of the two dispatch arms
that disagree with each other.

```js
const a = ['1px','solid'];
export const styles = stylex.create({
  x: { borderTop: a.concat(['red']).join(' ') },
});
```

Eleven array methods fail on a named receiver today. Two more fail at the
*second* link of a chain, because one dispatch arm accepts `join` for a value
the evaluation produced while another refuses it for the array a fold
produced — which is the disagreement ticket 04 made removable.

One of them is not merely missing but wrong: a filter whose callback returns a
truthy string reports `Value in not a number`, because the callback's
truthiness is decided by converting its result to a number. The module holding
that logic is deleted here; the behaviour still wants a pinned test so a later
change cannot reintroduce it.

**Blocked by:** 05.

**Status:** resolved

- [x] The eleven array methods that fail on a named receiver compile, each
      matching the reference compiler's declaration text and class name
- [x] A chain folds at every link when a middle link is a binding
- [x] A mapped list can be joined — the two arms that disagreed are one
- [x] A filter with a truthy non-numeric callback result folds correctly, and
      that shape is pinned
- [x] The array method name table and the hand-written native array methods
      are both deleted
- [x] The mutating-method behaviour proved in 03 still holds now that a named
      receiver reaches the guard

## Answer

### Arrays cross, and so does everything an array holds

The inward bridge was one shape — a string. It is now a `Carried` value: a
string, a number, a boolean, `null`, a list, or a plain object, nested to any
depth of those. That is what the ticket needed and no more than it needed: a
`[1, 'a', true, null]` join has to write `1|a|true|` to match, so an element's
kind cannot be the bridge's business, and `[['1px'], ['2px']].map(p => p[0])`
has to reach the second level.

Objects cross with them, which is wider than "the same for arrays" and is not
scope taken for its own sake: `const o = { list: ['a'] }; o.list.join()` folded
through the array table this ticket deletes, and a bridge that carried only
arrays would have refused the receiver and lost it. It also closes the
divergence 05 recorded and assigned to "tickets 06/07" —
`const o = { a: '1' }; o.a.toUpperCase()` folds now, matching upstream.

What a *name* may hold stays narrower than what the bridge carries, and the
narrowing is one predicate with the reason written on it: a number or a boolean
crosses as an element, where it is part of the value the receiver is, but a name
holding one alone is a receiver of its own and `Number.prototype` is ticket 08.

Three bounds, all on the value rather than on the syntax that named it, all
counted across every name one fold carries, and all reusing the numbers the
outward direction already used: text in UTF-16 code units, entries against
`MAX_FOLDED_ENTRIES`, and nesting against the same `Depth` counter the guard's
walk and the conversion out spend. Only the entry refusal needed a sentence of
its own — `bound_value_has_too_many_entries`, naming the binding, because a
thousand empty arrays are no text at all and still a thousand values to build.

The conversion happens in two steps, and that is what keeps every refusal in
front of the engine: the evaluator's value is walked and measured with no engine
in hand, and only what survives is built as the engine's own — an array and an
object need a `Context` to exist at all.

### The two arms are one

`EvaluateResultValue::Vec` and `Expr::Array` were two dispatch arms carrying two
different sets of method names for one value in two shapes, which is why a
mapped list could not be joined. Both are gone, and so is the string arm beside
them: one condition now answers for every receiver whose prototype the fold owns
whole, and it cannot disagree with itself. An object receiver is deliberately not
in it — that arm is where a folded function map's own methods are looked up.

Deleted with them: `ArrayJS` and its `TryFrom` and unit test, `CallbackType::
Array`, the whole of `native_functions.rs` (`evaluate_map`, `evaluate_filter`,
`evaluate_join` and the two callback helpers), and `is_mutating_array_method` —
the second copy of a predicate the spec asked to keep single, whose only caller
was the arm that has just stopped needing it. Mutation is answered where the
reference compiler answers it, by disqualifying the binding, which the module
visitor collects.

The filter bug went with the module holding it: `a.filter(v => v).join('-')`
reported `Value in not a number` because a callback's truthiness was decided by
converting its result to a number. The language decides it now, and both
spellings are pinned.

### The regression this caught, and the one rule that moved

Deleting the array table took `Object.entries(o).filter(f)` with it — it folded
through that table, and after the deletion nothing below the fold answers for an
array. The guard was handing every receiver that names one of the dispatch's
globals straight back, including one buried in a chain that nothing else will
ever be asked to fold.

So the hand-back is now asked only of the call the caller asked about. A static
inside a chain is answered by the engine, and the names the reference compiler
refuses are refused with it by `INVALID_METHODS` — that compiler's own set, and
the one the dispatch already gates its statics on, which is what keeps
`Math.random().toString()` from folding a different class name on every build.
Measured, every static shape this makes foldable matches upstream exactly.

It leaves one asymmetry, recorded in `admit_call`, in the glossary and on ticket
07: `Math.trunc(1.5)` refuses written alone and folds written inside a chain,
because alone it is the seven-name table that answers. Position deciding the
answer is the opposite of what one guard walk is for. The fix is the surface
moving to the engine; the alternative until then is taking the chain away from
folds that have it today.

A global's name is admitted as a *receiver* and nowhere else. Admitted as a
value, `['a'].concat(String).join('')` would fold a function's own source text
into a declaration; measured, upstream refuses it, and so does this.

### Computed reads

`entry[1]` and `p[0]` were refused, which is what actually broke the chain above
— a computed key was read as "a lookup that needs the scope". It is a value in
its own right and is walked as one now. The escaping-property rule applies to a
key written as a string, since `x['constructor']` spells the read `x.constructor`
spells; a key whose value the guard cannot read is admitted, and that is a
boundary rather than a hole — what such a read can reach is a function, which is
refused on the way out and cannot be applied on the way in, because a call whose
method name is computed is not a candidate at all.

### Measured against `@stylexjs/babel-plugin` 0.19.0

**43 shapes fold to the reference compiler's own class name and rule text**, each
measured in-process under the same options: the 24 non-mutating
`Array.prototype` methods on a named receiver, the chains, the mapped-list join,
the static chains, every element kind, nested arrays, arrays of objects, a named
object read through a member, the nine element values that could not be printed
safely, and an index read. Every input upstream rejects, this rejects.

Divergences left, each measured and each owned elsewhere:

| Input | Upstream | Here | Owner |
| --- | --- | --- | --- |
| `const n = 5; n.toFixed(1)` | `5.0` | refuses | 08 |
| `a['join'](',')` | folds | refuses (computed method name) | pre-existing rule |
| `a.map(x => x.toLocaleUpperCase('tr'))` | folds | refuses | deliberate, spec |
| `a.map(x => x.repeat(3))`, `a.join('').repeat(3)` | folds | refuses | 12 |
| `a.join(undefined)` | `b,a` | refuses — `undefined` is not a carried value | unowned, recorded here |
| `Object.keys({ __proto__: 'x', a: 'y' })` | `a` | `__proto__,a` | 07 |

The `__proto__` reading is not new: the dispatch has always answered
`__proto__,a`, and the inward bridge was made to agree with the dispatch rather
than with upstream, so the two paths give one answer — wrong in the same way —
rather than two. Ticket 07 settles it for both.

### Tests

`transform_stylex_create_test::named_array_receivers`, 29 cases at the highest
seam there is. Beyond the surface and the chains: the mutating methods on a
named receiver (which refuse on the rule the *binding* broke, re-proving 03 now
that a named receiver reaches the guard), a hole and a spread in the bound
declaration, a method whose answer is `undefined` or an iterator, an escaping
read inside a callback in both spellings, the three inward bounds, a global read
as a value, the surrogate round trip, a theme-resolved string as an element, the
dynamic-style call left to the runtime, and a folded value still reaching the
CSS guards.

`assert_folds` and `assert_refuses` moved to `tests/utils/transform.rs`, where
`fold_module` and `base_style_module` already lived, so the two files whose
subject is the same fold cannot come to assert it differently.

Two tests were deleted rather than moved, both pinning things that no longer
exist. `engine_fold_refusals::a_nested_array_reaching_the_older_join_refuses_
rather_than_panicking` pinned that a nested array reaching the older `join` did
not panic; there is no older `join`, and the shape folds now, pinned as
agreement instead. `evaluate_path_mutation_detection::should_bail_out_when_array_
is_mutated_via_push` pinned the deleted dispatch predicate: that harness runs no
module visitor, so a mutated binding is not disqualified in it, and the claim
belongs — and lives — at the transform level. A comment in its place says so.

The last one is `named_string_receivers::a_name_bound_to_a_value_the_bridge_
cannot_carry_is_handed_back`, which used a nested array and now uses a number.

**Parity corpus: 1123 subjects, 0 changed, 0 unexpected.** No corpus verdict
moved — the corpus has no named-receiver rows to move, which is ticket 14's
work. One row was already unaccounted for before this ticket:
`modules-mutated-binding-read-through-a-method-call` reads
`both-reject-divergent` because 05 improved the sentence, and it carried no
expectation. It carries one now, with the reason written next to it.

The JS suite is green (86/86 turbo tasks).

### Measured — Apple M1 Max, `aarch64-apple-darwin`

The benchmark from 01, at `--measurement-time 6 --warm-up-time 2` rather than
criterion's defaults, so the noise floor is higher than 05's column. Every leg
has written-out receivers, so what they price is what this ticket costs a fold
that resolves nothing — every expression that folded before this effort.

| Leg | `fold` | `engine` | Round trip adds | 05 recorded |
| --- | --- | --- | --- | --- |
| string | 4.16 µs | 2.41 µs | 1.75 µs | 1.74 µs |
| callback | 15.74 µs | 11.31 µs | 4.43 µs | 4.20 µs |
| chain | 9.09 µs | 6.33 µs | 2.76 µs | 2.89 µs |
| array-answer | 8.41 µs | 5.42 µs | 2.99 µs | 2.85 µs |

Cold start 120.08 µs against 117.14 µs. Read the `engine/` leg beside its
`fold/` leg rather than against 05's column: the engine legs are unchanged code
and moved 0.02–0.14 µs between sessions on their own. The gap between the pair
moved −0.13 to +0.23 µs, which is inside that drift in both directions — the
guard does one more question per call (the binding table, once) and the rest of
this ticket is paid only by a fold that resolves a name, which no leg does. That
leg still needs `push_declaration`, and ticket 13 still owns it.

### A note for whoever reads this next

The review of this work caught a mistake worth naming: an in-place edit of
`mod.rs` written as `open(p, 'w').write(open(p).read()...)` truncated the file
before reading it, which unregistered all thirty test modules in the directory.
`cargo test` then reported 27 green suites and no failures, because the tests
were not compiled at all. Rust does not warn on an unreferenced test file. A
green run is not evidence a test ran; the count is.
