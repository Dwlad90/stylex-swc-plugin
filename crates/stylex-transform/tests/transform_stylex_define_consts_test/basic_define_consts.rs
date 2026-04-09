use crate::utils::prelude::*;
use crate::utils::transform::stringify_js;
use std::path::PathBuf;
use swc_core::common::FileName;
use swc_core::ecma::transforms::testing::test;

fn transform(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_cwd(PathBuf::from("/stylex/packages/"))
      .with_filename(FileName::Real(
        "/stylex/packages/TestTheme.stylex.js".into(),
      ))
      .with_unstable_module_resolution(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      })
      .into_pass()
  })
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

stylex_test!(
  constants_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
    })
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

stylex_test!(
  constants_object_haste,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

stylex_test!(
  constant_names_special_characters,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
    })
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          'font-size*large': '18px',
        });
      "#
);

stylex_test!(
  constant_names_number,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
    })
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          1: 'one'
        });
      "#
);

stylex_test!(
  constant_names_double_dash_prefix,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
    })
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          '--small': '8px',
          '--large': '24px',
        });
      "#
);
