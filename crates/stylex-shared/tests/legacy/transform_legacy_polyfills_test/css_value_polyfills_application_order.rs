use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_value_start_aka_inline_start_for_float_property,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'start' } });
      "#
);
