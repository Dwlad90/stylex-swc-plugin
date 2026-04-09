use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

#[test]
#[should_panic(expected = "create() calls must be bound to a bare variable.")]
fn invalid_use_not_bound() {
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
          stylex.create({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "create() should have 1 argument.")]
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
          export const styles = stylex.create();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "create() should have 1 argument.")]
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
          export const styles = stylex.create({}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "create() can only accept an object.")]
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
          export const styles = stylex.create(genStyles());
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
          export const styles = stylex.create({});
        "#
);
