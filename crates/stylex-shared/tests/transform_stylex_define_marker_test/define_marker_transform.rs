use std::path::PathBuf;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::parser::{Syntax, TsSyntax};
use swc_core::ecma::transforms::testing::test;

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    }),
  ),
  member_call,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fooBar = stylex.defineMarker();
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
      filename: FileName::Real("/stylex/packages/vars.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: None,
      }),
      ..Default::default()
    }),
  ),
  named_import_call,
  r#"
    import { defineMarker } from '@stylexjs/stylex';
    export const baz = defineMarker();
  "#
);
