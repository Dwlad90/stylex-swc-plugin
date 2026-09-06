//! `ToBoolean` at the two places that ask for it: a ternary's test and `!`.
//!
//! There used to be two truthiness tables. `coercions::to_js_boolean` is the
//! one the logical operators read; `convert_expr_to_bool` was a second copy
//! written before the coercion crate existed, and it had drifted -- it called
//! `NaN` true, so `NaN ? a : b` chose the consequent while `NaN || x` chose
//! `x`, on the same value. The second copy is gone and both askers read the
//! bridge over the one table, which is what every case below is about.
//!
//! The second half of that fix is *what* the question is asked of. Both askers
//! used to require the test to have an expression form, and the evaluator has
//! value shapes that do not: an array is its own vector and a folded namespace
//! is a function map. Each stands for an object, every object is truthy, and
//! requiring an expression refused `[] ? a : b` -- a test the language has no
//! doubt about -- where the reference implementation folds it.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options and agrees with it. Runtime injection is on so each
//! snapshot records the rule text beside the class name: `1px` is the consequent
//! and `2px` is the alternate, so the rule text alone says which branch was
//! taken.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── The falsy list, and the near misses around it ───────────────────

// The whole falsy list a ternary can be handed, each beside a value that looks
// like it and is not. `'0'` and `'false'` are non-empty strings and so truthy;
// `-0` is a zero and so falsy.
stylex_test!(
  the_falsy_list_and_its_near_misses,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      zero: { height: 0 ? '1px' : '2px' },
      negativeZero: { height: -0 ? '1px' : '2px' },
      one: { height: 1 ? '1px' : '2px' },
      emptyString: { height: '' ? '1px' : '2px' },
      stringZero: { height: '0' ? '1px' : '2px' },
      stringFalse: { height: 'false' ? '1px' : '2px' },
      whitespaceString: { height: ' ' ? '1px' : '2px' },
      nul: { height: null ? '1px' : '2px' },
      no: { height: false ? '1px' : '2px' },
      yes: { height: true ? '1px' : '2px' },
    });
  "#
);

// The falsy numbers reached by arithmetic rather than written, so the table is
// asked the same question about a value it did not read off a literal.
stylex_test!(
  the_falsy_numbers_reached_by_arithmetic,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: (0 / 0) ? '1px' : '2px' },
      subtractedToZero: { height: (1 - 1) ? '1px' : '2px' },
      multipliedToZero: { height: (0 * 5) ? '1px' : '2px' },
      infinite: { height: (1 / 0) ? '1px' : '2px' },
      negativelyInfinite: { height: (-1 / 0) ? '1px' : '2px' },
      negated: { height: -1 ? '1px' : '2px' },
      tildeOfZero: { height: ~0 ? '1px' : '2px' },
      tildeOfMinusOne: { height: ~(-1) ? '1px' : '2px' },
      unaryPlusZero: { height: +0 ? '1px' : '2px' },
    });
  "#
);

// The two named numbers, which have no literal and arrive as identifiers. The
// row that made the drift visible: `NaN` is falsy, and while the two tables
// disagreed a ternary read it as true where `||` read it as false.
stylex_test!(
  the_named_numbers_as_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: NaN ? '1px' : '2px' },
      infinite: { height: Infinity ? '1px' : '2px' },
      negativelyInfinite: { height: -Infinity ? '1px' : '2px' },
      undef: { height: undefined ? '1px' : '2px' },
      voided: { height: void 0 ? '1px' : '2px' },
      voidedTruthy: { height: void 'x' ? '1px' : '2px' },
    });
  "#
);

// ── The values with no expression form ──────────────────────────────

// An array is truthy however empty, and the evaluator spells one as its own
// vector rather than as an expression. Written inline and reached through a
// binding, since the two arrive at the question by different routes.
stylex_test!(
  an_array_is_truthy_as_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const EMPTY = [];
    const FILLED = ['a'];
    const NESTED = [[]];
    export const styles = stylex.create({
      inlineEmpty: { height: [] ? '1px' : '2px' },
      inlineFilled: { height: ['a'] ? '1px' : '2px' },
      boundEmpty: { height: EMPTY ? '1px' : '2px' },
      boundFilled: { height: FILLED ? '1px' : '2px' },
      boundNested: { height: NESTED ? '1px' : '2px' },
      arrayOfFalsy: { height: [0, '', null] ? '1px' : '2px' },
    });
  "#
);

// An object and a function are truthy on the same terms, whatever they hold.
stylex_test!(
  an_object_or_a_function_is_truthy_as_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const EMPTY_OBJECT = {};
    const ARROW = () => 1;
    export const styles = stylex.create({
      inlineObject: { height: {} ? '1px' : '2px' },
      boundObject: { height: EMPTY_OBJECT ? '1px' : '2px' },
      inlineArrow: { height: (() => 1) ? '1px' : '2px' },
      boundArrow: { height: ARROW ? '1px' : '2px' },
    });
  "#
);

// The folded namespace map and one entry of it. A fold stands for an object and
// so is truthy, which is the row that used to refuse -- recorded until now as
// `invalid_values::a_static_fold_read_as_a_condition_is_refused`.
stylex_test!(
  a_fold_is_truthy_as_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { keyframes, firstThatWorks } from '@stylexjs/stylex';
    export const styles = stylex.create({
      namespace: { height: stylex ? '1px' : '2px' },
      oneEntry: { height: keyframes ? '1px' : '2px' },
      anotherEntry: { height: firstThatWorks ? '1px' : '2px' },
    });
  "#
);

// ── `!` reads the same table ────────────────────────────────────────

// Every shape above, negated. `!` and a ternary asking the same value opposite
// questions is how the drift would come back, so both are pinned on one input.
stylex_test!(
  negation_reads_the_same_table,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const EMPTY = [];
    export const styles = stylex.create({
      notNaN: { height: !NaN ? '1px' : '2px' },
      notZero: { height: !0 ? '1px' : '2px' },
      notOne: { height: !1 ? '1px' : '2px' },
      notEmptyString: { height: !'' ? '1px' : '2px' },
      notUndefined: { height: !undefined ? '1px' : '2px' },
      notArray: { height: ![] ? '1px' : '2px' },
      notBoundArray: { height: !EMPTY ? '1px' : '2px' },
      notObject: { height: !{} ? '1px' : '2px' },
      notFold: { height: !stylex ? '1px' : '2px' },
      doubleNegated: { height: !!NaN ? '1px' : '2px' },
      tripleNegated: { height: !!![] ? '1px' : '2px' },
    });
  "#
);

// ── The table asked where a branch is not a style value ─────────────

// A ternary nested in every position that folds one, so a branch chosen wrongly
// shows as a moved class name rather than as a value that happens to agree.
stylex_test!(
  the_table_at_every_position_that_folds_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      interpolated: { fontFamily: `a${NaN ? 'b' : 'c'}d` },
      inAnArray: { height: [NaN ? '1px' : '2px', '3px'] },
      nestedTernary: { height: NaN ? '1px' : ([] ? '2px' : '3px') },
      inACondition: { ':hover': { height: NaN ? '1px' : '2px' } },
      inAMediaQuery: { '@media (min-width: 1px)': { height: [] ? '1px' : '2px' } },
      asAKey: { [NaN ? 'height' : 'width']: '1px' },
      inALogical: { height: (NaN ? '1px' : '') || '2px' },
      chained: { height: NaN ? '1px' : NaN ? '2px' : '3px' },
    });
  "#
);

// A dynamic style's parameter has no compile-time truthiness, so the ternary
// stays in the output unfolded rather than picking a branch -- which is the
// refusal path the bridge takes, reached from the one position that survives it.
stylex_test!(
  a_runtime_condition_stays_unfolded,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (flag) => ({ height: flag ? '1px' : '2px' }),
      negated: (flag) => ({ width: !flag ? '1px' : '2px' }),
    });
  "#
);
