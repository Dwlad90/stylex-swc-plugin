use crate::utils::prelude::*;
use std::path::PathBuf;
use swc_core::common::FileName;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  constants_object_with_runtime_injection_true,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(Some(
      "/stylex/packages/".to_string()
    )))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_runtime_injection()
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
  numeric_constants_with_runtime_injection_true,
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
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          small: 8,
          medium: 16,
          large: 24,
        });
      "#
);

stylex_test!(
  string_constants_with_runtime_injection_true,
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
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const colors = stylex.defineConsts({
          primary: 'rebeccapurple',
          secondary: 'coral',
          tertiary: 'turquoise',
        });
      "#
);

stylex_test!(
  mixed_string_and_numeric_constants_with_runtime_injection_true,
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
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const theme = stylex.defineConsts({
          spacing: 16,
          color: 'blue',
          breakpoint: '(min-width: 768px)',
        });
      "#
);

stylex_test!(
  constants_with_special_characters_with_runtime_injection_true,
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
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const urls = stylex.defineConsts({
          background: "url('bg.png')",
        });
      "#
);

stylex_test!(
  constants_with_custom_inject_path_with_runtime_injection,
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
    .with_runtime_injection_option(RuntimeInjection::Regular("@custom/inject-path".to_string()))
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
        });
      "#
);

stylex_test!(
  haste_module_with_runtime_injection_true,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "haste".to_string(),
      root_dir: None,
      theme_file_extension: None,
    })
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
        });
      "#
);

stylex_test!(
  constants_with_numeric_keys_with_runtime_injection_true,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_cwd(PathBuf::from("/stylex/packages/"))
    .with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
    .with_unstable_module_resolution(StyleXOptions::get_haste_module_resolution(Some(
      "/stylex/packages/".to_string()
    )))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          0: 'zero',
          1: 'one',
          2: 'two',
        });
      "#
);

stylex_test!(
  multiple_define_consts_calls_with_runtime_injection_true,
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
    .with_runtime_injection()
    .into_pass(),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
        });
        export const colors = stylex.defineConsts({
          primary: 'blue',
        });
      "#
);
