# 23 — A string the ToString coercion joins

**What to build:** The character ceiling reaches the third place a string grows
— the join an array's `ToString` performs — so a refusal there arrives before
the allocation rather than after it.

```js
const x = 'y'.repeat(900000);
const a = [x, x, /* … 200 in all … */];
width: `${a}`.length
```

Measured on this compiler after 20 landed: **4.4 seconds** for the template
above and **8.3 seconds** for the `+` spelling of it, and *then* a refusal. The
refusal is correct and names the right ceiling; what is wrong is everything
before it. Twenty elements is 0.43s and three is 0.09s, so the cost is linear in
what the array holds.

**Not the same fix as 20.** 20 bounds every append the evaluator makes, and
`GrownString` is where it does it — so the sentence an author reads is already
right and the number is already the one they configured. But an array reaches
that buffer as *one already-joined string*: `evaluate_result_to_string_of`'s
`Vec` arm renders every element and joins them inside
`stylex_js::coercions::join_js_elements`, and the buffer counts the result. So
the bound is one layer too late, in the same shape 20 found the depth budget in.

**Why it was recorded separately.** The join is a shared coercion in another
crate, and it is shared on purpose: the same rule joins an array literal, so a
second copy of it in the evaluator would be the one thing this effort keeps
deleting. Bounding it means either moving a ceiling into `stylex-js` or bounding
the elements *before* they render — and rendering is where the cost is, since
each element's `ToString` clones a string the value already holds. Which of the
two is right is the work, and it is the same argument 18 made when it split 20
out for being a different subsystem's file.

**What bounds it today, and what does not.** An array that crosses a fold is
bounded by `maxFoldedEntries` on the way in and out, so a fold cannot hand one
of these back. An array the *evaluator* builds is not: an array literal of `n`
names is `n` elements, and each name may hold up to `maxFoldedCharacters`
characters of its own. The product is the number nothing measures — the default
pair is ten thousand times a million.

**Blocked by:** none. 20 is the ticket whose measurement found it.

**Status:** resolved

- [x] `` `${a}` `` and `'' + a` over an array past the character ceiling refuse
      before the join allocates, measured against the 4.4s and 8.3s above
- [x] The join rule stays in one place — no second copy of the separator and the
      empty-joining elements
- [x] The number is `maxFoldedCharacters`, as it is for every other append
- [x] The nested cases are covered: an array of arrays, an array holding an
      object, and an element with no string form at all, since each renders
      through a different arm
- [x] The corpus records what upstream folds and this compiler refuses —
      upstream folds the 200-element case above to `180000199px` in well under a
      second

## Answer

**Bounded by streaming the join, not by measuring it.** The two options this
ticket named were moving a ceiling into `stylex-js` and bounding the elements
before they render. Neither is what landed. The coercion crate has no ceiling and
should not learn one — it answers what the language says and nothing about who is
asking — and the elements cannot be bounded before rendering, because rendering is
where the cost is: each element's `ToString` copies a string the value already
holds. So the join became *streamable*, and the ceiling stayed where issue 20 put
it.

`to_js_string_with` is now a collecting wrapper over `write_js_string_of`, which
writes its pieces into a `StringSink` the caller supplies. `write_js_join` is the
one place the separator and the join's two endings live, and both joins reach it —
an array literal's and the evaluator's own `Vec`. `join_js_elements` and
`js_array_element_to_string` are gone, so the rule is written once rather than
twice. A `String` is a sink that refuses nothing, which is what every caller with
no ceiling uses; `GrownString` is a sink that charges every piece against
`maxFoldedCharacters`, which is what an interpolation and a `+` use through the
new `GrownString::push_string_of`.

**Measured, on the reported input.** The template spelling went from 3.9 s to
65 ms and the `+` spelling from 7.5 s to 92 ms, each refusing at the element that
passes the ceiling. Upstream folds the same source to `180000199px` in well under
a second, because V8 never materialises the join it builds. Two corpus rows record
the divergence under `maxFoldedCharacters`, and a test asserts the timing with a
threshold an order of magnitude above the measurement.

**Two things the change decided that the ticket did not ask about.** Both
operands of a `+` now grow one buffer, where the left operand's finished string
used to be taken over — so an array on the *left* of a `+` is measured too, and
`GrownString::of` is gone. And a refusal keeps naming the expression the author
wrote, `concatenation` or `template literal`, rather than the join inside it:
what an author has to look at is the line they typed.

**The one decision measured rather than argued.** Dropping the left operand's
adoption looked like it traded a linear chain of `+` for a quadratic one, since
the deleted constructor's own comment said adoption was what made a chain grow one
buffer instead of one per link. It does not: the old path copied the left prefix
one level up, inside the coercion that built the string it then adopted, so the
copy was always there. Priced against each other on the same inputs — a folding
twenty-term chain at default settings, 205 ms against 201 ms; a five-hundred-term
chain at the maximum depth and ceiling, 55.1 s against 54.3 s; an ordinary short
concatenation, 291 µs against 288 µs. The five-hundred-term case is slow on both
and is a property of a chain that long at those settings rather than of this
change. Both readings of an operand at the boundary agree too: a long operand at
exactly the ceiling folds, and an array past it refuses on the left as on the
right, because the adoption skipped the left's own measurement and only the sum
was ever refused. Both are pinned as tests.

**One gap measured beside it, and it is not this one.** `+a` over the same array
folds to `NaNpx` — upstream's own answer — after 10.2 seconds, because `ToNumber`
reaches its number through the same join and the number bridge collects it. It
needs a sink and a ruling, since refusing there would diverge from a fold upstream
completes. Filed as issue 25.
