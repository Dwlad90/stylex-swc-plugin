//! `+`'s string side coerces its operands the way the language does.
//!
//! Regression coverage for `'x' + true`, which deopted the declaration to the
//! runtime because the addition arm kept a string coercion of its own that read
//! only strings, numbers and big integers. The whole falsy list, arrays and
//! objects all have a string, and all of them now reach it through the coercion
//! the rest of the evaluator already shares.
//!
//! The expected class names and rule text are measured output of
//! `@stylexjs/babel-plugin@0.19.0` for the same input. Two operands are not
//! pinned here because the two compilers disagree on them: `'x' + 1n` and
//! `'x' + /ab/g` fold here, where the reference implementation refuses either
//! literal outright.
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

// `.xnqz4ln{content:"xtrue"}` and `.x1pkb7mx{content:"xfalse"}` — a boolean
// spells itself out on either side of the operator, and being the falsy one
// changes nothing: `ToString` is not `ToBoolean`.
stylex_test!(
  a_boolean_operand_spells_itself_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'x' + true },
      b: { content: 'x' + false },
    });
  "#
);

// `.x1ekmyci{content:"truex"}` — and does so from the left, where the string is
// the operand that decided the path rather than the one being converted.
stylex_test!(
  a_boolean_on_the_left_spells_itself_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: true + 'x' },
    });
  "#
);

// `.x1ram8kq{content:"xnull"}` and `.x1h03oxg{content:"xundefined"}` — the two
// nullish values are the ones this effort exists for, and `+` spells them out
// rather than skipping them the way an array join does.
stylex_test!(
  the_nullish_values_spell_themselves_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'x' + null },
      b: { content: 'x' + undefined },
    });
  "#
);

// `.xi0xkq2{content:"xNaN"}` and `.xvd8z61{content:"xInfinity"}` — the two
// numeric globals that survive as identifiers rather than literals, so they
// reach the coercion by the same route `undefined` does.
stylex_test!(
  the_numeric_globals_spell_themselves_out,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'x' + NaN },
      b: { content: 'x' + Infinity },
    });
  "#
);

// `.x10k9aj8{content:"x1,2"}` and `.x1qj0nkt{content:"x"}` — an array joins its
// elements with commas, and an empty one contributes nothing at all.
stylex_test!(
  an_array_operand_joins_its_elements,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'x' + [1, 2] },
      b: { content: 'x' + [] },
    });
  "#
);

// `.x1wrhqvx{content:"x[object Object]"}` — an object with no `toString` of its
// own takes the `Object.prototype` default, which is a string like any other
// and not a refusal.
stylex_test!(
  an_object_operand_takes_the_prototype_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: 'x' + {} },
    });
  "#
);

// The reporter's own pattern, end to end: `'0.25rem'` reaches the template
// through `??`, and the template's own fold is the addition arm under another
// name. `.x1v5h5rg{border-radius:0 0 .25rem .25rem}`.
stylex_test!(
  a_token_guarded_by_nullish_folds_into_its_template,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const radius = { s: '0.25rem' };
    export const styles = stylex.create({
      a: { borderRadius: `0 0 ${radius.s ?? ''} ${radius.s ?? ''}` },
    });
  "#
);

// An operator with no string result reaches this path only after the number
// path has already refused, and is refused again rather than coerced, so the
// declaration deopts and falls to the runtime as `"a" * "b"` and `null - 1`.
//
// Dynamic styles, because that is where the refusal shape is observable: in a
// static position an unfoldable value is a refused fold either way, so the two
// shapes differ only in which diagnostic is printed.
//
// The refusal used to be a build failure carrying a diagnostic the language
// does not agree with — `'a' * 'b'` is `NaN`, not an error. Left as a failure,
// the widened coercion would have widened it too: `null` and an array both have
// a string now, so these three would have started failing builds where they
// previously reached the runtime.
stylex_test!(
  an_operator_with_no_string_result_deopts_rather_than_failing,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: (props) => ({ flexGrow: 'a' * 'b' }),
      b: (props) => ({ flexGrow: null - 1 }),
      c: (props) => ({ flexGrow: [1, 2] * 2 }),
    });
  "#
);
