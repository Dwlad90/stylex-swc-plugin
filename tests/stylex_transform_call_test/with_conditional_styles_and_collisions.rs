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
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_conditions,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
        },
        active: {
          color: 'blue',
        }
      });
      stylex(styles.default, isActive && styles.active);
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
    Option::None
  ),
  stylex_call_with_conditions_skip_conditional,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
        },
        active: {
          color: 'blue',
        }
      });
      stylex(styles.default, isActive && styles.active);
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
    Option::None
  ),
  stylex_call_with_property_collisions,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        red: {
          color: 'red',
        },
        blue: {
          color: 'blue',
        }
      });
      stylex(styles.red, styles.blue);
      stylex(styles.blue, styles.red);
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
    Option::None
  ),
  stylex_call_with_short_form_property_collisions,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          padding: 5,
          paddingEnd: 10,
        },

        bar: {
          padding: 2,
          paddingStart: 10,
        },
      });
      stylex(styles.foo, styles.bar);
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
    Option::None
  ),
  stylex_call_with_short_form_property_collisions_with_null,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          padding: 5,
          paddingEnd: 10,
        },

        bar: {
          padding: 2,
          paddingStart: null,
        },
      });
      stylex(styles.foo, styles.bar);
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
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_conditions_and_collisions,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        red: {
          color: 'red',
        },
        blue: {
          color: 'blue',
        }
      });
      stylex(styles.red, isActive && styles.blue);
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
    Option::None
  ),
  stylex_call_with_conditions_and_collisions_skip_conditional,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        red: {
          color: 'red',
        },
        blue: {
          color: 'blue',
        }
      });
      stylex(styles.red, isActive && styles.blue);
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
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_conditions_and_null_collisions,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        red: {
          color: 'red',
        },
        blue: {
          color: null,
        }
      });
      stylex(styles.red, isActive && styles.blue);
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
    Option::None
  ),
  stylex_call_with_conditions_and_null_collisions_skip_conditional,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      },
      blue: {
        color: null,
      }
    });
    stylex(styles.red, isActive && styles.blue);
"#
);
