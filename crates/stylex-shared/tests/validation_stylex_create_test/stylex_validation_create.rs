use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn invalid_use_not_bound() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          stylex.create({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn invalid_use_not_called_at_top_level() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          if (bar) {
            const styles = stylex.create({});
          }
       "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn invalid_argument_none() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn invalid_argument_too_many() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object")]
fn invalid_argument_non_static() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::new(None, None),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create(genStyles());
        "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::new(None, None),
      None,
    )
  },
  valid_argument_object,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({});
        "#
);
