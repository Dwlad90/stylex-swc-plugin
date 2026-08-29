//! Two `stylex.defineVars` calls that read the same, in one module.
//!
//! `defineVars` resolves its declarator by the expression it holds rather than
//! by the name it binds, so two identical calls share one key. What keeps them
//! apart is that rewriting the first moves it off that key, which leaves the
//! second alone in the bucket by the time it is processed. Nothing else pins
//! that recovery, and what it costs if it stops working is a wrong CSS custom
//! property name, because `defineVars` derives its export ID from the variable
//! the call is bound to.
//!
//! Every expectation below is measured output of `@stylexjs/babel-plugin`
//! 0.19.0 under the same options.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string(),
        ))),
    )
  })
}

stylex_test!(
  two_identical_define_vars_calls_keep_their_own_export_ids,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const a = stylex.defineVars({ color: 'red' });
    export const b = stylex.defineVars({ color: 'red' });
  "#
);
