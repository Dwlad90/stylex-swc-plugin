use stylex_swc_plugin::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
  },
};

fn get_default_opts() -> StyleXOptionsParams {
  StyleXOptionsParams {
    unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(
      None,
    )),
    class_name_prefix: Some("x".to_string()),
    ..StyleXOptionsParams::default()
  }
}

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      ..get_default_opts()
    })
  ),
  output_of_stylex_define_vars,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
    "#
);
