use crate::utils::prelude::*;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

stylex_test!(
  non_standard_end_aka_inset_inline_end,
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
        export const styles = stylex.create({ x: { end: 5 } });
      "#
);

stylex_test!(
  non_standard_margin_end_aka_margin_inline_end,
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
        export const styles = stylex.create({ x: { marginEnd: 0 } });
      "#
);

stylex_test!(
  non_standard_margin_horizontal_aka_margin_inline,
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
        export const styles = stylex.create({ x: { marginHorizontal: 0 } });
      "#
);

stylex_test!(
  non_standard_margin_start_aka_margin_inline_start,
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
        export const styles = stylex.create({ x: { marginStart: 0 } });
      "#
);

stylex_test!(
  non_standard_margin_vertical_aka_margin_block,
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
        export const styles = stylex.create({ x: { marginVertical: 0 } });
      "#
);

stylex_test!(
  non_standard_padding_end_aka_padding_inline_end,
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
        export const styles = stylex.create({ x: { paddingEnd: 0 } });
      "#
);

stylex_test!(
  non_standard_padding_horizontal_aka_padding_inline,
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
        export const styles = stylex.create({ x: { paddingHorizontal: 0 } });
      "#
);

stylex_test!(
  non_standard_padding_start_aka_padding_inline_start,
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
        export const styles = stylex.create({ x: { paddingStart: 0 } });
      "#
);

stylex_test!(
  non_standard_padding_vertical_aka_padding_block,
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
        export const styles = stylex.create({ x: { paddingVertical: 0 } });
      "#
);

stylex_test!(
  non_standard_start_aka_inset_inline_start,
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
        export const styles = stylex.create({ x: { start: 5 } });
      "#
);
