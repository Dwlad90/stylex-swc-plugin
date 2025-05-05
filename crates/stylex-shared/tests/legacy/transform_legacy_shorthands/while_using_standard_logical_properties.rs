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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),
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
      runtime_injection: Some(true),
      style_resolution: Some(StyleResolution::LegacyExpandShorthands),

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
