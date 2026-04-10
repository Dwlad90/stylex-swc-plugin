use crate::utils::prelude::*;

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
    stylex(styles.default, isActive && styles.active);
  "#
);

stylex_test!(
  stylex_call_with_conditions_skip_conditional,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
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
    stylex(styles.red, styles.blue);
    stylex(styles.blue, styles.red);
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
    stylex(styles.foo, styles.bar);
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
    stylex(styles.foo, styles.bar);
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
    stylex(styles.red, isActive && styles.blue);
  "#
);

stylex_test!(
  stylex_call_with_conditions_and_collisions_skip_conditional,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
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

stylex_test!(
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

stylex_test!(
  stylex_call_with_conditions_and_null_collisions_skip_conditional,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
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
