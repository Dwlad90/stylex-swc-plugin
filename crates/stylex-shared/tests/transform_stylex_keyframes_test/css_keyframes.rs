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
  keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    });
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
  local_variables_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    export const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
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
  template_literals_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
    export const styles = stylex.create({
      root: {
        animationName: `${name}`,
      }
    });
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
  keyframes_object_used_inline,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: stylex.keyframes({
          from: {
            color: 'red',
          },
          to: {
            color: 'blue',
          },
        }),
      },
    });
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
  keyframes_object_rtl_polyfills_legacy,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        insetBlockStart: 0,
      },
      to: {
        insetBlockStart: 100,
      }
    });
  "#
);
