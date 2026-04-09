use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

#[test]
#[should_panic(expected = "The return value of defineConsts() must be bound to a named export.")]
fn invalid_export_not_bound() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineConsts() calls must be bound to a bare variable.")]
fn invalid_export_not_bound_unbound() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          stylex.defineConsts({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineConsts() should have 1 argument.")]
fn invalid_argument_none() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineConsts() should have 1 argument.")]
fn invalid_argument_too_many() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineConsts() can only accept an object.")]
fn invalid_argument_number() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts(1);
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineConsts() can only accept an object.")]
fn invalid_argument_string() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts('1');
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineConsts() call.")]
fn invalid_argument_non_static() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts(genStyles());
        "#,
    r#""#,
  )
}

stylex_test!(
  valid_argument_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({});
        "#
);

stylex_test!(
  valid_export_separate_const_and_export_statement,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
          export { constants };
        "#
);

#[test]
#[should_panic(expected = "The return value of defineConsts() must be bound to a named export.")]
fn invalid_export_re_export_from_another_file_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
          export { constants } from './other.stylex.js';
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineConsts() must be bound to a named export.")]
fn invalid_export_renamed_re_export_from_another_file_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
          export { constants as otherConstants } from './other.stylex.js';
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineConsts() must be bound to a named export.")]
fn invalid_export_default_export_does_not_count() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
          export default constants;
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineConsts() must be bound to a named export.")]
fn invalid_export_renamed_export_with_as_syntax() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const constants = stylex.defineConsts({});
          export { constants as themeConstants };
        "#,
    r#""#,
  )
}

/* Properties */

stylex_test!(
  valid_key_starts_with_double_dash,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      '--small': '8px'
    });
  "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineConsts() call.")]
fn invalid_key_non_static() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            [labelColor]: 'red',
          });
        "#,
    r#""#,
  )
}

/* Values */

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineConsts() call.")]
fn invalid_value_non_static_variable() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            labelColor: labelColor,
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineConsts() call.")]
fn invalid_value_non_static_function_call() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            labelColor: labelColor(),
          });
        "#,
    r#""#,
  )
}

stylex_test!(
  valid_value_number,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            small: 5,
          });
        "#
);

stylex_test!(
  valid_value_string,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_runtime_injection()
    .into_pass(),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            small: '5px',
          });
        "#
);

#[test]
#[ignore]
fn valid_value_keyframes() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_pass(PluginPass::test_default())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            fadeIn: stylex.keyframes({
              '0%': { opacity: 0 },
              '100%': { opacity: 1}
            }),
          });
        "#,
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            fadeIn: stylex.keyframes({
              '0%': { opacity: 0 },
              '100%': { opacity: 1}
            }),
          });
        "#,
  )
}
