use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  stylex_call_with_conditions,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection()
    .into_pass(),
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
        stylex.attrs([styles.default, isActive && styles.active]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.default, isActive && styles.active]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.red, styles.blue]);
        stylex.attrs([styles.blue, styles.red]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.red, styles.revert]);
        stylex.attrs([styles.revert, styles.red]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.foo, styles.bar]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.foo, styles.bar]);
    "#
);

stylex_test!(
  stylex_call_with_conditions_and_collisions,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection()
    .into_pass(),
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
        stylex.attrs([styles.red, isActive && styles.blue]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.red, isActive && styles.blue]);
    "#
);

stylex_test!(
  stylex_call_with_conditions_and_null_collisions,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection()
    .into_pass(),
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
        stylex.attrs([styles.red, isActive && styles.blue]);
    "#
);

stylex_test!(
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
        stylex.attrs([styles.red, isActive && styles.blue]);
    "#
);
