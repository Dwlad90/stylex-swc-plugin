use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  using_stylex_in_a_for_loop,
  r#"
      import * as stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex(color.style);
        }
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
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  using_stylex_props_in_a_loop,
  r#"
      import * as stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex.props(color.style);
        }
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
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  trying_to_use_an_unknown_style_in_stylex,
  r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex(styles.unknown);
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  trying_to_use_an_unknown_style_in_stylex_props,
  r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex.props(styles.unknown);
"#
);
