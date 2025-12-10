use stylex_shared::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptions, StyleXOptionsParams},
  }, StyleXTransform
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      debug: Some(true),
      enable_debug_class_names: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        filename: swc_core::common::FileName::Real("/html/js/components/Foo.react.js".into()),
        ..PluginPass::default()
      },
      Some(&mut config),
    )
  },
  adds_debug_data,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      debug: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        filename: swc_core::common::FileName::Real(
          "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
        ),
        ..PluginPass::default()
      },
      Some(&mut config),
    )
  },
  adds_debug_data_for_npm_packages,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      debug: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        filename: swc_core::common::FileName::Real("/html/js/components/Foo.react.js".into()),
        ..PluginPass::default()
      },
      Some(&mut config),
    )
  },
  adds_debug_data_haste,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      debug: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(ModuleResolution {
        r#type: "haste".to_string(),
        root_dir: None,
        theme_file_extension: None,
      }),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass {
        filename: swc_core::common::FileName::Real(
          "/node_modules/npm-package/dist/components/Foo.react.js".into(),
        ),
        ..PluginPass::default()
      },
      Some(&mut config),
    )
  },
  adds_debug_data_for_npm_packages_haste,
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
