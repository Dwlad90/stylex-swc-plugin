//! `??`, `||` and `&&` around a style value fold at compile time.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1254, where a guarded
//! design token failed the build with `For string expressions, only addition is
//! supported, got "??"`. The expected class names and rule text are measured
//! output of `@stylexjs/babel-plugin@0.19.0` for the same input.
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

// The reproduction from issue #1254, verbatim:
// `.x1v5h5rg{border-radius:0 0 .25rem .25rem}`.
stylex_test!(
  nullish_in_a_template_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const radius = { s: '0.25rem' };
    export const styles = stylex.create({
      a: { borderRadius: `0 0 ${radius.s ?? ''} ${radius.s ?? ''}` },
    });
  "#
);

// `.x9hkwd3{margin:4px 2px}` — `||` and `&&` fold inside a template literal on
// the same terms `??` does.
stylex_test!(
  or_and_and_in_a_template_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const space = { s: '4px' };
    const fallback = '8px';
    export const styles = stylex.create({
      a: { margin: `${space.s || fallback} ${space.s && '2px'}` },
    });
  "#
);

// `.x1e2nbdu{color:red}` — the guard folds in a direct style value too, not
// only inside a template literal.
stylex_test!(
  nullish_in_a_direct_style_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({
      a: { color: color.primary ?? 'blue' },
    });
  "#
);

// `.x1u857p9{background-color:green}` — a property simply missing from an
// object is `undefined`, which the operator also takes its right side for.
stylex_test!(
  nullish_takes_the_fallback_for_a_missing_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({
      a: { backgroundColor: color.missing ?? 'green' },
    });
  "#
);

// `.xju2f9n{color:blue}` — a `null` left side is one of the two the operator
// takes its right side for.
stylex_test!(
  nullish_takes_the_fallback_for_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const token = null;
    export const styles = stylex.create({ a: { color: token ?? 'blue' } });
  "#
);

// `.x1e2nbdu{color:red}` for both — `void x` is the third spelling of
// `undefined`, and the operators take their right side for it the way they do
// for the other two. The operand is never evaluated, so the string it is
// applied to here neither reaches the fold nor could deopt it.
stylex_test!(
  nullish_and_or_take_the_fallback_for_void,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color: void 0 ?? 'red' },
      b: { color: void 'blue' || 'red' },
    });
  "#
);

// `.xju2f9n{color:blue}` and `.x1u857p9{background-color:green}` — `||` takes
// the fallback for an empty string, `&&` takes the right side for a set one.
stylex_test!(
  or_and_and_over_strings,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const empty = '';
    const set = 'red';
    export const styles = stylex.create({
      a: { color: empty || 'blue', backgroundColor: set && 'green' },
    });
  "#
);

// `.x1e2nbdu{color:red}` and `.x17z2mba:hover{color:blue}` — the winning
// operand is returned as the object it is, and the nested conditions inside it
// are read as usual.
stylex_test!(
  a_winning_object_stays_an_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const config = null;
    export const styles = stylex.create({
      a: { color: config ?? { default: 'red', ':hover': 'blue' } },
    });
  "#
);

// `.x1e565ft{font-family:Arial;font-family:sans-serif}` — a winning array is
// still the fallback list it was written as.
stylex_test!(
  a_winning_array_stays_an_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const list = null;
    export const styles = stylex.create({
      a: { fontFamily: list ?? ['Arial', 'sans-serif'] },
    });
  "#
);

// A falsy confident left side is returned as it is, and the empty string it
// wins with is a blank value, so the property is left undeclared and compiles
// to `null`. The reference implementation returns the same operand and then
// crashes on it downstream with a bare `TypeError`, which is not a behaviour
// worth reproducing.
stylex_test!(
  and_returns_a_falsy_left_side,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const empty = '';
    export const styles = stylex.create({ a: { color: empty && 'green' } });
  "#
);

// The reference implementation's nullish guard tests the left side's
// truthiness rather than its nullishness, so a left side that is falsy but
// present refuses to fold and deopts with `unknown error`. The restriction is
// inherited rather than corrected: folding here where the reference
// implementation does not would be a silent CSS difference between two builds
// of the same source.
//
// The message is asserted rather than the mere fact of failure — before the
// operator was implemented at all these inputs failed too, for the unrelated
// reason that every `??` was refused.
//
// The property path is asserted with it. A value that genuinely cannot fold has
// to be findable inside a large style object, and the deopt reason alone would
// name every such value identically.
stylex_test_panic!(
  nullish_refuses_a_zero_left_side,
  "a > flexGrow > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const zero = 0;
    export const styles = stylex.create({ a: { flexGrow: zero ?? 5 } });
  "#
);

stylex_test_panic!(
  nullish_refuses_a_false_left_side,
  "a > color > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const off = false;
    export const styles = stylex.create({ a: { color: off ?? 'red' } });
  "#
);

stylex_test_panic!(
  nullish_refuses_an_empty_string_left_side,
  "a > color > unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const blank = '';
    export const styles = stylex.create({ a: { color: blank ?? 'red' } });
  "#
);

// A missing property reads as `undefined` whether or not a logical operator is
// waiting for it, so a bare one now reaches the style-value check and fails the
// build there. Before, it deopted and the whole declaration fell to the runtime
// instead, which is the shape that kept `token.missing ?? fallback` from
// folding.
//
// The reference implementation fails the same input, wording it `A style value
// can only contain an array, string or number.`; which of the two refusals an
// `undefined` value earns is a pre-existing difference in the style-value
// check, not something the operator decides.
stylex_test_panic!(
  a_bare_missing_property_is_rejected_as_a_style_value,
  "Only static values are allowed inside of a stylex() call.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const color = { primary: 'red' };
    export const styles = stylex.create({ a: { color: color.missing } });
  "#
);
