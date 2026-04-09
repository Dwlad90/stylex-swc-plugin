use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_debug(true)
        .with_enable_debug_class_names(true)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  adds_debug_data,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/html/js/components/Foo.react.js".into(),
    ))
  }),
  r#"
            import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            foo: {
              color: 'red'
            },
            'bar-baz': {
              display: 'block'
            },
            1: {
              fontSize: '1em'
            }
          });
        "#
);

stylex_test!(
  adds_debug_data_for_npm_packages,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
  }),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            foo: {
              color: 'red'
            },
            'bar-baz': {
              display: 'block'
            },
            1: {
              fontSize: '1em'
            }
          });
        "#
);

stylex_test!(
  adds_debug_data_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/html/js/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "haste".to_string(),
      root_dir: None,
      theme_file_extension: None,
    })
  }),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            foo: {
              color: 'red'
            },
            'bar-baz': {
              display: 'block'
            },
            1: {
              fontSize: '1em'
            }
          });
        "#
);

stylex_test!(
  adds_debug_data_for_npm_packages_haste,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_filename(swc_core::common::FileName::Real(
      "/node_modules/npm-package/dist/components/Foo.react.js".into(),
    ))
    .with_unstable_module_resolution(ModuleResolution {
      r#type: "haste".to_string(),
      root_dir: None,
      theme_file_extension: None,
    })
  }),
  r#"
          import * as stylex from '@stylexjs/stylex';
          export const styles = stylex.create({
            foo: {
              color: 'red'
            },
            'bar-baz': {
              display: 'block'
            },
            1: {
              fontSize: '1em'
            }
          });
        "#
);
