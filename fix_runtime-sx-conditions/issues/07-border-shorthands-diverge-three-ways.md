# 07 — `borderTop` is emitted here, dropped upstream, rejected by design

Status: `resolved`
Phase: Phase 2

**What was built:** the `property-specificity` rejection table keyed by the
property names an author actually writes.

Found while comparing the reporter's output. It is **not** an `sx` bug — the
conditional-style pipeline is fine. `sx={[styles.base, flag && styles.alt]}`
with `alt: { color: "red" }` folds identically in both compilers. The
divergence is the declaration `borderTop: "none"` in the reporter's `alternate`
style.

## No decision was needed

This issue was filed asking for a decision between rejecting, dropping
silently, and keeping the emitted rule, on the reading that upstream's stated
intent and its observed behaviour disagreed. They do not.

`shared/utils/default-options.js` sets `propertyValidationMode: 'silent'` and
`styleResolution: 'property-specificity'`. The `throw` in
`preprocess-rules/property-specificity.js` is not swallowed — it is caught by
`preprocess-rules/index.js:57` and, under the default mode, reported as
nothing. Rejecting the property and dropping it silently are the same
behaviour, selected by an option that already exists here with the same three
values and the same default. There was nothing to choose.

## Root cause

`Shorthands::get` in
`crates/stylex-css/src/order/constants/property_specificity_order.rs` was keyed
by the Rust function names rather than by the property names:

```rust
"border_top" => Some(Shorthands::border_top),
```

Nothing ever looks a property up as `border_top`, so every multi-word shorthand
missed the table and fell through to `OrderPair(key, value)` — reaching the
stylesheet as `.x76ihet{border-top:none}` and defeating the specificity model
the table exists to enforce, because a later `borderTopWidth` cannot reliably
override `border-top`.

`border`, `background` and `animation` are spelled as one lowercase word, so
those three matched by accident. They are also the only three the existing
tests covered, which is how the gap survived. `all` was missing from the table
outright, and the six deprecated aliases delegated to their own names
(`Shorthands::get("borderHorizontal")`) rather than to the shorthand they are
an alias of (`borderInline`), so each returned `None` as well.

The other two resolutions were already keyed correctly. This was one table.

## Measured against `@stylexjs/babel-plugin@0.19.0`

Full parity corpus, 809 subjects, before and after:

| verdict                | before | after |
| ---------------------- | -----: | ----: |
| identical              |    713 |   717 |
| identical (empty)      |      5 |    30 |
| divergent              |      0 |     0 |
| structurally divergent |     17 |     1 |
| acceptance divergent   |     34 |    34 |
| both reject            |     27 |    27 |

Sixteen of the seventeen structural divergences were this bug, including the
reporter's own module (`modules-1265-through-a-binding`, now `identical`). The
survivor is `toString: "notfn"`, unrelated. Nothing moved in a worse direction.
Report saved as `parity/results/ticket-07.json`.

## Release note

Phase 2 requires one, and this repo carries no `CHANGELOG.md` or changeset —
they were removed in `47a97c5d6` — so the text lives here for whoever cuts the
release:

> **Fixed:** under the default `property-specificity` style resolution, the
> border shorthands `borderTop`, `borderRight`, `borderBottom`, `borderLeft`,
> `borderInline`, `borderBlock`, `borderInlineStart`, `borderInlineEnd` and the
> `all` shorthand were emitted as shorthand CSS rules instead of being refused.
> They are now refused, matching `@stylexjs/babel-plugin` and the behaviour
> `propertyValidationMode` selects. The six deprecated aliases
> (`borderHorizontal`, `borderVertical`, `borderBlockStart`, `borderBlockEnd`,
> `borderStart`, `borderEnd`) are refused too, and report the name of the
> shorthand they alias.
>
> **This changes emitted CSS and generated class names.** A style that declared
> only one of these properties now produces no class at all — for example
> `{ borderTop: "none" }` no longer emits `.x76ihet{border-top:none}`. Builds
> relying on the old output must switch to the longhands
> (`borderTopWidth`/`borderTopStyle`/`borderTopColor`), or set
> `styleResolution: "application-order"` or `"legacy-expand-shorthands"`, both
> of which still expand these. Set `propertyValidationMode: "throw"` or
> `"warn"` to be told rather than have them dropped silently.

## Verified against `@stylexjs/babel-plugin@0.19.0`

Mechanically, not by eye — each check diffs the two implementations and reports
a count, so "match" means an empty diff rather than a spot check.

| checked | result |
| --- | --- |
| `shorthands` keys | 12 / 12, empty diff both directions |
| `aliases` keys | 56 / 56, empty diff both directions |
| deprecated alias delegation targets | 6 / 6 (confirmed against the shipped `.d.ts`, which types them as `typeof shorthands.borderInline` and so on) |
| alias expansion outputs | 50 / 50 — the ordered property list each one expands to |
| rejection message text | 12 / 12, character for character |
| real rejection count | 1 in `application-order`, matching upstream's one `all` throw |
| table key/alias collisions | none, so `Aliases`-before-`Shorthands` precedence cannot differ from upstream's `{...shorthands, ...aliases}` |

Nine of the twelve messages had never been reachable, so nothing had ever read
them: all nine lacked upstream's trailing period, and `borderLeft` concatenated
its three sentences where upstream joins with a space. Fixed, and pinned by a
test, because the parity harness compares outcomes and never messages.

Two message divergences found in the same sweep are **outside this table** and
stay filed rather than fixed here: `list_style` in the
`legacy-expand-shorthands` table
([11](./11-listStyle-rejection-text-diverges.md)), and `null` as a style value
([10](./10-a-null-style-value-is-rejected-not-dropped.md)), which reproduces on
a plain `color`.

## Coverage added

- `property_specificity_structures_test.rs` — every rejecting shorthand is
  reachable by its authored name, and a snake_case spelling is not a property
  name.
- `property_specificity_constants_test.rs` — each deprecated alias answers with
  the rejection of the shorthand it delegates to, not its own.
- `property_validation_mode_test.rs` — all twelve rejecting shorthands and all
  six aliases declare nothing under `silent`; `throw` names the right property;
  a rejected shorthand leaves its longhand neighbours alone; the rejection
  ignores its value, including empty, blank, unclosed, unbalanced, unicode and
  `!important` ones; `application-order` and `legacy-expand-shorthands` still
  expand these.

  That value coverage deliberately stops short of `null` and `false`. A
  validator one layer up rejects both before the shorthand table is consulted,
  and upstream drops `null` rather than rejecting it — a real divergence, but
  one that reproduces on a plain `color` too. Filed as
  [10](./10-a-null-style-value-is-rejected-not-dropped.md).
- `parity/corpus/reported.json` — sixteen entries pinning the whole table
  against upstream.

## One harmless collision, recorded so the next audit skips it

`legacy_expand_shorthands_order.rs` registers `borderInlineWidth`,
`borderInlineStyle` and `borderInlineColor` in *both* its tables.
`get_expansion_fn` consults `Aliases` first, matching upstream's
`{...shorthands, ...aliases}` precedence, and both sides resolve to
byte-identical expansions — so the collision is harmless. Recorded because the
audit surfaced it, not because anything needs doing.

## One test was measuring the wrong option

`legacy/transform_call/keep_stylex_create_when_needed_test/` was ported without
the `styleResolution: 'application-order'` that upstream's `transform()` pins
for that `describe`, so it ran under the default resolution and its border
snapshot recorded the bug as expected output. Both files in the module now pin
the option, matching upstream's snapshot exactly.
