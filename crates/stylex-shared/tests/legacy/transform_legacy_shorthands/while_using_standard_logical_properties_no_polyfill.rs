use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
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
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_inline_basic_shorthand,
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_with_longhand_property_collisions,
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_block_basic_multivalue_shorthand,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        paddingBlock: "5px 10px"
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  padding_with_null_longhand_property_collisions,
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
      enable_logical_styles_polyfill: Some(false),
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
