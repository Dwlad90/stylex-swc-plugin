//! A `keyframes` call on one side of `||`, `&&` or `??` inside a method call
//! that the engine folds.
//!
//! Both operands are evaluated before the operator picks one. So a call on
//! the side that loses still declares its rule, and the rules keep the order
//! of the source. A call on a side whose argument has no compile-time value
//! declares nothing, and the other side is still the answer.

use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| b.with_runtime_injection())
}

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_operand_of_or_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['x' || stylex.keyframes({ from: { color: 'red' } })].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1c0h6uh{animation-name:false,y}`
stylex_test!(
  a_losing_operand_of_and_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [false && stylex.keyframes({ from: { color: 'red' } }), 'y'].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_operand_of_nullish_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['x' ?? stylex.keyframes({ from: { color: 'red' } })].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_operand_in_a_receiver_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ('x' || stylex.keyframes({ from: { color: 'red' } })).concat(''),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1fj5o6u{animation-name:yx}`
stylex_test!(
  a_losing_operand_in_an_argument_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: 'y'.concat('x' || stylex.keyframes({ from: { color: 'red' } })),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_operand_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['x'].map((v) => v || stylex.keyframes({ from: { color: 'red' } })).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  a_winning_operand_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [''].map((v) => v || stylex.keyframes({ from: { color: 'red' } })).join(),
      },
    });
  "#
);

// `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_operand_with_no_value_declares_nothing,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['x' || stylex.keyframes({ from: { color: rt } })].join(),
      },
    });
  "#
);

// `@keyframes xl7tn2a-B{from{color:blue;}}`, `@keyframes xbjjx1r-B{from{color:red;}}`,
// `@keyframes x19cet5l-B{to{color:green;}}`, `.x1og1fke{animation-name:xl7tn2a-B,x,x19cet5l-B}`
stylex_test!(
  rules_keep_the_order_of_the_source,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [
          stylex.keyframes({ from: { color: 'blue' } }),
          'x' || stylex.keyframes({ from: { color: 'red' } }),
          stylex.keyframes({ to: { color: 'green' } }),
        ].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  a_deciding_operand_declares_its_rule_once,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } }) || 'y'].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x6o31q6{animation-name:b}`
stylex_test!(
  a_deciding_test_declares_its_rule_once,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } }) ? 'b' : 'c'].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  a_deciding_operand_beside_a_value_with_none,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } }) || rt].join(),
      },
    });
  "#
);

// A left operand of `??` that is falsy and not nullish decides nothing in the
// logical node, so the fold refuses, as the reference does.
stylex_test_panic!(
  a_falsy_left_operand_of_nullish_refuses,
  "unknown error",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    export const styles = stylex.create({
      root: {
        animationName: [0 ?? stylex.keyframes(A)].join(),
      },
    });
  "#
);

// The same rule inside a callback, where only the engine knows the left value.
stylex_test_panic!(
  a_falsy_left_operand_of_nullish_in_a_callback_refuses,
  "unknown error",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    export const styles = stylex.create({
      root: {
        animationName: [false].map((v) => v ?? stylex.keyframes(A)).join(),
      },
    });
  "#
);

// The same rule where the right operand calls no compiler function.
stylex_test_panic!(
  a_falsy_left_operand_of_nullish_refuses_with_no_compiler_call,
  "unknown error",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [0 ?? 5].join(),
      },
    });
  "#
);

// The same, inside a callback.
stylex_test_panic!(
  a_falsy_left_operand_of_nullish_in_a_callback_refuses_with_no_compiler_call,
  "unknown error",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => 0 ?? 5).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xmqsyqj{animation-name:true}`
//
// A call through a local function whose body calls a compiler function runs on
// the losing side too.
stylex_test!(
  a_losing_call_through_a_local_function_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    const mk = () => stylex.keyframes(A);
    export const styles = stylex.create({
      root: {
        animationName: [true || mk()].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xmqsyqj{animation-name:true}`
stylex_test!(
  a_losing_call_through_a_local_function_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    const mk = () => stylex.keyframes(A);
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || mk()).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x1wt5zd5{animation-name:x}`
//
// The search follows a chain of local functions.
stylex_test!(
  a_losing_call_through_two_local_functions_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    const mk = () => stylex.keyframes(A);
    const mk2 = () => mk();
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => 'x' || mk2()).join(),
      },
    });
  "#
);

// `@keyframes xl7tn2a-B{from{color:blue;}}`, `.x1wt5zd5{animation-name:x}`
stylex_test!(
  a_losing_call_through_a_local_function_with_an_argument_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const mk = (c) => stylex.keyframes({ from: { color: c } });
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => 'x' || mk('blue')).join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// The losing side throws before it reaches the compiler call. The reference
// stops the build with `Cannot read properties of undefined (reading 'bar')`.
// This compiler ignores a throw on the side that loses, as it does for a dead
// operand that would throw, so no rule is declared.
stylex_test!(
  a_losing_side_that_throws_first_declares_nothing,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const A = { from: { color: 'red' } };
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map((x) => true || (x.foo.bar || stylex.keyframes(A))).join(),
      },
    });
  "#
);
