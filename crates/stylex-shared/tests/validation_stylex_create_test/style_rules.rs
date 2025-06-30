use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "Referenced constant is not defined.")]
fn invalid_key_non_static() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            [root]: {
              backgroundColor: 'red',
            }
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "A stylex namespace must be an object.")]
fn invalid_rule_non_object() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            namespace: false,
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Object spreads are not allowed in stylex.create call.")]
fn invalid_rule_spread() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const shared = { foo: { color: 'red' } };
          const styles = stylex.create({
            ...shared,
            bar: { color: 'blue' }
          });
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
      PluginPass::default(),
      None,
    )
  },
  valid_rule_object,
  r#"
          const styles = stylex.create({
            namespace: {},
          });
        "#
);

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed"
)]
fn invalid_dynamic_rule_default_object_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            dynamic: (props = {}) => ({
              color: props.color,
            }),
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed"
)]
fn invalid_dynamic_rule_default_string_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            dynamic: (color = 'red') => ({
              color,
            }),
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed"
)]
fn invalid_dynamic_rule_destructuring() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            dynamic: ({ color }) => ({
              color,
            }),
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(
  expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed"
)]
fn invalid_dynamic_rule_rest_param() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            dynamic: (...rest) => ({
              color: rest[0],
            }),
          });
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
      PluginPass::default(),
      None,
    )
  },
  valid_dynamic_rule,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            dynamic: (backgroundColor) => ({
              backgroundColor,
            }),
          });
        "#
);
