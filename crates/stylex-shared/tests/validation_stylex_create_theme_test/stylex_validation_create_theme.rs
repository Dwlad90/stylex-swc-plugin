use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[should_panic(expected = "createTheme() calls must be bound to a bare variable.")]
fn must_be_bound_to_a_variable() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            stylex.createTheme({__varGroupHash__: 'x568ih9'}, {});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "createTheme() should have 1 argument")]
fn it_must_have_two_arguments_no_args() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme();
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "createTheme() should have 1 argument.")]
fn it_must_have_two_arguments_one_args() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme({});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a createTheme() call.")]
fn it_must_have_two_arguments_fn_args() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(genStyles(),{});
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Can only override variables theme created with defineVars().")]
fn it_must_have_two_arguments_empty_object_args() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme({},{});
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  it_must_have_two_arguments_valid,
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {}
        );
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a createTheme() call.")]
fn variable_keys_must_be_a_static_value() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {[labelColor]: 'red',}
            );
        "#,
    r#""#,
  )
}

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  values_must_be_static_number_or_string_in_stylex_create_theme_v1,
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', cornerRadius: 'var(--cornerRadiusHash)'},
            {cornerRadius: 5,}
        );
    "#
);

test!(
  Default::default(),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  values_must_be_static_number_or_string_in_stylex_create_theme_v2,
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: 'red',}
        );
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a createTheme() call.")]
fn values_must_be_static_number_or_string_in_stylex_create_theme_var() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor,}
            );
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a createTheme() call.")]
fn values_must_be_static_number_or_string_in_stylex_create_theme_fn() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            const variables = stylex.createTheme(
                {__varGroupHash__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor(),}
            );
        "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "createTheme() can only accept an object as the second argument")]
fn second_arg_cant_be_imported_variable_in_stylex_create_theme_fn() {
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
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
            import stylex from 'stylex';
            import { buttonTokens } from "./ButtonTokens";

            export const variables = stylex.createTheme(buttonTokens, buttonTokens);
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
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(None)),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  second_arg_can_be_local_variable_in_stylex_create_theme_fn,
  r#"
            import stylex from 'stylex';

            const buttonTokens ={
                __varGroupHash__: 'TestTheme.stylex.js//buttonTheme',
                bgColor: 'var(--xgck17p)',
            };

            const simpleTheme = {
                bgColor: {
                    default: {
                        default: 'green',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.3 -0.4)',
                    },
                    '@media (prefers-color-scheme: dark)': {
                        default: 'lightgreen',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.2 -0.4)',
                    },
                },
            }

            export const variables = stylex.createTheme(buttonTokens, simpleTheme);
        "#
);
