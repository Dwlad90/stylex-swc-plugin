use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
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
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  local_static_styles,
  r#"
import stylex from 'stylex';
const styles = stylex.create({
  red: {
    color: 'red',
  }
});
function Foo() {
  return (
    <>
      <div id="test" {...stylex.props(styles.red)}>Hello World</div>
      <div className="test" {...stylex.props(styles.red)} id="test">Hello World</div>
      <div id="test" {...stylex.props(styles.red)} className="test">Hello World</div>
    </>
  );
}
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
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  local_dynamic_styles,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      },
      opacity: (opacity) => ({
        opacity
      })
    });
    function Foo() {
      return (
        <div id="test" {...stylex.props(styles.red, styles.opacity(1))}>
          Hello World
        </div>
      );
    }
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
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  non_local_styles,
  r#"
import stylex from 'stylex';
const styles = stylex.create({
  red: {
    color: 'red',
  }
});
function Foo(props) {
  return (
    <div id="test" {...stylex.props(props.style, styles.red)}>
      Hello World
    </div>
  );
}
  "#
);
