use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{test, test_transform},
  },
};

#[test]
#[should_panic(
  expected = "The return value of stylex.defineVars() must be bound to a named export."
)]
fn must_be_bound_to_a_named_export_const() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            const styles = stylex.defineVars({});
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn must_be_bound_to_a_named_export() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            stylex.defineVars({});
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn its_only_argument_must_be_a_single_object_fn() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars(genStyles());
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object.")]
fn its_only_argument_must_be_a_single_object_number() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars(1);
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_empty() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars();
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_two_args() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars({}, {});
        "#,
    r#""#,
    false,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    None
  ),
  its_only_argument_must_be_a_single_object_valid,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.defineVars({});
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn variable_keys_must_be_a_static_value() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars({
                [labelColor]: 'red',
            });
        "#,
    r#""#,
    false,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    None
  ),
  values_must_be_static_number_or_string_or_keyframes_in_stylex_define_vars,
  r#"
        import stylex from 'stylex';
        export const styles1 = stylex.defineVars({
            cornerRadius: 5,
        });

        export const styles2 = stylex.defineVars({
            labelColor: 'red',
        });

        export const styles3 = stylex.defineVars({
            fadeIn: stylex.keyframes({
                '0%': { opacity: 0 },
                '100%': { opacity: 1}
            }),
        });
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_number_or_string_or_keyframes_in_stylex_define_vars_var() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars({
                labelColor: labelColor,
            });
        "#,
    r#""#,
    false,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_number_or_string_or_keyframes_in_stylex_define_vars_fn() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        &PluginPass::default(),
        None,
      )
    },
    r#"
            import stylex from 'stylex';
            export const styles = stylex.defineVars({
                labelColor: labelColor(),
            });
        "#,
    r#""#,
    false,
  )
}
