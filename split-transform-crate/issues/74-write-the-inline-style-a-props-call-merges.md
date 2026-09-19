# 74 — Write the inline style a `props` call merges

**What to fix:** A `stylex.props` call that is given a plain object beside a
compiled style stops the build. The reference implementation compiles it.

```js
import stylex from '@stylexjs/stylex';
const styles = stylex.create({ red: { color: 'red' } });
export default stylex.props(styles.red, { color: 'blue' });
```

Measured with `pnpm run parity:probe` from `crates/stylex-rs-compiler`:

```
rust  REFUSED: [StyleX] [UNREACHABLE] Encountered an unsupported value type during AST conversion.
babel out:    export default { className: "x1e2nbdu", style: { color: "blue" } };
```

**Where it stops.** `props_map` files an argument carrying no `$$css` marker
under `style`, as `FlatCompiledStylesValue::KeyValues`. `convert_object_to_ast`
in `js_to_ast.rs` writes a `String`, a `Null` and a `Bool`, and reaches
`stylex_unreachable!` for everything else — so the one value `props` builds
that is not a scalar is the one the writer has no arm for. `attrs` does not
stop, because it turns the same pairs into the text a `style` attribute holds
before the writer sees them.

So the shape is built on purpose, carried on purpose, and then refused by a
guard that calls it unreachable.

**What it holds open elsewhere.** `stylex_merge.rs:258` is the fall-through a
JSX spread takes when a property of the merged object spells no attribute. The
`style` object is the only property `props` can build that spells none, so that
fall-through cannot be reached until this is fixed, and it is the last
unexercised region of `stylex_merge.rs`.

**Blocked by:** nothing. Found while taking
[63](./63-cover-the-transform-shared-utils.md).

**Status:** resolved

- [x] The source above compiles, and the module both compilers print is the
      same one.
- [x] A JSX spread of such a call keeps the object the merge built rather than
      becoming attributes, and `stylex_merge.rs:258` is exercised by that test.
- [x] Tests cover an inline style on its own, one beside a compiled style, one
      whose value is not static, and an empty one.
- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
      region in `js_to_ast.rs` or `stylex_merge.rs`.

## Comments

### What was built

`convert_values_to_ast` was given the one arm it had no answer for: an inline
style is written as the object a `style` property holds, each value kept as
text. A compiled value that reads as a number is written as a number, and an
inline one is not -- `gridRow: '1'` is a string where the author wrote it, and
upstream prints it as one.

`props_map` also stopped writing the CSS spelling of each name. It is the one
map both `props` and `attrs` read, and only `attrs` wants CSS text; it asks
for that spelling itself, through `inline_style_to_css_string`. So the name a
`props` call answers is now the name the author wrote --
`{ backgroundColor: "blue" }`, as upstream prints it -- and no attribute
changed: no snapshot outside the new ones moved.

That second edit is part of the fix, not beside it. The headline source spells
`color`, which is one name in both spellings, so it alone does not need it.
Every camelCase name does: without the edit the first `props` call ever to
answer an inline style would answer `{ "background-color": "blue" }`, which
React does not apply and which upstream does not print.

Ten sources were compared with `pnpm run parity:probe`, and both compilers
print the same module for every one: an inline style alone, beside a compiled
style, empty, holding only a null, under a condition, merged with a second
inline style, with a value that is not static, with a name in each of the two
spellings, read as attributes, and with a value that reads as a number.

### Where the JSX spread lands

A spread is written out as attributes only where every property spells one.
An inline style is an object, so it spells none, and the spread keeps the
object the merge built:
`<div {...{ className: "...", style: { color: "blue" } }} />`.

This is the one place the two compilers answer differently, and it is
deliberate. Upstream writes the object into a string attribute and prints
`style="[object Object]"`, which no browser can read. The fall-through this
exercises is what stops that.

### The coverage reading

`scripts/coverage-missing.sh -p stylex_transform --skip-toolchain-check`, run
after the change: 71 uncovered regions over 16 files, and 2 counted against
the gate in `visit_mut_var_declarator.rs`. Neither `js_to_ast.rs` nor
`stylex_merge.rs` is among them, and neither are `props.rs` and `attrs.rs`.
The count before the change was 84 over 20 files, as ticket 67 records.

### What the review changed, and what it did not

Five reviews ran over the change. What they asked for and what was done:

- The custom-property case no longer told the two spellings apart once the
  name stopped being dashed, so both name cases now read the one declaration
  through `props` and through `attrs` together. A name that must not be dashed
  and a name that must be are each pinned by the contrast rather than by one
  half of it.
- The snapshot of two merged inline styles pins an order. A snapshot cannot
  say whether an order is right, so the test now records that the order was
  compared with the reference.
- The `style` property holding nothing has no source that makes it, and the
  two unit tests that build the shape now say so and name 84.
- The reader of a written object was keyed by position, which would have read
  the wrong property after a reorder rather than failing. It reads by name now,
  through one helper both cases share.
- The pair list is sized where it is made. The props map is not, and a
  reservation put there in the first round was taken out again: a review had
  said an unsized map pays two growths per call, and a measurement says it
  pays none. `IndexMap` reserves three on the first insert, and three names
  are the most that map ever holds, so a reservation saves no growth and costs
  an allocation for a merge that writes nothing.

One was declined. The new arm sits in `convert_values_to_ast`, which also
writes `defineVars`, `defineConsts`, the theme overrides and every compiled
namespace, and a reviewer asked for it to sit on a writer of its own. There
is no writer of its own to move it to: `props`, `attrs` and `stylex` all
answer through the one path, so a second writer would copy the string, null
and boolean arms to serve one caller.

### A second round over the same change

Three more reviews read the first round. They found one comment that said no
source reaches an inline style holding nothing, which is false --
`stylex.props({ color: true })` reaches it today, and 84 is what stops it.
Two more comments named the wrong step as the one that leaves the value out:
the merge keeps it, and the step that writes the properties leaves it out.
All three are corrected, and a duplicate assertion that restated a
neighbouring test was taken out again.

### What is left

Three inline values still cannot be carried -- a number, a boolean and a
nested object. They are filed as
[84](./84-carry-an-inline-value-that-is-not-text.md), with the measurements.
A number is the common one.
