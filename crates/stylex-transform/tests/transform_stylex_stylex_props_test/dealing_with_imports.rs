use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  all_local_styles,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/TestFile.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  local_array_styles,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/TestFile.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .with_enable_minified_keys(false)
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  regular_style_import,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/TestFile.js".into()))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .with_runtime_injection()
    .into_pass(),
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

stylex_test!(
  default_import_from_stylex_js_file,
  |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture");
    let filename = fixture_path.join("consts/constants.stylex");
    StyleXTransform::test(tr.comments.clone())
      .with_filename(FileName::Real(filename.clone()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        fixture_path.to_string_lossy().to_string(),
      )))
      .with_runtime_injection()
      .into_pass()
  },
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

stylex_test!(
  object_import_from_stylex_js_file,
  |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture");
    let filename = fixture_path.join("consts/constants.stylex");
    StyleXTransform::test(tr.comments.clone())
      .with_filename(FileName::Real(filename.clone()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        fixture_path.to_string_lossy().to_string(),
      )))
      .with_runtime_injection()
      .into_pass()
  },
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
