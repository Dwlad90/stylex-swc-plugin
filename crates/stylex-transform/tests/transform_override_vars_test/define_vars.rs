use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_runtime_injection_option(RuntimeInjection::Boolean(false)),
    )
  })
}

stylex_test!(
  test_one_output_of_stylex_define_vars,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("TestTheme.stylex.js".into()))
  }),
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

stylex_test!(
  output_of_stylex_define_vars,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
  }),
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

stylex_test!(
  output_of_stylex_define_vars_with_literals,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real(
      "/stylex/packages/TestTheme.stylex.js".into()
    ))
  }),
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
