use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
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
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_names, // dev:true
  r#"
      import stylex from 'stylex';

      const styles = stylex.create({
        default: {
          color: 'red',
        },
      });
      stylex(styles.default);
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
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_name_with_conditions, // dev:true and genConditionalClasses:true
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
      stylex(styles.default, isActive && otherStyles.default);
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
      stylex(styles.default, isActive && otherStyles.default);
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
    stylex(styles.default, isActive && styles.active);
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
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_produces_dev_class_name_with_collisions_skip_conditional,
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
      stylex(styles.default, isActive && styles.active);
"#
);
