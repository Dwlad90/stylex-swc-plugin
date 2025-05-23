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

fn get_default_opts() -> StyleXOptionsParams {
  StyleXOptionsParams {
    unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
    class_name_prefix: Some("x".to_string()),
    ..StyleXOptionsParams::default()
  }
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      ..get_default_opts()
    })
  ),
  test_one_output_of_stylex_define_vars,
  r#"
    import * as stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: 'green',
      bgColorDisabled: 'antiquewhite',
      cornerRadius: '6px',
      fgColor: 'coral',
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
      runtime_injection: Some(false),
      ..get_default_opts()
    })
  ),
  output_of_stylex_define_vars_with_literals,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      '--bgColor': {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      '--bgColorDisabled': {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      '--cornerRadius': 10,
      '--fgColor': {
        default: 'pink',
      },
    });
    "#
);
