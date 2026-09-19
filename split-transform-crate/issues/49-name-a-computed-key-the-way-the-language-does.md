# 49 — Name a computed key the way the language does

**What to build:** A computed property key that folds to something other than a
string or a number refuses here and names a property there. The language names
the property `String(key)`, and so does the reference implementation, so
`{ [true]: 'red' }` declares `true: red` for it and stops the build here.
Decide which answer each shape gets, and make the readings one.

Every row is measured. The reference implementation's column is
`stylex.create({ base: { <key>: 'red' } })` compiled through
`@stylexjs/babel-plugin`; this compiler's column is the same key asked of
`evaluate_obj_key`, which is the entry point the transform calls for a property
name.

| Key written    | This compiler                       | Reference implementation      |
| -------------- | ----------------------------------- | ----------------------------- |
| `[true]`       | refuses: `Key is not a string`      | declares `true: red`          |
| `[false]`      | refuses: `Key is not a string`      | declares `false: red`         |
| `[null]`       | refuses: `Key is not a string`      | declares `null: red`          |
| `[{}]`         | refuses: `Key is not a string`      | declares `[object object]: red` |
| `[!1]`         | refuses: `Key is not a string`      | declares `false: red`         |
| `[1 > 2]`      | declares `0: red`                   | declares `false: red`         |
| `1n`           | declares `1: red`                   | declares `1: red`             |
| `['color']`    | declares `color: red`               | declares `color: red`         |
| `[2]`          | declares `2: red`                   | declares `2: red`             |

**Two defects, and the second is the one that reaches a stylesheet.**

1. **A key the other compiler names.** A boolean, `null` and an object all name
   a property there and refuse here, so what changes is whether a build stops.
   None of the four is a CSS property anybody writes on purpose, so the refusal
   may well be the better answer -- but it has to be one refusal with one
   sentence, decided rather than inherited.
2. **A key both compilers name, differently.** `[1 > 2]` is `false` to the
   language and to the reference implementation. It reads as `0` here, because
   a comparison is folded through the numeric reading of a binary expression
   before the key is asked for a string. So the two compilers write two
   different property names for one source, with no error either side.

The second is the one to answer first. The first has two readings: name the key
`String(key)` as the language does, or refuse every key that is not a string or
a number, and say so in one sentence rather than the two the two paths give
today -- `Key is not a string` from `evaluate_obj_key`, and
`Expected a string value but received a non-string expression.` from the same
key inside a folded object literal.

Also note that a big-integer key already agrees: `1n` and `1` name the same
property in both compilers, which is the row that says the digits reading is
right.

**Where the code is:** `evaluate_obj_key` in
`crates/stylex-evaluator/src/evaluate/mod.rs`, which reads the folded key
through `convert_expr_to_str`; and `nodes/object_expression.rs`, which reads the
same key through its own conversion and writes the second sentence.

**Blocked by:** None. Found by the ninth instalment of
[ticket 15](./15-cover-the-evaluator-crate.md), which covered these readings
rather than changing them.

**Status:** resolved

## Comments

**2026-09-13, resolved.** Both defects are closed, and the whole table now
agrees with the reference implementation.

**The decision: `String(key)`.** A computed key names the property the language
names it, read through the compiler's own ECMA `ToString`
(`stylex_js::coercions::to_js_string`) rather than through the converter that
spells a string. That converter answers for a string and a number and nothing
else, so the refusal was inherited rather than chosen. Measured against
`@stylexjs/babel-plugin@0.19.0`, every row:

| Key written | Before | After | Reference implementation |
| --- | --- | --- | --- |
| `[true]` | refuses | `true: red` | `true: red` |
| `[false]` / `[!1]` | refuses | `false: red` | `false: red` |
| `[null]` | refuses | `null: red` | `null: red` |
| `[[1, 2]]` | refuses | `1,2: red` | `1,2: red` |
| `[[]]` | refuses | `: red` | `: red` |
| `[1 > 2]` | `0: red` | `false: red` | `false: red` |
| `1n` / `['color']` / `[2]` | unchanged | unchanged | agree |

The transform snapshots are byte-identical to upstream, class names included.

**Defect 2 was a comparison, not a key.** `[1 > 2]` named `0` because *every*
comparison folded to a number. `BinaryExprType` now carries a `Boolean`, so the
eight comparison operators answer what the language and the reference
implementation answer. A comparison still reads as `1` or `0` where a number is
asked for, and as its word where a string is.

That also closes **row 5 of [ticket 50](./50-answer-the-member-reads-the-reference-answers.md)**:
`content: (1 === 1)` wrote `content: "1px"` here and stops the build there.
Both now refuse under the same sentence, and the two parity corpus rows
`modules-comparison-as-a-whole-value-numbers` and `-strings` moved from
`acceptance-divergent` to `both-reject`.

**One refusal, one sentence.** The two key readers -- `evaluate_obj_key` and the
object-literal fold -- answered one mistake with two sentences. They now read
the key the same way and refuse with the same one, reworded to say what it
actually means: `The key has no name at compile time.` Two values reach it, and
both are tested: a function, whose `String` is its source text, and a text
holding a lone surrogate, which no Rust string can spell.

**Measured and not changed.** `!=` answers the negation of `===` here, where the
language answers the negation of `==`. The three pairs the two readings part on
-- `1 != '1'`, `null != undefined` and `0 != ''` -- were measured against the
reference implementation, which takes the strict answer for all three. The
parting is with the language and not with upstream, so it stays.

**One row still diverges.** `{ [{}]: 'red' }` names `[object Object]` in both
compilers now. Upstream then writes
`.x1ffvn62[object Object]{[object object]:red}` -- a selector with a space and a
bracket in it -- and the CSS layer here drops the declaration. Agreement would
mean emitting a broken selector on purpose. Recorded by
`an_object_as_a_computed_key_names_its_default_text` and the row beside it.

**The read side moved with it, which is ticket 50 row 2.** The ticket says one
rule naming a key `String(key)` settles the read and the write at once, and
leaving the write alone made them disagree inside one compiler. Four readers
answered "what property is this key" and none of them named a boolean, `null`
or an object; they are now one, `property_name`, over `as_string_key`, which is
the real `ToPropertyKey`. So `({ a: { b: 1 } }).a[true] ?? 'red'` folds to
`red`, and `({ true: 1 })[true]` reads `1`.

**Three refusals became one, and one abort became a refusal.** A key written as
a text with no `str` -- one holding a lone surrogate -- aborted the build from
inside the converter that spells an atom, reporting neither the key nor the
object it sits in. It now refuses beside every other key that has no name.

**Four more APIs moved toward the reference implementation** with the
comparison, each measured:

| Source | Before | After | Reference implementation |
| --- | --- | --- | --- |
| `keyframes({ from: { width: 1 === 1 } })` | `from{width:1px}` | `from{}` | `from{}` |
| `positionTry({ positionAnchor: '--a', width: 1 === 1 })` | wrote `width:1px` | drops it | drops it |
| `defineConsts({ x: 1 === 1 })` | `"1"` | `"true"` | `"true"` |
| `when.hover(1 === 1)` | read `"1"` | refuses | refuses |
