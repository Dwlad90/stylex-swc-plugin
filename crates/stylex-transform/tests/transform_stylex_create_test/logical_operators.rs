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
stylex_test_panic!(
  nullish_refuses_a_zero_left_side,
  "unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const zero = 0;
    export const styles = stylex.create({ a: { flexGrow: zero ?? 5 } });
  "#
);

stylex_test_panic!(
  nullish_refuses_a_false_left_side,
  "unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const off = false;
    export const styles = stylex.create({ a: { color: off ?? 'red' } });
  "#
);

stylex_test_panic!(
  nullish_refuses_an_empty_string_left_side,
  "unknown error",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const blank = '';
    export const styles = stylex.create({ a: { color: blank ?? 'red' } });
  "#
);
