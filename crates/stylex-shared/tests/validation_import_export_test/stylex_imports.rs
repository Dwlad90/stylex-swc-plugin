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
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_non_stylex,
  r#"
    import classnames from 'classnames';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_named_export_of_stylex_create,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({});
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_default_export_of_stylex_create,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export default stylex.create({});
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_named_position_try,
  r#"
    import { positionTry } from '@stylexjs/stylex';
    const positionName = positionTry({});
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_namespace_position_try,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const positionName = stylex.positionTry({});
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_named_view_transition_class,
  r#"
    import { viewTransitionClass } from '@stylexjs/stylex';
    const transitionCls = viewTransitionClass({});
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  valid_import_namespace_view_transition_class,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const transitionCls = stylex.viewTransitionClass({});
  "#
);
