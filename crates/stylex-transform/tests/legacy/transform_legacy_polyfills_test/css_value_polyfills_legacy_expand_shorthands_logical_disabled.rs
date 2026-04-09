use crate::utils::prelude::*;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

stylex_test!(
  non_standard_value_end_aka_inline_end_for_clear_property,
  |tr| {
StyleXTransform::test(tr.comments.clone())
    
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    .with_enable_logical_styles_polyfill(false)
    .with_runtime_injection()
    .into_pass()
  },
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'end' } });
      "#
);

stylex_test!(
  non_standard_value_start_aka_inline_start_for_clear_property,
  |tr| {
StyleXTransform::test(tr.comments.clone())
    
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    .with_enable_logical_styles_polyfill(false)
    .with_runtime_injection()
    .into_pass()
  },
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'start' } });
      "#
);

stylex_test!(
  non_standard_value_end_aka_inline_end_for_float_property,
  |tr| {
StyleXTransform::test(tr.comments.clone())
    
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    .with_enable_logical_styles_polyfill(false)
    .with_runtime_injection()
    .into_pass()
  },
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'end' } });
      "#
);

stylex_test!(
  non_standard_value_start_aka_inline_start_for_float_property,
  |tr| {
StyleXTransform::test(tr.comments.clone())
    
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)
    .with_enable_logical_styles_polyfill(false)
    .with_runtime_injection()
    .into_pass()
  },
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'start' } });
      "#
);
