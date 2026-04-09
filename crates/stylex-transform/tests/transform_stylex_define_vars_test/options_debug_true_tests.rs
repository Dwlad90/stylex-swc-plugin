use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_filename(FileName::Real("/stylex/packages/vars.stylex.js".into()))
        .with_debug(true)
        .with_enable_debug_class_names(true)
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          "/stylex/packages/".to_string()
        ))),
    )
  })
}

stylex_test!(
  tokens_object_includes_debug_data,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': {
          default: 'lightblue',
          '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.3 -0.4)',
        }
      },
      otherColor: 'green'
    });
  "#
);

stylex_test!(
  tokens_object_includes_debug_data_keys_with_special_characters,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      '10': 'green',
      '1.5 pixels': 'blue',
      'corner#radius': 'purple',
      '@@primary': 'pink'
    });
  "#
);
