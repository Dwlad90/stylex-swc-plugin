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
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  tokens_object_including_keys_with_special_characters,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      '10': 'green',
      '1.5 pixels': 'blue',
      'corner#radius': 'purple',
      '@@primary': 'pink'
    });
  "#
);
