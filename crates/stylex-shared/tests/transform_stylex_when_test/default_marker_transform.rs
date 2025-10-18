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
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  default_marker_named_import,
  r#"
    import { defaultMarker, props } from '@stylexjs/stylex';

    export const classNames = props(defaultMarker());
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
  default_marker_namespace_import,
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const classNames = stylex.props(stylex.defaultMarker());
  "#
);
