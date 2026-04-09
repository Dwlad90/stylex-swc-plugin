use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  adds_debug_data,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(swc_core::common::FileName::Real(
        "/html/js/components/Foo.react.js".into(),
      ))
      .with_debug(true)
      .with_enable_debug_class_names(true)
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(swc_core::common::FileName::Real(
        "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
      ))
      .with_debug(true)
      .with_enable_debug_class_names(true)
      .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(swc_core::common::FileName::Real(
        "/html/js/components/Foo.react.js".into(),
      ))
      .with_debug(true)
      .with_enable_debug_class_names(true)
      .with_unstable_module_resolution(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      })
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_filename(swc_core::common::FileName::Real(
        "/node_modules/npm-package/dist/components/Foo.react.js".into(),
      ))
      .with_debug(true)
      .with_enable_debug_class_names(true)
      .with_unstable_module_resolution(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      })
      .with_runtime_injection()
      .into_pass()
  },
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
