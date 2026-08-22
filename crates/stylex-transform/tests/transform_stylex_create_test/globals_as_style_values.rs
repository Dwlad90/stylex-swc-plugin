//! `NaN` and `Infinity` written where a style value goes.
//!
//! The two are numbers, but the grammar has no literal for either, so an author
//! writes them as identifiers and they reach the evaluator as identifiers. What
//! the resolution chain answers with therefore decides whether every consumer
//! downstream sees a number or an unresolved name -- and style-value validation
//! is a consumer that reads the expression's shape rather than coercing it. It
//! admits a number and refuses an identifier.
//!
//! While the chain answered the name, the two disagreed with themselves: `0/0`
//! and `1/0` are the same values reached by arithmetic, and those folded and
//! agreed with the reference implementation, while `NaN` and `Infinity`
//! refused. The chain answers the value now, and the pair below is the whole
//! point of the file -- each spelling is written next to the arithmetic that
//! produces the same number, so a regression that separates them fails here.
//!
//! `undefined` is deliberately absent from most of it. It is the third name the
//! chain answers for, and the only one with no other spelling, so it keeps
//! answering the identifier and keeps being refused as a style value -- which
//! the reference implementation does too.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0` with
//! the same options, and agrees with it. Runtime injection is on so each
//! snapshot records the rule text beside the class name: the class name is what
//! a divergence moves, and the rule text is what shows the value behind it.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// ── The value position a property carries directly ──────────────────

// `height:NaNpx` and `height:Infinitypx`, beside the arithmetic that reaches
// the same two numbers. Neither declaration is one a browser accepts, and that
// is not this compiler's call to make: a class name is a hash of the
// declaration text, so emitting different nonsense than the reference
// implementation is the one answer that helps nobody.
stylex_test!(
  the_globals_and_the_arithmetic_that_reaches_them_agree,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: NaN },
      dividedByZero: { height: 0 / 0 },
      infinite: { height: Infinity },
      overflowed: { height: 1 / 0 },
      negativelyInfinite: { height: -Infinity },
      negativelyOverflowed: { height: -1 / 0 },
    });
  "#
);

// A property with no numeric unit takes the same spelling with `px` appended
// all the same, because the unit comes from the property table and not from the
// value being plausible.
stylex_test!(
  the_globals_on_a_property_that_takes_no_length,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { color: NaN },
      infinite: { color: Infinity },
    });
  "#
);

// ── Inside a fallback array ─────────────────────────────────────────

// The two part company here, and the parting is the reference implementation's
// rather than a rule of this compiler's: `NaN` is dropped as an absent value
// and `Infinity` is spelled into a declaration of its own ahead of the one that
// works. A fallback chain is exactly where an unusable first declaration is
// harmless -- the browser discards it and reads the next.
stylex_test!(
  the_globals_as_the_leading_element_of_a_fallback_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: [NaN, '2px'] },
      infinite: { height: [Infinity, '2px'] },
      negativelyInfinite: { height: [-Infinity, '2px'] },
    });
  "#
);

// Alone in the array, the parting shows in the style object rather than in the
// rules: an array of nothing but `NaN` declares nothing and keeps its key as an
// absence, where `Infinity` declares its one unusable rule.
stylex_test!(
  the_globals_as_the_only_element_of_a_fallback_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: [NaN] },
      infinite: { height: [Infinity] },
    });
  "#
);

// Trailing rather than leading, so the fallback order is read rather than
// assumed.
stylex_test!(
  the_globals_as_the_trailing_element_of_a_fallback_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: ['2px', NaN] },
      infinite: { height: ['2px', Infinity] },
    });
  "#
);

// ── Under a condition, and inside a dynamic style ───────────────────

// The other style-value position: written under a condition key rather than on
// the property. It reads the same, which is what says this is about the value
// and not about the position.
stylex_test!(
  the_globals_under_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: { height: { default: NaN, ':hover': [NaN, '2px'] } },
      infinite: { height: { default: Infinity, ':hover': [Infinity, '2px'] } },
    });
  "#
);

// A dynamic style's body reaches the same two positions through a different
// consumer of a style value, and has to answer the same.
stylex_test!(
  the_globals_inside_a_dynamic_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      notANumber: (h) => ({ height: [NaN, '2px'], width: h }),
      infinite: (h) => ({ height: [Infinity, '2px'], width: h }),
    });
  "#
);

// ── Reached other than by writing the name ──────────────────────────

// Through a binding, through arithmetic on the global, and through a member
// read that produces one. Each is a different path into the value position, and
// none of them may take a different answer than the bare name above.
stylex_test!(
  the_globals_reached_through_a_binding_and_through_arithmetic,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const NOT_A_NUMBER = NaN;
    const INFINITE = Infinity;

    export const styles = stylex.create({
      bound: { height: NOT_A_NUMBER, width: INFINITE },
      added: { height: NaN + 1, width: Infinity + 1 },
      multiplied: { height: NaN * 0, width: Infinity * 2 },
      negated: { height: -NaN, width: -Infinity },
      subtracted: { height: Infinity - Infinity },
    });
  "#
);

// Coerced to a string rather than read as a number. The string coercion already
// spelled both names correctly, and it goes on doing so now that the value
// arriving at it is a number rather than the name -- which is the half of this
// change that must not move.
stylex_test!(
  the_globals_coerced_to_a_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      templated: { fontFamily: `a${NaN}b`, content: `"${Infinity}"` },
      concatenated: { fontFamily: 'a' + NaN + 'b' },
    });
  "#
);

// A comparison and a conditional read the numbers rather than the names, which
// is the reading a reference to a bare identifier could not have given.
stylex_test!(
  the_globals_read_as_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      compared: { height: Infinity > 1 ? '1px' : '2px' },
      selfCompared: { height: NaN === NaN ? '1px' : '2px' },
      truthy: { height: Infinity ? '1px' : '2px' },
      logical: { height: NaN || '2px' },
    });
  "#
);

// `NaN` is falsy, so both rows choose `'2px'`, and getting there took a
// one-line fix outside this file.
//
// A ternary used to read its test through `convert_expr_to_bool`, a second
// truthiness table beside `coercions::to_js_boolean` whose numeric arm asked
// `n.value != 0.0` -- the one comparison `NaN` answers true. The arithmetic row
// shows the second table was already wrong before the globals answered numbers.
// The named row shows why it could not be left: while `NaN` resolved to an
// identifier the ternary *refused the build*, so answering the number turned a
// refusal into silently wrong CSS.
//
// The second table is gone now and both askers read the one bridge over the
// coercion; the file that holds the rest of that question is
// `truthiness_table`. This stays here because it is the row that made the drift
// visible, and because it is the pair of spellings this file exists to keep
// together.
stylex_test!(
  a_nan_test_in_a_ternary_takes_the_falsy_branch,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      named: { height: NaN ? '1px' : '2px' },
      computed: { height: (0 / 0) ? '1px' : '2px' },
      negative: { height: -1 ? '1px' : '2px' },
      zero: { height: 0 ? '1px' : '2px' },
      infinite: { height: Infinity ? '1px' : '2px' },
    });
  "#
);

// ── The name taken over by a binding ────────────────────────────────

// The whole answer stays keyed to the binding. A dynamic style's parameter
// named `NaN` is not the global, so the reference refuses and the value falls
// through to the inline-style path -- unchanged by the value the *unshadowed*
// name now answers with.
stylex_test!(
  a_shadowing_parameter_still_wins_over_the_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      shadowed: (NaN) => ({ height: NaN }),
      alsoShadowed: (Infinity) => ({ width: Infinity }),
    });
  "#
);

// A module-level binding of the name is the other way to take it over, and it
// refuses rather than folding either the binding's value or the global.
stylex_test_panic!(
  a_module_binding_of_the_name_refuses,
  "Referenced constant is not initialized.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const NaN = '5px';

    export const styles = stylex.create({
      shadowed: { height: NaN },
    });
  "#
);

// ── `undefined`, the name with no other spelling ────────────────────

// Refused as a style value on both compilers, and refused in an array too. It
// is in this file because it is the third name the chain answers for and the
// one that did not move -- a change that gave it a value would show here. Both
// sentences below are the reference implementation's own.
stylex_test_panic!(
  undefined_is_still_refused_as_a_style_value,
  "A style value can only contain an array, string or number.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      absent: { height: undefined },
    });
  "#
);

stylex_test_panic!(
  undefined_is_still_refused_inside_a_fallback_array,
  "A style array value can only contain strings or numbers.",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      absent: { height: [undefined, '2px'] },
    });
  "#
);
