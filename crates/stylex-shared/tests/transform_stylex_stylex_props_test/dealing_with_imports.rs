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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestFile.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  all_local_styles,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            default: {
              color: 'black',
            },
            red: {
              color: 'red',
            },
            blueBg: {
              backgroundColor: 'blue',
            },

          });

          <div {...stylex.props(styles.default, styles.red, styles.blueBg)} />
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
      filename: FileName::Real("/stylex/packages/TestFile.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      enable_minified_keys: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  local_array_styles,
  r#"
          import * as stylex from '@stylexjs/stylex';
          const styles = stylex.create({
            default: {
              color: 'black',
            },
            red: {
              color: 'red',
            },
            blueBg: {
              backgroundColor: 'blue',
            },
          });

          const base = [styles.default, styles.red];

          <div {...stylex.props(base, styles.blueBg)} />
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
      filename: FileName::Real("/stylex/packages/TestFile.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  regular_style_import,
  r#"
          import * as stylex from '@stylexjs/stylex';
          import {someStyle} from './otherFile';
          const styles = stylex.create({
            default: {
              color: 'black',
            },
          });
          <div {...stylex.props(styles.default, someStyle)} />
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture");
    let filename = fixture_path.join("consts/constants.stylex");

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real(filename.clone()),
      },
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
          fixture_path.to_string_lossy().to_string(),
        ))),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  default_import_from_stylex_js_file,
  r#"
          import * as stylex from '@stylexjs/stylex';
          import {someStyle, vars} from './constants.stylex.js';
          const styles = stylex.create({
            default: {
              color: 'black',
              backgroundColor: vars.foo,
            },
          });
          <div {...stylex.props(styles.default, someStyle)} />
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture");
    let filename = fixture_path.join("consts/constants.stylex");

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: FileName::Real(filename.clone()),
      },
      Some(&mut StyleXOptionsParams {
        unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
          fixture_path.to_string_lossy().to_string(),
        ))),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  object_import_from_stylex_js_file,
  r#"
          import * as stylex from '@stylexjs/stylex';
          import {someStyle} from './constants.stylex.js';
          const styles = stylex.create({
            default: {
              color: 'black',
              backgroundColor: someStyle.foo,
            },
          });
          <div {...stylex.props(styles.default, someStyle.foo)} />
  "#
);
