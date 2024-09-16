use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  ModuleTransformVisitor,
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
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_names,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: 'red',
            },
        });
        stylex.props(styles.default);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_name_with_conditions,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: 'red',
            },
        });
        const otherStyles = stylex.create({
            default: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles.default, isActive && otherStyles.default]);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_name_with_conditions_skip_conditional,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: 'red',
            },
        });
        const otherStyles = stylex.create({
            default: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles.default, isActive && otherStyles.default]);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_name_with_collisions,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: 'red',
            },
            active: {
                color: 'blue',
            }
        });
        stylex.props([styles.default, isActive && styles.active]);
    "#
);
