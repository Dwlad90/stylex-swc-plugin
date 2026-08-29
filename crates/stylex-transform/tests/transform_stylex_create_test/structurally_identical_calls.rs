//! Two `stylex.create` calls that read the same, where only one of them is
//! exported.
//!
//! The state manager pins a declarator to its top-level entry by the name it
//! binds. Pinning it by the expression instead conflates these two calls, since
//! nothing but the name tells them apart: the exported declarator resolved to
//! the statement entry the unexported one recorded, read that entry's kind, and
//! skipped the pruning that belongs to an export. What that costs is visible
//! only in the emitted module, which is what these cases hold.
//!
//! Every expectation below is measured output of `@stylexjs/babel-plugin`
//! 0.19.0 under the same options.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  exported_style_object_survives_an_identical_unexported_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const b = stylex.create({ red: { color: 'red' } });
    export const a = stylex.create({ red: { color: 'red' } });
    export const x = stylex.props(b.red);
  "#
);

stylex_test!(
  exported_style_object_survives_when_it_is_declared_first,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const a = stylex.create({ red: { color: 'red' } });
    const b = stylex.create({ red: { color: 'red' } });
    export const x = stylex.props(b.red);
  "#
);

stylex_test!(
  two_identical_exported_style_objects_both_survive,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const a = stylex.create({ red: { color: 'red' } });
    export const b = stylex.create({ red: { color: 'red' } });
  "#
);
