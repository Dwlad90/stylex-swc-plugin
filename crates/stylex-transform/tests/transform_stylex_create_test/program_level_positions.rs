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

// The cases below read the array-bound spelling under runtime injection, and
// the name-bound spelling beside it. The rules an array-bound call declares are
// injected before the statement that holds it, which is where the name-bound
// spelling puts them: the declaration a rule belongs to is the statement, not
// the object inside it. The first case in this file is the same module with
// runtime injection off, where no call is written at all.

stylex_test!(
  an_array_bound_create_injects_the_rules_it_declares,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [stylex.create({ a: { color: 'red' } })];
  "#
);

stylex_test!(
  a_name_bound_create_injects_the_same_rules,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = stylex.create({ a: { color: 'red' } });
  "#
);

stylex_test!(
  every_create_an_array_holds_injects_its_own_rules,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [
      stylex.create({ a: { color: 'red' } }),
      [stylex.create({ b: { color: 'blue' } })],
    ];
  "#
);

stylex_test!(
  an_array_holding_more_than_styles_keeps_the_injection_before_it,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const label = 'all';
    export const all = [label, 1, stylex.create({ a: { color: 'red' } })];
  "#
);

// A `keyframes` call inside an array is not bound to a name, so neither
// compiler compiles it. Recorded here because the walk that looks through an
// array hashes a string initializer too, which is the shape a compiled
// `keyframes` leaves behind.
stylex_test!(
  an_array_bound_keyframes_is_left_where_it_was_written,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } })];
  "#
);

// Arrays nested deeper than a module writes by hand, to show the walk reaches
// the object whatever level it sits at.
stylex_test!(
  a_deeply_nested_array_bound_create_still_injects_its_rules,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [[[[[[[[stylex.create({ a: { color: 'red' } })]]]]]]]];
  "#
);

// An array can hold the styles inside an object the author wrote. The rules
// still belong to the statement, so the walk looks through the object as it
// looks through the array.
stylex_test!(
  an_object_an_array_holds_injects_the_rules_inside_it,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [{ s: stylex.create({ a: { color: 'red' } }) }];
  "#
);

stylex_test!(
  an_object_nested_deeper_in_an_array_injects_its_rules_too,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const all = [{ outer: { inner: stylex.create({ a: { color: 'red' } }) } }];
  "#
);
