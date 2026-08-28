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

// The environment object is this compiler's own value, not a JavaScript one, so
// it has no form the bridge carries and the engine never sees it. The conversion
// behind the fold answers for it instead, with the object default upstream also
// answers — `[object Object]`, which no stylesheet can use, but which is what
// the language says and so what the two compilers have to agree on.
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

// Without the option there is no environment object to convert, and the refusal
// is about the missing configuration rather than about the conversion.
stylex_test_panic!(
  string_of_an_unconfigured_environment_object_is_rejected,
  "The stylex.env object is not configured.",
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

// `String(fn)` is the function's source text upstream — and upstream's own
// answer is the source of a wrapper from inside its evaluator, not of the arrow
// the author wrote. This compiler has no source to give either: the engine it
// folds in is built without function source text, so the conversion throws and
// the fold refuses rather than writing a spelling no other build produces.
stylex_test_panic!(
  string_of_a_function_is_rejected,
  "A function has no source text at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(() => 'x') },
    });
  "#
);

// An object's conversion runs through its own `toString` where it has one, so
// this is `.x1e2nbdu{color:red}` -- the rule a plain `'red'` produces, and the
// class name `@stylexjs/babel-plugin@0.19.0` emits for the same input.
// Answering the `Object.prototype` default here would be a confidently wrong
// colour rather than a refused one.
stylex_test!(
  string_of_an_object_that_overrides_to_string,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String({ toString: () => 'red' }) },
    });
  "#
);

// A number prefers `valueOf` where a string prefers `toString`, which is the
// whole of the difference between the two conversions. Measured output:
// `.xfo62xy{width:2px}` for the pair, `.x1ftt334{width:5px}` for the lone
// `valueOf`.
stylex_test!(
  number_of_an_object_that_overrides_value_of,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      valueOfOnly: { width: Number({ valueOf: () => 5 }) },
      bothMethods: { width: Number({ toString: () => '1', valueOf: () => 2 }) },
      toStringOnly: { width: Number({ toString: () => '7' }) },
    });
  "#
);

// `Object.prototype.toString` answers a primitive, so a string conversion
// never reaches an own `valueOf` -- `.x19y1wga{color:[object Object]}`.
stylex_test!(
  string_of_an_object_that_overrides_only_value_of,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String({ valueOf: () => 'v' }) },
    });
  "#
);

// `Array.prototype.join` takes each element's `ToString`, own method and all:
// `.xiotjdv{color:1,z}`.
stylex_test!(
  string_of_an_array_holding_an_overriding_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String([1, { toString: () => 'z' }]) },
    });
  "#
);

// A spread is printed as written and the language does the spreading, so a
// spread of plain values folds: `.x19y1wga{color:[object Object]}`, the class
// name measured upstream.
stylex_test!(
  string_of_an_object_spreading_plain_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const base = { a: 1 };
    export const styles = stylex.create({
      root: { color: String({ ...base }) },
    });
  "#
);

// A spread of an object holding a *function* folds through its own `toString`.
// The spread operand is a name whose value holds a function, so nothing crosses
// the bridge and the engine never sees it — the conversion behind the fold reads
// the override instead, and answers `blue` as upstream does.
stylex_test!(
  string_of_an_object_spreading_an_override,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const base = { toString: () => 'blue' };
    export const styles = stylex.create({
      root: { color: String({ ...base }) },
    });
  "#
);

// A method that is not callable, and one that answers an object rather than a
// primitive, both end in a `TypeError` upstream rather than in a value. There
// is nothing to fold, so the build fails here too — in the language's own
// words, which is the same sentence upstream reports.
stylex_test_panic!(
  string_of_an_object_whose_to_string_is_not_callable_is_rejected,
  "cannot convert object to primitive value",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String({ toString: 'notfn' }) },
    });
  "#
);

stylex_test_panic!(
  string_of_an_object_whose_to_string_answers_an_object_is_rejected,
  "cannot convert object to primitive value",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String({ toString: () => ({}) }) },
    });
  "#
);

// Only an *own* key replaces the default. A nested one is just a property
// value, so the object still folds.
stylex_test!(
  string_of_an_object_carrying_the_override_deeper,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String({ a: { toString: () => 'red' } }) },
    });
  "#
);

// A lone surrogate is a legal JavaScript string with no Rust `str`, so it
// crosses back from the engine with the replacement character substituted for
// it. The declaration text is what upstream writes to disk; only the class name
// diverges, because upstream hashes the surrogate itself.
//
// That is not a decision this case takes — it is the one issue 06 took for the
// outward bridge, pinned in `engine_fold_tests::a_fold_whose_result_is_an_
// unpaired_surrogate_becomes_the_replacement_character` and carried in the
// parity corpus. This is the first coercion to reach it, which is why it is
// pinned here rather than left implied.
stylex_test!(
  string_of_a_lone_surrogate_substitutes_the_replacement_character,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { content: String('\uD800') },
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

// Upstream reaches `NaN` here, because a number is reached *through* the string
// and no function's source text is a numeric literal. This compiler refuses
// instead: its engine is built without function source text, so the conversion
// throws before it can reach the number. A written divergence, and in the safe
// direction — a refused build never names a class the other build does not
// define. The operators keep the `NaN`, because they coerce in Rust and never
// ask the engine; `unary_operand_kinds` pins that side.
stylex_test_panic!(
  number_of_a_function_is_rejected,
  "A function has no source text at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { opacity: Number(() => 1) },
    });
  "#
);

stylex_test_panic!(
  number_of_an_array_holding_a_function_is_rejected,
  "A function has no source text at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { opacity: Number([() => 1, 2]) },
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
// array to fold and the build fails rather than inventing one. The sentence is
// the language's own `RangeError`, which is what `Array(n)` answers a bad count
// with — upstream reports the same throw in its own words.
stylex_test_panic!(
  array_of_a_fractional_length_is_rejected,
  "Cannot fold 'Array' at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(2.5) },
    });
  "#
);

stylex_test_panic!(
  array_of_a_negative_length_is_rejected,
  "Cannot fold 'Array' at compile time.",
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
  "Cannot fold 'Array' at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(NaN) },
    });
  "#
);

stylex_test_panic!(
  array_of_an_infinite_length_is_rejected,
  "Cannot fold 'Array' at compile time.",
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
  "Cannot fold 'Array' at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Array(4294967296) },
    });
  "#
);

// A length JavaScript accepts but the compiler will not materialise. Every
// hole past the first already fails the style-array check, so the refusal
// costs nothing a stylesheet could have used -- and it arrives before the array
// exists, because the argument says how long it will be.
stylex_test_panic!(
  array_of_an_unmaterialisable_length_is_rejected,
  "It declares a length of 4294967295 elements, and at most 10000 are supported.",
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

// `Object()`, `Object(null)` and `Object(undefined)` are all a fresh empty
// object, which carries no declaration: upstream emits `{ root: { $$css: true
// } }` and no rules for each.
stylex_test!(
  object_of_a_nullish_value_declares_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      nullValue: { color: Object(null) },
      undefinedValue: { color: Object(undefined) },
      noArguments: { color: Object() },
    });
  "#
);

// An object argument is returned unchanged, so a coerced nested value emits
// what the bare one does: `.x1e2nbdu{color:red}` and
// `.x17z2mba:hover{color:blue}`.
stylex_test!(
  object_of_an_object_is_the_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      wrapped: { color: Object({ default: 'red', ':hover': 'blue' }) },
      bare: { color: { default: 'red', ':hover': 'blue' } },
    });
  "#
);

// An array is an object too, so it is returned unchanged as well: both write
// `.x1rrpg6l{color:red;color:blue}`.
stylex_test!(
  object_of_an_array_is_the_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      wrapped: { color: Object(['red', 'blue']) },
      bare: { color: ['red', 'blue'] },
    });
  "#
);

// Surplus arguments are ignored, as they are for the other globals: the second
// object never reaches the declaration.
stylex_test!(
  object_ignores_arguments_past_the_first,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      surplus: { color: Object({ default: 'red' }, { default: 'blue' }) },
    });
  "#
);

// The identity composes with itself and with the other coercions. An empty
// object takes the `Object.prototype` string default and the number that
// string has, so `String(Object(null))` is `[object Object]` and
// `Number(Object(null))` is `NaN`.
stylex_test!(
  object_calls_nest_with_the_other_globals,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      nested: { color: Object(Object({ default: 'red' })) },
      stringified: { color: String(Object(null)) },
      numbered: { width: Number(Object(null)) },
    });
  "#
);

// A local binding is inlined before the coercion, and a coerced branch of a
// nested value drops out of it the same way a bare nullish one would: only
// `.x1e2nbdu{color:red}` survives in `branch`.
stylex_test!(
  object_folds_over_a_binding_and_inside_a_nested_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const value = { default: 'red', ':hover': 'blue' };
    export const styles = stylex.create({
      binding: { color: Object(value) },
      branch: { color: { default: 'red', ':hover': Object(null) } },
    });
  "#
);

// A declared `Object` is an ordinary function, so it is called rather than
// folded and its return value is the declaration's.
stylex_test!(
  a_locally_declared_object_shadows_the_global,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const Object = () => 'red';
    export const styles = stylex.create({
      root: { color: Object('ignored') },
    });
  "#
);

// A parameter has no compile-time value, so the call is left for the runtime
// and the declaration becomes a custom property rather than folding.
stylex_test!(
  object_of_a_dynamic_style_parameter,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: (color) => ({ color: Object(color) }),
    });
  "#
);

// `Math` is a valid callee because its methods fold, so a bare call reaches
// the fold and has nothing to fold. The reference implementation leaks a
// `TypeError` from inside its own evaluator for this input; the observable
// outcome both share is that the module does not compile.
stylex_test_panic!(
  math_called_as_a_function_is_rejected,
  "Math is not a function.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { width: Math(1) },
    });
  "#
);

stylex_test_panic!(
  math_called_with_no_arguments_is_rejected,
  "Math is not a function.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { width: Math() },
    });
  "#
);

// The rejection is of the call, not of the global: its methods keep folding,
// and a declared `Math` is an ordinary function that is called.
stylex_test!(
  math_methods_fold_and_a_declared_math_is_called,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      method: { width: Math.pow(2, 3) },
    });
  "#
);

stylex_test!(
  a_locally_declared_math_shadows_the_global,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const Math = () => 'red';
    export const styles = stylex.create({
      root: { color: Math('ignored') },
    });
  "#
);

// A primitive argument is boxed in a wrapper object, which is not a plain one
// and so has no form the bridge carries back. Upstream folds the wrapper and
// then refuses it as a style value; both compilers reject the module, and this
// one says why one step earlier.
stylex_test_panic!(
  object_of_a_string_is_rejected,
  "Cannot carry a folded object back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object('red') },
    });
  "#
);

stylex_test_panic!(
  object_of_a_number_is_rejected,
  "Cannot carry a folded object back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { width: Object(10) },
    });
  "#
);

stylex_test_panic!(
  object_of_a_boolean_is_rejected,
  "Cannot carry a folded object back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object(true) },
    });
  "#
);

// `NaN` reaches the evaluator as its identifier rather than as a numeric
// literal, and is a number that boxes like one.
stylex_test_panic!(
  object_of_a_not_a_number_is_rejected,
  "Cannot carry a folded object back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { width: Object(NaN) },
    });
  "#
);

// A coercion of a primitive is still a primitive, so wrapping one changes
// nothing about the rejection.
stylex_test_panic!(
  object_of_a_string_call_is_rejected,
  "Cannot carry a folded object back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object(String('red')) },
    });
  "#
);

// A spread argument is not evaluated, for the same reason it is not for the
// other globals.
stylex_test_panic!(
  object_of_a_spread_argument_is_rejected,
  "Unsupported expression: SpreadElement",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object(...[{ default: 'red' }]) },
    });
  "#
);

// A function is returned unchanged rather than boxed, and is no more usable
// for it: a function has no form the bridge carries either, so it ends at the
// same rejection a wrapper does.
stylex_test_panic!(
  object_of_a_function_is_rejected,
  "Cannot carry a folded function back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object(() => 'red') },
    });
  "#
);

// A name holding a function ends where the arrow written out in place does. The
// name crosses as the declaration it came from, `Object` hands the function back
// unchanged, and there is no form to carry it back in -- so naming it changes
// nothing about the answer, which is what the reference compiler does too.
stylex_test_panic!(
  object_of_a_declared_function_is_rejected,
  "Cannot carry a folded function back from the engine.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const value = () => 'red';
    export const styles = stylex.create({
      root: { color: Object(value) },
    });
  "#
);

// A regular expression has no value this compiler carries. The conversion behind
// the fold evaluates the argument like any other expression, so what an author
// reads is the sentence the reference compiler reads for the same source —
// which refuses a regular expression outright, wherever it is written.
stylex_test_panic!(
  object_of_a_regular_expression_is_rejected,
  "Unsupported expression: RegExpLiteral",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: Object(/re/) },
    });
  "#
);

// --- Positions the fold has to reach, and coercions composing ---------------
//
// The per-global cases above all coerce a plain declaration value. These pin
// the remaining positions a style object offers, so a later narrowing of where
// the fold applies cannot pass unnoticed.

// A computed key is coerced before it names a property, so `[String('color')]`
// declares `color` and hashes as the literal key does — including when the
// coercion composes: `.x1t391ir{background-color:blue}`.
stylex_test!(
  a_coerced_computed_key_folds,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      fromString: { [String('color')]: 'red' },
      fromNumber: { [String('width')]: Number('10') },
      fromNested: { [String(String('backgroundColor'))]: 'blue' },
    });
  "#
);

// Every branch of a nested value coerces on its own, default and condition
// alike: `.x1e2nbdu{color:red}` beside `.x17z2mba:hover{color:blue}`, and the
// hexadecimal `0x14` reaching `:hover{width:20px}`.
stylex_test!(
  coercions_fold_at_every_branch_of_a_nested_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      strings: { color: { default: String('red'), ':hover': String('blue') } },
      numbers: { width: { default: Number('10'), ':hover': Number('0x14') } },
      arrays: { color: { default: Array('red', 'blue'), ':hover': Array('green') } },
    });
  "#
);

// A media query is a branch like any other, so a coercion inside one folds and
// keeps its at-rule priority.
stylex_test!(
  a_coercion_folds_inside_a_media_query_branch,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: {
          default: String('red'),
          ':hover': String('blue'),
          '@media (min-width: 768px)': String('green'),
        },
      },
    });
  "#
);

// The pairwise nestings are covered per global above; this is the one claim
// they leave open — all four in a single expression. `Number` reads the
// hexadecimal, `Object` hands its argument back, `Array` collects both, and
// `String` joins the result: `.x8qzml7{color:31,[object Object]}`.
stylex_test!(
  all_four_globals_compose_in_one_expression,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String(Array(Number('0x1f'), Object({ a: 1 }))) },
    });
  "#
);

// --- Names, chains and callbacks the conversions now reach ------------------
//
// The four globals are folded by being called rather than by a conversion
// written out in Rust, so a conversion is a fold like any other: its argument
// may be a name, its answer may be a receiver, and it may sit inside a callback.
// Each class name below is measured output of `@stylexjs/babel-plugin@0.19.0`.

// A named array crosses the bridge as a value and joins with commas:
// `.x1cc2d69{font-family:Inter,sans-serif}`.
stylex_test!(
  string_of_a_named_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const fonts = ['Inter', 'sans-serif'];
    export const styles = stylex.create({
      root: { fontFamily: String(fonts) },
    });
  "#
);

// Two names as two elements, and a hexadecimal string read from a third:
// `.x1rrpg6l{color:red;color:blue}` and `.xq14iec{width:31px}`.
stylex_test!(
  the_conversions_read_named_arguments,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const a = 'red';
    const b = 'blue';
    const size = '0x1f';
    export const styles = stylex.create({
      list: { color: Array(a, b) },
      width: { width: Number(size) },
    });
  "#
);

// An object crossing back from the engine is a value, so a property read off one
// folds: `.x1e2nbdu{color:red}`. So does a key list taken of one:
// `.xprt6xs{content:"a,b"}`.
stylex_test!(
  a_folded_object_is_read_and_chained,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const o = { a: 'red' };
    export const styles = stylex.create({
      property: { color: Object(o).a },
      keys: { content: Object.keys(Object({ a: 1, b: 2 })).join(',') },
    });
  "#
);

// A conversion inside a callback runs once per element: `.x1ulm48k{color:1-2}`.
stylex_test!(
  a_conversion_inside_a_callback_folds,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const xs = [1, 2];
    export const styles = stylex.create({
      root: { color: xs.map(x => String(x)).join('-') },
    });
  "#
);

// --- A conversion the engine never sees, and what bounds it -----------------
//
// The three cases above each hand one of this compiler's own values to a
// conversion. These pin what happens when such a value is *nested* in an
// argument the bridge would otherwise carry, which is the shape where the
// conversion behind the fold does real work rather than reading one value.

// The namespace map inside an array. The array alone would cross the bridge, so
// nothing but the map stops it — and the conversion behind the fold joins the
// two through the same coercion an interpolation uses. Upstream folds it to the
// same rule.
stylex_test!(
  string_of_an_array_holding_the_namespace_map,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: String([stylex, 'x']) },
    });
  "#
);

// The same shape carrying a string past the ceiling. The join is measured as it
// is written, so the build refuses at the element that passes the ceiling rather
// than after the whole megabyte has been copied — and the sentence names the
// conversion, which is what the author has to look at.
//
// Upstream folds this: it has no ceiling and writes a one-megabyte declaration.
// The ceiling is this compiler's own and is configurable, so raising
// `maxFoldedCharacters` past what the value needs folds the same source.
stylex_test_panic!(
  string_of_an_array_grown_past_the_ceiling_is_rejected,
  "This string conversion builds a string too large to evaluate at compile time.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const huge = 'x'.repeat(1000000);
    export const styles = stylex.create({
      root: { color: String([stylex, huge]) },
    });
  "#
);
