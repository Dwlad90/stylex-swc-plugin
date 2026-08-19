//! `null` as a style value: the one value that declares nothing.
//!
//! A `null` is not a failure and not a blank string. It compiles, and it
//! compiles to a property that is *present in the style object carrying no
//! class name* — which is how merging a later style unsets an earlier
//! declaration of the same property rather than shadowing it. Dropping the
//! property instead would leave the earlier declaration standing, so every
//! expectation here is about which keys survive, not only about which CSS is
//! emitted.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options, and agrees with it.

use crate::utils::prelude::*;
use stylex_enums::property_validation_mode::PropertyValidationMode;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── A `null` written directly ───────────────────────────────────────

// The property is present and carries no class name.
stylex_test!(
  a_null_longhand_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: null },
    });
  "#
);

// A property the specificity table refuses expands to nothing, so a `null`
// written on it leaves no key behind either — the rejection happens before the
// value is looked at. This is the one shape where a `null` declares nothing at
// all rather than an absent value.
stylex_test!(
  a_null_rejecting_shorthand_leaves_no_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { borderTop: null },
    });
  "#
);

// Under a resolution that expands it, the absence reaches every longhand the
// shorthand expands to — four keys, all unset.
stylex_test!(
  a_null_shorthand_unsets_every_longhand_it_expands_to,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::ApplicationOrder)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { borderTop: null },
    });
  "#
);

stylex_test!(
  a_null_shorthand_under_the_legacy_resolution,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { borderTop: null },
    });
  "#
);

// `margin` is not in the specificity table's rejection list and has no
// expansion there, so it keeps its own key.
stylex_test!(
  a_null_margin_keeps_its_own_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { margin: null },
    });
  "#
);

// The rejection of a shorthand is a property-name decision, so `throw` mode
// reports the property rather than looking at the `null` on it.
stylex_test_panic!(
  a_null_on_a_rejecting_shorthand_still_reports_the_property,
  "borderTop is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      x: { borderTop: null },
    });
  "#
);

// ── A `null` in a fallback array ────────────────────────────────────

// An array whose entries all carry nothing still declares the absent value.
// The array is not a different kind of value from the `null` inside it.
stylex_test!(
  an_array_of_one_null_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: [null] },
    });
  "#
);

stylex_test!(
  an_array_of_only_nulls_declares_one_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: [null, null, null] },
    });
  "#
);

// An empty array carries no property to declare an absence *of*, which is a
// different answer from `[null]`: no key at all.
stylex_test!(
  an_empty_array_leaves_no_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: [] },
    });
  "#
);

// A `null` beside a real entry drops out of the chain and the class name is
// hashed from what survives — the same class name a lone `'red'` yields.
stylex_test!(
  a_null_drops_out_of_a_fallback_chain,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      leading: { color: [null, 'red'] },
      trailing: { color: ['red', null] },
      lone: { color: 'red' },
    });
  "#
);

// Between two `var()` entries a `null` must not break their contiguity: the
// fallback chain is composed from the entries that survive, so the pair still
// reads as adjacent.
stylex_test!(
  a_null_between_two_vars_keeps_them_contiguous,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: ['var(--a)', null, 'var(--b)'] },
    });
  "#
);

// An array of nulls on a shorthand reaches the same answer the bare `null`
// does, per resolution.
stylex_test!(
  an_array_of_nulls_on_a_shorthand_matches_the_bare_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::ApplicationOrder)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      bare: { borderTop: null },
      wrapped: { borderTop: [null] },
    });
  "#
);

stylex_test!(
  an_array_of_nulls_on_a_rejecting_shorthand_leaves_no_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { borderTop: [null] },
    });
  "#
);

// An absent value keeps its place among its neighbours, and does not take them
// with it.
stylex_test!(
  an_absent_value_leaves_its_neighbours_alone,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: {
        backgroundColor: 'blue',
        color: [null],
        borderColor: 'green',
      },
    });
  "#
);

// The last declaration of a property wins, in both directions: an absence
// written after a value unsets it, and a value written after an absence stands.
stylex_test!(
  the_last_declaration_of_a_property_wins_over_an_absence,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      unset: { color: 'red', color: null },
      reset: { color: null, color: 'red' },
    });
  "#
);

// ── A `null` under a condition ──────────────────────────────────────

stylex_test!(
  a_null_default_leaves_the_conditional_branch_standing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: { default: null, ':hover': 'red' } },
    });
  "#
);

stylex_test!(
  a_null_conditional_branch_leaves_the_default_standing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      pseudo: { color: { default: 'red', ':hover': null } },
      media: { color: { default: 'red', '@media (min-width: 1px)': null } },
      attribute: { color: { default: 'red', '[data-x]': null } },
    });
  "#
);

// Every branch absent collapses to a single absent value, not to one per
// branch.
stylex_test!(
  every_conditional_branch_absent_collapses_to_one_absence,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: { default: null, ':hover': null, '@media print': null } },
    });
  "#
);

stylex_test!(
  a_null_inside_a_pseudo_object_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { ':hover': { color: null } },
    });
  "#
);

// A condition nested past any depth an author would write by hand: the answer
// must not depend on how deep the absence sits, and an absence at depth must
// not swallow the branch written beside it.
stylex_test!(
  a_null_survives_a_deeply_nested_condition_chain,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: {
        color: {
          default: 'red',
          ':hover': {
            default: 'blue',
            '@media (min-width: 1px)': {
              default: 'green',
              '@media (min-width: 2px)': {
                default: null,
                '@media (min-width: 3px)': null,
              },
            },
          },
        },
      },
      reached: {
        color: {
          ':hover': {
            '@media (min-width: 1px)': {
              '@media (min-width: 2px)': {
                default: null,
                '@media (min-width: 3px)': 'purple',
              },
            },
          },
        },
      },
    });
  "#
);

stylex_test!(
  a_null_array_under_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: { default: [null], ':hover': ['var(--a)', null, 'var(--b)'] } },
    });
  "#
);

stylex_test!(
  a_null_default_on_a_shorthand_under_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::ApplicationOrder)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { borderTop: { default: null, ':hover': '1px solid red' } },
    });
  "#
);

// ── The properties a `null` is easiest to get wrong on ──────────────

// A custom property is not in any expansion table, and its dashed name reaches
// the style object unchanged.
stylex_test!(
  a_null_custom_property_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      bare: { '--x': null },
      wrapped: { '--x': [null] },
      beside: { '--x': null, '--y': 'red' },
    });
  "#
);

// A vendor-prefixed property is spelled in the style object by its own name,
// so an absence on it must not be confused with one on the unprefixed
// property.
stylex_test!(
  a_null_vendor_prefixed_property_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { WebkitLineClamp: null, MozOsxFontSmoothing: [null] },
    });
  "#
);

// `content` is the property whose blank string is *not* blank CSS text, so an
// absence on it has to be told apart from `''`.
stylex_test!(
  an_absence_on_content_is_not_a_blank_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      absent: { content: null },
      blank: { content: '' },
      arrayAbsent: { content: [null] },
    });
  "#
);

// A numeric property, where `0` is a value and `null` is not.
stylex_test!(
  an_absence_on_a_numeric_property_is_not_a_zero,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      absent: { zIndex: null },
      zero: { zIndex: 0 },
      arrayAbsent: { zIndex: [null] },
    });
  "#
);

// A `null` reached through a binding is the same `null`.
stylex_test!(
  a_null_reached_through_a_binding_declares_an_absent_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const absent = null;

    export const styles = stylex.create({
      x: { color: absent },
    });
  "#
);

// A property name spelled with non-ASCII text still names a property, and the
// absence on it is reported under that name.
stylex_test!(
  a_null_on_a_non_ascii_custom_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { '--✓': null, '--Å': [null], color: 'red' },
    });
  "#
);

// An absence beside a value the CSS parser cannot make sense of. The two are
// judged independently: the neighbour is what refuses, and it refuses whether
// or not an absence sits next to it. Both compilers refuse an unclosed
// function here.
stylex_test_panic!(
  an_unclosed_function_beside_an_absence_still_refuses,
  "Rule contains an unclosed function",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      x: { color: null, backgroundImage: 'url("unterminated', borderColor: [null] },
    });
  "#
);

stylex_test_panic!(
  an_unbalanced_paren_beside_an_absence_still_refuses,
  "Rule contains an unclosed function",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      x: { color: null, width: 'calc((1px + 2px)', borderColor: [null] },
    });
  "#
);

// A neighbour both compilers accept, to hold the other half of the same claim:
// an absence beside an odd-but-legal value is unchanged by it.
stylex_test!(
  an_absence_beside_an_important_neighbour,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: null, backgroundColor: 'red !important', borderColor: [null] },
    });
  "#
);

// An absence beside a blank string. Both spell nothing in the emitted CSS, and
// both leave their property present and unset -- so this pins that the two
// arrive there by different routes and still agree.
//
// The reference implementation crashes on the blank neighbour rather than
// reaching this answer; that divergence predates this change and is tracked
// with the other blank-value entries in the parity corpus.
stylex_test!(
  an_absence_beside_a_blank_neighbour,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: { color: null, backgroundColor: '', borderColor: [null] },
    });
  "#
);

// ── The sizes nothing should depend on ──────────────────────────────

// A hundred absences still collapse to one, and the fallback chain never sees
// an entry. The count is written out rather than generated because a spread of
// a generated array is an unfoldable expression in its own right, and this test
// is about the collapse, not about the evaluator.
stylex_test!(
  a_hundred_absences_collapse_to_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: {
        color: [
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null, null,
        ],
      },
    });
  "#
);

// An absence at the end of a long `var()` chain must not disturb the chain it
// follows -- the contiguity check runs over what survives.
stylex_test!(
  an_absence_after_a_long_var_chain,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: {
        color: [
          'var(--v0)', 'var(--v1)', 'var(--v2)', 'var(--v3)', 'var(--v4)',
          'var(--v5)', 'var(--v6)', 'var(--v7)', 'var(--v8)', 'var(--v9)',
          null,
        ],
      },
    });
  "#
);

// Conditions nested forty deep, which no author writes and the recursion has
// to survive anyway.
stylex_test!(
  an_absence_forty_conditions_deep,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      x: {
        color: { '@media (min-width: 1px)': { '@media (min-width: 2px)': {
          '@media (min-width: 3px)': { '@media (min-width: 4px)': {
          '@media (min-width: 5px)': { '@media (min-width: 6px)': {
          '@media (min-width: 7px)': { '@media (min-width: 8px)': {
          '@media (min-width: 9px)': { '@media (min-width: 10px)': {
          '@media (min-width: 11px)': { '@media (min-width: 12px)': {
          '@media (min-width: 13px)': { '@media (min-width: 14px)': {
          '@media (min-width: 15px)': { '@media (min-width: 16px)': {
          '@media (min-width: 17px)': { '@media (min-width: 18px)': {
          '@media (min-width: 19px)': { '@media (min-width: 20px)': null,
          } } } } } } } } } } } } } } } } } } } },
      },
    });
  "#
);

// ── The calls with no value validator in front of them ──────────────
//
// `keyframes` and `positionTry` validate no values, so an absence there is
// dropped where `create` would keep the key. Both compilers agree, and this
// is what the boolean arm in `flat_map_expanded_shorthands` exists for.

stylex_test!(
  a_null_keyframe_value_is_dropped,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fade = stylex.keyframes({
      from: { opacity: null },
      to: { opacity: 1 },
    });
  "#
);

stylex_test!(
  a_boolean_keyframe_value_is_dropped,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fade = stylex.keyframes({
      from: { opacity: false },
      to: { opacity: 1 },
    });
  "#
);

stylex_test!(
  an_absent_position_try_value_is_dropped,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fallback = stylex.positionTry({
      positionAnchor: '--anchor',
      top: null,
      left: false,
    });
  "#
);
