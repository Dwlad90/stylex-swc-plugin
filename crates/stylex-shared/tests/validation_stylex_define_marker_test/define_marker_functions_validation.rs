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

#[test]
fn valid_export_direct_named_export() {
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
        export const marker = stylex.defineMarker();
    "#,
    r#"
        import * as stylex from '@stylexjs/stylex';
        export const marker = {
            x1allf69: "x1allf69",
            $$css: true
        };
    "#,
  )
}

#[test]
fn valid_export_separate_const_and_export_statement() {
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
        const marker = stylex.defineMarker();
        export { marker };
    "#,
    r#"
        import * as stylex from '@stylexjs/stylex';
        const marker = {
            x1allf69: "x1allf69",
            $$css: true
        };
        export { marker };
    "#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_re_export_from_another_file_does_not_count() {
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
        const marker = stylex.defineMarker();
        export { marker } from './other.stylex.js';
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_renamed_re_export_from_another_file_does_not_count() {
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
        const marker = stylex.defineMarker();
        export { marker as otherMarker } from './other.stylex.js';
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_default_export_does_not_count() {
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
        const marker = stylex.defineMarker();
        export default marker;
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "The return value of defineMarker() must be bound to a named export.")]
fn invalid_export_renamed_export_with_as_syntax() {
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
        const marker = stylex.defineMarker();
        export { marker as themeMarker };
    "#,
    r#""#,
  )
}
