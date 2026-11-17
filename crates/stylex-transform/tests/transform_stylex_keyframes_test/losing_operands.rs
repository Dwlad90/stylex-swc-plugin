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
