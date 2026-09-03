# 32 — The namespace map keeps source order where the reference reorders it

**What to build:** Own-key ordering is applied to a style object but not to the
namespace map that `create` receives, so the two implementations disagree on
the order the namespaces reach the emitted object.

`create({ '+0': { color: 'red' }, 0: { color: 'blue' } })` emits `0` before
`+0` in the reference implementation, because it builds a real JavaScript
object and the language enumerates an array-index key first. This compiler
walks the properties in source order and emits `+0` first.

Found while measuring
[ticket 22](./22-settle-the-css-affecting-key-corrections.md), which proved the
same ordering **inside** a style object already agrees — `order_own_keys` runs
there, through the evaluator, and the array-index guard behaves. The namespace
map never reaches that reader.

The filing said this makes no CSS changes, because a namespace name is a key of
the emitted object rather than a declaration. **That was wrong**, and measuring
the reference implementation showed why: the namespaces are compiled in the
order they enumerate, so their rules are injected in that order too. An
index-like namespace therefore moves its whole rule set to the front of the
stylesheet, which decides the winner between two rules at equal specificity.
The divergence affects the emitted CSS and not only the object shape.

**Status:** resolved

- [x] Decide whether the namespace map must enumerate the way the language
      does, or whether source order is acceptable because no CSS depends on it
      -- it must; CSS does depend on it, see the Comments
- [x] If it must, route the namespace properties through the same own-key
      ordering the style object uses, and add a test for an index-like
      namespace name

## Comments

### What the reference implementation does

`evaluateStyleXCreateArg` builds a real object -- `const value = {}`, then
`value[key] = ...` per namespace -- so the language decides the order. The same
holds for the map of dynamic style functions beside it, which is why the
`@property` rules move with their namespace.

Measured against `@stylexjs/babel-plugin@0.19.0` under the parity harness's
options. Every case below now agrees, class names and injection order included:

| Namespaces written | Order both compilers emit |
| --- | --- |
| `'+0'`, `0`, `root` | `0`, `+0`, `root` |
| `z`, `2`, `'01'`, `0`, `4294967295`, `4294967294`, `''` | `0`, `2`, `4294967294`, `z`, `01`, `4294967295`, `''` |
| `10`, `9`, `100` | `9`, `10`, `100` |
| `named: c => ..`, `0: w => ..` | `0`, `named`, and the `@property` rules follow |

### What changed

`order_own_map_keys` in `stylex-ast` applies the order to an ordered map, beside
`order_own_keys`, which applies it to a property list. One file, one rule, and
`array_index_of` stays private. `evaluate_stylex_create_arg` orders its namespace
map and its dynamic-function map through it. The sort is skipped where no name is
an index, so an ordinary `create` call pays one scan over its namespace names.

The two readers keep separate mechanics, and a measurement decided that rather
than taste. A property list is split in one pass; rewriting it as a guarded
stable sort measured 1.30x the time over a 26-property object with two index
keys. An ordered map cannot be split where it stands, so it sorts. A second
measurement moved the key reading off `prop_key`, which renders a number key to
a string the order does not need: 0.88x on an object keyed by numbers, the shape
`{ 0: x }` parses to, and no worse on any other.

### Text for the pull request description

To be added beside the two paragraphs drafted in
[ticket 22](./22-settle-the-css-affecting-key-corrections.md):

> *Namespace order.* The same own-key reading now applies to the names a
> `create` call declares, not only to the keys inside a style object. A
> namespace named `0` is compiled before one named `root` or `+0`, whatever the
> order they are written in. Because a namespace carries its whole rule set, the
> rules move with it: the stylesheet order changes, and so does the winner
> between two rules at equal specificity. A `create` call with no index-like
> namespace name is unaffected.

### What the snapshots recorded

Four new cases in `numeric_and_index_like_keys.rs`, and five existing outputs
moved: `static_styles::style_object_multiple`, the four `debug_options` rows,
and both `namespace-cleaning` fixtures in dev and prod. Each moved output was
re-measured against the reference implementation and matches it.
