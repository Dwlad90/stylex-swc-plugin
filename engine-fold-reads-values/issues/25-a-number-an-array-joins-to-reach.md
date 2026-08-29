# 25 — A number an array joins a string to reach

**What to build:** The character ceiling reaches the fourth place a string
grows — the join `ToNumber` performs on its way to a number — so the cost of an
array nothing can be a number is bounded rather than paid in full.

```js
const x = 'y'.repeat(900000);
const a = [x, x, /* … 200 in all … */];
width: +a
```

Measured on this compiler after 23 landed: **10.2 seconds**, and then a fold to
`NaNpx`, which is upstream's answer. So this is not a wrong value and not a
refusal arriving late; it is the right answer bought at a hundred and eighty
megabytes of string. `ToNumber` of an array is `ToNumber` of its join, and the
join is materialised in full before a single character of it is read.

**It is 23 one bridge over.** 23 bounded the join where a string is *grown* — an
interpolation and a `+` — by writing every element into the measured buffer.
`evaluate_result_to_js_number` reaches the same coercion through
`evaluate_result_to_string_of`, which collects, because a number bridge has no
buffer and no ceiling: its answer is one `f64` however wide the string it read.
The streaming machinery 23 added is already general — the arm needs a sink, not
a rewrite.

**What makes it a decision rather than an edit.** A sink that refuses turns a
fold both compilers agree on into a divergence: upstream answers `NaN` here and
this compiler would refuse. That is a new row in the category the record of 16
is counting, and it is the *third* kind — a refusal an author can configure away
— rather than a gap. Whether the trade is right is the ticket's question, and
there is a second answer worth pricing against it: `ToNumber` needs only to know
whether the whole text is a numeric literal, and an array of two or more
elements joins with a comma, which no numeric literal holds. A bound that
refuses and a shortcut that answers `NaN` without building the string reach
different verdicts, and only one of them keeps agreement.

**Reached only through the number path**, which is narrower than 23's. `Number(a)`
and `String(a)` refuse in the value bridge at 51 ms, and `a * 1` and `a - 0` are
refused as an unsupported binary expression at 14 ms. Unary `+` is the spelling
that arrives, so a fix has one caller to change and the whole of the number
bridge to keep correct.

**Found while building 23**, in the sweep that measured which shapes still pay
for a join they cannot use. The mechanism is untouched by 23 — the bridge
collected before it and collects after — so this is a gap of its own rather than
a consequence of that change. 23 in fact made it slightly cheaper, since an
element now reaches the buffer as the `str` the value holds instead of as a copy.

**Blocked by:** none. 23 is the ticket whose measurement found it.

**Status:** resolved

- [x] `+a` over an array past the character ceiling stops before the join
      allocates, measured against the 10.2 s above — or answers `NaN` without
      allocating, if that is the ruling
- [x] Whichever way it is ruled, the ruling is written down: a refusal here is a
      divergence from a fold upstream completes, and a reader needs the reason
- [x] The number a refusal names is `maxFoldedCharacters`, as it is for every
      other reading
- [x] `ToNumber` keeps every answer it has today — an array of one numeric
      element is that number, an empty array is zero, and a nested array reaches
      its number through its own join
- [x] The corpus records the shape, so the day it changes is a changed verdict

## Answer

**The ruling is the second answer, and both halves are built.** A bound alone
would have turned a fold both compilers complete into a divergence, so the
bridge reads what the ceiling bounds instead. `ToNumber` never keeps the text it
reads — its answer is one `f64` however wide the string was — and its only
question is whether the text spells a numeric literal. So the sink drops the
text at the first character no numeric literal holds and answers `NaN` from
there, and a comma is such a character: an array of two or more elements is
settled at the separator after its first, before the second renders.

The character test is *sound* rather than exact — every character a numeric
literal can hold is admitted, so a rejection proves the whole text is `NaN`
however it continues. That is what keeps the ceiling honest without a second
grammar: what it is left bounding is a text that really could still be a number,
and a refusal there is the configurable kind.

**Measured.** The reported array — two hundred elements of nine hundred thousand
characters — folds to upstream's own `NaN` in 104 ms where it took 10.2 seconds
and 180 MB before. The transform suite asserts under a second, an order of
magnitude above the measurement, so it does not become a benchmark of the
machine.

**Every answer `ToNumber` had is kept**, checked against
`@stylexjs/babel-plugin` 0.19.0 on twenty-five shapes: `+[]` is zero, `+[5]` is
five, `+[[7]]` and `+[[[9]]]` reach their number through their own joins, the
three radix prefixes and the exponent and sign forms read, surrounding
whitespace is trimmed, `+['1_0']` is `NaN` because a separator belongs to a
numeric *literal* and not to a string, a function has a number though it has no
string, an own `valueOf` answers ahead of a `toString`, and `-` and `~` read the
same bridge.

**Where the refusal is reachable.** Only through a text written out: every
expression that could grow a million-character numeric string is bounded by the
same number first — `'1'.repeat(1000001)` refuses on the amplification
arithmetic and `x + x` on the concatenation — so the corpus carries the
agreement row and the ceiling half is pinned as a test. The sentence names the
numeric conversion the author wrote rather than the join inside it, the way a
growing string's names the `+` or the interpolation.

`to_js_number` is now a collecting wrapper over `write_js_number_of`, so the
streamed reading and the collected one cannot come to disagree; a test asserts
they do not.

Tests are a section of `array_join_ceiling.rs`, plus unit cases on the sink and
on the coercion. The corpus row is
`modules-25-a-number-an-array-joins-to-reach`, recording `identical`.
