use crate::utils::transform::stringify_js;
use std::path::PathBuf;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

fn transform(input: &str) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(PathBuf::from("/stylex/packages/")),
          filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Some(&mut StyleXOptionsParams {
          unstable_module_resolution: Some(ModuleResolution {
            r#type: "commonJS".to_string(),
            root_dir: Some("/stylex/packages/".to_string()),
            theme_file_extension: None,
          }),
          ..Default::default()
        }),
      )
    },
  )
}

#[test]
fn constants_are_unique() {
  let input1 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input2 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input3 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ margin: '10px' });
      "#;

  let output1 = transform(input1);
  let output2 = transform(input2);
  let output3 = transform(input3);

  // Assert the generated constants are consistent for the same inputs
  assert_eq!(output1, output2);

  // Assert the generated constants are different for different inputs
  assert_ne!(output1, output3);
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constants_object,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..Default::default()
    })
  ),
  constants_object_haste,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constant_names_special_characters,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          'font-size*large': '18px',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  constant_names_number,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          1: 'one'
        });
      "#
);
