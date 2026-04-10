use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  valid_import_non_stylex,
  r#"
    import classnames from 'classnames';
  "#
);

stylex_test!(
  valid_import_named_export_of_stylex_create,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({});
  "#
);

stylex_test!(
  valid_import_default_export_of_stylex_create,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export default stylex.create({});
  "#
);

stylex_test!(
  valid_import_named_position_try,
  r#"
    import { positionTry } from '@stylexjs/stylex';
    const positionName = positionTry({});
  "#
);

stylex_test!(
  valid_import_namespace_position_try,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const positionName = stylex.positionTry({});
  "#
);

stylex_test!(
  valid_import_named_view_transition_class,
  r#"
    import { viewTransitionClass } from '@stylexjs/stylex';
    const transitionCls = viewTransitionClass({});
  "#
);

stylex_test!(
  valid_import_namespace_view_transition_class,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const transitionCls = stylex.viewTransitionClass({});
  "#
);
