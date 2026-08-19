# 01 — `"abc".length` evaluates to `"abc"`

Status: `resolved`
Phase: Phase 2

**What to build:** A member access of `length` on an evaluated string answers
its length.

```js
stylex.create({ x: { content: 'abc'.length } });
```

The reference implementation folds this to `3`, emitting
`.xxxxxxx{content:3px}`. This compiler answers `"abc"` — the receiver itself —
and emits `content:"abc"`. It does not error. It ships a wrong CSS value.

That makes this the most severe finding of the investigation even though it is
not what [#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265)
reported: a build error stops the line, a wrong value does not.

This is the member-expression path, not the call path — `length` is a property,
not a method — so it lives in `nodes/member_expression.rs` and is independent
of the panic/deopt work in 02.

`length` is measured in UTF-16 code units, not bytes and not scalars:
`"\u{1F600}a".length` is `3`. `crates/stylex-utils/src/string.rs` already
counts in code units for `charCodeAt`; reuse that view rather than introducing
a second convention.

Array `length` follows the same rule and the same reasoning; today
`["a","b"].length` raises `This evaluation result type is not yet supported`.

Independent of 02 — that work is the call path, this is the member path — so
it does not gate the regression fix and must not delay it. It changes emitted
CSS and a generated class name, so it needs its own release note.

## Comments

### What shipped

`6bd3477d2` `feat(stylex-utils)`, `777e01239` `fix(stylex-transform)`,
`e14540e32` `test(rs-compiler)`, `70d242621` `docs`, `127c56a48` `test`, then
`6ce970da0` `fix(stylex-transform)` and `283f0f917` `test(rs-compiler)` from
review.

A string answers `atom_utf16_length` and an array its slot count. Both match
`@stylexjs/babel-plugin@0.19.0` byte for byte -- `content:"3px"`, `content:"2px"`
-- measured by five new module subjects in the parity corpus, three of which
report `identical`.

### The defect was wider than `length`

The arm being fixed read _every_ property of _every_ literal by re-evaluating
the receiver and dropping the property. `"abc".length`, `"abc".foo`, `(5).x` and
`true.y` all answered the receiver. So the fix is not "add `length`" but "stop
answering the receiver".

What replaces it is what the reference implementation does, which is
`object[property]` -- real JavaScript:

- `length` on a string or an array folds to a count.
- A property the receiver does not carry answers `undefined`. That is the same
  answer the object arm already gives for a key an object does not hold, and it
  is what lets `"abc".foo ?? "red"` fold to `red` in both compilers.
- An index refuses. This is the one lookup where parity is not reachable yet:
  upstream folds `"\u{1F600}"[0]` to a lone surrogate, and no Rust `String` can
  hold one, so folding an index needs a value type that carries WTF-8 through
  the whole value pipeline. Answering `undefined` instead would be confidently
  wrong rather than merely incomplete, which is the trade this ticket exists to
  refuse.

Pinned as `modules-length-an-indexed-string-read`. The first draft of this fix
refused the missing-property case too; review caught that it cost a divergence
for nothing, since `undefined` is both the parity answer and the smaller change.

### Where the count is read from

Two places, and the reason is holes. `array_expression.rs` evaluates elements
with `iter().flatten()`, which drops a hole entirely, so an evaluated `[, 1]` is
a one-element `Vec`. Counting that would answer one where the language says two.
An array literal is therefore counted off the AST and a non-literal receiver off
its evaluated elements; the two agree wherever there is no hole.

**A spread breaks both readings**, which the first draft of this fix missed:
`[...[1, 2]]` writes one element and evaluates to one element, and the language
says two. It folded confidently to `1` until review caught it -- the exact class
of silently-wrong value this ticket is about, reintroduced while removing it. An
array carrying a spread now refuses, and both compilers reject it with the same
message. Pinned as `modules-length-on-an-array-with-a-spread`.

The gap that forces the split is still there for every other reader of an
evaluated array -- a binding to `[, 1]`, `Object.keys([, 1])`, `[, 1]` as a
style array value. Filed as
[08](./08-a-hole-is-dropped-from-an-evaluated-array.md).

### One diagnostic, three receivers

A string, an array a fold produced, and an evaluated array each carried their own
copy of the property test, so `[1, 2]["7"]` described the member expression while
`"abc"[7]` named the index -- two answers to one author mistake. One
classification and one refusal now serve all three, swept by
`a_refusal_names_the_index_it_could_not_read`.

### A panic found on the way

Reading the length through `convert_atom_to_string` aborts the build on
`"\uD83D".length`: an unpaired surrogate is a legal JavaScript string literal
and has no UTF-8 form, so the convertor reaches `stylex_panic!(INVALID_UTF8)`
from inside an evaluation that is allowed to fail. Exactly the family
[02](./02-separate-the-two-kinds-of-evaluation-failure.md) is about, in a
convertor rather than in `evaluate/`. `atom_utf16_length` reads the atom's code
units instead and never needs a scalar.

The same convertor is still reached with the same input from other string folds
-- `"\uD83D".charCodeAt(0)` and `"\uD83D".concat("a")` both abort, measured.
Filed as [09](./09-an-unpaired-surrogate-aborts-a-string-fold.md).

### `??` over a zero length

`[].length ?? 3` refuses rather than folding to `0`, because the nullish guard
tests the left operand's truthiness where it meant its nullishness. Inherited
deliberately from the reference implementation and already documented in
`nodes/logical_expression.rs`; pinned here so a later correction of that guard
shows up as a failing test rather than as a silent CSS change.

### On the release note

The ticket asks for one. There is no per-change artifact to add: `.changeset/`
was removed in `cb8e4dfde` (March 2025) and no `CHANGELOG.md` is maintained, so
`.github/workflows/release.yml` generates notes from commit and PR titles. The
commit subjects are therefore the note, which is why the second one says what it
changes about _reading a member lookup_ rather than only about `length`.
