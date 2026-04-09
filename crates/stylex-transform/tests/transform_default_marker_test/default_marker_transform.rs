use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b))
}

stylex_test!(
  default_marker_named_import,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { defaultMarker, props } from '@stylexjs/stylex';

    const classNames = props(defaultMarker());
  "#
);

stylex_test!(
  default_marker_namespace_import,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const classNames = stylex.props(stylex.defaultMarker());
  "#
);
