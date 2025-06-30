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

const FIXTURE: &str = r#"
import * as stylex from '@stylexjs/stylex';
export const constants = stylex.defineConsts({
  YELLOW: 'yellow',
  ORANGE: 'var(--orange)'
});
export const styles = stylex.create({
  root: {
    animationName: stylex.keyframes({
      '0%': {
        boxShadow: '1px 2px 3px 4px red',
        color: constants.YELLOW
      },
      '100%': {
        boxShadow: '10px 20px 30px 40px green',
        color: constants.ORANGE
      }
    }),
    backgroundColor: 'red',
    borderColor: {
      default: 'green',
      '@media (min-width:320px)': 'blue'
    },
    textShadow: {
      default: '1px 2px 3px 4px red',
      '@media (min-width:320px)': '10px 20px 30px 40px green'
    }
  }
});
"#;

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/test.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  no_rules,
  r#"
    import * as stylex from '@stylexjs/stylex';
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
      cwd: None,
      filename: FileName::Real("/stylex/packages/test.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  all_rules_use_layers_false,
  FIXTURE
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/test.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  all_rules_use_layers_true,
  FIXTURE
);
