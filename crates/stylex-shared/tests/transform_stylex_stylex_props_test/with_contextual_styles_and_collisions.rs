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
    None
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
        stylex.props([styles.default, isActive && styles.active]);
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
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
        stylex.props([styles.default, isActive && styles.active]);
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
    None
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
        stylex.props([styles.red, styles.blue]);
        stylex.props([styles.blue, styles.red]);
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
    None
  ),
  stylex_call_with_reverting_by_null,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            },
            revert: {
                color: null,
            }
        });
        stylex.props([styles.red, styles.revert]);
        stylex.props([styles.revert, styles.red]);
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
    None
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
        stylex.props([styles.foo, styles.bar]);
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
    None
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
        stylex.props([styles.foo, styles.bar]);
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
    None
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
        stylex.props([styles.red, isActive && styles.blue]);
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
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
        stylex.props([styles.red, isActive && styles.blue]);
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
      enable_inlined_conditional_merge: Some(false),
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
        stylex.props([styles.red, isActive && styles.blue]);
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
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
        stylex.props([styles.red, isActive && styles.blue]);
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_conditions_and_null_collisions_tranform_successfully,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            },
        });
        stylex.props(Math.random() > 1 ? styles.red : null);
        stylex.props(true ? styles.red : null);
        stylex.props(false ? styles.red : null);

        stylex.props(Math.random() > 1 ? null: styles.red);
        stylex.props(true ? null: styles.red );
        stylex.props(false ? null : styles.red);
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_conditions_and_undefined_collisions_tranform_successfully,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            },
        });
        stylex.props(Math.random() > 1 ? styles.red : undefined);
        stylex.props(true ? styles.red : undefined);
        stylex.props(false ? styles.red : undefined);

        stylex.props(Math.random() > 1 ? undefined: styles.red);
        stylex.props(true ? undefined: styles.red );
        stylex.props(false ? undefined : styles.red);
    "#
);
