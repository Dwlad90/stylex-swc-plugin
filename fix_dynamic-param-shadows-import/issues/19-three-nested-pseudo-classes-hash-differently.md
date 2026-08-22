# 19 — Three nested pseudo-classes hash a different class name

Status: `resolved`
Blocked by: None

**What was found:** A style value nesting three pseudo-classes in an order that
is not already alphabetical compiles to a different class name than the
reference implementation produces for the same input. A class name is a hash of
the selector plus the declaration, so this is a compatibility divergence, not a
cosmetic one.

```js
export const styles = stylex.create({
  w: { zIndex: { default: '1', ':hover': { default: '1', ':focus': { default: '1', ':active': '1' } } } },
});
```

| | leaf selector | class |
| --- | --- | --- |
| `@stylexjs/babel-plugin` 0.19.0 | `:active:focus:hover` | `.x12rlomf` |
| rs-compiler HEAD | `:focus:hover:active` | `.x1dv1xrr` |

Both emit the same declaration (`z-index:1`) and the same number of rules, so
the verdict is `divergent`: same properties, different class names.

Measured on both compilers as
`modules-three-nested-pseudo-classes-hash-differently`, which records `divergent`
so the divergence reports as a changed verdict the day it moves.

## The mechanism, as far as it is measured

Upstream sorts the whole accumulated pseudo-class list; this compiler appears to
sort each pair as it nests and append the next one. That is what the four probes
say, and it is a description of the outputs, not of the code — the producing
call site is not pinned yet:

| nesting order | this compiler | upstream | verdict |
| --- | --- | --- | --- |
| `:hover` > `:focus` | `:focus:hover` | `:focus:hover` | identical |
| `:focus` > `:hover` | `:focus:hover` | `:focus:hover` | identical |
| `:hover` > `:active` | `:active:hover` | `:active:hover` | identical |
| `:active` > `:hover` | `:active:hover` | `:active:hover` | identical |
| `:hover` > `:focus` > `:active` | `:focus:hover:active` | `:active:focus:hover` | **divergent** |
| `:active` > `:focus` > `:hover` | `:active:focus:hover` | `:active:focus:hover` | identical |

Two pseudo-classes agree in either nesting order, which is why nothing before
this saw it: a pair sorted from either direction lands in the same place. Three
agree only where the nesting order is already alphabetical. At-rules are not
affected — they nest in the same order on both sides at every depth probed,
including eight levels.

## How it was found

Written as a depth guard for the identifier chain
(`modules-1266-a-shadowed-param-at-extreme-condition-depth`, ticket 09): a
shadowed dynamic parameter read at eight levels of condition nesting. The
resolution half agreed — identical declarations, one inline variable per leaf —
and the selectors did not. That guard now nests its pseudo-classes
alphabetically so it measures resolution, and this ticket owns the ordering.

Nothing to do with reference resolution, and nothing this effort's commits
introduced: the ordering is the same before and after the chain reorder.

Reproduced a second time by [12](./12-a-string-named-specifier-in-a-style-value.md),
whose own depth boundary -- a theme member read eight condition levels deep --
hashed a different selector until its pseudo-classes were nested alphabetically
too. Four of them there rather than three, and `@media` / `@supports` above them
change nothing, so the shape is the pseudo-class list alone at any length above
two. Two depth guards written for unrelated questions have now had to route
around this; the third should fix it instead.

- [x] The producing call site is named — where the pseudo-class list is
      assembled and where it is sorted
- [x] The order matches the reference implementation for three or more nested
      pseudo-classes, in any nesting order -- for the **run grouping**, which is
      this ticket's mechanism. Not absolutely: a run whose keys differ by letter
      case or in a non-ASCII position still sorts differently, at two keys as
      well as three, because the *comparator* diverges as well. That is a second
      mechanism, older than this fix, and
      [32](./32-the-pseudo-comparator-is-not-locale-aware.md) owns it
- [x] The corpus row's `expected` becomes `identical`, which is how the fix
      reports
- [x] Snapshots that carry a three-deep pseudo-class nesting are re-recorded,
      and any that do not exist are written — the divergence survived because no
      test nested three

## Answer

The producing call site is **`sort_pseudos`** in
`crates/stylex-css/src/utils/pre_rule.rs`. It is the only place the list is
ordered; the two callers -- `StylesPreRule::get_pseudos` in
`crates/stylex-transform/src/shared/structures/pre_rule.rs`, which filters the
key path down to keys opening with `:` or `[`, and
`convert_style_to_class_name`, which sorts again before hashing -- both hand it
the same list and neither reorders anything itself.

The defect was one condition. The function partitioned the list into groups and
sorted each group, but a group could only ever hold **two** keys:

```rust
if last_element.len() == 1 && !is_pseudo_element(&last_element[0]) {
```

`last_element.len() == 1` closed a group as soon as it held a pair, so a third
key opened a new one and appended after an already-sorted pair. Upstream's
`sortPseudos` closes a group only on a pseudo *element* -- `Array.isArray` is its
whole test -- so its groups grow without bound. That is exactly why two keys
agreed in either nesting order and three agreed only where the nesting order was
already alphabetical.

The group is now a two-variant `PseudoRun` -- `Element`, which pins its position
because it names which part of the element the rule targets rather than a state
the element is in, and `Sortable`, which grows for as long as no element
interrupts it and is sorted whole at whatever length it reached. That is
upstream's partition expressed as a type rather than as a length check, so the
length cannot come back.

### What it measures at

`.x12rlomf:active:focus:hover{z-index:1}` on both compilers for the reported
input, which is the class name this ticket recorded upstream naming. The corpus
row now records `identical` -- renamed to
`modules-three-nested-pseudo-classes-sort-as-one-run`, since the old id asserted
a divergence its own verdict no longer records.

The harness reports `changed: 0`, run against a `dist/` built from this working
tree -- a report is only ever about the last build, so the figure means nothing
without that. It counts 998 subjects, which is what the four corpus sets' 1086
entries collapse to once the harness has deduplicated them; both numbers are
real and they are not the same number.

The two depth guards that had to nest their pseudo-classes alphabetically to
route around this nest out of order again, so they measure both halves:
`modules-1266-a-shadowed-param-at-extreme-condition-depth`,
`modules-1266-a-string-named-theme-member-eight-conditions-deep`, and the Rust
case behind the second one. Reordering the keys must not cost a level -- the
first pass through this shortened the second guard from five pseudo-classes to
four, which quietly dropped it from eight condition levels to seven while its
name, comment and corpus note all still said eight. Restored, and the leaf still
hashes `x4f6iwg`, because it is the same set of keys either way.

### The emitted selector is not the sorted list

Worth writing down, because it is what makes the pseudo-element cases readable:
the *hash* reads the sorted list, and the *selector* prints the pseudo classes in
sorted order followed by every pseudo element. So a run on each side of an
element sorts separately and the element still prints last -- two separate facts
that only a case with a run on both sides tells apart, and
`a_pseudo_element_splits_two_runs_of_three` is that case.

### What the edge cases turned up

Filed as [32](./32-the-pseudo-comparator-is-not-locale-aware.md): the
**comparator** diverges too. Upstream sorts a run with `localeCompare`; this
compiler sorts by bytes, so `:HOVER` sorts ahead of `:active` here and behind it
there. Two keys is enough to see it, so it is not this ticket's run-length shape
and it did not arrive with this fix -- it is as old as `sort_pseudos`. Recorded
as `modules-pseudo-names-differing-only-in-case-hash-differently` (`divergent`)
and pinned in the suite rather than routed around.

Everything else measured agrees byte for byte with Babel 0.19.0, including the
keys neither compiler validates as CSS: an unclosed bracket, paren or quote, a
bare `:`, a triple colon (which reads as an *element*, since the `::` test is a
prefix test), a legacy single-colon `:before` (which reads as a *class*, and
sorts into the run), CSS escapes, non-ASCII names, functional pseudo-classes
carrying commas and spaces, and attribute selectors alone or mixed into a run.

### Where it is pinned

- `crates/stylex-css/src/utils/tests/pre_rule_test.rs` -- the unit level: the
  run of three, all six nesting orders, a seven-key run, an element splitting a
  run in two, an attribute selector inside one.
- `crates/stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs`
  -- new file, the transform level: the reported shape, all six nesting orders
  collapsing to one rule, four nested in reverse, every pseudo-element position,
  the degenerate and malformed keys, and the boundaries (a run as wide as the
  32-level nesting ceiling allows, an element splitting a wide run, a
  five-thousand-character pseudo name, and a repeated key, which both compilers
  refuse before the sort is reached -- which is why the sort need not be stable).
- The fixture `dynamic-param-shadows-import-edges`, whose `deeplyNested` case
  already nested three and four out of order and was recording the divergence:
  re-recorded, and its three changed class names are the ones Babel names.
