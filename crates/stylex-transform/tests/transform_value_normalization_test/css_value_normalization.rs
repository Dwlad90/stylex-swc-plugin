use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b)
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_enable_font_size_px_to_rem(true)
  })
}

stylex_test!(
  normalize_whitespace_in_css_values_transform,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: {
        transform: '  rotate(10deg)  translate3d( 0 , 0 , 0 )  '
      }
    });
  "#
);

stylex_test!(
  normalize_whitespace_in_css_values_color,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: 'rgba( 1, 222,  33 , 0.5)' } });
  "#
);

stylex_test!(
  no_dimensions_for_zero_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: {
        margin: '0px',
        marginLeft: '1px'
      } });
  "#
);

stylex_test!(
  zero_timings_are_all_zero_s,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { transitionDuration: '500ms' } });
  "#
);

stylex_test!(
  zero_angles_are_all_zero_deg,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { transform: '0rad' },
      y: { transform: '0turn' },
      z: { transform: '0grad' }
    });
  "#
);

stylex_test!(
  calc_preserves_spaces_around_plus_and_minus,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { width: 'calc((100% + 3% -   100px) / 7)' } });
  "#
);

stylex_test!(
  calc_preserves_spaces_around_minus_and_var,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({ x: { width: 'calc(0 - var(--someVar))' } });
    export const styles2 = stylex.create({ x: { width: 'calc(0px - var(--someVar))' } });
  "#
);

stylex_test!(
  strip_leading_zeros,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: {
        transitionDuration: '0.01s',
        transitionTimingFunction: 'cubic-bezier(.08,.52,.52,1)'
      } });
  "#
);

// Negative decimals between -1 and 0 must keep their leading zero.
// The class name hash is derived from the normalized value, so a divergence here
// produces mismatched class names across compilers (issue #1049).
stylex_test!(
  keep_leading_zero_on_negative_decimals,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: {
        letterSpacing: '-0.24px',
        marginTop: '-0.5px'
      } });
  "#
);

stylex_test!(
  use_double_quotes_in_empty_strings,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { quotes: "''" } });
  "#
);

stylex_test!(
  timing_values_are_converted_to_seconds_unless_than_ten_ms,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { transitionDuration: '1234ms' },
      y: { transitionDuration: '10ms' },
      z: { transitionDuration: '1ms' }
    });
  "#
);

stylex_test!(
  transforms_non_unitless_property_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      normalize: {
        height: 500,
        margin: 10,
        width: 500
      },
      unitless: {
        fontWeight: 500,
        lineHeight: 1.5,
        opacity: 0.5,
        zoom: 2
      },
    });
  "#
);

stylex_test!(
  number_values_rounded_down_to_four_decimal_points,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { height: 100 / 3 } });
  "#
);

stylex_test!(
  content_property_values_are_wrapped_in_quotes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        content: '',
      },
      other: {
        content: 'next',
      },
      withQuotes: {
        content: '"prev"',
      }
    });
  "#
);

stylex_test!(
  legacy_no_space_before_bang_important,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: 'red !important' } });
  "#
);

// A unit suffix belongs only to a value the author wrote as a number. Each case
// below is a distinct producer of a style value, so together they pin the JS
// type all the way from the source to the emitted declaration.
// See https://github.com/Dwlad90/stylex-swc-plugin/issues/1249.

stylex_test!(
  numeric_string_keeps_no_unit,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: {
        gridTemplateColumns: {
          '@media (min-width: 768px)': 'repeat(2, 1fr)',
          default: '1',
        },
      },
    });
  "#
);

stylex_test!(
  numeric_string_and_number_side_by_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { width: 1, height: '1', padding: '2' } });
  "#
);

stylex_test!(
  numeric_string_from_a_local_constant_keeps_no_unit,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const asNumber = 10;
    const asString = '10';
    const styles = stylex.create({ x: { width: asNumber, height: asString } });
  "#
);

stylex_test!(
  template_literal_is_a_string_not_a_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { width: `${10}` } });
  "#
);

stylex_test!(
  first_that_works_keeps_each_element_type,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { width: stylex.firstThatWorks(1, '2') } });
  "#
);

stylex_test!(
  keyframes_value_types_are_preserved,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const name = stylex.keyframes({ '0%': { width: 10, height: '10' } });
  "#
);

// SWC's tokenizer lowercases function names, so every function in a value has
// to have its camelCase spelling restored — not just the first one.
// See https://github.com/Dwlad90/stylex-swc-plugin/issues/1249.

stylex_test!(
  camel_case_restored_for_every_transform_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { transform: 'translateX(0px) translateY(0) scale(1) rotate(30deg)' },
    });
  "#
);

stylex_test!(
  camel_case_restored_inside_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const slide = stylex.keyframes({
      '0%': { transform: 'translateX(0px) translateY(0) scale(1) rotate(30deg)' },
      '100%': { transform: 'translateX(100px) translateY(-300px) scale(0.7)' },
    });
    const styles = stylex.create({ a: { animationName: slide } });
  "#
);

// A number handed to a shorthand reaches each expanded property as a number, so
// every part takes that property's own unit suffix.
stylex_test!(
  numeric_shorthand_expands_with_units,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { borderWidth: 1, margin: 4, flex: 1 } });
  "#
);

// Fallback arrays drop falsy values and keep only the first of each repeat.
// `0` is falsy where the string `"0"` is not, and the two are distinct entries.
stylex_test!(
  fallback_array_drops_falsy_and_repeated_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { width: stylex.firstThatWorks(0, '2px') },
      y: { height: stylex.firstThatWorks('0', 0, '2px') },
      z: { top: stylex.firstThatWorks('1px', '2px', '1px') },
    });
  "#
);

// Numbers keep the single spelling a JS number has. SWC's minifier folds
// trailing zeros into an exponent and holds integers as i64, neither of which a
// style value ever carries in.
stylex_test!(
  numbers_keep_their_js_spelling,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      a: { width: 1000, height: 123000, top: 1e21 },
      b: { opacity: 1e20, zIndex: 12345678901234567890 },
    });
  "#
);

// A BigInt is unsupported as a style *value*, but as a key it is stringified
// like any other numeric key. Only the value position rejects it.
stylex_test!(
  big_int_key_is_stringified_like_a_numeric_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ 10n: { color: 'red' } });
    const name = stylex.keyframes({ 10n: { opacity: 1 } });
  "#
);

// A value with no CSS text emits no declaration. `.x1tfe9bt{color:}` is not
// valid CSS and a browser discards the whole declaration, so the property is
// reverted the way an explicit `null` reverts it — the neighbouring property
// still compiles. Measured against `@stylexjs/babel-plugin@0.19.0` for
// `color: null`, which is the case it handles deliberately; a blank string
// reaches a null dereference inside its value normaliser instead.
//
// The styles are exported so the compiled object is pinned too: the property
// survives as `null`, which is what lets a later namespace revert it. Dropping
// the key instead would compile the same CSS and merge differently.
stylex_test!(
  empty_value_emits_no_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({ x: { color: '', backgroundColor: 'red' } });
  "#
);

// Whitespace normalization leaves a space-only value empty, so it drops for the
// same reason — including every longhand a shorthand expands to.
stylex_test!(
  whitespace_only_value_emits_no_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { color: ' ' },
      y: { padding: '  ' },
    });
  "#
);

// A blank entry drops out of a fallback array, leaving the class name of the
// values that remain — the same one a lone `'red'` produces.
stylex_test!(
  blank_value_in_a_fallback_array_is_dropped,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: [' ', 'red'] } });
  "#
);

// `null` is the one element a fallback array drops rather than refuses: the
// entries around it keep their order and their place in the chain, and an
// array holding nothing else declares nothing at all. Measured against
// `@stylexjs/babel-plugin@0.19.0`: `x` is `color:red;color:blue`, `y` is the
// `.xju2f9n` a lone `'blue'` produces, and `z` declares nothing.
stylex_test!(
  null_in_a_fallback_array_is_dropped,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { color: ['red', null, 'blue'] },
      y: { color: [null, 'blue'] },
      z: { color: [null] },
    });
  "#
);

// Only the blank branch of a nested value drops; the conditions around it are
// untouched.
stylex_test!(
  blank_value_in_a_nested_value_drops_only_that_branch,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { color: { default: '', ':hover': 'red' } },
    });
  "#
);

// `content` is the exception, and the reason a lone value is judged after
// transformation rather than before: a blank `content` is quoted into `""`,
// which is CSS text and a meaningful declaration.
//
// `content_property_values_are_wrapped_in_quotes` above already covers the
// empty string. This case is the whitespace-only one, which reaches the same
// declaration only because the drop reads the quoted text rather than the
// authored space.
stylex_test!(
  blank_content_value_still_emits_its_empty_quotes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { content: ' ' } });
  "#
);

// `defineVars` keeps an empty token value. A custom property definition is
// valid with an empty value, and the token is still read back through `var()`,
// so the drop belongs to declaration emission inside `create` and stops there.
// `:root, .xop34xu{--xcb2f4a:;}` is measured output of
// `@stylexjs/babel-plugin@0.19.0` for this file name and root directory.
stylex_test!(
  empty_token_value_still_emits_its_custom_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({ background: '' });
  "#
);
