//! A call to a JavaScript global around a style value folds at compile time.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1253, where the call
//! failed the build with `Unsupported expression: Unknown` instead of being
//! folded. The expected class names and rule text are measured output of
//! `@stylexjs/babel-plugin@0.19.0` for the same input.
//!
//! Runtime injection is enabled so each snapshot records the emitted rule text
//! next to the class name: the class name is what a coercion divergence would
//! move, and the rule text is what proves the coerced value itself is right.

use crate::utils::prelude::*;
use stylex_ast::ast::convertors::create_string_expr;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// `.xfungia{color:#fff}` — the same rule a plain `'#fff'` produces.
stylex_test!(
  string_of_a_string_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String('#fff') },
    });
  "#
);

// `1` → `"1"`, `true` → `"true"`, `null` → `"null"`, `undefined` →
// `"undefined"`: the JavaScript spellings, not CSS-shaped substitutes.
stylex_test!(
  string_of_the_other_primitives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      number: { color: String(1) },
      boolean: { color: String(true) },
      nullValue: { color: String(null) },
      undefinedValue: { color: String(undefined) },
      notANumber: { color: String(NaN) },
      infinite: { color: String(Infinity) },
    });
  "#
);

// Numbers use the JavaScript spelling: `1e+21`, not `1000000000000000000000`.
stylex_test!(
  string_of_a_number_needing_exponential_form,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      large: { color: String(1e21) },
      small: { color: String(0.0000001) },
      negativeZero: { color: String(-0) },
    });
  "#
);

// An array joins with commas; `null` and `undefined` elements join as nothing.
// An object takes the `Object.prototype` default.
stylex_test!(
  string_of_an_array_or_an_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      array: { color: String(['a', 'b']) },
      nested: { color: String([1, [2, 3]]) },
      nullish: { color: String([null, undefined, 1]) },
      object: { color: String({ a: 1 }) },
    });
  "#
);

// Zero arguments give the empty string; surplus arguments are ignored.
stylex_test!(
  string_ignores_arguments_past_the_first,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      surplus: { color: String('#fff', '#000') },
    });
  "#
);

// Coercions compose: the inner call folds to a value the outer one coerces.
stylex_test!(
  string_of_a_string_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(String(1)) },
    });
  "#
);

// A local binding is inlined before the coercion, so the fold sees its value.
stylex_test!(
  string_of_a_local_binding,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const brand = '#fff';
    export const styles = stylex.create({
      root: { color: String(brand) },
    });
  "#
);

// A declared `String` is an ordinary function, so it is called rather than
// folded: `.x1be0z9o{color:shadowed}`.
stylex_test!(
  a_locally_declared_string_shadows_the_global,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const String = () => 'shadowed';
    export const styles = stylex.create({
      root: { color: String('#fff') },
    });
  "#
);

// A dynamic style's parameter has no compile-time value, so the coercion stays
// in the emitted function and the declaration becomes a custom property.
stylex_test!(
  string_of_a_dynamic_style_parameter,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (color) => ({ color: String(color) }),
    });
  "#
);

// The environment is an object, so it takes the `Object.prototype` default —
// `[object Object]` — rather than deopting or leaking its contents.
stylex_test!(
  string_of_the_environment_object,
  |tr| {
    let mut env = IndexMap::new();
    env.insert(
      "brandPrimary".to_string(),
      EnvEntry::Expr(create_string_expr("#123456")),
    );
    stylex_transform(tr.comments.clone(), |b| {
      b.with_runtime_injection().with_env(env)
    })
  },
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(stylex.env) },
    });
  "#
);

// A spread argument is not evaluated: the argument list is unknowable without
// the operand's length, so the build fails rather than folding a guess.
stylex_test_panic!(
  string_of_a_spread_argument_is_rejected,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(...['a', 'b']) },
    });
  "#
);

// `String(fn)` is the function's source text upstream. This evaluator retains
// no source, so it refuses rather than fold a confidently wrong value.
stylex_test_panic!(
  string_of_a_function_is_rejected,
  "Cannot coerce this value at compile time",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(() => 'x') },
    });
  "#
);

// The numeric-literal grammar, not Rust's float parsing: `0x1f` is `31` and
// surrounding whitespace is part of no literal, so `'  10  '` is `10`. The
// empty string is `0`, which drops the unit — `.xnalus7{width:0}`.
stylex_test!(
  number_of_a_numeric_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      plain: { width: Number('10') },
      hex: { width: Number('0x1f') },
      binary: { width: Number('0b101') },
      octal: { width: Number('0o17') },
      padded: { width: Number('  10  ') },
      exponent: { width: Number('1e3') },
      empty: { width: Number('') },
    });
  "#
);

// `null` is `0` and `undefined` is `NaN` — the one coercion where the two
// disagree. `NaN` and `Infinity` are values here, not refusals.
stylex_test!(
  number_of_the_other_primitives,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      number: { opacity: Number(1.5) },
      boolean: { opacity: Number(true) },
      falseValue: { opacity: Number(false) },
      nullValue: { opacity: Number(null) },
      undefinedValue: { opacity: Number(undefined) },
      notANumber: { opacity: Number(NaN) },
      infinite: { opacity: Number(Infinity) },
    });
  "#
);

// `NaN` flows into the declaration rather than failing the build, which is
// what upstream does: `.x1yfwku{opacity:NaN}`. `inf` and `nan` are Rust float
// spellings JavaScript rejects, so they are `NaN` too rather than infinity.
stylex_test!(
  number_of_a_string_that_is_not_a_numeric_literal,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      suffixed: { opacity: Number('10px') },
      rustInfinity: { opacity: Number('inf') },
      rustNan: { opacity: Number('nan') },
    });
  "#
);

// An array coerces through its join, so an empty one is `0`, a one-element one
// is its element, and a longer one is `NaN`. `[object Object]` is not a
// numeric literal either.
stylex_test!(
  number_of_an_array_or_an_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      emptyArray: { opacity: Number([]) },
      oneElement: { opacity: Number([5]) },
      coercedElement: { opacity: Number(['0x1f']) },
      longArray: { opacity: Number([1, 2]) },
      object: { opacity: Number({ a: 1 }) },
    });
  "#
);

// Zero arguments give `0` — not `Number(undefined)`, which is `NaN` — and
// surplus arguments are ignored.
stylex_test!(
  number_ignores_arguments_past_the_first,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      none: { opacity: Number() },
      surplus: { opacity: Number('1', '2') },
    });
  "#
);

// The two coercions compose in either order.
stylex_test!(
  number_and_string_calls_nest,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      numberOfString: { width: Number(String(10)) },
      stringOfNumber: { color: String(Number('0x1f')) },
    });
  "#
);

// A local binding is inlined before the coercion, so the fold sees its value.
stylex_test!(
  number_of_a_local_binding,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const size = '0x1f';
    export const styles = stylex.create({
      root: { width: Number(size) },
    });
  "#
);

// A declared `Number` is an ordinary function, so it is called rather than
// folded: `.x10h3iyq{width:42px}`.
stylex_test!(
  a_locally_declared_number_shadows_the_global,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const Number = () => 42;
    export const styles = stylex.create({
      root: { width: Number('0x1f') },
    });
  "#
);

// A dynamic style's parameter has no compile-time value, so the coercion stays
// in the emitted function and the declaration becomes a custom property.
stylex_test!(
  number_of_a_dynamic_style_parameter,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (size) => ({ width: Number(size) }),
    });
  "#
);

// Unlike `String(fn)`, this needs no source text: whatever a function's source
// says, it is not a numeric literal, so the answer is `NaN`.
stylex_test!(
  number_of_a_function_is_not_a_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { opacity: Number(() => 1) },
      inAnArray: { opacity: Number([() => 1]) },
      besideANumber: { opacity: Number([() => 1, 2]) },
    });
  "#
);

// A spread argument is not evaluated, for the same reason it is not for
// `String`: the argument list is unknowable without the operand's length.
stylex_test_panic!(
  number_of_a_spread_argument_is_rejected,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { opacity: Number(...[1]) },
    });
  "#
);

// Two arguments are two elements, so the declaration repeats exactly as the
// equivalent array literal's does: `.x1rrpg6l{color:red;color:blue}`.
stylex_test!(
  array_of_several_values_is_a_style_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      fromCall: { color: Array('red', 'blue') },
      fromLiteral: { color: ['red', 'blue'] },
      numbers: { opacity: Array(1, 2) },
      durations: { transitionDuration: Array('1s', '2s') },
    });
  "#
);

// A lone argument is a length only when it is a number. A string is an
// element, so `Array('3')` is `.xvck8lq{color:3}` rather than three holes, and
// a `null` element drops out as it does from a literal array.
stylex_test!(
  array_of_a_single_value_is_a_one_element_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      string: { color: Array('red') },
      numericString: { color: Array('3') },
      withNull: { color: Array(null, 'red') },
    });
  "#
);

// No elements is no declaration, whether the length is absent or zero.
stylex_test!(
  array_of_no_elements_declares_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      noArguments: { color: Array() },
      zeroLength: { color: Array(0) },
    });
  "#
);

// Calls compose: the coerced elements produce `.x433f35{color:red;color:1}`,
// and an array reached through a coercion joins with commas.
stylex_test!(
  array_calls_nest_with_the_other_globals,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      coercedElements: { color: Array(String('red'), String(1)) },
      joined: { color: String(Array('a', 'b')) },
      joinedHoles: { color: String(Array(3)) },
    });
  "#
);

// The fold reaches a computed key and every branch of a nested value object:
// `.x1rrpg6l` for the default and `.x1ehdwse:hover{color:green}` beside it.
stylex_test!(
  array_folds_in_a_computed_key_and_a_nested_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      computed: { [String('color')]: Array('red', 'blue') },
      nested: { color: { default: Array('red', 'blue'), ':hover': 'green' } },
    });
  "#
);

// A declared `Array` is an ordinary function, so it is called rather than
// folded: `.xrrf2x5{color:x}`.
stylex_test!(
  a_locally_declared_array_shadows_the_global,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const Array = () => ['x'];
    export const styles = stylex.create({
      root: { color: Array('red', 'blue') },
    });
  "#
);

// A dynamic style's parameter has no compile-time value, so the call stays in
// the emitted function and the declaration becomes a custom property.
stylex_test!(
  array_of_a_dynamic_style_parameter,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (color) => ({ color: Array(color, 'blue') }),
    });
  "#
);

// A single numeric argument is a length, and every hole it makes is
// `undefined`. The fold succeeds; the style-array check is what refuses.
stylex_test_panic!(
  array_of_a_length_reaches_the_style_array_check,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(3) },
    });
  "#
);

// One hole is enough, and the same check refuses an array element.
stylex_test_panic!(
  array_of_a_length_of_one_reaches_the_style_array_check,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(1) },
    });
  "#
);

stylex_test_panic!(
  array_of_an_array_reaches_the_style_array_check,
  "A style array value can only contain strings or numbers.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(Array('red', 'blue')) },
    });
  "#
);

// A fraction, a negative, and `NaN` are not array lengths, so there is no
// array to fold and the build fails rather than inventing one.
stylex_test_panic!(
  array_of_a_fractional_length_is_rejected,
  "Invalid array length.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(2.5) },
    });
  "#
);

stylex_test_panic!(
  array_of_a_negative_length_is_rejected,
  "Invalid array length.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(-1) },
    });
  "#
);

// `NaN` and `Infinity` reach the evaluator as their identifiers rather than as
// numeric literals. Both are counts all the same, and neither is a length —
// read as elements instead, they would be refused by the wrong check.
stylex_test_panic!(
  array_of_a_not_a_number_length_is_rejected,
  "Invalid array length.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(NaN) },
    });
  "#
);

stylex_test_panic!(
  array_of_an_infinite_length_is_rejected,
  "Invalid array length.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(Infinity) },
    });
  "#
);

// `2 ** 32` is one past the largest length, so it is not a length either.
stylex_test_panic!(
  array_of_a_length_past_the_limit_is_rejected,
  "Invalid array length.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(4294967296) },
    });
  "#
);

// A length JavaScript accepts but the compiler will not materialise. Every
// hole past the first already fails the style-array check, so the refusal
// costs nothing a stylesheet could have used.
stylex_test_panic!(
  array_of_an_unmaterialisable_length_is_rejected,
  "Array length is too large to evaluate at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(4294967295) },
    });
  "#
);

// A spread argument is not evaluated, for the same reason it is not for
// `String` and `Number`.
stylex_test_panic!(
  array_of_a_spread_argument_is_rejected,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(...['red', 'blue']) },
    });
  "#
);
