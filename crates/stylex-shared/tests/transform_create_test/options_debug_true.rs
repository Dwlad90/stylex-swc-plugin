use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{parser::Syntax, parser::TsSyntax, transforms::testing::test},
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
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
