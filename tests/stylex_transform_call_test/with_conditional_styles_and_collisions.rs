use stylex_swc_plugin::{
    shared::structures::{
        named_import_source::RuntimeInjection, stylex_options::StyleXOptionsParams,
    },
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
        Some(StyleXOptionsParams {
            gen_conditional_classes: Some(true),
            runtime_injection: Some(RuntimeInjection::Boolean(true)),
            treeshake_compensation: Option::Some(true),
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
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
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
