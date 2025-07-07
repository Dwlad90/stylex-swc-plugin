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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(true),
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams {
      runtime_injection: Some(true),
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
  border_color_basic_shorthand,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderColor: 'red blue green yellow'
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
      enable_logical_styles_polyfill: Some(true),
      ..StyleXOptionsParams::default()
    };

    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut config),
    )
  },
  border_width_basic_shorthand,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderWidth: '1px 2px 3px 4px'
      }
    });
    stylex(styles.foo);
  "#
);
