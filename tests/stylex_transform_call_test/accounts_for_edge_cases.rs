use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  using_stylex_in_a_for_loop,
  r#"
      import stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex(color.style);
        }
      }
"#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  using_stylex_props_in_a_loop,
  r#"
      import stylex from '@stylexjs/stylex';
      function test(colors, obj) {
        for (const color of colors) {
          obj[color.key] = stylex.props(color.style);
        }
      }
"#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  trying_to_use_an_unknown_style_in_stylex,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex(styles.unknown);
"#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_styles(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  trying_to_use_an_unknown_style_in_stylex_props,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
      });
      stylex.props(styles.unknown);
"#
);
