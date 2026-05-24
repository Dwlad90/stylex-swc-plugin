use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

stylex_test!(
  transforms_define_vars_from_commonjs_destructured_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const { defineVars } = require('@stylexjs/stylex');
    export const vars = defineVars({
      color: 'red',
    });
  "#
);

stylex_test!(
  transforms_define_vars_from_commonjs_destructured_alias_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const { defineVars: defineVarsLocal } = require('@stylexjs/stylex');
    export const vars = defineVarsLocal({
      color: 'red',
    });
  "#
);
