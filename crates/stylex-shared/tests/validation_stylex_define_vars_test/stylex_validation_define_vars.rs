use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "The return value of defineVars() must be bound to a named export.")]
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
          const styles = stylex.defineVars({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineVars() calls must be bound to a bare variable.")]
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
          stylex.defineVars({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineVars() should have 1 argument.")]
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
          export const vars = stylex.defineVars();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineVars() should have 1 argument.")]
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
          export const vars = stylex.defineVars({}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineVars() can only accept an object.")]
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
          export const vars = stylex.defineVars(1);
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineVars() can only accept an object.")]
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
          export const vars = stylex.defineVars('1');
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineVars() call.")]
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
          export const vars = stylex.defineVars(genStyles());
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
          export const vars = stylex.defineVars({});
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
  valid_export_separate_const_and_export_statement,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const vars = stylex.defineVars({});
          export { vars };
        "#
);

#[test]
#[should_panic(expected = "The return value of defineVars() must be bound to a named export.")]
fn invalid_export_re_export_from_another_file_does_not_count() {
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
          const vars = stylex.defineVars({});
          export { vars } from './other.stylex.js';
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineVars() must be bound to a named export.")]
fn invalid_export_renamed_re_export_from_another_file_does_not_count() {
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
          const vars = stylex.defineVars({});
          export { vars as otherVars } from './other.stylex.js';
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineVars() must be bound to a named export.")]
fn invalid_export_default_export_does_not_count() {
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
          const vars = stylex.defineVars({});
          export default vars;
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineVars() must be bound to a named export.")]
fn invalid_export_renamed_export_with_as_syntax() {
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
          const vars = stylex.defineVars({});
          export { vars as themeVars };
        "#,
    r#""#,
  )
}

/* Properties */

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineVars() call.")]
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
          export const vars = stylex.defineVars({
            [labelColor]: 'red',
          });
        "#,
    r#""#,
  )
}

/* Values */

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineVars() call.")]
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
          export const vars = stylex.defineVars({
            labelColor: labelColor,
          });
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a defineVars() call.")]
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
          export const vars = stylex.defineVars({
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
          export const vars = stylex.defineVars({
            cornerRadius: 5,
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
          export const vars = stylex.defineVars({
            labelColor: 'red',
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
  valid_value_keyframes,
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const vars = stylex.defineVars({
            fadeIn: stylex.keyframes({
              '0%': { opacity: 0 },
              '100%': { opacity: 1}
            }),
          });
        "#
);
