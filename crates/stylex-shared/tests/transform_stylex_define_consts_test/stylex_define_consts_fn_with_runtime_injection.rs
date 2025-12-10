use std::path::PathBuf;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      ..Default::default()
    })
  ),
  constants_object_with_runtime_injection_true,
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
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
  numeric_constants_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          small: 8,
          medium: 16,
          large: 24,
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
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
  string_constants_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const colors = stylex.defineConsts({
          primary: 'rebeccapurple',
          secondary: 'coral',
          tertiary: 'turquoise',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
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
  mixed_string_and_numeric_constants_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const theme = stylex.defineConsts({
          spacing: 16,
          color: 'blue',
          breakpoint: '(min-width: 768px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
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
  constants_with_special_characters_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const urls = stylex.defineConsts({
          background: "url('bg.png')",
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
      runtime_injection: Some(RuntimeInjection::Regular("@custom/inject-path".to_string())),
      ..Default::default()
    })
  ),
  constants_with_custom_inject_path_with_runtime_injection,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..Default::default()
    })
  ),
  haste_module_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      ..Default::default()
    })
  ),
  constants_with_numeric_keys_with_runtime_injection_true,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          0: 'zero',
          1: 'one',
          2: 'two',
        });
      "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
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
  multiple_define_consts_calls_with_runtime_injection_true,
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
