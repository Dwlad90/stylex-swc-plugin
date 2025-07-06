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

// =============================================================================
// [transform] CSS value polyfills (styleResolution: legacy-expand-shorthands and enableLogicalStylesPolyfill: true)
// =============================================================================

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
  non_standard_value_end_aka_inline_end_for_clear_property,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'end' } });
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
  non_standard_value_start_aka_inline_start_for_clear_property,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'start' } });
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
  non_standard_value_end_aka_inline_end_for_float_property,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'end' } });
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
  non_standard_value_start_aka_inline_start_for_float_property,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'start' } });
      "#
);