use stylex_shared::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  StyleXTransform,
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
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_mode_classnames_enabled,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(false),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_mode_classnames_disabled,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_mode_classnames_enabled_with_many_styles,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
        content: {
          gridArea: 'content',
        },
        root: {
          display: 'grid',
          gridTemplateRows: '100%',
          gridTemplateAreas: '"content"',
        },
      });
      stylex.props(styles.unknown);
"#
);
test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(false),
      debug: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_mode_classnames_enabled_with_dev_disabled,
  r#"
      import stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        tileHeading: {
          marginRight: 12,
        },
        content: {
          gridArea: 'content',
        },
        root: {
          display: 'grid',
          gridTemplateRows: '100%',
          gridTemplateAreas: '"content"',
        },
      });
      stylex.props(styles.unknown);
"#
);
