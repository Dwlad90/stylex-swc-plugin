use crate::utils::prelude::*;
use stylex_enums::property_validation_mode::PropertyValidationMode;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
        .with_style_resolution(StyleResolution::LegacyExpandShorthands)
        .with_enable_logical_styles_polyfill(true)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  padding_basic_shorthand,
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
  margin_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        margin: '10px 20px 30px 40px'
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
          paddingInline: 5
        }
      });
      stylex(styles.foo);
    "#
);

stylex_test!(
  padding_inline_basic_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        paddingInline: "5px 10px"
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  stylex_call_with_short_form_property_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          padding: 5,
          paddingInlineEnd: 10,
        },

        bar: {
          padding: 2,
          paddingInlineStart: 10,
        },
      });
      stylex(styles.foo, styles.bar);
    "#
);

stylex_test!(
  stylex_call_with_short_form_property_collisions_with_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5,
        paddingInlineEnd: 10,
      },

      bar: {
        padding: 2,
        paddingInlineStart: null,
      },
    });
    stylex(styles.foo, styles.bar);
    "#
);

stylex_test!(
  margin_inline_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        marginInline: 5
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  margin_inline_basic_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        marginInline: "5px 10px"
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  margin_block_basic_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        marginBlock: "5px 10px"
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  margin_with_longhand_property_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        margin: 5,
        marginInlineEnd: 10,
      },

      bar: {
        margin: 2,
        marginInlineStart: 10,
      },
    });
    stylex(styles.foo, styles.bar);
  "#
);

stylex_test!(
  margin_with_null_longhand_property_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        margin: 5,
        marginInlineEnd: 10,
      },

      bar: {
        margin: 2,
        marginInlineStart: null,
      },
    });
    stylex(styles.foo, styles.bar);
  "#
);

stylex_test!(
  border_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        border: '1px solid red'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  border_inline_color_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderInlineColor: 'red'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  border_inline_width_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderInlineWidth: 1
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        inset: 10
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        inset: '10px 20px 30px 40px'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_inline_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        insetInline: 10
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_inline_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        insetInline: '10px 20px'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_block_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        insetBlock: 10
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  inset_block_multivalue_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        insetBlock: '10px 20px'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  list_style_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_debug(true)
      .with_enable_debug_class_names(true)
      .with_enable_dev_class_names(false)
  }),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      none: {
        listStyle: 'none'
      },
      square: {
        listStyle: 'square'
      },
      inside: {
        listStyle: 'inside'
      },
      custom1: {
        listStyle: '"--"'
      },
      custom2: {
        listStyle: "'=='"
      }
    });
  "#
);

stylex_test!(
  list_style_multi_value_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_debug(true)
      .with_enable_debug_class_names(true)
      .with_enable_dev_class_names(false)
  }),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      one: {
        listStyle: 'none inside'
      },
      two: {
        listStyle: 'none square'
      },
      three: {
        listStyle: 'simp-chinese-informal linear-gradient(90deg, white 100%)'
      },
      four: {
        listStyle: 'outside "+" linear-gradient(90deg, white 100%)'
      },
    });
  "#
);

stylex_test!(
  list_style_with_longhand_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_debug(true)
      .with_enable_debug_class_names(true)
      .with_enable_dev_class_names(false)
  }),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      one: {
        listStyle: 'none inside',
        listStyleType: 'square'
      },
      two: {
        listStyle: 'none georgian',
        listStylePosition: 'outside'
      },
      three: {
        listStyle: 'simp-chinese-informal linear-gradient(90deg, white 100%)',
        listStylePosition: 'outside',
        listStyleType: 'square',
      },
      four: {
        listStyle: 'inside "--" linear-gradient(90deg, white 100%)',
        listStylePosition: 'outside',
        listStyleType: 'square',
      },
    });
  "#
);

stylex_test_panic!(
  list_style_invalid_values_none_inherit,
  "Invalid listStyle value: 'none inherit'",
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_property_validation_mode(PropertyValidationMode::Throw)
  }),
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        none: {
          listStyle: 'none inherit'
        },
      });
    "#
);

stylex_test_panic!(
  list_style_invalid_values_none_var,
  "Invalid listStyle value: 'none var(--image)'",
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_property_validation_mode(PropertyValidationMode::Throw)
  }),
  r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        none: {
          listStyle: 'none var(--image)'
        },
      });
    "#
);
