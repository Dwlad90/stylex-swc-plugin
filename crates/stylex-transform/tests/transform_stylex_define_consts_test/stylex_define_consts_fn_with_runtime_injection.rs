use crate::utils::prelude::*;
use std::path::PathBuf;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_cwd(PathBuf::from("/stylex/packages/"))
        .with_filename(FileName::Real(
          "/stylex/packages/TestTheme.stylex.js".into(),
        )),
    )
  })
}

stylex_test!(
  constants_object_with_runtime_injection_true,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string(),
    )))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const urls = stylex.defineConsts({
      background: "url('bg.png')",
    });
  "#
);

stylex_test!(
  constants_with_custom_inject_path_with_runtime_injection,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection_option(RuntimeInjection::Regular("@custom/inject-path".to_string()))
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const breakpoints = stylex.defineConsts({
      sm: '(min-width: 768px)',
    });
  "#
);

stylex_test!(
  haste_module_with_runtime_injection_true,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: None,
      theme_file_extension: None,
      ..ModuleResolution::haste(None)
    })
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution::haste(Some(
      "/stylex/packages/".to_string(),
    )))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_runtime_injection()
  }),
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
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution {
      root_dir: Some("/stylex/packages/".to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    })
    .with_runtime_injection()
  }),
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
