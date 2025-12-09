use std::path::PathBuf;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: Some(PathBuf::from("/stylex/packages/")),
      filename: FileName::Real("/stylex/packages/src/vars/default.cssvars.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/stylex/packages/".to_string()),
        theme_file_extension: Some("cssvars".to_string()),
      }),
      ..StyleXOptionsParams::default()
    })
  ),
  processes_tokens_in_files_with_configured_extension,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
  "#
);
