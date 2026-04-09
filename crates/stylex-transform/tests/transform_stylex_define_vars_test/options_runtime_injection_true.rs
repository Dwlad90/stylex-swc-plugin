use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  tokens_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(Some(
      "/stylex/packages/".to_string()
    )))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      nextColor: 'green',
      otherColor: 'blue'
    });
  "#
);
