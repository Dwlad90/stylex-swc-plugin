# 28 — A callback measures the receiver it has

**What to build:** The element count an amplifying callback is priced against
comes from the receiver's resolved value, so a fold refuses because it is large
rather than because its receiver was written a way the guard has no name for.

```js
'ab'.split('').map(x => x.padStart(2, '0')).join('')
  upstream: "ab"
  here:     Cannot bound the string 'padStart' would build inside a callback.
```

The element count there is **two**. So is it in `a.map(n => 'x'.repeat(n))` and
`a.map((s, i) => s.repeat(i + 1))`. Nothing about these is large; the guard
refuses because it cannot read a count off a receiver that is itself a call, and
falls back to refusing rather than to measuring.

**The cause is a table.** `PER_ELEMENT_METHODS` lists ten receiver methods whose
callback element count the guard knows how to read. A callback on any other —
`reduce`, `reduceRight`, `sort`, `Array.from`'s mapper — is unmeasured and
therefore refused if its body amplifies. Spec story 32 is precisely this: *"I
want the set of foldable methods to stop being a list I maintain, so that the
method nobody wrote down is not the next bug report."* Five such tables were
deleted by this effort; this is the sixth, and it survived.

**What makes it a decision.** The count exists to bound allocation before it
happens, and a receiver that is itself a call has no count until it is evaluated.
Two answers are worth pricing against each other: resolve the receiver and read
its length — which the guard can now do, since receivers resolve — or drop the
per-callback bound and let the cumulative budget from 29 catch the growth as it
is produced. The second removes the arithmetic entirely; the first keeps the
refusal early, where its message is better. Zero-padding over a `split` or an
`Object.keys` is an ordinary idiom, so whichever is chosen has to fold it.

**Blocked by:** 26.

**Status:** resolved

- [x] `PER_ELEMENT_METHODS` is deleted, not extended
- [x] The four shapes above fold to the values `@stylexjs/babel-plugin` 0.19.0
      produces, with identical class names
- [x] A callback whose body really does amplify past the ceiling still refuses,
      and names `maxFoldedCharacters` when it does
- [x] `reduce`, `sort` and `Array.from`'s mapper carry a case each, since they
      are the methods the table never listed
- [x] The ruling is written down: if a bound is dropped, what now catches the
      growth, and where

## The ruling

**No bound was dropped.** The first of the two answers was taken: the receiver
is resolved and its length read, so the refusal stays in front of the engine
where its message names both factors and the ceiling. Ticket 29's cumulative
budget is still worth building, and now has one fewer case leaning on it.

**What replaced the table.** The count belongs to the receiver, so it is taken
off the receiver's own value and is the same count whatever method reads it. The
method is asked one question only — `element_parameter_of` — and only two
families answer it differently:

- a **comparator** (`sort`, `toSorted`) runs once per comparison, which is more
  often than the array is long, so no count is given and a body that amplifies
  inside one keeps the blanket refusal;
- a **reducer** (`reduce`, `reduceRight`) is handed the element second rather
  than first, so the element's width is placed on the second parameter.

Everything else — named or not, present in the language today or added to it
tomorrow — is measured. That inverts what the table did: an unlisted method used
to refuse, and now folds.

**Where a miss would land.** A future method that ran its callback more often
than its receiver is long would be measured short here, and the growth would be
refused on the way out of the engine by the same two ceilings, later and slower
rather than not at all. That is the trade the inversion buys, and it is the same
shape `Array(n)` already had before its declaration was bounded.

**Three readings, one measurement.** The count, the widest element's rendered
width, and the largest index all come off the single reading of the receiver, so
they cannot come to disagree. The index is what bounds a count written as
`i + 1`; an element is what bounds one written as `n`. Sums and products of
those bounds are read as bounds, under three conditions that are what make the
arithmetic sound rather than plausible: every part must be a value the guard
saw is a **number** (`'2' + 1` joins rather than adds), must be **at or above
zero** (`(-5) * (-5)` is twenty-five against a bound of nothing), and is rounded
**up** before it is added or multiplied, since the language truncates the result
rather than the parts — `0.9 * 2000000` is one million eight hundred thousand
characters.

**One rider, named rather than silent.** `repeat` with a count of one or none
builds nothing its receiver was not already charged for, so it no longer asks
for that receiver's length. It rides along because it is the same complaint the
ticket makes — a fold refusing for the shape of what it was written on rather
than for size — and it is what lets a reducer pass its own accumulator through.

**What still refuses a call.** `Walk::receiver_length` — the `repeat` rule —
keeps refusing a receiver that is itself a call, and the reason is ordering
rather than shape: it is asked *before* the receiver's own bound has been
checked, so resolving one would build the very string the bound exists to
prevent. A **receiver's** count is taken *after* the receiver has been admitted,
so what it resolves to is already inside both ceilings. The two readings are
named apart — `module_value_of` and `countable_value_of` — so the difference
cannot be lost to an edit.

`Array.from`'s source is the one count that ordering does not cover: an argument
is walked after the call's own rules rather than before them. It costs nothing
extra all the same, because the entry rule's `length_property` has already
resolved the same expression one step earlier, to compare the length it declares
against the ceiling — so the mapper's reading is a memo hit, and the exposure
that is there belongs to the entry rule and predates this change.
