use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));
    config.style_resolution = Option::Some(StyleResolution::LegacyExpandShorthands);

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));
    config.style_resolution = Option::Some(StyleResolution::LegacyExpandShorthands);

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    let mut config = StyleXOptionsParams::default();

    config.runtime_injection = Option::Some(RuntimeInjection::Boolean(true));
    config.style_resolution = Option::Some(StyleResolution::LegacyExpandShorthands);

    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      PluginPass::default(),
      Option::Some(config),
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
