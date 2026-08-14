# 05 — `Array(…)` around a style value compiles

**What to build:** A generated list of fallback values compiles. `Array(a, b)`
folds to a style array and emits the same declarations the literal array would.

The count form — `Array(3)` — produces that many holes. Represent a hole as
`undefined`, which everything downstream already understands, and let it reach
the existing "a style array value can only contain strings or numbers"
rejection. That is exactly what the reference implementation does with it: the
value folds, and the *validator* is what rejects it. The sparse/dense
distinction has no observable consequence here, because nothing downstream
iterates with hole-skipping semantics.

**Blocked by:** 03 — reuses the globals enum, the callback variant, and the
apply-site wiring introduced there.

**Status:** done

- [x] `Array(a, b)` folds to a style array and emits the same declarations as
      the equivalent literal array
- [x] `Array(n)` reaches the existing style-array rejection, asserted by its
      diagnostic rather than by "something failed"
- [x] Expected values are taken from measured reference output

**Found while measuring:** a count is a count only when the argument is a
*number*, so `Array('3')` is the one-element list and `Array(3)` is three
holes. `NaN` and `Infinity` reach the evaluator as their identifiers rather
than as numeric literals, so recognising a count means reading both forms —
without that, `Array(NaN)` folds to a one-element list and gets refused by the
wrong check.

A count that is not an array length — a fraction, a negative, `NaN`, or a
value at or past `2 ** 32` — is a `RangeError` in JavaScript, so there is no
array to fold. Reference and compiler agree that these do not compile, and the
wording matches too.

**Deliberate divergence, wants a sign-off:** `Array(2 ** 32 - 1)` is a legal
length. The reference builds it sparsely for nothing; here a hole costs the
width of an evaluated value — 80 bytes — so materialising it is an allocation
the compiler does not survive. Counts are refused above a documented limit of
65,536 instead.

The limit is a real divergence, not a free one, and it was measured rather
than assumed. A counted array used as a style value is refused whatever its
length, so there the two compilers still agree that the program does not
compile and only the wording differs. The one shape that folds to something
usable is the join: `String(Array(n))` is `n - 1` commas, which the reference
compiles for any length. So `String(Array(65537))` compiles there and is
refused here — a CSS value of 65,536 commas, which is why the limit sits where
it does rather than at the 1,024 first tried, where `String(Array(1025))`
diverged.

The parity check ran 36 modules through both compilers, comparing emitted
class names and rule metadata: 23 compile identically, 12 are rejected by
both, and the single divergence is the 65,537 join above. That harness is a
development instrument and is not committed.

**Sign-off — accepted.** Raised in review as shipped without one, which was
correct: the ticket asked for a sign-off, was marked done, and recorded none.
Signed off now on the evidence above. `String(Array(65537))` — a CSS value of
65,536 commas — compiles under the reference implementation and is refused here,
and that is the entire divergence. It is a bounded, documented refusal that fails
the build rather than emitting wrong output, which is the trade this branch makes
everywhere else too. The limit is now `MAX_FOLDED_ARRAY_LENGTH` in
`crates/stylex-js/src/coercions.rs`, beside the coercion rather than in the error
module it was interpolated into: `to_array_length` answers what JavaScript says,
and the budget is the caller's.

**On "represent a hole as `undefined`".** Implemented as
`EvaluateResultValue::Null`, which the evaluator documents as a confidently
evaluated value that is *absent* — its spelling of `undefined`, not of `null`.
The representation is the one this ticket asked for; only the name differs, and
it differs because the evaluator has one vocabulary and JavaScript has another.
