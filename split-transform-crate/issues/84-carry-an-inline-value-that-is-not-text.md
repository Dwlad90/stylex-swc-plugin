# 84 — Carry an inline value that is not text

**What to fix:** three inline-style values a `props` call cannot carry. Text
now reaches the `style` property that
[74](./74-write-the-inline-style-a-props-call-merges.md) opened, and a number,
a boolean and a nested object do not. Two of the three stop the build and the
third drops the declaration without a word.

Measured with `pnpm run parity:probe` from `crates/stylex-rs-compiler` against
`@stylexjs/babel-plugin` 0.19.0:

| Source                                     | Here                                                                       | Upstream                                     |
| ------------------------------------------ | -------------------------------------------------------------------------- | -------------------------------------------- |
| `stylex.props({ opacity: 0.5 })`           | REFUSED: `Unhandled literal type in nullable style parsing array`          | `{ style: { opacity: 0.5 } }`                |
| `stylex.props({ color: true })`            | `{ style: {} }`                                                            | `{ style: { color: true } }`                 |
| `stylex.props({ ':hover': { ... } })`      | REFUSED: `[UNIMPLEMENTED] Encountered an unsupported expression type ...`  | `{ style: { ":hover": { color: "blue" } } }` |

A number is the common one of the three: `opacity`, `zIndex`, `flexGrow` and
`order` are all written without a unit.

**Where each starts.**

- The number: `parse_nullable_key_value` in `parse_nullable_style.rs` reads a
  string, a boolean and a null, and refuses every other literal.
- The boolean: `parse_nullable_key_value` reads it, and `props_map` in
  `props.rs` keeps only the values that are text, so the declaration is gone
  by the time the property is written.
- The nested object: `parse_nullable_object` reads a literal value only.

**What the answer has to hold.** `Pair` carries two strings, and the `style`
property is built from a list of them, so no shape below `String` can be
written back. The three above are a number, a boolean and a map, so the value
a pair carries has to say which of those it is. Whatever carries them must
also answer `attrs`, which writes the same declarations out as CSS text --
what upstream writes for a boolean and for a nested object under `attrs` is
the first thing to measure.

**Two things measured while [74](./74-write-the-inline-style-a-props-call-merges.md)
was taken.** `{ color: true }` is the only source that reaches
`{ style: {} }` -- a `style` property holding nothing -- so that shape has no
end-to-end test until this ticket lands, and the unit tests that build it say
so. And `FlatCompiledStylesValue::KeyValue`, the single-pair variant, has no
producer anywhere outside tests; whatever carries the value kinds above should
take it in or take it out rather than leave a third spelling beside them.

**Blocked by:** nothing. [74](./74-write-the-inline-style-a-props-call-merges.md)
carries the text case and names this ticket for the rest.

**Status:** resolved

- [x] Each of the three sources above compiles, and the module both compilers
      print is the same one.
- [x] The same three read as attributes are measured against upstream first,
      and match it.
- [x] Tests cover each value kind on its own, one beside a compiled style, and
      a merge where a later declaration of another kind wins.
- [x] The full workspace suite stays green, and so is the coverage gate:
      100.00% of regions, functions and lines.

## Comments

### Where it landed

`ea5427c77 fix(stylex_transform): carry an inline value that is not text`,
then `971893b8e fix(stylex_transform): read every value an inline style can
hold`, then `49a69cd38 fix(stylex_transform): keep the kind a defineConsts
constant was written with`.

### What carries the kinds

`FlatCompiledStylesValue` carries them, because that is already the map an
inline style crosses `styleq` in: a list of pairs was built below it and threw
every kind but text away. The type gained `Number(f64)` and
`Object(FlatCompiledStyles)` -- the second one is both a pseudo-class body and
the inline style itself once a merge has written it under `style` -- and lost
`KeyValue` and `KeyValues`, which answers what the ticket asked about the
single-pair variant: it is taken out, and the list beside it with it. A
hand-written `Hash` replaces the derived one, since a number has none.

`props` now moves the merged map into the `style` property rather than copying
every name and value into a pair, and `attrs` borrows both halves of each
declaration rather than owning them, so both paths allocate less than before.
`inline_style_to_css_string` takes `&[PairCow<'_>]` for the second of those.

### What upstream writes, measured

`pnpm run parity:probe` from `crates/stylex-rs-compiler` against
`@stylexjs/babel-plugin` 0.19.0. Every row below is what both compilers now
print.

| Source                                     | Both                                         |
| ------------------------------------------ | -------------------------------------------- |
| `props({ opacity: 0.5 })`                  | `{ style: { opacity: 0.5 } }`                |
| `props({ color: true })`                   | `{ style: { color: true } }`                 |
| `props({ ':hover': { color: 'blue' } })`   | `{ style: { ":hover": { color: "blue" } } }` |
| `attrs({ opacity: 0.5 })`                  | `{ style: "opacity:0.5" }`                   |
| `attrs({ color: true })`                   | `{ style: "color:true" }`                    |
| `attrs({ ':hover': { color: 'blue' } })`   | `{ style: ":hover:[object Object]" }`        |

`attrs` spells each value the way JavaScript spells it, because the runtime
writes `${toKebabCase(key)}:${value}` and nothing else. An object therefore
spells `[object Object]`, which is what upstream writes too.

### The numbers the emitter spells its own way

Three of them. The emitter writes `-0` where the language writes `0`, and it
has no numeral at all for `NaN` or for an infinity, so it invents `0 / 0` and
`1 / 0`. Each inline number therefore carries the spelling JavaScript gives
it, and the emitter prints that rather than inventing one. Measured over
`1e-7`, `1e-6`, `1e20`, `1e21`, `100`, `0.1`, `1.50` and
`1.2345678901234568e+29` as well: every one of those already agreed.

`attrs` was right about all three from the start, because it spells its text
through the same reader. The two writers of one value now agree.

### One unreadable value, one outcome

A value kind an inline style cannot hold now refuses in both writers. It used
to stop the build where a `style` property is written and write a style short
of one declaration where an attribute is. No source reaches either -- the kinds
the reader produces are text, a number, a boolean, a `null` and an object --
so this is about what a later change is caught by, not about a live defect.

### The rest of the kinds, and two neighbours left alone

A second pass closed every other kind a `props` call can be given. Measured
with `pnpm run parity:probe` against `@stylexjs/babel-plugin` 0.19.0, and both
compilers now print the same module for each:

| Source                                         | Both                                              |
| ---------------------------------------------- | ------------------------------------------------- |
| `props({ color: undefined, margin: '1px' })`   | `{ style: { margin: "1px" } }`                    |
| `props({ color: ['red', 'blue'] })`            | `{ style: { color: { "0": "red", "1": "blue" } } }` |
| `attrs({ color: ['red', 'blue'] })`            | `{ style: "color:red,blue" }`                     |
| `props({ __proto__: 'red', color: 'blue' })`   | `{ style: { color: "blue" } }`                    |
| `attrs({ '--myColor': 'red' })`                | `{ style: "--my-color:red" }`                     |

Four readings behind those rows:

- A declaration with no value is its own kind. The merge already knew how to
  skip one -- `StyleqValue::is_undefined` is the question it asks -- and
  nothing could answer yes until now. It is not a `null`: a null clears the
  property and holds it against every style behind it, and an absence leaves
  the property for a later style to set.
- A list has no form of its own in a style object, so each element is named by
  the place it holds, and an attribute joins the elements with commas. The two
  disagree about a slot that holds nothing: a joined slot writes nothing, and
  a written one has no object form at all, so a list holding an absence is
  refused where a style object is written. The reference throws there.
- The name that sets what an object inherits from declares no property, in any
  of its three spellings, so it is left out.
- A `style` attribute is spelled by a rule of the runtime's own: every capital
  takes a hyphen and the whole name is lowercased, with no name left alone.
  That is not the stylesheet rule, and a custom property is where the two part
  company -- `--myColor` in a style property, `--my-color` in a style
  attribute. A test used to pin the wrong answer and now pins both.

The reading of the prototype name is the one that took two passes. The first
left it out whole, which is right for a value that is not an object and wrong
for one that is: the merge walks what a style inherits from, so a prototype
declares its own names after the style's and shadowed by them, and a prototype
that has one of its own reads the same way again. A prototype a *nested*
object was given is still left out whole, because nothing merges such an
object -- the runtime is handed it as it stands.

### The constants a `defineConsts` call answers

Taken in a third pass. Every constant lost the kind the author gave it, and
three readers guessed one back out of the text. Measured with `pnpm run
parity:probe` against `@stylexjs/babel-plugin` 0.19.0, over the module, the
metadata and the injected call; all three now agree with the reference.

| Source                        | Was                | Both now            |
| ----------------------------- | ------------------ | ------------------- |
| `defineConsts({ a: true })`    | `{ a: "true" }`    | `{ a: true }`       |
| `defineConsts({ a: null })`    | `{ a: "null" }`    | `{ a: null }`       |
| `defineConsts({ a: '100' })`   | `{ a: 100 }`       | `{ a: "100" }`      |
| `defineConsts({ a: 'Infinity' })` | `{ a: 1 / 0 }`  | `{ a: "Infinity" }` |
| `defineConsts({ a: { b: 1 } })` | `{ a: '{"b":1}' }` | `{ a: { b: 1 } }`   |
| `defineConsts({ a: ['x'] })`   | `{ a: '["x"]' }`   | `{ a: { "0": "x" } }` |

The reference keeps the evaluated value and writes it back with one function,
so this does the same. A constant is read by the reader the inline path
already used, which moved to `stylex-state` beside the value it builds, and
written by the one writer every compiled value goes through. The guess that
turned a numeric-looking string into a number is gone with the reason for it.

Three readers, two spellings. The metadata carries the value itself, and the
injected `_inject2` call carries a number as a number and every other kind as
the text JavaScript spells for it -- which is the reference's own split, not a
choice made here. Measured over all eleven kinds, including a `null`, which
writes no `constKey` and no `constVal` at all. The value crosses
`InjectableConstStyle`, whose crate sits below the value vocabulary, so it
travels as JSON and is read back where each spelling is written.

### One finding measured and left alone

A style nesting about 3000 levels deep aborts the process, where the reference
refuses at about 400 with a clean message. It is not this work and no guard
placed in the transform can reach it: a plain object of the same depth, with
no StyleX call anywhere in the file, aborts at the same depth, and so does one
that is cut short before its last brace. The abort is inside the SWC parser,
before any code here runs. SWC guards its statement recursion with `stacker`
and does not guard its expression recursion, which is the shape that dies.

The only reading that could refuse rather than abort is a budget applied to
the source before it is parsed, at `crates/stylex-rs-compiler/src/lib.rs`,
where the one parse happens. That needs a depth count taken over tokens rather
than over bytes -- a brace inside a string or a comment is not nesting -- and
a budget below the smallest stack the compiler runs on, which is not this
machine's. It is a piece of work of its own.


