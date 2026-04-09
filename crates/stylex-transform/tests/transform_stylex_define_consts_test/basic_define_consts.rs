use crate::utils::prelude::*;
use crate::utils::transform::stringify_js;
use std::path::PathBuf;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_cwd(PathBuf::from("/stylex/packages/"))
        .with_filename(FileName::Real(
          "/stylex/packages/TestTheme.stylex.js".into(),
        ))
        .with_unstable_module_resolution(ModuleResolution {
          r#type: "commonJS".to_string(),
          root_dir: Some("/stylex/packages/".to_string()),
          theme_file_extension: None,
        }),
    )
  })
}

#[test]
fn constants_are_unique() {
  let input1 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input2 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ padding: '10px' });
      "#;

  let input3 = r#"
        import stylex from 'stylex';
        export const breakpoints = stylex.defineConsts({ margin: '10px' });
      "#;

  let output1 = stringify_js(input1, ts_syntax(), |tr| stylex_transform(tr.comments.clone(), |b| b));
  let output2 = stringify_js(input2, ts_syntax(), |tr| stylex_transform(tr.comments.clone(), |b| b));
  let output3 = stringify_js(input3, ts_syntax(), |tr| stylex_transform(tr.comments.clone(), |b| b));

  // Assert the generated constants are consistent for the same inputs
  assert_eq!(output1, output2);

  // Assert the generated constants are different for different inputs
  assert_ne!(output1, output3);
}

stylex_test!(
  constants_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

stylex_test!(
  constants_object_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_unstable_module_resolution(ModuleResolution::haste(None))
  }),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const breakpoints = stylex.defineConsts({
          sm: '(min-width: 768px)',
          md: '(min-width: 1024px)',
          lg: '(min-width: 1280px)',
        });
      "#
);

stylex_test!(
  constant_names_special_characters,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          'font-size*large': '18px',
        });
      "#
);

stylex_test!(
  constant_names_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const levels = stylex.defineConsts({
          1: 'one'
        });
      "#
);

stylex_test!(
  constant_names_double_dash_prefix,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const sizes = stylex.defineConsts({
          '--small': '8px',
          '--large': '24px',
        });
      "#
);
