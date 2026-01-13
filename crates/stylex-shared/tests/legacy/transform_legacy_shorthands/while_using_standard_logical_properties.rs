use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{PropertyValidationMode, StyleResolution, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  stylex_call_with_exported_short_form_properties,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_basic_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  stylex_call_with_short_form_property_collisions,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),

      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  stylex_call_with_short_form_property_collisions_with_null,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_inline_basic_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_block_basic_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_with_longhand_property_collisions,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  margin_with_null_longhand_property_collisions,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_color_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_inline_width_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_inline_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  inset_block_multivalue_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(true),
      enable_dev_class_names: Some(false),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  list_style_basic_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(true),
      enable_dev_class_names: Some(false),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  list_style_multi_value_shorthand,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(RuntimeInjection::Boolean(true)),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(true),
      enable_dev_class_names: Some(false),
      ..StyleXOptionsParams::default()
    };
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  list_style_with_longhand_collisions,
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

#[test]
#[should_panic(expected = "Invalid listStyle value: 'none inherit'")]
fn list_style_invalid_values_none_inherit() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams {
        runtime_injection: Some(RuntimeInjection::Boolean(true)),
        style_resolution: Some(StyleResolution::LegacyExpandShorthands),
        enable_logical_styles_polyfill: Some(true),
        property_validation_mode: Some(PropertyValidationMode::Throw),
        ..StyleXOptionsParams::default()
      };
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        none: {
          listStyle: 'none inherit'
        },
      });
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Invalid listStyle value: 'none var(--image)'")]
fn list_style_invalid_values_none_var() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams {
        runtime_injection: Some(RuntimeInjection::Boolean(true)),
        style_resolution: Some(StyleResolution::LegacyExpandShorthands),
        enable_logical_styles_polyfill: Some(true),
        property_validation_mode: Some(PropertyValidationMode::Throw),
        ..StyleXOptionsParams::default()
      };
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
      import stylex from 'stylex';
      export const styles = stylex.create({
        none: {
          listStyle: 'none var(--image)'
        },
      });
    "#,
    r#""#,
  )
}
