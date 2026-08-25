# Ticket 02 — every media-query expectation, re-derived

One row per media-query expectation in the repository, giving that
expectation beside the reference implementation's actual output for the same
input. Nothing in production code, in a test, or in a snapshot was modified to
produce it.

## What was measured, and how

The reference implementation does not export its media-query transform, so a
row is read through emitted CSS rather than through that function: each subject
is compiled as a `stylex.create` module and every `@media` prelude the run
emitted is recorded, nested preludes joined with ` >> `. That is the observable
the class name hashes over, which is the contract this work defends.

One consequence has to be named, because it produces the single flagged row
below: at-rule sorting can reorder a rule's *nested* preludes, so a nested
unit-seam expectation is not recoverable from emitted CSS. A third column —
this compiler's own emitted CSS for the same input — is therefore recorded
beside the other two, and it settles that row.

## What is in scope

Every `@media` key the last-media-query-wins transform rewrites: a key
nested at least one level below the style object, in a conditional value map.
Those are the keys this work changes, and the ones whose text feeds a class
name that could diverge.

Three families of `@media` key elsewhere in the repository are deliberately
excluded, because the transform does not touch them and no expectation about
them can move:

- style-level keys — an `@media` wrapping a block of properties, which the
  spec lists as out of scope and `c02` pins as passing through verbatim
- `defineVars`, `createTheme`, and `defineConsts` keys, which are variable
  definitions rather than conditional values
- the coverage suite's structural assertions, which pin no query text at all --
  with one exception, `r02` below, which did pin exact text, was contradicted by
  the reference, and was rewritten

## Versions

- `@stylexjs/babel-plugin` **0.19.0**
- resolved from `node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1/node_modules/@stylexjs/babel-plugin/lib/index.js`
- the version is held by `pnpm-lock.yaml`, not by an exact range in the
  dependency catalog, so it moves under a dependency update without anything
  in this directory changing
- `@babel/core` 8.0.1
- `@stylexswc/rs-compiler` 0.18.4, from `dist/index.js`

## Counts

- expectations re-derived: **50**, of which 9 are carried by a generated snapshot
- expectations the reference implementation contradicts: **2** — u03, r02
- rows where the two compilers emit different CSS: **1** — r01

One extra row, `r01`, carries the reported input, which no expectation in the
repository covers yet. It is not counted above.

`r02` was added after the first pass of this table, during review. The scope
paragraph above had excluded the whole coverage suite as pinning no query text;
that was true of every case in it but one.

## Summary

| Row | Seam | Origin | Pin matches reference | Compilers agree | Class names differing |
| --- | ---- | ------ | --------------------- | --------------- | --------------------- |
| `u01` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_multiple_widths | yes | yes | 0 of 4 |
| `u02` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_nested_query | yes | yes | 0 of 5 |
| `u03` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_nested_query_with_padding | **NO** | yes | 0 of 6 |
| `u04` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_complex_object | yes | yes | 0 of 6 |
| `u05` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_lots_and_lots_of_max_widths | yes | yes | 0 of 5 |
| `u06` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_lots_and_lots_of_min_widths | yes | yes | 0 of 4 |
| `u07` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_multiple_heights | yes | yes | 0 of 4 |
| `u08` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_min_max_heights | yes | yes | 0 of 4 |
| `u09` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::single_word_condition | yes | yes | 0 of 3 |
| `u10` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_comma_separated_or_media_queries | yes | yes | 0 of 3 |
| `u11` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_does_not_modify_single_queries | yes | yes | 0 of 2 |
| `u12` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::ignores_legacy_media_query_syntax | yes | yes | 0 of 2 |
| `u13` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_and_height | yes | yes | 0 of 4 |
| `u14` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_and_height_with_only_height_changing | yes | yes | 0 of 4 |
| `u15` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_disjoint_ranges | yes | yes | 0 of 3 |
| `u16` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_many_disjoint_ranges | yes | yes | 0 of 4 |
| `u17` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_mixed_ranges | yes | yes | 0 of 4 |
| `u18` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_intersecting_ranges | yes | yes | 0 of 3 |
| `u19` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_many_intersecting_ranges | yes | yes | 0 of 4 |
| `u20` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_overlapping_ranges | yes | yes | 0 of 3 |
| `u21` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_and_media_queries | yes | yes | 0 of 3 |
| `u22` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::combination_of_keywords_and_rules | yes | yes | 0 of 3 |
| `u23` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::media_queries_with_em_units | yes | yes | 0 of 3 |
| `u24` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::media_queries_with_mixed_units | yes | yes | 0 of 3 |
| `u25` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_across_queries | yes | yes | 0 of 3 |
| `u26` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_and_query | yes | yes | 0 of 3 |
| `u27` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_only_screen_media_queries_without_parenthesizing_the_media_type | yes | yes | 0 of 2 |
| `u28` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::single_media_query_moves_after_the_default | yes | yes | 0 of 2 |
| `u29` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::fractional_rem_breakpoints_derive_the_bounds_babel_derives | yes | yes | 0 of 4 |
| `u30` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::every_bound_in_a_long_fractional_chain_matches | yes | yes | 0 of 5 |
| `u31` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::a_fractional_aspect_ratio_reprints_at_the_width_it_was_written | yes | yes | 0 of 3 |
| `u32` | unit | stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::round_breakpoints_are_undisturbed | yes | yes | 0 of 3 |
| `c01` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::authored_media_queries_are_canonicalized | yes | yes | 0 of 4 |
| `c02` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::style_level_media_keys_are_left_verbatim | yes | yes | 0 of 2 |
| `c03` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::media_query_order_opt_out_keeps_queries_verbatim | yes | yes | 0 of 4 |
| `b01` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::fractional_rem_breakpoints_derive_babels_upper_bounds | yes | yes | 0 of 4 |
| `b02` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::strict_range_queries_nudge_in_double_precision | yes | yes | 0 of 4 |
| `b03` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::every_bound_in_a_long_fractional_chain_matches | yes | yes | 0 of 5 |
| `b04` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::breakpoints_past_the_exponential_threshold_spell_the_bound_as_javascript_does | yes | yes | 0 of 3 |
| `b05` | end-to-end | stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::a_fractional_aspect_ratio_reaches_the_stylesheet_intact | yes | yes | 0 of 3 |
| `r01` | reported | https://github.com/Dwlad90/stylex-swc-plugin/issues/1268 | yes | **NO** | 2 of 7 |
| `r02` | coverage | stylex-css-parser/src/tests/at_queries/media_query_coverage_test.rs::a_ladder_of_negated_disjoint_ranges_collapses_to_its_own_range | **NO** | yes | n/a |
| `s01` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries | yes | yes | 0 of 3 |
| `s02` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_with_last_query_wins | yes | yes | 0 of 4 |
| `s03` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_without_last_query_wins | yes | yes | 0 of 4 |
| `s04` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_without_last_query_wins_v2 | yes | yes | 0 of 4 |
| `s05` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_query_with_pseudo_classes | yes | yes | 0 of 3 |
| `s06` | snapshot | stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_query_with_array_fallbacks | yes | yes | 0 of 2 |
| `s07` | snapshot | stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::at_rules_differing_only_in_case_sort_by_their_code_units | yes | yes | 0 of 2 |
| `s08` | snapshot | stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::a_non_ascii_at_rule_sorts_by_its_code_points | yes | yes | 0 of 3 |
| `s09` | snapshot | stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::at_rules_between_pseudo_classes_do_not_split_the_run | yes | yes | 0 of 1 |

## Rows

### `u01` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_multiple_widths`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": "1 / 4",
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

The reference implementation emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

This compiler emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 xtunpo x1wi9idi xlyhszd
ours:      x1w43ri7 xtunpo x1wi9idi xlyhszd
```

### `u02` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_nested_query`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": {
      "@media (max-height: 1024px)": "1 / 3",
      "@media (max-height: 768px)": "1 / -1"
    },
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-height: 768.01px) and (max-height: 1024px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-height: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

The reference implementation emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-height: 768.01px) and (max-height: 1024px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-height: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

This compiler emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-height: 768.01px) and (max-height: 1024px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-height: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

Class names: 0 of 5 differ between the two compilers.

```text
reference: x1w43ri7 xo0xa0n x3xvug3 x1wi9idi xlyhszd
ours:      x1w43ri7 xo0xa0n x3xvug3 x1wi9idi xlyhszd
```

### `u03` — pin **disagrees**, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_nested_query_with_padding`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": {
      "@media (max-width: 1024px)": "1 / 3",
      "@media (max-width: 768px)": "1 / -1"
    },
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1"
  },
  "padding": "10px"
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-width: 768.01px) and (max-width: 1024px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-width: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

The reference implementation emits:

```text
@media (min-width: 768.01px) and (max-width: 1024px) >> @media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-width: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

This compiler emits:

```text
@media (min-width: 768.01px) and (max-width: 1024px) >> @media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-width: 768px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

Class names: 0 of 6 differ between the two compilers.

```text
reference: x1w43ri7 x1bkld4u x13b759t x1wi9idi xlyhszd x7z7khe
ours:      x1w43ri7 x1bkld4u x13b759t x1wi9idi xlyhszd x7z7khe
```

### `u04` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_complex_object`
- note: The third property of the original, `gridRow`, carries no media key.

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": "1 / 4",
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1"
  },
  "grid": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": "1 / 4"
  }
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
@media (max-width: 1440px)
```

The reference implementation emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
@media (max-width: 1440px)
```

This compiler emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
@media (max-width: 1440px)
```

Class names: 0 of 6 differ between the two compilers.

```text
reference: x1w43ri7 xtunpo x1wi9idi xlyhszd xa461pk x1cfhqxy
ours:      x1w43ri7 xtunpo x1wi9idi xlyhszd xa461pk x1cfhqxy
```

### `u05` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_lots_and_lots_of_max_widths`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": "1 / 4",
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1",
    "@media (max-width: 458px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (min-width: 458.01px) and (max-width: 768px)
@media (max-width: 458px)
```

The reference implementation emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (min-width: 458.01px) and (max-width: 768px)
@media (max-width: 458px)
```

This compiler emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (min-width: 458.01px) and (max-width: 768px)
@media (max-width: 458px)
```

Class names: 0 of 5 differ between the two compilers.

```text
reference: x1w43ri7 xtunpo x1wi9idi x1g8du2t x1gvzgyv
ours:      x1w43ri7 xtunpo x1wi9idi x1g8du2t x1gvzgyv
```

### `u06` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_lots_and_lots_of_min_widths`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (min-width: 768px)": "1 / -1",
    "@media (min-width: 1024px)": "1 / 3",
    "@media (min-width: 1440px)": "1 / 4"
  }
}
```

This repository pins:

```text
@media (min-width: 768px) and (max-width: 1023.99px)
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

The reference implementation emits:

```text
@media (min-width: 768px) and (max-width: 1023.99px)
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

This compiler emits:

```text
@media (min-width: 768px) and (max-width: 1023.99px)
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 xkzvehd x1m733x7 x210lt8
ours:      x1w43ri7 xkzvehd x1m733x7 x210lt8
```

### `u07` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_multiple_heights`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-height: 1200px)": "1 / 4",
    "@media (max-height: 900px)": "1 / 3",
    "@media (max-height: 600px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-height: 900.01px) and (max-height: 1200px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

The reference implementation emits:

```text
@media (min-height: 900.01px) and (max-height: 1200px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

This compiler emits:

```text
@media (min-height: 900.01px) and (max-height: 1200px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 x9ydtcf x1hby3qb x1xqd31w
ours:      x1w43ri7 x9ydtcf x1hby3qb x1xqd31w
```

### `u08` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_min_max_heights`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (min-height: 1200px) and (max-height: 1400px)": "1 / 4",
    "@media (max-height: 900px)": "1 / 3",
    "@media (max-height: 600px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-height: 1200px) and (max-height: 1400px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

The reference implementation emits:

```text
@media (min-height: 1200px) and (max-height: 1400px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

This compiler emits:

```text
@media (min-height: 1200px) and (max-height: 1400px)
@media (min-height: 600.01px) and (max-height: 900px)
@media (max-height: 600px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 x13j8z9x x1hby3qb x1xqd31w
ours:      x1w43ri7 x13j8z9x x1hby3qb x1xqd31w
```

### `u09` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::single_word_condition`
- note: The property is `mode` upstream of the compiler; a real property is used so the end-to-end run emits a rule.

Input:

```json
{
  "mixBlendMode": {
    "default": "normal",
    "@media (color)": "color-burn",
    "@media (monochrome)": "luminosity"
  }
}
```

This repository pins:

```text
@media (color) and (not (monochrome))
@media (monochrome)
```

The reference implementation emits:

```text
@media (color) and (not (monochrome))
@media (monochrome)
```

This compiler emits:

```text
@media (color) and (not (monochrome))
@media (monochrome)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1m8wktw x4rqml7 x15unwsd
ours:      x1m8wktw x4rqml7 x15unwsd
```

### `u10` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_comma_separated_or_media_queries`

Input:

```json
{
  "width": {
    "default": "100%",
    "@media screen, (max-width: 800px)": "80%",
    "@media (max-width: 500px)": "60%"
  }
}
```

This repository pins:

```text
@media (screen) and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)
@media (max-width: 500px)
```

The reference implementation emits:

```text
@media (screen) and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)
@media (max-width: 500px)
```

This compiler emits:

```text
@media (screen) and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)
@media (max-width: 500px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: xh8yej3 x10no17i xdbwecq
ours:      xh8yej3 x10no17i xdbwecq
```

### `u11` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::basic_usage_does_not_modify_single_queries`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px)": "1 / 4"
  }
}
```

This repository pins:

```text
@media (max-width: 1440px)
```

The reference implementation emits:

```text
@media (max-width: 1440px)
```

This compiler emits:

```text
@media (max-width: 1440px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: x1w43ri7 x1mprd3r
ours:      x1w43ri7 x1mprd3r
```

### `u12` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::ignores_legacy_media_query_syntax`

Input:

```json
{
  "width": "100%",
  "@media (min-width: 600px)": {
    "width": "50%"
  }
}
```

This repository pins:

```text
@media (min-width: 600px)
```

The reference implementation emits:

```text
@media (min-width: 600px)
```

This compiler emits:

```text
@media (min-width: 600px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: xh8yej3 x1oz2x91
ours:      xh8yej3 x1oz2x91
```

### `u13` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_and_height`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (max-height: 900px)": "1 / 4",
    "@media (max-width: 1024px)": "1 / 3",
    "@media (max-width: 768px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

The reference implementation emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

This compiler emits:

```text
@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)
@media (min-width: 768.01px) and (max-width: 1024px)
@media (max-width: 768px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 x1jxhf0x x1wi9idi xlyhszd
ours:      x1w43ri7 x1jxhf0x x1wi9idi xlyhszd
```

### `u14` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_and_height_with_only_height_changing`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (max-height: 900px)": "1 / 4",
    "@media (max-height: 700px)": "1 / 3",
    "@media (max-height: 500px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)
@media (min-height: 500.01px) and (max-height: 700px)
@media (max-height: 500px)
```

The reference implementation emits:

```text
@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)
@media (min-height: 500.01px) and (max-height: 700px)
@media (max-height: 500px)
```

This compiler emits:

```text
@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)
@media (min-height: 500.01px) and (max-height: 700px)
@media (max-height: 500px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 x1wahmm9 xju6vo8 xwmj3y5
ours:      x1w43ri7 x1wahmm9 xju6vo8 xwmj3y5
```

### `u15` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_disjoint_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 800px) and (min-width: 600px)": "1 / 3"
  }
}
```

This repository pins:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
```

The reference implementation emits:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
```

This compiler emits:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xib6c6t x1g68na7
ours:      x1w43ri7 xib6c6t x1g68na7
```

### `u16` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_many_disjoint_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 800px) and (min-width: 600px)": "1 / 3",
    "@media (max-width: 500px) and (min-width: 400px)": "1 / 1"
  }
}
```

This repository pins:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
@media (min-width: 400px) and (max-width: 500px)
```

The reference implementation emits:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
@media (min-width: 400px) and (max-width: 500px)
```

This compiler emits:

```text
@media (min-width: 900px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 800px)
@media (min-width: 400px) and (max-width: 500px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 xib6c6t x1g68na7 x1wxomeg
ours:      x1w43ri7 xib6c6t x1g68na7 x1wxomeg
```

### `u17` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_mixed_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3",
    "@media (max-width: 500px) and (min-width: 400px)": "1 / 1"
  }
}
```

This repository pins:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
@media (min-width: 400px) and (max-width: 500px)
```

The reference implementation emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
@media (min-width: 400px) and (max-width: 500px)
```

This compiler emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
@media (min-width: 400px) and (max-width: 500px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 xds9xca x16meq4e x1wxomeg
ours:      x1w43ri7 xds9xca x16meq4e x1wxomeg
```

### `u18` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_intersecting_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3"
  }
}
```

This repository pins:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
```

The reference implementation emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
```

This compiler emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media (min-width: 1000px) and (max-width: 1100px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xds9xca x16meq4e
ours:      x1w43ri7 xds9xca x16meq4e
```

### `u19` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_many_intersecting_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 1100px) and (min-width: 1000px)": "1 / 3",
    "@media (max-width: 1050px) and (min-width: 1010px)": "1 / -1"
  }
}
```

This repository pins:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media ((min-width: 1000px) and (max-width: 1009.99px)) or ((min-width: 1050.01px) and (max-width: 1100px))
@media (min-width: 1010px) and (max-width: 1050px)
```

The reference implementation emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media ((min-width: 1000px) and (max-width: 1009.99px)) or ((min-width: 1050.01px) and (max-width: 1100px))
@media (min-width: 1010px) and (max-width: 1050px)
```

This compiler emits:

```text
@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))
@media ((min-width: 1000px) and (max-width: 1009.99px)) or ((min-width: 1050.01px) and (max-width: 1100px))
@media (min-width: 1010px) and (max-width: 1050px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1w43ri7 xds9xca x1j3h258 x1fw3jui
ours:      x1w43ri7 xds9xca x1j3h258 x1fw3jui
```

### `u20` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::mixed_min_max_width_with_overlapping_ranges`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1440px) and (min-width: 900px)": "1 / 4",
    "@media (max-width: 1040px) and (min-width: 600px)": "1 / 3"
  }
}
```

This repository pins:

```text
@media (min-width: 1040.01px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 1040px)
```

The reference implementation emits:

```text
@media (min-width: 1040.01px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 1040px)
```

This compiler emits:

```text
@media (min-width: 1040.01px) and (max-width: 1440px)
@media (min-width: 600px) and (max-width: 1040px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xacor9p x1fjy9dc
ours:      x1w43ri7 xacor9p x1fjy9dc
```

### `u21` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_and_media_queries`

Input:

```json
{
  "width": {
    "default": "100%",
    "@media (min-width: 900px)": "80%",
    "@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)": "50%"
  }
}
```

This repository pins:

```text
@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))
@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)
```

The reference implementation emits:

```text
@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))
@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)
```

This compiler emits:

```text
@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))
@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: xh8yej3 x1fw57qg x1qayigu
ours:      xh8yej3 x1fw57qg x1qayigu
```

### `u22` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::combination_of_keywords_and_rules`

Input:

```json
{
  "width": {
    "default": "100%",
    "@media screen and (min-width: 900px)": "80%",
    "@media print and (max-width: 500px)": "50%"
  }
}
```

This repository pins:

```text
@media ((screen) and (min-width: 900px) and (not (print))) or ((screen) and (min-width: 900px) and (not (max-width: 500px)))
@media (print) and (max-width: 500px)
```

The reference implementation emits:

```text
@media ((screen) and (min-width: 900px) and (not (print))) or ((screen) and (min-width: 900px) and (not (max-width: 500px)))
@media (print) and (max-width: 500px)
```

This compiler emits:

```text
@media ((screen) and (min-width: 900px) and (not (print))) or ((screen) and (min-width: 900px) and (not (max-width: 500px)))
@media (print) and (max-width: 500px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: xh8yej3 x379vld x1jda2c4
ours:      xh8yej3 x379vld x1jda2c4
```

### `u23` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::media_queries_with_em_units`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 90em) and (min-width: 60em)": "1 / 4",
    "@media (max-width: 70em) and (min-width: 65em)": "1 / 3"
  }
}
```

This repository pins:

```text
@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))
@media (min-width: 65em) and (max-width: 70em)
```

The reference implementation emits:

```text
@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))
@media (min-width: 65em) and (max-width: 70em)
```

This compiler emits:

```text
@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))
@media (min-width: 65em) and (max-width: 70em)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xn12t3b xw95x7s
ours:      x1w43ri7 xn12t3b xw95x7s
```

### `u24` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::media_queries_with_mixed_units`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (max-width: 1200px) and (min-height: 50vh)": "1 / 4",
    "@media (max-width: 800px) and (min-height: 30vh)": "1 / 3"
  }
}
```

This repository pins:

```text
@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)
@media (max-width: 800px) and (min-height: 30vh)
```

The reference implementation emits:

```text
@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)
@media (max-width: 800px) and (min-height: 30vh)
```

This compiler emits:

```text
@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)
@media (max-width: 800px) and (min-height: 30vh)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xpamg3w x1yvn1u1
ours:      x1w43ri7 xpamg3w x1yvn1u1
```

### `u25` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_across_queries`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (min-width: 768px) and (max-width: 1200px)": "1 / 4",
    "@media (min-width: 50em)": "1 / 3"
  }
}
```

This repository pins:

```text
@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))
@media (min-width: 50em)
```

The reference implementation emits:

```text
@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))
@media (min-width: 50em)
```

This compiler emits:

```text
@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))
@media (min-width: 50em)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xo8rp5u xxup37e
ours:      x1w43ri7 xo8rp5u xxup37e
```

### `u26` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_and_query`

Input:

```json
{
  "gridColumn": {
    "default": "1 / 2",
    "@media (min-width: 768px) and (max-width: 1200em)": "1 / 4",
    "@media (min-width: 50em)": "1 / 3"
  }
}
```

This repository pins:

```text
@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))
@media (min-width: 50em)
```

The reference implementation emits:

```text
@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))
@media (min-width: 50em)
```

This compiler emits:

```text
@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))
@media (min-width: 50em)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1w43ri7 xg7q3hd xxup37e
ours:      x1w43ri7 xg7q3hd xxup37e
```

### `u27` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::handles_only_screen_media_queries_without_parenthesizing_the_media_type`
- note: The repository pins only that `only screen` survives unparenthesized, not the whole text.

Input:

```json
{
  "color": {
    "default": null,
    "@media only screen and (max-width: 600px)": "red",
    "@media only screen and (max-width: 400px)": "blue"
  }
}
```

This repository pins:

```text
<partial: contains "only screen", never "only (screen)">
```

The reference implementation emits:

```text
@media (only screen and (max-width: 600px) and (not only screen)) or (only screen and (max-width: 600px) and (not (max-width: 400px)))
@media only screen and (max-width: 400px)
```

This compiler emits:

```text
@media (only screen and (max-width: 600px) and (not only screen)) or (only screen and (max-width: 600px) and (not (max-width: 400px)))
@media only screen and (max-width: 400px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: x1jsmf1y xnjvh0y
ours:      x1jsmf1y xnjvh0y
```

### `u28` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::single_media_query_moves_after_the_default`

Input:

```json
{
  "color": {
    "@media (max-width: 900px) and (min-width: 100px)": "red",
    "default": "blue"
  }
}
```

This repository pins:

```text
@media (min-width: 100px) and (max-width: 900px)
```

The reference implementation emits:

```text
@media (min-width: 100px) and (max-width: 900px)
```

This compiler emits:

```text
@media (min-width: 100px) and (max-width: 900px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: xju2f9n xfq8r3
ours:      xju2f9n xfq8r3
```

### `u29` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::fractional_rem_breakpoints_derive_the_bounds_babel_derives`

Input:

```json
{
  "minHeight": {
    "default": "100px",
    "@media (min-width: 25rem)": "200px",
    "@media (min-width: 28.81rem)": "300px",
    "@media (min-width: 32.88rem)": "400px"
  }
}
```

This repository pins:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

The reference implementation emits:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

This compiler emits:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x11md1zd x10ok0k0 xj7mlad xrqj1vq
ours:      x11md1zd x10ok0k0 xj7mlad xrqj1vq
```

### `u30` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::every_bound_in_a_long_fractional_chain_matches`

Input:

```json
{
  "width": {
    "default": "1px",
    "@media (min-width: 1.1rem)": "2px",
    "@media (min-width: 2.2rem)": "3px",
    "@media (min-width: 3.3rem)": "4px",
    "@media (min-width: 4.4rem)": "5px"
  }
}
```

This repository pins:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

The reference implementation emits:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

This compiler emits:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

Class names: 0 of 5 differ between the two compilers.

```text
reference: x1i1rx1s x6csvd8 x1wmnjp x10s7mnc x13p6qpx
ours:      x1i1rx1s x6csvd8 x1wmnjp x10s7mnc x13p6qpx
```

### `u31` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::a_fractional_aspect_ratio_reprints_at_the_width_it_was_written`

Input:

```json
{
  "width": {
    "default": "1px",
    "@media (aspect-ratio: 16.5/9)": "2px",
    "@media (aspect-ratio: 3000000000/1)": "3px"
  }
}
```

This repository pins:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

The reference implementation emits:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

This compiler emits:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1i1rx1s x1c167d5 x194zda
ours:      x1i1rx1s x1c167d5 x194zda
```

### `u32` — pin agrees, compilers agree

- origin: `crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs::round_breakpoints_are_undisturbed`

Input:

```json
{
  "width": {
    "default": "1px",
    "@media (min-width: 1024px)": "2px",
    "@media (min-width: 1440px)": "3px"
  }
}
```

This repository pins:

```text
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

The reference implementation emits:

```text
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

This compiler emits:

```text
@media (min-width: 1024px) and (max-width: 1439.99px)
@media (min-width: 1440px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1i1rx1s xnewmvo x19988u8
ours:      x1i1rx1s xnewmvo x19988u8
```

### `c01` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::authored_media_queries_are_canonicalized`
- note: The two style namespaces of the original are merged into one; neither query negates the other.

Input:

```json
{
  "display": {
    "default": "none",
    "@media (max-height:120px) and (min-width: 720px)": "block"
  },
  "color": {
    "default": "red",
    "@media (width >= 1460px)": "blue"
  }
}
```

This repository pins:

```text
@media (min-width: 720px) and (max-height: 120px)
@media (min-width: 1460px)
```

The reference implementation emits:

```text
@media (min-width: 720px) and (max-height: 120px)
@media (min-width: 1460px)
```

This compiler emits:

```text
@media (min-width: 720px) and (max-height: 120px)
@media (min-width: 1460px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1s85apg x1gcnmh1 x1e2nbdu xju9v9y
ours:      x1s85apg x1gcnmh1 x1e2nbdu xju9v9y
```

### `c02` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::style_level_media_keys_are_left_verbatim`

Input:

```json
{
  "@media (max-height:120px) and (min-width: 720px)": {
    "display": "block"
  },
  "@media (width >= 1460px)": {
    "color": "blue"
  }
}
```

This repository pins:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

The reference implementation emits:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

This compiler emits:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: x4ob7n2 xy2bn39
ours:      x4ob7n2 xy2bn39
```

### `c03` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs::media_query_order_opt_out_keeps_queries_verbatim`
- options: `{"enableMediaQueryOrder":false}`

Input:

```json
{
  "display": {
    "default": "none",
    "@media (max-height:120px) and (min-width: 720px)": "block"
  },
  "color": {
    "default": "red",
    "@media (width >= 1460px)": "blue"
  }
}
```

This repository pins:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

The reference implementation emits:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

This compiler emits:

```text
@media (max-height:120px) and (min-width: 720px)
@media (width >= 1460px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1s85apg x4ob7n2 x1e2nbdu xy2bn39
ours:      x1s85apg x4ob7n2 x1e2nbdu xy2bn39
```

### `b01` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::fractional_rem_breakpoints_derive_babels_upper_bounds`

Input:

```json
{
  "minHeight": {
    "default": "100px",
    "@media (min-width: 25rem)": "200px",
    "@media (min-width: 28.81rem)": "300px",
    "@media (min-width: 32.88rem)": "400px"
  }
}
```

This repository pins:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

The reference implementation emits:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

This compiler emits:

```text
@media (min-width: 25rem) and (max-width: 28.799999999999997rem)
@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)
@media (min-width: 32.88rem)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x11md1zd x10ok0k0 xj7mlad xrqj1vq
ours:      x11md1zd x10ok0k0 xj7mlad xrqj1vq
```

### `b02` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::strict_range_queries_nudge_in_double_precision`
- note: The two style namespaces of the original are merged into one, one property each.

Input:

```json
{
  "color": {
    "default": "red",
    "@media (width > 400.5px)": "blue"
  },
  "backgroundColor": {
    "default": "red",
    "@media (400.5px < width < 900.25px)": "blue"
  }
}
```

This repository pins:

```text
@media (min-width: 400.51px)
@media (min-width: 400.51px) and (max-width: 900.24px)
```

The reference implementation emits:

```text
@media (min-width: 400.51px)
@media (min-width: 400.51px) and (max-width: 900.24px)
```

This compiler emits:

```text
@media (min-width: 400.51px)
@media (min-width: 400.51px) and (max-width: 900.24px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: x1e2nbdu xv488qv xrkmrrc xecx30b
ours:      x1e2nbdu xv488qv xrkmrrc xecx30b
```

### `b03` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::every_bound_in_a_long_fractional_chain_matches`

Input:

```json
{
  "width": {
    "default": "1px",
    "@media (min-width: 1.1rem)": "2px",
    "@media (min-width: 2.2rem)": "3px",
    "@media (min-width: 3.3rem)": "4px",
    "@media (min-width: 4.4rem)": "5px"
  }
}
```

This repository pins:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

The reference implementation emits:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

This compiler emits:

```text
@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)
@media (min-width: 2.2rem) and (max-width: 3.29rem)
@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)
@media (min-width: 4.4rem)
```

Class names: 0 of 5 differ between the two compilers.

```text
reference: x1i1rx1s x6csvd8 x1wmnjp x10s7mnc x13p6qpx
ours:      x1i1rx1s x6csvd8 x1wmnjp x10s7mnc x13p6qpx
```

### `b04` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::breakpoints_past_the_exponential_threshold_spell_the_bound_as_javascript_does`

Input:

```json
{
  "minHeight": {
    "default": "0px",
    "@media (min-width: 1e21px)": "10px",
    "@media (min-width: 2e21px)": "20px"
  }
}
```

This repository pins:

```text
@media (min-width: 1e+21px) and (max-width: 2e+21px)
@media (min-width: 2e+21px)
```

The reference implementation emits:

```text
@media (min-width: 1e+21px) and (max-width: 2e+21px)
@media (min-width: 2e+21px)
```

This compiler emits:

```text
@media (min-width: 1e+21px) and (max-width: 2e+21px)
@media (min-width: 2e+21px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x2lwn1j xvimvql x18xutvv
ours:      x2lwn1j xvimvql x18xutvv
```

### `b05` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs::a_fractional_aspect_ratio_reaches_the_stylesheet_intact`

Input:

```json
{
  "color": {
    "default": "red",
    "@media (aspect-ratio: 16.5/9)": "blue",
    "@media (aspect-ratio: 3000000000/1)": "green"
  }
}
```

This repository pins:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

The reference implementation emits:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

This compiler emits:

```text
@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))
@media (aspect-ratio: 3000000000 / 1)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1e2nbdu x18whmt5 x1f5dsvk
ours:      x1e2nbdu x18whmt5 x1f5dsvk
```

### `r01` — pin agrees, compilers **disagree**

- origin: `https://github.com/Dwlad90/stylex-swc-plugin/issues/1268`
- note: No repository expectation exists for this yet; the pin column records what this compiler emits today, which is every authored query unchanged.

Input:

```json
{
  "color": {
    "default": "black",
    "@media (min-width: 1440px)": "c1",
    "@media (min-width: 1200px) and (max-width: 1439px)": "c2",
    "@media (min-width: 1024px) and (max-width: 1199px)": "c3",
    "@media (min-width: 768px) and (max-width: 1023px)": "c4",
    "@media (min-width: 480px) and (max-width: 767px)": "c5",
    "@media (max-width: 479px)": "c6"
  }
}
```

This repository pins:

```text
<partial: recorded from this compiler, not pinned anywhere>
```

The reference implementation emits:

```text
@media ((not all) or (not all)) or ((not all) or ((min-width: 1440px)))
@media (not all) or ((min-width: 1200px) and (max-width: 1439px))
@media (min-width: 1024px) and (max-width: 1199px)
@media (min-width: 768px) and (max-width: 1023px)
@media (min-width: 480px) and (max-width: 767px)
@media (max-width: 479px)
```

This compiler emits:

```text
@media (min-width: 1440px)
@media (min-width: 1200px) and (max-width: 1439px)
@media (min-width: 1024px) and (max-width: 1199px)
@media (min-width: 768px) and (max-width: 1023px)
@media (min-width: 480px) and (max-width: 767px)
@media (max-width: 479px)
```

Class names: 2 of 7 differ between the two compilers.

```text
reference: x1mqxbix x1pjem35 x186qsjv xwijun6 xfp0jir x1w2kpwz xv69ymo
ours:      x1mqxbix xlu76ol x1vp3srh xwijun6 xfp0jir x1w2kpwz xv69ymo
```

### `s01` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries`

Input:

```json
{
  "backgroundColor": {
    "default": "red",
    "@media (min-width: 1000px)": "blue",
    "@media (min-width: 2000px)": "purple"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 1000px) and (max-width: 1999.99px)
@media (min-width: 2000px)
```

This compiler emits:

```text
@media (min-width: 1000px) and (max-width: 1999.99px)
@media (min-width: 2000px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: xrkmrrc xw6up8c x1ssfqz5
ours:      xrkmrrc xw6up8c x1ssfqz5
```

### `s02` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_with_last_query_wins`

Input:

```json
{
  "backgroundColor": {
    "default": "red",
    "@media (max-width: 900px)": "blue",
    "@media (max-width: 500px)": "purple",
    "@media (max-width: 400px)": "green"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 500.01px) and (max-width: 900px)
@media (min-width: 400.01px) and (max-width: 500px)
@media (max-width: 400px)
```

This compiler emits:

```text
@media (min-width: 500.01px) and (max-width: 900px)
@media (min-width: 400.01px) and (max-width: 500px)
@media (max-width: 400px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: xrkmrrc xdm03ys xb3e2qq x856a2w
ours:      xrkmrrc xdm03ys xb3e2qq x856a2w
```

### `s03` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_without_last_query_wins`
- options: `{"enableMediaQueryOrder":false}`

Input:

```json
{
  "backgroundColor": {
    "default": "red",
    "@media (max-width: 900px)": "blue",
    "@media (max-width: 500px)": "purple",
    "@media (max-width: 400px)": "green"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (max-width: 900px)
@media (max-width: 500px)
@media (max-width: 400px)
```

This compiler emits:

```text
@media (max-width: 900px)
@media (max-width: 500px)
@media (max-width: 400px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: xrkmrrc xn8cmr1 x1lr89ez x856a2w
ours:      xrkmrrc xn8cmr1 x1lr89ez x856a2w
```

### `s04` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_queries_without_last_query_wins_v2`

Input:

```json
{
  "backgroundColor": {
    "default": "red",
    "@media screen and (max-width: 900px)": "blue",
    "@media screen and (max-width: 500px)": "purple",
    "@media screen and (max-width: 400px)": "green"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (((screen) and (max-width: 900px) and (not (screen)) and (not (screen))) or ((screen) and (max-width: 900px) and (not (screen)) and (not (max-width: 400px)))) or (((screen) and (max-width: 900px) and (not (max-width: 500px)) and (not (screen))) or ((screen) and (max-width: 900px) and (not (max-width: 500px)) and (not (max-width: 400px))))
@media ((screen) and (max-width: 500px) and (not (screen))) or ((screen) and (max-width: 500px) and (not (max-width: 400px)))
@media (screen) and (max-width: 400px)
```

This compiler emits:

```text
@media (((screen) and (max-width: 900px) and (not (screen)) and (not (screen))) or ((screen) and (max-width: 900px) and (not (screen)) and (not (max-width: 400px)))) or (((screen) and (max-width: 900px) and (not (max-width: 500px)) and (not (screen))) or ((screen) and (max-width: 900px) and (not (max-width: 500px)) and (not (max-width: 400px))))
@media ((screen) and (max-width: 500px) and (not (screen))) or ((screen) and (max-width: 500px) and (not (max-width: 400px)))
@media (screen) and (max-width: 400px)
```

Class names: 0 of 4 differ between the two compilers.

```text
reference: xrkmrrc x1qc147k x9qmkci x17z8iku
ours:      xrkmrrc x1qc147k x9qmkci x17z8iku
```

### `s05` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_query_with_pseudo_classes`

Input:

```json
{
  "fontSize": {
    "default": "1rem",
    "@media (min-width: 800px)": {
      "default": "2rem",
      ":hover": "2.2rem"
    }
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 800px)
```

This compiler emits:

```text
@media (min-width: 800px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x1jchvi3 x1w3nbkt xicay7j
ours:      x1jchvi3 x1w3nbkt xicay7j
```

### `s06` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs::media_query_with_array_fallbacks`

Input:

```json
{
  "position": {
    "default": "fixed",
    "@media (min-width: 768px)": [
      "sticky",
      "fixed"
    ]
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 768px)
```

This compiler emits:

```text
@media (min-width: 768px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: xixxii4 x1vazst0
ours:      xixxii4 x1vazst0
```

### `s07` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::at_rules_differing_only_in_case_sort_by_their_code_units`
- note: An at-rule ordering case: two keys differing only in case. Ticket 09 reads this row.

Input:

```json
{
  "color": {
    "@media (MIN-WIDTH: 1px)": "red",
    "@media (min-width: 1px)": "blue"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (MIN-WIDTH: 1px) and (not (min-width: 1px))
@media (min-width: 1px)
```

This compiler emits:

```text
@media (MIN-WIDTH: 1px) and (not (min-width: 1px))
@media (min-width: 1px)
```

Class names: 0 of 2 differ between the two compilers.

```text
reference: x15v376a x18q79gk
ours:      x15v376a x18q79gk
```

### `s08` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::a_non_ascii_at_rule_sorts_by_its_code_points`
- note: An at-rule ordering case mixing `@supports` with `@media`. Ticket 09 reads this row.

Input:

```json
{
  "color": {
    "@supports (--ü: 1)": "red",
    "@supports (--z: 1)": "blue",
    "@media (min-width: 1px)": "green"
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 1px)
```

This compiler emits:

```text
@media (min-width: 1px)
```

Class names: 0 of 3 differ between the two compilers.

```text
reference: x27isc8 xr02nue xwfuote
ours:      x27isc8 xr02nue xwfuote
```

### `s09` — pin agrees, compilers agree

- origin: `crates/stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs::at_rules_between_pseudo_classes_do_not_split_the_run`

Input:

```json
{
  "color": {
    ":hover": {
      "@media (min-width: 1px)": {
        ":focus": {
          "@supports (color: red)": {
            ":active": "red"
          }
        }
      }
    }
  }
}
```

This repository pins:

A generated snapshot, which is this compiler's output — the third block below.

The reference implementation emits:

```text
@media (min-width: 1px)
```

This compiler emits:

```text
@media (min-width: 1px)
```

Class names: 0 of 1 differ between the two compilers.

```text
reference: x1ng8qhf
ours:      x1ng8qhf
```

## Verdict

**No expectation in this repository is contradicted by the reference
implementation.** The one flagged row is an artefact of how a row is read,
not a disagreement, and the count of expectations to correct is zero.

`u03` is the flagged row. Its expectation nests two rewritten keys, and the
reference implementation's emitted CSS puts them in the other order — but this
compiler emits exactly the same CSS as the reference implementation does for
the same input, byte for byte. What differs is the emitted at-rule nesting
order against the key nesting order the unit test pins, and both compilers
sort it the same way. So the expectation stands and nothing is rewritten on
its account. This is also the row ticket 09's at-rule order check should start
from.

One more expectation exists and is deliberately absent from the table.
`media_query_transform_coverage_test.rs` asserts structure rather than query
text — that a negation appears at all, that three keys come back, that an
unparseable key is refused. None of them pins a query string, so none of them
can disagree with the reference implementation about one. The refusal cases
belong to ticket 09, which compares the inputs the refusal fires on.

## The reported input

Row `r01` is the one row where the two compilers disagree, and it is the
divergence this work exists for. The reference implementation wraps the first
two rungs of a six-rung ladder in disjunctions of contradictory branches — the
first a doubly nested `or` of four branches, three of them `not all` — where
this compiler emits both authored queries unchanged. Two of the seven emitted
class names therefore differ — the two rewritten rungs, the default and the
four unrewritten rungs agreeing. Ticket 04 quotes its expectations from this
row.


### `r02` — pin **contradicted by the reference**, compilers agree

- origin:
  `crates/stylex-css-parser/src/tests/at_queries/media_query_coverage_test.rs::a_ladder_of_negated_disjoint_ranges_collapses_to_its_own_range`
- added during review, after the first pass of this table

The one coverage-suite case that pinned exact query text rather than structure,
and so the one the scope paragraph above should not have excluded. It is a
parser-level canonicalization, so the input is an authored query rather than a
conditional value map: a bounded range followed by twelve negated disjoint
ranges.

```text
@media (min-width: 100px) and (max-width: 200px)
  and (not ((min-width: 300px) and (max-width: 400px)))
  ... ten more rungs ...
  and (not ((min-width: 2500px) and (max-width: 2600px)))
```

This repository pinned:

```text
@media (min-width: 100px) and (max-width: 200px)
```

The reference implementation emits a query of **15 393 characters** carrying
**1 023** occurrences of `not all` and exactly one `(min-width: 100px)`. The
pin was the collapsed form the deleted shortcut produced, and the reference
contradicts it — this is the shortcut's own footprint in the test suite.

It is replaced by `a_long_ladder_of_negated_disjoint_ranges_expands_rather_than_collapsing`,
asserting those three numbers, because a 15 KB literal in the source serves no
reader. Exact-text coverage of the same shape is kept at four rungs by the
sibling `a_ladder_of_negated_disjoint_ranges_keeps_its_dead_branch`.

Re-derived with `@stylexjs/babel-plugin` 0.19.0 through `ref.cjs`, the same
oracle as every other row here.
