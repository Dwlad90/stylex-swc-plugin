use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b)
      .with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  })
}

stylex_test!(
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

stylex_test!(
  stylex_call_with_conditions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
        stylex.props([styles.red, styles.blue]);
        stylex.props([styles.blue, styles.red]);
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
        stylex.props([styles.red, styles.revert]);
        stylex.props([styles.revert, styles.red]);
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
        stylex.props([styles.foo, styles.bar]);
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
        stylex.props([styles.foo, styles.bar]);
    "#
);

stylex_test!(
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

stylex_test!(
  stylex_call_with_conditions_and_collisions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_conditions_and_null_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_conditions_and_null_collisions_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_conditions_and_null_collisions_tranform_successfully,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  stylex_call_with_conditions_and_undefined_collisions_tranform_successfully,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from 'stylex';

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
        stylex.props(false ? null : styles.red);
        stylex.props(true ? null : styles.red);

        export function TestComponent({removeStyle, isAnimation}) {
            stylex.props(Math.random() > 1 ? styles.red : undefined);
            stylex.props(true ? styles.red : undefined);
            stylex.props(false ? styles.red : undefined);

            stylex.props(Math.random() > 1 ? undefined: styles.red);
            stylex.props(true ? undefined: styles.red );
            stylex.props(false ? undefined : styles.red);
            stylex.props(false ? null : styles.red);
            stylex.props(true ? null : styles.red);
            stylex.props(removeStyle ? undefined : styles.red);
            stylex.props(removeStyle ? null : styles.red);

            const { className: classNameDiv2, style: styleDiv2 } = sx.props(
              removeStyle ? null : c.red,
              isAnimation && c.red
            );

            return <div className={classNameDiv2} style={styleDiv2} />;
        }
    "#
);

stylex_test!(
  stylex_call_with_conditions_and_undefined_collisions_tranform_successfully_with_inlined,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_inlined_conditional_merge(true)
      .with_runtime_injection()
  }),
  r#"
        import * as stylex from 'stylex';

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
        stylex.props(false ? null : styles.red);
        stylex.props(true ? null : styles.red);

        export function TestComponent({removeStyle, isAnimation}) {
            stylex.props(Math.random() > 1 ? styles.red : undefined);
            stylex.props(true ? styles.red : undefined);
            stylex.props(false ? styles.red : undefined);

            stylex.props(Math.random() > 1 ? undefined: styles.red);
            stylex.props(true ? undefined: styles.red );
            stylex.props(false ? undefined : styles.red);
            stylex.props(false ? null : styles.red);
            stylex.props(true ? null : styles.red);
            stylex.props(removeStyle ? undefined : styles.red);
            stylex.props(removeStyle ? null : styles.red);

            const { className: classNameDiv2, style: styleDiv2 } = sx.props(
              removeStyle ? null : c.red,
              isAnimation && c.red
            );

            return <div className={classNameDiv2} style={styleDiv2} />;
        }
    "#
);
