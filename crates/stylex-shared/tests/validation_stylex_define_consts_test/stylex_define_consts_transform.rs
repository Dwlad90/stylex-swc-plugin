use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(
  expected = "The return value of stylex.defineVars() must be bound to a named export."
)]
fn invalid_export_not_bound() {
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
          const constants = stylex.defineConsts({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn invalid_export_not_bound_unbound() {
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
          stylex.defineConsts({});
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
          export const constants = stylex.defineConsts();
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
          export const constants = stylex.defineConsts({}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object.")]
fn invalid_argument_number() {
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
          export const constants = stylex.defineConsts(1);
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object.")]
fn invalid_argument_string() {
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
          export const constants = stylex.defineConsts('1');
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
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
          export const constants = stylex.defineConsts(genStyles());
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
          export const constants = stylex.defineConsts({});
        "#
);

/* Properties */

#[test]
#[should_panic(expected = r#"Keys in defineConsts() cannot start with "--"."#)]
fn invalid_key_starts_with_double_dash() {
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
          export const constants = stylex.defineConsts({
            '--small': '8px'
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
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
        PluginPass::new(None, None),
        None,
      )
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
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn invalid_value_non_static_variable() {
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
          export const constants = stylex.defineConsts({
            labelColor: labelColor,
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn invalid_value_non_static_function_call() {
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
          export const constants = stylex.defineConsts({
            labelColor: labelColor(),
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
      PluginPass::new(None, None),
      None,
    )
  },
  valid_value_number,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const constants = stylex.defineConsts({
            small: 5,
          });
        "#
);

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
  valid_value_string,
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
