use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test_transform,
  },
};

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn must_be_bound_to_a_named_export() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    None,
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
        },
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
            "/stylex/packages/".to_string(),
          ))),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
      import * as stylex from '@stylexjs/stylex';
      const marker = stylex.defineMarker();
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "defineMarker() should have 0 arguments.")]
fn no_arguments_allowed() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: None,
          filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
        },
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
            "/stylex/packages/".to_string(),
          ))),
          ..StyleXOptionsParams::default()
        }),
      )
    },
    r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = stylex.defineMarker(1);
    "#,
    r#""#,
  )
}
