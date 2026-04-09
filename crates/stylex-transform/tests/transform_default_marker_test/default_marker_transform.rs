use crate::utils::prelude::*;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

stylex_test!(
  default_marker_named_import,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import { defaultMarker, props } from '@stylexjs/stylex';

    const classNames = props(defaultMarker());
  "#
);

stylex_test!(
  default_marker_namespace_import,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const classNames = stylex.props(stylex.defaultMarker());
  "#
);

