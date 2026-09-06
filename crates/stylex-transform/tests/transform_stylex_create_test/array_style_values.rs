//! An array written as a style value, in a dynamic style's body and beside it.
//!
//! A dynamic style's body is walked by `evaluate_partial_object_recursively`,
//! and its two style-value positions read the evaluated value through
//! `materialize_style_value`. An array evaluates to a value with no expression
//! form, so every array written inside a dynamic style aborted with `Style value
//! must evaluate to a static expression.` -- including the ones the reference
//! implementation compiles. `dynamic_styles.rs` pins what a dynamic style does
//! with a parameter; this file asks what it does with a *fallback array*, in the
//! shapes an author can reach and in the ones only a fuzzer would.
//!
//! What an array may hold is namespace validation's question in both compilers,
//! not the value position's: an element that is not a string or a number is
//! refused with `A style array value can only contain strings or numbers.`,
//! which this position could not reach while the array aborted ahead of it.
//!
//! Every accepting case was measured against `@stylexjs/babel-plugin` 0.19.0
//! under the parity harness's options and agrees with it on class names and rule
//! text. The two shapes that still diverge are named at the test, each with the
//! reason it is not this file's to fix.

use crate::utils::{prelude::*, transform::stringify_js};

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// ──────────────────────────────────────────────
// The shapes the reference implementation compiles
// ──────────────────────────────────────────────

// The reported shape: a fallback chain of two lengths, written on a property of
// a dynamic style.
stylex_test!(
  an_array_of_strings_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: ['1px', '2px'] }),
    });
  "#
);

// A number takes its unit from the property inside a fallback array exactly as
// it does on the property itself.
stylex_test!(
  an_array_of_numbers_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ margin: [1, 2] }),
    });
  "#
);

// The array need not be written at the value position to reach it.
stylex_test!(
  an_array_read_through_a_binding_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = ['1px', '2px'];
    export const styles = stylex.create({
      dyn: (h) => ({ height: FALLBACKS }),
    });
  "#
);

// Both style-value positions a dynamic style's body has: the one a property
// carries directly, and the one under a condition key.
stylex_test!(
  an_array_under_every_condition_shape_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({
        height: { default: ['1px', '2px'], ':hover': ['3px', '4px'] },
        width: { default: '0px', '@media (min-width: 100px)': ['5px', '6px'] },
      }),
    });
  "#
);

// An empty array declares nothing, and declaring nothing is not an error.
stylex_test!(
  an_empty_array_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [], width: h }),
    });
  "#
);

// A `null` is a style value, so it survives the array check and drops out of
// the rule: one declaration, not two.
stylex_test!(
  an_array_carrying_a_null_element_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [null, '2px'] }),
    });
  "#
);

// A hole refuses the array, and in a dynamic style's body a refusal is not an
// error -- the value falls to the runtime as an inline custom property, which is
// what the reference implementation emits here. Dropping the hole instead
// emitted `height: 2px`, a value the source does not describe.
stylex_test!(
  an_array_carrying_a_hole_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [, '2px'] }),
    });
  "#
);

// The parameter refuses to fold, so the array is not confident and the whole
// value falls to the runtime rather than being folded short. This shape agreed
// before the fold and has to keep agreeing.
stylex_test!(
  an_array_holding_the_dynamic_parameter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [h, '2px'] }),
    });
  "#
);

// A spread refuses the array before the fold is reached, which is a different
// agreement from the rows above: the value falls to the runtime.
stylex_test!(
  an_array_carrying_a_spread_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const xs = ['1px'];
    export const styles = stylex.create({
      dyn: (h) => ({ height: [...xs, '2px'] }),
    });
  "#
);

// Neither expansion is skipped for a fallback array: the custom property keeps
// its name unhashed and the prefixed one is still spelled `-webkit-line-clamp`.
stylex_test!(
  an_array_on_a_custom_property_and_on_a_prefixed_property,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({
        '--foo': ['1px', '2px'],
        WebkitLineClamp: [1, 2],
        height: h,
      }),
    });
  "#
);

// An element is normalized as a value in its own right, escapes and all.
stylex_test!(
  an_array_of_non_ascii_and_escaped_elements,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({
        content: ['"日本語"', '"\\2014 A"'],
        fontFamily: ['"Foo\\"Bar"', 'serif'],
      }),
    });
  "#
);

// An element that is itself a fold: a template literal, a concatenation, a
// method call, a member read.
stylex_test!(
  an_array_of_folded_elements_in_a_dynamic_style,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({
        height: [`${1}px`, '1' + 'px', String(2) + 'px', [3, 'px'].join('')],
      }),
    });
  "#
);

// An unterminated quote is not malformed to either compiler: the value is a
// string to StyleX and the quote is a character in it.
stylex_test!(
  an_array_carrying_an_unterminated_quote,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ content: ['"unterminated', '"b"'] }),
    });
  "#
);

// A fallback array beside a static namespace holding one, so the two positions
// are measured against each other in a single module.
stylex_test!(
  an_array_in_a_dynamic_style_beside_one_in_a_static_namespace,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      s: { height: ['1px', '2px'] },
      dyn: (h) => ({ height: ['3px', '4px'], width: h }),
    });
  "#
);

// A theme member read inside a fallback array beside the dynamic parameter: the
// import stays a theme reference while the parameter stays a parameter.
stylex_test!(
  an_array_holding_a_theme_member_beside_a_dynamic_parameter,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';
    export const styles = stylex.create({
      dyn: (h) => ({ zIndex: [zIndex._10, 1], height: h }),
    });
  "#
);

// The parameter shadows the import, and the array is read where the shadowing
// decides what the name means -- the fold has to reach the same answer inside an
// array as on a property.
stylex_test!(
  an_array_beside_a_parameter_shadowing_a_theme_import,
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';
    export const styles = stylex.create({
      wrapper: { zIndex: zIndex._10 },
      dyn: (zIndex) => ({ zIndex: [zIndex, 1], height: ['1px', '2px'] }),
    });
  "#
);

// ──────────────────────────────────────────────
// What an array may not hold
//
// Every one of these reads the reference implementation's own sentence for the
// same input, which is the half that was unreachable while the array aborted
// first.
// ──────────────────────────────────────────────

stylex_test_panic!(
  an_array_carrying_a_nested_array_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [['1px'], '2px'] }),
    });
  "#
);

stylex_test_panic!(
  an_array_carrying_an_object_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [{ a: 1 }, '2px'] }),
    });
  "#
);

// `undefined` is not a style value where `null` is, which is the pair worth
// measuring together.
stylex_test_panic!(
  an_array_carrying_undefined_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [undefined, '2px'] }),
    });
  "#
);

stylex_test_panic!(
  an_array_carrying_a_boolean_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [true, '2px'] }),
    });
  "#
);

stylex_test_panic!(
  an_array_carrying_an_arrow_function_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: [() => 1, '2px'] }),
    });
  "#
);

// A parameter shadowing the namespace import folds to the injected function map,
// which has no array-element form. The message is the array-specific one because
// the fold happens inside an array -- upstream reads the same sentence here, and
// this is the input ticket 08 recorded as reachable only once the array folded.
stylex_test_panic!(
  an_array_holding_a_shadowed_namespace_parameter_is_refused,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (stylex) => ({ height: [stylex, '1px'] }),
    });
  "#
);

// A theme import read with no member access has no array-element form either,
// and is refused by the array check rather than by the value position.
stylex_test_panic!(
  an_array_holding_a_theme_object_is_refused,
  "A style array value can only contain strings or numbers.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';
    export const styles = stylex.create({
      dyn: (h) => ({ zIndex: [zIndex, 1] }),
    });
  "#
);

// The element is a string to both compilers, so what refuses it is the CSS lint
// the joined rule fails -- reached only because the array now folds.
stylex_test_panic!(
  an_array_carrying_an_unclosed_css_function_is_refused,
  "Rule contains an unclosed function",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: ['rgb(0,0,', '2px'] }),
    });
  "#
);

// ──────────────────────────────────────────────
// A hole outside a dynamic style
//
// The refusal belongs to the array rather than to the position, so it travels
// with the value: a static namespace refuses, and so does a binding read from
// one. Both read the reference implementation's own words -- it evaluates
// element paths, and a hole's path carries no node.
// ──────────────────────────────────────────────

stylex_test_panic!(
  an_array_carrying_a_hole_in_a_static_namespace_is_refused,
  "Could not resolve the code being evaluated",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      s: { height: [, '2px'] },
    });
  "#
);

stylex_test_panic!(
  a_trailing_hole_in_a_static_namespace_is_refused,
  "Could not resolve the code being evaluated",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      s: { height: ['1px', ,] },
    });
  "#
);

stylex_test_panic!(
  an_array_of_nothing_but_a_hole_is_refused,
  "Could not resolve the code being evaluated",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      s: { height: [,] },
    });
  "#
);

stylex_test_panic!(
  a_hole_read_through_a_binding_is_refused,
  "Could not resolve the code being evaluated",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = [, '2px'];
    export const styles = stylex.create({
      s: { height: FALLBACKS },
    });
  "#
);

// A `length` read through a binding to a holey array has no literal at the read,
// so the refusal travels with the value. It answered `1` where the language says
// `2` before, and nothing errored.
stylex_test_panic!(
  a_length_read_through_a_binding_to_a_holey_array_is_refused,
  "Could not resolve the code being evaluated",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const FALLBACKS = [, '2px'];
    export const styles = stylex.create({
      s: { height: FALLBACKS.length },
    });
  "#
);

// A trailing comma is punctuation, not a slot, so the array it ends still folds
// -- the pair every refusal above is measured against.
stylex_test!(
  a_trailing_comma_is_not_a_hole_in_a_style_value,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      dyn: (h) => ({ height: ['1px', '2px',] }),
    });
  "#
);

// ──────────────────────────────────────────────
// The shapes only a fuzzer writes
// ──────────────────────────────────────────────

// A thousand elements is one declaration a thousand fallbacks long. The fold is
// recursive over elements, so width is the boundary an author can actually
// reach; depth is bounded by the array check, which refuses a nested array.
#[test]
fn a_thousand_element_array_in_a_dynamic_style() {
  let elements = (0..1000)
    .map(|index| format!("'{}px'", index))
    .collect::<Vec<_>>()
    .join(", ");

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        dyn: (h) => ({{ height: [{}] }}),
      }});
    "#,
    elements
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection())
  });

  assert_eq!(output.matches("height:").count(), 1000);
  assert!(output.contains("height:0;height:1px;"));
  assert!(output.contains("height:999px}"));
}

// An array eight condition levels deep still reaches the fold, which is the
// depth the shadowing suite already measures for a plain value.
//
// The class name diverges from Babel 0.19.0 here, and for a reason that has
// nothing to do with the array: the two compilers order nested pseudo-classes
// differently, so a condition tree more than two deep hashes differently
// whatever it carries. Recorded in the corpus and filed as its own issue; the
// declaration text agrees.
#[test]
fn an_array_eight_conditions_deep_in_a_dynamic_style() {
  const KEYS: [&str; 8] = [
    ":hover",
    ":focus",
    ":active",
    "@media (min-width: 100px)",
    ":nth-child(2n)",
    "::before",
    "@supports (display: flex)",
    ":disabled",
  ];

  let mut value = String::from("['1px', '2px']");
  for key in KEYS.iter().rev() {
    value = format!("{{ '{}': {} }}", key, value);
  }

  let input = format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        dyn: (h) => ({{ height: {} }}),
      }});
    "#,
    value
  );

  let output = stringify_js(&input, ts_syntax(), |tr| {
    stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection())
  });

  assert!(output.contains("height:1px;height:2px"));
  assert!(output.contains("@media (min-width: 100px)"));
  assert!(output.contains(":disabled"));
  assert!(output.contains("::before"));
}
