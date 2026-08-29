//! Where a `stylex.create` is written, and what that decides.
//!
//! A call at program level keeps the compiled object where the author put it. A
//! call written inside a function is hoisted to a program-level `const` and read
//! from there, so the object is built once rather than on every call.
//!
//! The question is about the position of *this* call. A module can hold a
//! top-level array of styles and a component that declares its own, and the
//! array says nothing about the component: reading the presence of an array as
//! an answer for every call in the module left a nested style object built on
//! each render.
//!
//! Every expectation below is compared against measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options. Which calls are
//! hoisted, and what each compiled object holds, agree with it.
//!
//! One thing does not, and it is neither new nor about the position being
//! decided here: this compiler puts a hoisted declaration at the top of the
//! module, where upstream puts it directly before the statement that holds the
//! call. The second case below shows the difference. It shows the same way in a
//! module with no array at all, so it belongs to the insertion rather than to
//! this question, and to a change of its own.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  a_create_inside_a_top_level_array_stays_where_it_was_written,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const lotsOfStyles = [stylex.create({ a: { color: 'red' } })];
  "#
);

stylex_test!(
  a_nested_create_is_hoisted_although_the_module_holds_an_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const lotsOfStyles = [stylex.create({ a: { color: 'red' } })];
    export function Component() {
      const styles = stylex.create({ b: { color: 'blue' } });
      return stylex.props(styles.b);
    }
  "#
);

stylex_test!(
  a_nested_create_is_hoisted_when_the_module_holds_no_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export function Component() {
      const styles = stylex.create({ b: { color: 'blue' } });
      return stylex.props(styles.b);
    }
  "#
);
