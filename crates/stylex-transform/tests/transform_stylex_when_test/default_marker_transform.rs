use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  default_marker_named_import,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import { defaultMarker, props } from '@stylexjs/stylex';

    export const classNames = props(defaultMarker());
  "#
);

stylex_test!(
  default_marker_namespace_import,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const classNames = stylex.props(stylex.defaultMarker());
  "#
);
