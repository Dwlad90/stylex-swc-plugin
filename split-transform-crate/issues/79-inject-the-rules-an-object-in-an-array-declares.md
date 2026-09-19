# 79 — Inject the rules a `create` inside an object inside an array declares

**What to fix:** A `stylex.create` call written inside an object literal that
is itself inside a top-level array compiles its class names and injects none of
its rules. It is the shape [69](./69-inject-the-rules-an-array-bound-create-declares.md)
closed, one level further in.

```js
import * as stylex from "@stylexjs/stylex";
export const all = [{ s: stylex.create({ a: { color: "red" } }) }];
```

What this compiler writes, with runtime injection on:

```js
import _inject from "@stylexjs/stylex/lib/stylex-inject";
export const all = [{ s: { a: { kMwMTN: "x1e2nbdu", $$css: true } } }];
```

The reference implementation writes `var _inject2 = _inject;` and the
`_inject2({ ltr: ".x1e2nbdu{color:red}", priority: 3000 })` call before the
statement. Measured against `@stylexjs/babel-plugin` 0.19.0.

The spelling is accepted rather than refused: the bare `export const all = { s:
stylex.create({ ... }) }` is refused with "create() calls must be bound to a
bare variable", but the array around it makes the validator read the call as
program level.

**Where it starts.** `push_decl_init_hashes` in
`crates/stylex-state/src/state_manager.rs` names the declaration a queued
injection call belongs to by hashing the declarator initializer, and
`push_init_hashes` looks through arrays and parentheses to reach the object.
It stops at an object literal, which it hashes whole, so the hash of the inner
object the call was replaced by is never taken.

**What was settled.** The walk looks inside an object a container holds, and
only there. An initializer that is an object *is* the object a producer
registered, so it is hashed and left alone; an object a container holds can be
either that object or a wrapper around one, so it is hashed and looked inside.

That is what keeps the cost where the author put it. Hashing an object walks
its whole subtree, so an object read inside another is read twice, and a chain
of containers costs the length of the chain. A large top-level object -- the
shape a real module writes -- is hashed exactly once, as it always was, because
the walk does not look inside an initializer. Only what an author nested inside
a top-level array or an object that array holds is read more than once.

Upstream settles the direction. Its `validateStyleXCreate` refuses only a call
with no parent, or one whose parent is an expression statement, so every shape
in this family compiles there. Parity is to inject, not to refuse.

**Blocked by:** None.

**Status:** resolved

- [x] A `create` inside an object inside a top-level array injects every rule it
      declares.
- [x] The walk that finds it costs the nesting the author wrote, and nothing for
      the nesting it did not.
- [x] A case compares the shape with the array-bound and name-bound spellings.

## Comments

Found by the review of [69](./69-inject-the-rules-an-array-bound-create-declares.md),
which closed the array-bound shape. Not a regression from that work: the
spelling behaved the same way before it.

### A heavy constant beside it

The same review measured that an array whose elements each hold more than 128
properties, or hold an arrow, now takes one deep clone and a second walk per
element rather than one per declarator -- the `stable_hash_unspanned` fallback
in `crates/stylex-utils/src/hash.rs`. It stays linear in the module, and no
benchmark covers the shape because `runtimeInjection` is not a key the
benchmark manifest accepts.

### What it prints now

Both compilers print the same module for a `create` inside an object an array
holds, and for one an object two levels down holds. Measured against
`@stylexjs/babel-plugin` 0.19.0 with `runtimeInjection: true`.

A spread carries a value the same way a named property does, so it is read the
same way. A shorthand carries a name, and a method or an accessor carries a
body, and a call inside a body is not at program level, so none of the three
can hold a registered object and all three are stepped over.

### One shape of the family is still refused

`export const all = { s: stylex.create({ ... }) }`, with no array, is refused
here with "create() calls must be bound to a bare variable" and compiled by
upstream. That is a rule of the validator rather than of this walk: the walk
would find the object if the call reached it. It is recorded in
[80](./80-decide-what-binds-a-create-call.md).

### What the descent costs, and what measures it

From the performance review of the change:

- A chain of containers `d` deep is walked `d` times at the top, `d - 1` times
  one level down, and so on -- the square of the depth. The parser's own
  recursion gives out long before that square is worth counting, and a compiled
  style object is three or four levels, so the real number is small.
- A large top-level object is untouched: one hash, exactly as before, because
  the walk does not look inside an initializer.
- An array of compiled styles costs more. For
  `apps/rollup-large-example/lotsOfStyles.js`, which holds 21,733 `create`
  calls in one array, the review put the walk at about 282,000 hashes and map
  probes rather than 21,700, and about 2.3 times the node visits. The hash
  buffer peaks near 4.5 MB for that one declarator.
- `stable_hash_unspanned` gives up on an object of more than 128 properties, or
  one holding an arrow, and deep-clones the subtree instead. Where such an
  object sits inside another, both clone, so the bytes copied grow with the
  depth as well.

**Nothing measures this.** `benchmark/fixtures.v1.json` holds one entry with
`runtimeInjection: true`, and it points at a module that binds its `create` to
a name -- the path the walk does not descend. The two large array fixtures run
with runtime injection off, where the walk is skipped whole. So the cost above
is reasoned, not measured, and a fixture that pairs a large array initializer
with runtime injection is what would measure it. Recorded in
[81](./81-measure-the-injection-walk-over-a-large-array.md).

### The same class, one step further out

A call a top-level array holds through an arrow, a conditional or another call
is still read as program level and still loses its rules. The walk cannot close
those, because the arrow shape needs the call to be hoisted, which is decided
before the walk runs. Recorded with its measurements in
[82](./82-read-what-a-top-level-array-really-holds.md).
