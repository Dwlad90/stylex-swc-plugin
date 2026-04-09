use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_style_resolution(StyleResolution::ApplicationOrder)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  empty_stylex_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    stylex();
  "#
);

stylex_test!(
  basic_stylex_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    stylex(styles.red);
  "#
);

stylex_test!(
  stylex_call_with_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      0: {
        color: 'red',
      },
      1: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles[0], styles[1]);
  "#
);

stylex_test!(
  stylex_call_with_computed_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      [0]: {
        color: 'red',
      },
      [1]: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles[0], styles[1]);
"#
);

stylex_test!(
  stylex_call_with_computed_number_without_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      [0]: {
        color: 'red',
      },
      [1]: {
        backgroundColor: 'blue',
      }
    });
    export default stylex(styles[0], styles[1]);
  "#
);

stylex_test!(
  stylex_call_with_multiple_namespaces,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
    stylex(styles.default, otherStyles.default);
"#
);

stylex_test!(
  stylex_call_within_variable_declarations,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: { color: 'red' }
    });
    const a = function() {
      return stylex(styles.foo);
    }
    a
"#
);

stylex_test!(
  stylex_call_with_styles_variable_assignment,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        color: 'red',
      },
      bar: {
        backgroundColor: 'blue',
      }
    });
    stylex(styles.foo, styles.bar);
    const foo = styles;
    foo;
"#
);

stylex_test!(
  stylex_call_with_short_form_properties,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      }
    });
    stylex(styles.foo);
"#
);

stylex_test!(
  stylex_call_with_exported_short_form_properties,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        padding: 5
      }
    });
    stylex(styles.foo);
"#
);

stylex_test!(
  stylex_call_keeps_only_the_styles_that_are_needed,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      },
      bar: {
        paddingBlock: 10,
      },
      baz: {
        paddingTop: 7,
      }
    });
    stylex(styles.foo);
    stylex(styles.foo, styles.bar);
    stylex(styles.bar, styles.foo);
    stylex(styles.foo, styles.bar, styles.baz);
    stylex(styles.foo, somethingElse);
"#
);

stylex_test!(
  stylex_keeps_all_null_when_applied_after_unknown,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      },
      bar: {
        paddingBlock: 10,
      },
      baz: {
        paddingTop: 7,
      }
    });
    stylex(styles.foo);
    stylex(styles.foo, styles.bar);
    stylex(styles.bar, styles.foo);
    stylex(styles.foo, styles.bar, styles.baz);
    stylex(somethingElse, styles.foo);
"#
);

stylex_test!(
  stylex_call_keeps_only_the_nulls_that_are_needed,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      },
      bar: {
        paddingBlock: 10,
      },
      baz: {
        paddingTop: 7,
      }
    });
    stylex(styles.foo);
    stylex(styles.foo, styles.bar);
    stylex(styles.bar, styles.foo);
    stylex(styles.foo, styles.bar, styles.baz);
    stylex(styles.baz, styles.foo, somethingElse);
"#
);

stylex_test!(
  stylex_call_keeps_only_the_nulls_that_are_needed_second,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5
      },
      bar: {
        paddingBlock: 10,
      },
      baz: {
        paddingTop: 7,
      }
    });
    stylex(styles.foo);
    stylex(styles.foo, styles.bar);
    stylex(styles.bar, styles.foo);
    stylex(styles.foo, styles.bar, styles.baz);
    stylex(styles.bar, styles.foo, somethingElse);
"#
);

stylex_test!(
  stylex_call_using_styles_with_pseudo_selectors,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: 'red',
        ':hover': {
          color: 'blue',
        }
      }
    });
    stylex(styles.default);
"#
);

stylex_test!(
  stylex_call_using_styles_with_pseudo_selectors_within_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        color: {
          default: 'red',
          ':hover': 'blue',
        }
      }
    });
    stylex(styles.default);
"#
);

stylex_test!(
  stylex_call_using_styles_with_media_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        backgroundColor: 'red',
        '@media (min-width: 1000px)': {
          backgroundColor: 'blue',
        },
        '@media (min-width: 2000px)': {
          backgroundColor: 'purple',
        },
      },
    });
    stylex(styles.default);
"#
);

stylex_test!(
  stylex_call_using_styles_with_media_queries_within_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        backgroundColor: {
          default:'red',
          '@supports (hover: hover)': 'blue',
          '@supports not (hover: hover)': 'purple',
        },
      },
    });
    stylex(styles.default);
"#
);
