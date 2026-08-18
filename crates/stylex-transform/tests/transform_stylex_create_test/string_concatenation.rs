//! `+` in a style value concatenates as soon as either side is a string.
//!
//! Regression coverage for `flexGrow: '1' + 2`, which folded to the number `3`
//! because dispatch asked whether numeric coercion had failed rather than
//! whether an operand was a string — both `'1'` and `2` coerce, so the string
//! result was never reached. The expected class names and rule text are
//! measured output of `@stylexjs/babel-plugin@0.19.0` for the same input.
//!
//! Runtime injection is enabled so each snapshot records the emitted rule text
//! next to the class name: the class name is what a divergence in the folded
//! value would move, and the rule text is what proves the value itself is
//! right.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// `.x19nkakv{flex-grow:12}` — a numeric-looking string is still a string, so
// the `2` is appended rather than added.
stylex_test!(
  a_numeric_string_on_the_left_concatenates,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: '1' + 2 },
    });
  "#
);

// `.xeqyxl0{flex-grow:21}` — either side being a string is enough; which side
// it is only decides the order.
stylex_test!(
  a_numeric_string_on_the_right_concatenates,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: 2 + '1' },
    });
  "#
);

// `.x3drhav{flex-grow:123}` — the string spreads leftwards through a chain,
// because each `+` sees the string its predecessor produced.
stylex_test!(
  a_string_carries_through_a_chain_of_additions,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: '1' + 2 + 3 },
    });
  "#
);

// `.x8ya21n{flex-grow:33}` — and does not spread rightwards: the leading `1 +
// 2` is still numeric addition, and only the trailing `'3'` concatenates.
stylex_test!(
  addition_before_a_string_is_still_addition,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: 1 + 2 + '3' },
    });
  "#
);

// `.x1ikap7u{flex-grow:3}` — two numbers add. The fix must not turn arithmetic
// into concatenation.
stylex_test!(
  two_numbers_still_add,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: 1 + 2 },
    });
  "#
);

// `.xgyuaek{flex-grow:2}` — only `+` asks the question. Every other arithmetic
// operator coerces a numeric string to its number as before.
stylex_test!(
  a_numeric_string_still_coerces_under_multiplication,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { flexGrow: '1' * 2 },
    });
  "#
);

// `.xarbti{content:"ab"}`, `.x16319ns{content:"a"}` twice — the cases that
// already agreed with the reference implementation, kept so that the dispatch
// change cannot quietly move them. An empty operand on either side is the one
// most at risk: it is what the deleted helper used to read as "not a string".
stylex_test!(
  two_strings_concatenate_with_either_side_empty,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'a' + 'b' },
      b: { content: 'a' + '' },
      c: { content: '' + 'a' },
    });
  "#
);

// `.x1i1rx1s{width:1px}`, `.x1g8rjiy{width:3px}` twice — a number joined to a
// unit, directly and through both spellings of a nested fold.
stylex_test!(
  a_number_joins_a_unit_directly_and_nested,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { width: 1 + 'px' },
      b: { width: 1 + 2 + 'px' },
      c: { width: `${1 + 2}px` },
    });
  "#
);

// `.x1i1rx1s{width:1px}` — the operands reaching the fold through bindings
// rather than as literals, which is how real source spells it.
stylex_test!(
  a_string_binding_concatenates_with_a_number_binding,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const size = 1;
    const unit = 'px';
    export const styles = stylex.create({
      a: { width: size + unit },
    });
  "#
);

// `+` is the only operator that has to look at its right side before its left
// has coerced. An object under `*` still refuses on the left, which is the
// claim; what changed is the shape of the refusal. It is a deopt rather than
// an abort, so the declaration falls to the runtime — where the expression is
// `NaN`, exactly as it is upstream — instead of failing the build. See
// `deopt_unsupported!`.
stylex_test!(
  a_left_side_with_no_numeric_form_still_refuses_under_other_operators,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: (props) => ({ flexGrow: ({}) * props.x }),
    });
  "#
);

// `undefined` is a value the evaluator now hands back rather than a failure to
// resolve — from `void x`, and from a key an object does not carry — so it
// reaches the arithmetic path as an ordinary identifier. `ToNumber` of it is
// `NaN`, which is a value and lands in the stylesheet; asked of the binding
// table instead it resolves to no declaration and fails the build, which is
// what this pins against. `Infinity` is the same identifier shape with a
// number that is not `NaN`.
stylex_test!(
  the_nullish_and_numeric_globals_coerce_under_arithmetic,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const size = { s: 1 };
    export const styles = stylex.create({
      a: { flexGrow: 1 + size.missing },
      b: { flexGrow: void 0 * 2 },
      c: { flexGrow: Infinity - 1 },
      d: { flexGrow: NaN + 1 },
    });
  "#
);
