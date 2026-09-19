# 42 — Keep a repeated key in the place it first took

**What to build:** `remove_duplicates` in `stylex-ast` read a property list
backwards and kept the first entry it met. That gives the last value, which is
correct, but it puts the key in the place the key took **last**. JavaScript
keeps the place the key took **first**:
`Object.entries({ color: 1, opacity: 2, color: 3 })` gives
`[["color", 3], ["opacity", 2]]`, and the function gave `opacity` first.

The reference implementation builds a real JavaScript object and assigns into
it, so it takes this rule from the language and never had to state it.
`assign_props`, beside this function in the same file, states the rule in its
own documentation and already followed it. Only this reader disagreed.

This is the fourth key-reading correction in the family that
[ticket 22](./22-settle-the-css-affecting-key-corrections.md) opened. It is
filed after the fact: the fix landed in `24978fc64` and its tests in
`d3448f43d`, and neither had a ticket to sit under.

**Why it can change CSS.** Property order decides declaration order, and
declaration order decides which of two rules at equal specificity wins. A
style object that declares one key twice with another key between them is the
shape that moves.

**Blocked by:** None.

**Status:** resolved

- [x] `remove_duplicates` keeps a repeated key in the place it first took, and
      keeps the last value
- [x] The two tests that asserted the old order assert the language instead.
      The evaluation test for objects with spreads read `{ age, name }` for a
      third line that ends as `{ name, age }` in the language
- [x] The repeated-key cases tell the two rules apart. The first attempt gave
      `b, a, b, a`, where each key takes its last place in the same order as
      its first, so the case passed against the old rule as well as the new
      one. A third key declared once, between the two repeats, breaks the
      symmetry. All six cases now fail against the old rule
- [x] A key declared three times is covered: the second replacement must still
      land in the place the key first took
- [x] A property dropped between two declarations of one key does not move
      that key, because the place comes from the properties kept so far and
      not from the place the property had in the input
- [x] The snapshot suite is re-run. No snapshot moved, so no fixture depended
      on the old order
- [x] The pull request description carries a paragraph for this change, drafted
      in [ticket 22](./22-settle-the-css-affecting-key-corrections.md)
- [x] The workspace gate is green in **debug** -- never `--release`

## Comments

**No CSS moved, and that is worth stating.** The correction is a real one and it
can change emitted CSS, but no snapshot in this repository moved. The shape that
would move it -- one key declared twice with a different key between the two --
appears in no fixture. The three new cases in `objects_test.rs` are therefore
the only place the rule is written down.

### Text for the pull request description

The fourth of the four paragraphs. It sits beside the two drafted in
[ticket 22](./22-settle-the-css-affecting-key-corrections.md) and the one in
[ticket 32](./32-namespace-map-keeps-source-order.md).

> *A key declared twice.* A style object that declares one key more than once
> keeps the **last value**, as it always did, but the key now sits in the place
> it took **first**, which is where JavaScript puts it. `{ color: 1, opacity: 2,
> color: 3 }` compiles as `color` then `opacity`, not the other way round.
> Declaration order decides which of two rules at equal specificity wins, so a
> stylesheet that holds this shape can change. No fixture in this repository
> holds it, and no snapshot moved.
