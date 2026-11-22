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
// A function declaration is not followed. The reference declares no keyframes
// for a call through one on the losing side, and this compiler agrees.
stylex_test!(
  a_losing_call_through_a_function_declaration_declares_nothing,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return stylex.keyframes({ from: { color: 'red' } });
    }
    export const styles = stylex.create({
      root: {
        animationName: [true || mk()].join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// Only the engine can tell whether the left side decides, so the walk reaches
// the function declaration, which it cannot carry. That refusal is a put-off
// refusal: the left side decides, so it has no effect, and no keyframes are
// declared.
stylex_test!(
  a_losing_call_through_a_function_declaration_in_a_callback_declares_nothing,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return stylex.keyframes({ from: { color: 'red' } });
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || mk()).join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// The same where the body calls no compiler function.
stylex_test!(
  a_losing_function_declaration_in_a_callback_is_ignored,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || mk()).join(),
      },
    });
  "#
);

// `.xsm0v3u{animation-name:false}` and `.x1mjuhdk{animation-name:v}`
stylex_test!(
  a_losing_function_declaration_in_a_callback_is_ignored_by_and_and_nullish,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      and: {
        animationName: ['a'].map(() => false && mk()).join(),
      },
      nullish: {
        animationName: ['a'].map(() => 'v' ?? mk()).join(),
      },
    });
  "#
);

// `.x170th1f{animation-name:a,b}`
//
// The left side is a callback parameter, so it decides for each element.
stylex_test!(
  a_losing_function_declaration_is_ignored_where_each_element_decides,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a', 'b'].map((v) => v || mk()).join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// Other values the walk cannot carry are put-off refusals in the same way.
stylex_test!(
  a_losing_operand_the_walk_cannot_carry_is_ignored,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { foo } from 'other';
    class Foo {}
    function* gen() {
      yield 1;
    }
    export const styles = stylex.create({
      klass: {
        animationName: ['a'].map(() => true || new Foo()).join(),
      },
      generator: {
        animationName: ['a'].map(() => true || gen()).join(),
      },
      imported: {
        animationName: ['a'].map(() => true || foo()).join(),
      },
      date: {
        animationName: ['a'].map(() => true || new Date()).join(),
      },
    });
  "#
);

// The reference refuses where the left side does not decide, with the reason
// of the right side. This compiler refuses with the reason the walk gave.
stylex_test_panic!(
  a_function_declaration_on_the_side_that_wins_in_a_callback_refuses,
  "root > animationName > Cannot carry the function 'mk' into a fold.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => false || mk()).join(),
      },
    });
  "#
);

// The same where only one element does not decide.
stylex_test_panic!(
  a_function_declaration_that_wins_for_one_element_refuses,
  "root > animationName > Cannot carry the function 'mk' into a fold.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a', ''].map((v) => v || mk()).join(),
      },
    });
  "#
);

// A left side of `??` that is falsy and not nullish decides nothing. The
// reason of the right side comes first, as in the reference.
stylex_test_panic!(
  a_function_declaration_after_a_falsy_left_operand_of_nullish_refuses,
  "root > animationName > Cannot carry the function 'mk' into a fold.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => 0 ?? mk()).join(),
      },
    });
  "#
);

// A value the walk does not recognise, on the side that wins, leaves the call
// to the evaluator, which refuses it.
stylex_test_panic!(
  a_class_instance_on_the_side_that_wins_in_a_callback_refuses,
  "root > animationName > Cannot fold 'map' at compile time.",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    class Foo {}
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => false || new Foo()).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xmqsyqj{animation-name:true}`
//
// The losing side runs up to the part the walk refuses, so the keyframes in
// front of it are declared. The put-off refusal has no effect.
stylex_test!(
  a_losing_side_declares_its_rule_in_front_of_a_put_off_refusal,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      callback: {
        animationName: ['a'].map(() => true || [stylex.keyframes({ from: { color: 'red' } }), mk()]).join(),
      },
      unknown: {
        animationName: ['a'].map(() => true || [stylex.keyframes({ from: { color: 'red' } }), unknownName]).join(),
      },
      module: {
        animationName: [true || [stylex.keyframes({ from: { color: 'red' } }), mk()]].join(),
      },
      object: {
        animationName: ['a'].map(() => true || ({ a: stylex.keyframes({ from: { color: 'red' } }), b: mk() })).join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// A part after the refused part does not run, so no keyframes are declared.
stylex_test!(
  a_losing_side_declares_nothing_after_a_put_off_refusal,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || [mk(), stylex.keyframes({ from: { color: 'red' } })]).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.x142ryb9{animation-name:true,true}`
//
// Each element takes one arm. The arm with the refused part throws, and the
// other arm declares the keyframes.
stylex_test!(
  a_losing_side_declares_its_rule_in_the_arm_without_a_put_off_refusal,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function mk() {
      return 'x';
    }
    export const styles = stylex.create({
      root: {
        animationName: ['a', ''].map((v) => true || (v ? mk() : stylex.keyframes({ from: { color: 'red' } }))).join(),
      },
    });
  "#
);

// A left operand of `??` that is falsy and not nullish decides nothing. A
// throw of the right operand is then the refusal, in front of the reason of
// the logical node, as in the reference.
stylex_test_panic!(
  a_throw_after_a_falsy_left_operand_of_nullish_refuses_with_its_own_reason,
  "cannot convert 'null' or 'undefined' to object",
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map((v) => 0 ?? v.x.y).join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// A declaration the walk refuses is printed as a parameter default, which runs
// before the operator decides. So the name that reads it holds the put-off
// refusal, and the left side decides.
stylex_test!(
  a_refused_declaration_on_the_losing_side_is_ignored,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const g = (__sxRead) => 'x';
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || g()).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xmqsyqj{animation-name:true}`
//
// The left side of the inner operator refuses, and the right side still runs,
// as in the reference.
stylex_test!(
  a_refused_left_side_on_the_losing_side_still_declares_the_right_side,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      callback: {
        animationName: ['a'].map(() => true || (unknownName || stylex.keyframes({ from: { color: 'red' } }))).join(),
      },
      module: {
        animationName: [true || (unknownName || stylex.keyframes({ from: { color: 'red' } }))].join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `@keyframes xl7tn2a-B{from{color:blue;}}`,
// `.x1oangzh{animation-name:xl7tn2a-B,[object Object]}`
//
// The engine declares both keyframes and then gives the call back to the
// dispatch below for the shorthand. The dispatch declares them again, but a
// rule is kept by its name, so each rule is declared once.
stylex_test!(
  a_call_given_back_after_a_rule_declares_each_rule_once,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const b = 1;
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } }) && [stylex.keyframes({ from: { color: 'blue' } }), { b }]].join(),
      },
    });
  "#
);

// `.xmqsyqj{animation-name:true}`
//
// A documented difference. The reference declares the keyframes in each of
// these: it evaluates the elements in order and stops at the first one it
// cannot evaluate. Here a hole, a spread, a shorthand, a computed key and a
// method make the whole literal one part that holds a put-off refusal, so the
// keyframes call in it does not run.
stylex_test!(
  a_literal_the_walk_does_not_take_declares_nothing_on_the_losing_side,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const x = [1];
    const b = 1;
    const k = 'b';
    export const styles = stylex.create({
      spread: {
        animationName: ['a'].map(() => true || [stylex.keyframes({ from: { color: 'red' } }), ...x]).join(),
      },
      hole: {
        animationName: ['a'].map(() => true || [stylex.keyframes({ from: { color: 'red' } }), , 1]).join(),
      },
      shorthand: {
        animationName: ['a'].map(() => true || { a: stylex.keyframes({ from: { color: 'red' } }), b }).join(),
      },
      computed: {
        animationName: ['a'].map(() => true || { a: stylex.keyframes({ from: { color: 'red' } }), [k]: 1 }).join(),
      },
      method: {
        animationName: ['a'].map(() => true || { a: stylex.keyframes({ from: { color: 'red' } }), m() { return 1; } }).join(),
      },
    });
  "#
);

// `@keyframes xbjjx1r-B{from{color:red;}}`, `.xmqsyqj{animation-name:true}`
//
// A documented difference. The callback on the losing side refuses, and the
// reference stops the build with `Referenced constant is not defined.`. This
// compiler ignores a throw on the side that loses, as it does in
// `a_losing_side_that_throws_first_declares_nothing`, so the keyframes in front
// of the callback are declared and the left side is the answer.
stylex_test!(
  a_refusal_in_a_callback_on_the_losing_side_is_ignored,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['a'].map(() => true || [stylex.keyframes({ from: { color: 'red' } }), ['a'].map(() => unknownName)]).join(),
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
