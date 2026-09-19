# 50 — Answer the member reads the reference implementation answers

**What to build:** Three member readings part company with
`@stylexjs/babel-plugin` 0.19.0, and each is pinned by a test the thirteenth
instalment of [ticket 15](./15-cover-the-evaluator-crate.md) added. The tests
record what this compiler does today; nothing decides that it is the answer to
keep. Decide each row, then make the test say the decision rather than the
observation.

Every row is measured through the built addon against the reference
implementation, with the probe beside the parity harness
(`crates/stylex-rs-compiler/parity/probe.ts`).

| Source                                   | This compiler                                       | Reference implementation                                    |
| ---------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------- |
| `'abc'[0]`                               | refuses: `Unsupported index: 0`                     | `a`                                                         |
| `({ a: { b: 1 } }).a[true] ?? 'red'`     | refuses: `Could not determine the property`         | `red`, because the read answers `undefined`                 |
| `({ color: 'red' })?.color`              | `red`                                               | refuses: `Unsupported expression: OptionalMemberExpression` |
| `(1 == ({})) ? 'red' : 'blue'`           | refuses: `Unsupported expression: BinaryExpression` | `blue`                                                      |
| `content: ('a' === 'a')`                 | `content: "1px"`                                    | refuses: a style value may not be a boolean                 |
| `Object.keys(sx.missing ?? '\u{1F600}')` | refuses: the text holds a lone surrogate            | `['0', '1']`                                                |

**Six decisions, and they do not all go the same way.**

1. **A member of a string.** The language indexes a string by UTF-16 code unit,
   which is the reading this compiler defends everywhere else. A read inside
   the Basic Multilingual Plane has one answer and no ambiguity; a read that
   lands on half an astral character has no Rust `str` to answer with, which is
   the same wall `Object.values` meets. So the shape of the answer is: index
   the code units, and refuse the read that lands on a lone surrogate.
2. **A key that is not a string or a number.** `[true]` is the property named
   `"true"`, which no object here carries, so the language answers `undefined`
   and `?? 'red'` folds. This is the same question
   [ticket 49](./49-name-a-computed-key-the-way-the-language-does.md) asks of a
   key being _written_; answer both together, because one rule that names a key
   `String(key)` settles the read and the write at once.
3. **An optional member read.** This compiler folds it and the reference
   implementation has no `Optional` handling at all, so it refuses. Here this
   compiler does more, not something else: the fold is correct, and an author
   who writes `?.` gets CSS here and a build error there. Keep it, and say in
   the test that it is a deliberate parting rather than the specification.

4. **An equality against a value that is not a primitive.** The language
   compares two objects by reference and this evaluator holds a copy, so the
   four equality operators fall to the numeric coercion for such a side, and
   the coercion refuses. The reference implementation compares the two values
   it holds and answers `false`. Whether a copy may answer `false` for two
   objects is the question: it is right for two object _literals_, which are
   two references, and wrong for one name read twice, which is one.

5. **A comparison as the whole style value.** A comparison folds to a number
   here and to a boolean there, so this compiler writes `content: "1px"` where
   the reference implementation stops the build -- a style value may be an
   array, a string or a number, and a boolean is none of them. The reading is
   this compiler's own and pre-dates ticket 15: `1 === 1` and `1 < 2` answered
   a number before the equality operators were made to compare two primitives.
   What that work changed is which comparisons reach the reading, which now
   holds the pairs neither side has a number for. Both rows are in the parity
   corpus as `acceptance-divergent`, so the answer moves visibly.

6. **The own keys of an astral string.** The engine answers `Object.keys` for
   an ordinary receiver and agrees with the reference implementation. The
   reading written out in Rust is reached only past a declined fold, and there
   an astral character is two code units, each a lone surrogate that no Rust
   string holds. It refuses whole rather than answering a key list that is
   short, which is the same choice a spread of the same string makes. The
   decision to take is whether a key list may be answered without its values,
   since the keys are `'0'` and `'1'` whatever the units hold.

**Where the code is:** `crates/stylex-evaluator/src/evaluate/nodes/`
— `member_expression.rs` for the first two readings and `optional_chain.rs` for
the third. The tests that pin the rows are
`evaluate/tests/member_lookup_tests.rs:67`, `:335` and
`evaluate/tests/optional_chain_tests.rs:18,25`.

**Blocked by:** None. Row 2 wants
[ticket 49](./49-name-a-computed-key-the-way-the-language-does.md) answered
first, or answered with it.

**Status:** resolved

## Comments

**2026-09-13, resolved.** Each of the six decisions is taken, and each row is
measured against `@stylexjs/babel-plugin@0.19.0`.

| Row | Before | After | Reference implementation |
| --- | --- | --- | --- |
| 1. `'abc'[0]` | refuses: `Unsupported index: 0` | `a` | `a` |
| 1b. `'\u{1F600}a'[0]` | refuses | the replacement character | the lone surrogate |
| 2. `({ a: { b: 1 } }).a[true] ?? 'red'` | refuses | `red` | `red` |
| 3. `({ color: 'red' })?.color` | `red` | `red` | refuses |
| 4. `(1 == ({})) ? 'red' : 'blue'` | refuses | refuses | `blue` |
| 5. `content: ('a' === 'a')` | `content: "1px"` | refuses | refuses |
| 6. `Object.keys(sx.missing ?? '\u{1F600}')` | refuses | refuses | `['0', '1']` |

**1. A string is indexed by code unit.** That is what the language counts, and
`'\u{1F600}a'[2]` is `a` in both compilers -- a character count called it index
one. The parity corpus row `modules-length-an-indexed-string-read` moved from
`acceptance-divergent` to `identical`.

Half of an astral character answers the replacement character, rather than
refusing. The first cut refused it, on the argument that writing the character
would put one in the stylesheet the source does not describe -- and the review
found the engine fold beside it already makes exactly that substitution, on
purpose and with a corpus row of its own: the reference implementation's lone
surrogate becomes the same character the moment the stylesheet is written to
disk, so the declaration text agrees and only the class name parts. So `s[0]`
refused where `s.charAt(0)` folded, for one read written two ways. They are now
one, and the paired row is `modules-an-index-that-lands-on-half-a-character`.

**No index refuses on any receiver now.** `unreadable_index` and the
`refuse_lookup` wrapper around it are deleted, and `ArrayLikeLookup::Index`
means an answer at every arm that matches it.

**2. Answered with ticket 49**, which is what the ticket asked for: one rule
naming a key `String(key)` settles the read and the write at once.

**3. `?.` stays folded.** This compiler does *more* here rather than something
else: the fold is the language's own answer, it writes no value the source does
not describe, and a build that compiles where the other stops is the one
direction a parting is safe in.

**4. A comparison with an object on either side refuses**, whichever operator
and whichever side. Two of the rows need no identity -- a primitive against an
object reduces the object through `ToPrimitive`, and a strict comparison of the
two is false on the types alone -- and folding those would leave `1 == ({})`
folding beside `({}) == ({})` refusing, for the same reason a copy cannot answer
either. One rule that never writes a wrong value beats two rows of a comparison
nobody writes on purpose. Pinned by
`a_comparison_with_an_object_on_either_side_refuses`.

**5. Answered with ticket 49**, by making a comparison answer a boolean.

**6. The own keys of an astral string stay refused whole.** The keys are the
indices whatever the units hold, so a key list could be answered without its
values -- but `keys`, `values` and `entries` are one question asked three ways,
and the values are two lone surrogates this compiler cannot write. Answering the
keys alone would make one spelling fold where the other two refuse.
