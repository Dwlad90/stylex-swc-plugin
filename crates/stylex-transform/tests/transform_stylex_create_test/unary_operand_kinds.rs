//! What the unary operators read out of an operand, and which operands they
//! can read it from.
//!
//! The companion to `truthiness_table`, and the other half of the same fix. `!`
//! learned to ask the `ToBoolean` bridge about the evaluated *value* rather
//! than requiring an expression form, because the evaluator has value shapes
//! that have none -- an array is its own vector, a folded namespace is a
//! function map -- and each stands for an object. The four operators below were
//! left behind the expression-form guard on the reasoning that they "read a
//! primitive out of the operand, which only the expression form carries".
//!
//! That reasoning does not hold for any of them. `typeof` reads the operand's
//! *kind*, not a primitive, and `+`, `-` and `~` read `ToNumber`, which has its
//! own bridge sitting beside the `ToBoolean` one. So `![]` folded while
//! `typeof []` aborted the build, on the same operand.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0`
//! under the same options and agrees with it:
//!
//! ```text
//! typeof [1, 2]      content:"object"     +[]     z-index:0
//! typeof ({ a: 1 })  content:"object"     -[1]    z-index:-1
//! typeof (() => 1)   content:"function"   ~[]     z-index:-1
//! ```
//!
//! Runtime injection is on so each snapshot records the rule text beside the
//! class name, which is what says the fold reached the right value.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── `typeof` over the shapes with no expression form ────────────────

// An array folds to the evaluator's own vector and an object literal to its own
// map. Both stand for a JavaScript object, and `typeof` answers `"object"` for
// either -- which is what upstream folds, and what this refused as
// "Only static values are allowed inside of create() call." before.
stylex_test!(
  typeof_reads_the_kind_of_a_value_with_no_expression_form,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      array: { content: typeof [1, 2] },
      object: { content: typeof ({ a: 1 }) },
      nested: { content: typeof [[1], [2]] },
    });
  "#
);

// The arrow is the case the fast path above the fold missed: it tested
// `is_fn_expr() || is_class()` and an arrow is neither, so the operand was
// folded to a callback and then refused. `typeof` says `"function"` for all
// three spellings.
stylex_test!(
  typeof_says_function_for_every_spelling_of_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      arrow: { content: typeof (() => 1) },
      expression: { content: typeof (function () { return 1; }) },
      klass: { content: typeof (class {}) },
    });
  "#
);

// The primitives, unchanged by any of this, kept beside the cases above so a
// change to the kind table has to answer for both halves at once.
stylex_test!(
  typeof_over_the_primitives,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      string: { content: typeof 'x' },
      number: { content: typeof 1 },
      boolean: { content: typeof true },
      nul: { content: typeof null },
      undef: { content: typeof undefined },
    });
  "#
);

// ── `ToNumber` over the same shapes ─────────────────────────────────

// `+[]` is `0` because `[].join(',')` is the empty string and `Number('')` is
// zero; `-[1]` is `-1` and `~[]` is `-1`. All three are the number bridge's
// answer, reached through the operand's primitive string form exactly as the
// language reaches it.
stylex_test!(
  the_numeric_operators_read_a_value_with_no_expression_form,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      plus: { zIndex: +[] },
      minus: { zIndex: -[1] },
      tilde: { zIndex: ~[] },
      plusOfOne: { zIndex: +[1] },
    });
  "#
);

// The operands whose `ToNumber` is `NaN`, which is a number like any other and
// reaches the stylesheet as one. Upstream emits `z-index:NaN` for all three, and
// this compiler already emits `height:NaNpx`, `color:NaNpx` and `opacity:NaN`
// elsewhere -- `-({})` was refused only because the older numeric reading bailed
// on an object rather than coercing it.
stylex_test!(
  the_numeric_operators_over_an_operand_with_no_numeric_string_form,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      negatedObject: { zIndex: -({}) },
      plusObject: { zIndex: +({}) },
      negatedLongArray: { zIndex: -[1, 2, 3] },
      tildeObject: { zIndex: ~({}) },
    });
  "#
);

// `~` coerces with `ToInt32` before negating, so the negation is 32-bit. Every
// value here is upstream's: `~[4294967296]` is `-1` because the operand wraps to
// zero first, and a 64-bit negation would answer `-4294967297`. These operands
// are reachable only because the number bridge above made them so, which is why
// they are pinned here rather than beside the small ones.
stylex_test!(
  tilde_wraps_its_operand_into_thirty_two_bits,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      wrapsToZero: { zIndex: ~[4294967296] },
      wrapsToMin: { zIndex: ~[2147483648] },
      bigNumber: { zIndex: ~4294967296 },
      farPastTheRange: { zIndex: ~[1e21] },
    });
  "#
);
