// Tests for the ECMAScript coercions.
// Source: crates/stylex-js/src/coercions.rs
//
// Expected values are what `String(x)` and `Number(x)` answer in a JavaScript
// runtime, which is what `@stylexjs/babel-plugin` folds those calls to.

use super::*;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    ArrayLit, ArrowExpr, AssignProp, BigInt, BindingIdent, BlockStmt, BlockStmtOrExpr, Bool,
    ComputedPropName, ExprOrSpread, GetterProp, Ident, IdentName, KeyValueProp, MethodProp, Null,
    Number, ObjectLit, Pat, Prop, PropName, PropOrSpread, Regex, SetterProp, SpreadElement, Str,
    ThisExpr, UnaryExpr,
  },
};

fn str_expr(value: &str) -> Expr {
  Expr::Lit(Lit::Str(Str {
    span: DUMMY_SP,
    value: value.into(),
    raw: None,
  }))
}

fn num_expr(value: f64) -> Expr {
  Expr::Lit(Lit::Num(Number {
    span: DUMMY_SP,
    value,
    raw: None,
  }))
}

fn bool_expr(value: bool) -> Expr {
  Expr::Lit(Lit::Bool(Bool {
    span: DUMMY_SP,
    value,
  }))
}

fn null_expr() -> Expr {
  Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))
}

fn ident_expr(name: &str) -> Expr {
  Expr::Ident(Ident::new(name.into(), DUMMY_SP, Default::default()))
}

fn ident(name: &str) -> Ident {
  Ident::new(name.into(), DUMMY_SP, Default::default())
}

fn big_int_expr(value: i64) -> Expr {
  Expr::Lit(Lit::BigInt(BigInt {
    span: DUMMY_SP,
    value: Box::new(value.into()),
    raw: None,
  }))
}

fn regex_expr(exp: &str, flags: &str) -> Expr {
  Expr::Lit(Lit::Regex(Regex {
    span: DUMMY_SP,
    exp: exp.into(),
    flags: flags.into(),
  }))
}

fn array_expr(elems: Vec<Option<Expr>>) -> Expr {
  Expr::Array(ArrayLit {
    span: DUMMY_SP,
    elems: elems
      .into_iter()
      .map(|elem| {
        elem.map(|expr| ExprOrSpread {
          spread: None,
          expr: Box::new(expr),
        })
      })
      .collect(),
  })
}

fn spread_array_expr(inner: Expr) -> Expr {
  Expr::Array(ArrayLit {
    span: DUMMY_SP,
    elems: vec![Some(ExprOrSpread {
      spread: Some(DUMMY_SP),
      expr: Box::new(inner),
    })],
  })
}

fn empty_object_expr() -> Expr {
  Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props: vec![],
  })
}

fn arrow_expr() -> Expr {
  Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    params: vec![],
    body: Box::new(BlockStmtOrExpr::Expr(Box::new(num_expr(1.0)))),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
    ctxt: Default::default(),
  })
}

#[test]
fn strings_are_their_own_coercion() {
  assert_eq!(to_js_string(&str_expr("#fff")).as_deref(), Some("#fff"));
  assert_eq!(to_js_string(&str_expr("")).as_deref(), Some(""));
  assert_eq!(to_js_string(&str_expr("null")).as_deref(), Some("null"));
}

#[test]
fn numbers_use_the_javascript_spelling() {
  assert_eq!(to_js_string(&num_expr(1.0)).as_deref(), Some("1"));
  assert_eq!(to_js_string(&num_expr(-0.0)).as_deref(), Some("0"));
  assert_eq!(to_js_string(&num_expr(1.5)).as_deref(), Some("1.5"));
  // Rust's `Display` would spell these out in full; JavaScript does not.
  assert_eq!(to_js_string(&num_expr(1e21)).as_deref(), Some("1e+21"));
  assert_eq!(to_js_string(&num_expr(1e-7)).as_deref(), Some("1e-7"));
  assert_eq!(to_js_string(&num_expr(f64::NAN)).as_deref(), Some("NaN"));
  assert_eq!(
    to_js_string(&num_expr(f64::INFINITY)).as_deref(),
    Some("Infinity")
  );
}

#[test]
fn booleans_null_and_undefined_use_their_javascript_spellings() {
  assert_eq!(to_js_string(&bool_expr(true)).as_deref(), Some("true"));
  assert_eq!(to_js_string(&bool_expr(false)).as_deref(), Some("false"));
  assert_eq!(to_js_string(&null_expr()).as_deref(), Some("null"));
  assert_eq!(
    to_js_string(&ident_expr("undefined")).as_deref(),
    Some("undefined")
  );
  assert_eq!(to_js_string(&ident_expr("NaN")).as_deref(), Some("NaN"));
  assert_eq!(
    to_js_string(&ident_expr("Infinity")).as_deref(),
    Some("Infinity")
  );
}

#[test]
fn a_big_integer_renders_its_digits_without_the_suffix() {
  assert_eq!(to_js_string(&big_int_expr(1)).as_deref(), Some("1"));
  assert_eq!(to_js_string(&big_int_expr(-42)).as_deref(), Some("-42"));
  // And it is a number, unlike every other object-shaped value.
  assert_eq!(to_js_number(&big_int_expr(10)), Some(10.0));
}

#[test]
fn a_regular_expression_renders_its_own_source() {
  assert_eq!(to_js_string(&regex_expr("x", "")).as_deref(), Some("/x/"));
  assert_eq!(
    to_js_string(&regex_expr("a+b", "gi")).as_deref(),
    Some("/a+b/gi")
  );
  // Its source is not a numeric literal, so it has a number all the same.
  assert!(
    to_js_number(&regex_expr("x", ""))
      .expect("a regular expression has a number")
      .is_nan()
  );
}

#[test]
fn an_identifier_that_is_not_a_global_value_has_no_coercion() {
  // Anything else would have been inlined from its binding before reaching
  // here, so an identifier that survives is not a value.
  assert_eq!(to_js_string(&ident_expr("someBinding")), None);
}

#[test]
fn arrays_join_with_commas() {
  assert_eq!(to_js_string(&array_expr(vec![])).as_deref(), Some(""));
  assert_eq!(
    to_js_string(&array_expr(vec![Some(str_expr("a")), Some(str_expr("b"))])).as_deref(),
    Some("a,b")
  );
  assert_eq!(
    to_js_string(&array_expr(vec![Some(num_expr(1.0))])).as_deref(),
    Some("1")
  );
}

#[test]
fn array_elements_with_no_value_join_as_nothing() {
  assert_eq!(
    to_js_string(&array_expr(vec![
      Some(null_expr()),
      Some(ident_expr("undefined")),
      Some(num_expr(1.0)),
    ]))
    .as_deref(),
    Some(",,1")
  );
  // A hole, which is neither `null` nor `undefined` but joins the same way.
  assert_eq!(
    to_js_string(&array_expr(vec![
      Some(num_expr(1.0)),
      None,
      Some(num_expr(2.0))
    ]))
    .as_deref(),
    Some("1,,2")
  );
}

#[test]
fn nested_arrays_flatten_through_the_join() {
  assert_eq!(
    to_js_string(&array_expr(vec![
      Some(num_expr(1.0)),
      Some(array_expr(vec![Some(num_expr(2.0)), Some(num_expr(3.0))])),
    ]))
    .as_deref(),
    Some("1,2,3")
  );
}

#[test]
fn an_unevaluated_spread_element_has_no_coercion() {
  assert_eq!(to_js_string(&spread_array_expr(array_expr(vec![]))), None);
}

#[test]
fn objects_use_the_object_prototype_default() {
  assert_eq!(
    to_js_string(&empty_object_expr()).as_deref(),
    Some(OBJECT_TO_STRING)
  );

  let with_prop = Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName::new("a".into(), DUMMY_SP)),
      value: Box::new(num_expr(1.0)),
    })))],
  });

  assert_eq!(to_js_string(&with_prop).as_deref(), Some("[object Object]"));
}

fn object_expr(props: Vec<PropOrSpread>) -> Expr {
  Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props,
  })
}

fn key_value_prop(key: PropName, value: Expr) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key,
    value: Box::new(value),
  })))
}

fn ident_key(name: &str) -> PropName {
  PropName::Ident(IdentName::new(name.into(), DUMMY_SP))
}

/// A zero-parameter arrow returning `body` — the shape an own conversion
/// method is written in.
fn returning_arrow(body: Expr) -> Expr {
  Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    params: vec![],
    body: Box::new(BlockStmtOrExpr::Expr(Box::new(body))),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
    ctxt: Default::default(),
  })
}

#[test]
fn an_own_to_string_answers_the_string_coercion() {
  // `String({ toString: () => 'red' })` is `'red'` in a runtime, so answering
  // the `Object.prototype` default would be a value no runtime produces.
  let overriding = object_expr(vec![key_value_prop(
    ident_key("toString"),
    returning_arrow(str_expr("red")),
  )]);

  assert_eq!(to_js_string(&overriding).as_deref(), Some("red"));

  // The same name spelled as a string key is the same own property.
  let string_key = object_expr(vec![key_value_prop(
    PropName::Str(Str {
      span: DUMMY_SP,
      value: "toString".into(),
      raw: None,
    }),
    returning_arrow(str_expr("red")),
  )]);

  assert_eq!(to_js_string(&string_key).as_deref(), Some("red"));
}

#[test]
fn the_two_hints_ask_for_the_methods_in_opposite_orders() {
  // `String` prefers `toString` and `Number` prefers `valueOf`, which is the
  // whole of the difference between them.
  let both = object_expr(vec![
    key_value_prop(ident_key("toString"), returning_arrow(str_expr("1"))),
    key_value_prop(ident_key("valueOf"), returning_arrow(num_expr(2.0))),
  ]);

  assert_eq!(to_js_string(&both).as_deref(), Some("1"));
  assert_eq!(to_js_number(&both), Some(2.0));
}

#[test]
fn a_missing_method_falls_through_the_way_object_prototype_does() {
  // `Object.prototype.valueOf` answers the object itself, which is not a
  // primitive, so a number falls through to an own `toString`.
  let to_string_only = object_expr(vec![key_value_prop(
    ident_key("toString"),
    returning_arrow(str_expr("7")),
  )]);

  assert_eq!(to_js_number(&to_string_only), Some(7.0));

  // `Object.prototype.toString` answers a primitive, so a string never reaches
  // an own `valueOf`.
  let value_of_only = object_expr(vec![key_value_prop(
    ident_key("valueOf"),
    returning_arrow(str_expr("v")),
  )]);

  assert_eq!(
    to_js_string(&value_of_only).as_deref(),
    Some(OBJECT_TO_STRING)
  );
  // The number still goes through that own `valueOf`, whose `'v'` is not a
  // numeric literal.
  assert!(to_js_number(&value_of_only).is_some_and(f64::is_nan));
}

#[test]
fn a_method_in_a_form_that_cannot_be_applied_has_no_coercion() {
  // A value that is not callable ends in a `TypeError` rather than a value,
  // and a method answering an object has not answered a primitive.
  let not_callable = object_expr(vec![key_value_prop(
    ident_key("toString"),
    str_expr("notfn"),
  )]);

  assert_eq!(to_js_string(&not_callable), None);

  let returns_object = object_expr(vec![key_value_prop(
    ident_key("toString"),
    returning_arrow(empty_object_expr()),
  )]);

  assert_eq!(to_js_string(&returns_object), None);
}

#[test]
fn an_object_whose_keys_cannot_be_named_has_no_coercion() {
  // A spread may carry an override in, and a computed key is how
  // `Symbol.toPrimitive` is spelled — neither can be read off the literal.
  let spread = object_expr(vec![PropOrSpread::Spread(SpreadElement {
    dot3_token: DUMMY_SP,
    expr: Box::new(empty_object_expr()),
  })]);

  assert_eq!(to_js_string(&spread), None);

  let computed = object_expr(vec![key_value_prop(
    PropName::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(str_expr("a")),
    }),
    num_expr(1.0),
  )]);

  assert_eq!(to_js_string(&computed), None);
}

#[test]
fn an_object_carrying_the_override_deeper_still_coerces() {
  // Only an *own* key replaces the default: a nested one does not.
  let nested = object_expr(vec![key_value_prop(
    ident_key("a"),
    object_expr(vec![key_value_prop(ident_key("toString"), arrow_expr())]),
  )]);

  assert_eq!(to_js_string(&nested).as_deref(), Some(OBJECT_TO_STRING));
}

#[test]
fn an_overriding_object_inside_an_array_converts_through_its_own_method() {
  // `Array.prototype.join` takes each element's `ToString`, own method and
  // all.
  let element = object_expr(vec![key_value_prop(
    ident_key("toString"),
    returning_arrow(str_expr("z")),
  )]);

  assert_eq!(
    to_js_string(&array_expr(vec![Some(num_expr(1.0)), Some(element)])).as_deref(),
    Some("1,z")
  );
}

#[test]
fn a_function_has_no_compile_time_coercion() {
  // `String(fn)` is the function's source text, which the evaluator does not
  // retain — so there is no answer to give.
  assert_eq!(to_js_string(&arrow_expr()), None);
}

#[test]
fn only_null_and_undefined_join_as_empty() {
  assert!(joins_as_empty(&null_expr()));
  assert!(joins_as_empty(&ident_expr("undefined")));
  assert!(!joins_as_empty(&str_expr("")));
  assert!(!joins_as_empty(&num_expr(0.0)));
  assert!(!joins_as_empty(&ident_expr("NaN")));
}

#[test]
fn a_string_of_decimal_digits_is_its_value() {
  assert_eq!(string_to_js_number("10"), 10.0);
  assert_eq!(string_to_js_number("00.5"), 0.5);
  assert_eq!(string_to_js_number(".5"), 0.5);
  assert_eq!(string_to_js_number("5."), 5.0);
  assert_eq!(string_to_js_number("+5"), 5.0);
  assert_eq!(string_to_js_number("-5"), -5.0);
  assert_eq!(string_to_js_number("1e3"), 1000.0);
  assert_eq!(string_to_js_number("1E3"), 1000.0);
  assert_eq!(string_to_js_number("1e-3"), 0.001);
  assert_eq!(string_to_js_number("1e+3"), 1000.0);
}

#[test]
fn an_empty_or_blank_string_is_zero() {
  assert_eq!(string_to_js_number(""), 0.0);
  assert_eq!(string_to_js_number("   "), 0.0);
  assert_eq!(
    string_to_js_number("\t\n\r\u{000B}\u{000C}\u{00A0}\u{FEFF}"),
    0.0
  );
}

#[test]
fn surrounding_whitespace_is_ignored() {
  assert_eq!(string_to_js_number("  10  "), 10.0);
  assert_eq!(string_to_js_number("\t\n\r 7  \u{FEFF}"), 7.0);
  assert_eq!(string_to_js_number("\u{2028}1\u{2029}"), 1.0);
  assert_eq!(string_to_js_number("\u{3000}1\u{2000}"), 1.0);
  // Interior whitespace is not, because it is not part of the literal.
  assert!(string_to_js_number("1 0").is_nan());
  // U+0085 is Unicode whitespace but not JavaScript whitespace, so a string
  // padded with it is not a numeric literal at all.
  assert!(string_to_js_number("\u{0085}1").is_nan());
}

#[test]
fn the_radix_prefixes_are_read_as_javascript_reads_them() {
  assert_eq!(string_to_js_number("0x1f"), 31.0);
  assert_eq!(string_to_js_number("0X1F"), 31.0);
  assert_eq!(string_to_js_number("0b101"), 5.0);
  assert_eq!(string_to_js_number("0B101"), 5.0);
  assert_eq!(string_to_js_number("0o17"), 15.0);
  assert_eq!(string_to_js_number("0O17"), 15.0);
  assert_eq!(string_to_js_number("  0x1f  "), 31.0);
  // Wider than `u128`, so the digits are accumulated rather than converted.
  assert_eq!(
    string_to_js_number("0xdeadbeefdeadbeefdeadbeef"),
    6.891571802158121e28
  );
  assert_eq!(
    string_to_js_number("0x1000000000000000000000000000000000"),
    5.444517870735016e39
  );
}

#[test]
fn a_radix_prefix_without_digits_of_that_radix_is_not_a_number() {
  assert!(string_to_js_number("0x").is_nan());
  assert!(string_to_js_number("0xg").is_nan());
  assert!(string_to_js_number("0b2").is_nan());
  assert!(string_to_js_number("0o8").is_nan());
  // A radix literal takes no sign, and is not a decimal literal either.
  assert!(string_to_js_number("-0x1f").is_nan());
  assert!(string_to_js_number("+0x1f").is_nan());
}

#[test]
fn infinity_is_spelled_the_way_javascript_spells_it() {
  assert_eq!(string_to_js_number("Infinity"), f64::INFINITY);
  assert_eq!(string_to_js_number("+Infinity"), f64::INFINITY);
  assert_eq!(string_to_js_number("-Infinity"), f64::NEG_INFINITY);
  assert_eq!(string_to_js_number("  Infinity  "), f64::INFINITY);
  assert!(string_to_js_number("infinity").is_nan());
  assert!(string_to_js_number("Infinity1").is_nan());
}

#[test]
fn the_rust_float_spellings_are_not_numbers() {
  // Rust's own float parsing accepts all of these; JavaScript accepts none of
  // them, and taking Rust's answer would put `inf` in a stylesheet.
  assert!(string_to_js_number("inf").is_nan());
  assert!(string_to_js_number("-inf").is_nan());
  assert!(string_to_js_number("nan").is_nan());
  assert!(string_to_js_number("NaN").is_nan());
}

#[test]
fn a_string_that_is_not_a_numeric_literal_is_not_a_number() {
  assert!(string_to_js_number("10px").is_nan());
  assert!(string_to_js_number("1_000").is_nan());
  assert!(string_to_js_number("1e").is_nan());
  assert!(string_to_js_number(".").is_nan());
  assert!(string_to_js_number("-").is_nan());
  assert!(string_to_js_number("1e5e5").is_nan());
}

#[test]
fn negative_zero_survives_the_coercion() {
  // It renders as `0`, but it is not `0`, and losing the sign here would lose
  // it everywhere downstream.
  assert!(string_to_js_number("-0").is_sign_negative());
  assert_eq!(string_to_js_number("-0"), 0.0);
  assert!(string_to_js_number("-0.0").is_sign_negative());
  assert!(!string_to_js_number("0").is_sign_negative());
}

#[test]
fn numbers_are_their_own_coercion() {
  assert_eq!(to_js_number(&num_expr(1.5)), Some(1.5));
  assert_eq!(to_js_number(&num_expr(0.0)), Some(0.0));
  assert_eq!(to_js_number(&num_expr(f64::INFINITY)), Some(f64::INFINITY));
  assert!(matches!(to_js_number(&num_expr(f64::NAN)), Some(value) if value.is_nan()));
}

#[test]
fn booleans_null_and_undefined_coerce_to_their_javascript_numbers() {
  assert_eq!(to_js_number(&bool_expr(true)), Some(1.0));
  assert_eq!(to_js_number(&bool_expr(false)), Some(0.0));
  // `null` is zero; `undefined` is not. The two differ here where they agree
  // under `ToString`.
  assert_eq!(to_js_number(&null_expr()), Some(0.0));
  assert!(matches!(to_js_number(&ident_expr("undefined")), Some(value) if value.is_nan()));
  assert!(matches!(to_js_number(&ident_expr("NaN")), Some(value) if value.is_nan()));
  assert_eq!(to_js_number(&ident_expr("Infinity")), Some(f64::INFINITY));
}

#[test]
fn strings_coerce_through_the_numeric_literal_grammar() {
  assert_eq!(to_js_number(&str_expr("0x1f")), Some(31.0));
  assert_eq!(to_js_number(&str_expr("  10  ")), Some(10.0));
  assert_eq!(to_js_number(&str_expr("")), Some(0.0));
  assert!(matches!(to_js_number(&str_expr("10px")), Some(value) if value.is_nan()));
}

#[test]
fn arrays_and_objects_coerce_through_their_string_form() {
  // An array's primitive value is its join, so a one-element array coerces
  // its element and a longer one cannot be a number.
  assert_eq!(to_js_number(&array_expr(vec![])), Some(0.0));
  assert_eq!(
    to_js_number(&array_expr(vec![Some(num_expr(5.0))])),
    Some(5.0)
  );
  assert_eq!(
    to_js_number(&array_expr(vec![Some(str_expr("0x1f"))])),
    Some(31.0)
  );
  assert_eq!(
    to_js_number(&array_expr(vec![Some(null_expr())])),
    Some(0.0)
  );
  assert_eq!(
    to_js_number(&array_expr(vec![Some(array_expr(vec![]))])),
    Some(0.0)
  );
  assert!(matches!(
    to_js_number(&array_expr(vec![Some(num_expr(1.0)), Some(num_expr(2.0))])),
    Some(value) if value.is_nan()
  ));
  // `[object Object]` is not a numeric literal.
  assert!(matches!(to_js_number(&empty_object_expr()), Some(value) if value.is_nan()));
}

#[test]
fn a_function_coerces_to_not_a_number() {
  // Its source text is unknown but irrelevant: no source text is a numeric
  // literal, so the number is `NaN` either way. `ToString` has no such luck.
  assert!(matches!(to_js_number(&arrow_expr()), Some(value) if value.is_nan()));
  // Which means a function inside an array does not make the array's number
  // unknowable either — the join is not a numeric literal whatever it holds.
  assert!(matches!(
    to_js_number(&array_expr(vec![Some(arrow_expr())])),
    Some(value) if value.is_nan()
  ));
  assert!(matches!(
    to_js_number(&array_expr(vec![Some(arrow_expr()), Some(num_expr(2.0))])),
    Some(value) if value.is_nan()
  ));
}

#[test]
fn a_value_with_no_string_form_has_no_number_either() {
  assert_eq!(to_js_number(&ident_expr("someBinding")), None);
  assert_eq!(to_js_number(&spread_array_expr(array_expr(vec![]))), None);
}

#[test]
fn only_a_number_has_a_number_value() {
  // What the value *is*, not what it coerces to: a numeric string coerces to
  // a number but is not one.
  assert_eq!(js_number_value(&num_expr(3.0)), Some(3.0));
  assert_eq!(js_number_value(&num_expr(-0.0)), Some(-0.0));
  assert_eq!(js_number_value(&str_expr("3")), None);
  assert_eq!(js_number_value(&bool_expr(true)), None);
  assert_eq!(js_number_value(&null_expr()), None);
  assert_eq!(js_number_value(&array_expr(vec![])), None);
  assert_eq!(js_number_value(&empty_object_expr()), None);
}

#[test]
fn the_numeric_globals_have_a_number_value() {
  // These survive evaluation as the identifiers they were written as, and are
  // numbers all the same. `undefined` arrives the same way and is not one.
  assert!(matches!(js_number_value(&ident_expr("NaN")), Some(value) if value.is_nan()));
  assert_eq!(
    js_number_value(&ident_expr("Infinity")),
    Some(f64::INFINITY)
  );
  assert_eq!(js_number_value(&ident_expr("undefined")), None);
  assert_eq!(js_number_value(&ident_expr("someBinding")), None);
}

#[test]
fn an_array_length_is_an_integer_below_two_to_the_thirty_second() {
  assert_eq!(to_array_length(0.0), Some(0));
  assert_eq!(to_array_length(-0.0), Some(0));
  assert_eq!(to_array_length(1.0), Some(1));
  assert_eq!(to_array_length(4_294_967_295.0), Some(4_294_967_295));
}

#[test]
fn a_count_that_is_not_an_array_length_has_none() {
  // Each of these is a `RangeError` in JavaScript, so no array exists.
  assert_eq!(to_array_length(2.5), None);
  assert_eq!(to_array_length(-1.0), None);
  assert_eq!(to_array_length(f64::NAN), None);
  assert_eq!(to_array_length(f64::INFINITY), None);
  assert_eq!(to_array_length(f64::NEG_INFINITY), None);
  // The limit is exclusive: `2 ** 32` is one past the largest length.
  assert_eq!(to_array_length(4_294_967_296.0), None);
  assert_eq!(to_array_length(1e30), None);
}

#[test]
fn null_and_undefined_coerce_to_an_empty_object() {
  // `ToObject` of either is a fresh object rather than a wrapper around
  // anything, which is why `Object(null)` carries no properties.
  assert_eq!(to_object(&null_expr()), Some(ObjectCoercion::EmptyObject));
  assert_eq!(
    to_object(&ident_expr("undefined")),
    Some(ObjectCoercion::EmptyObject)
  );
}

#[test]
fn a_value_that_is_already_an_object_coerces_to_itself() {
  // An array is an object too, so `ToObject` returns it unchanged.
  assert_eq!(
    to_object(&empty_object_expr()),
    Some(ObjectCoercion::Identity)
  );
  assert_eq!(
    to_object(&array_expr(vec![Some(str_expr("a"))])),
    Some(ObjectCoercion::Identity)
  );
}

#[test]
fn a_function_coerces_to_itself_and_says_so() {
  // Also returned unchanged, and reported apart because a caller may have no
  // way to hold a function.
  assert_eq!(to_object(&arrow_expr()), Some(ObjectCoercion::Function));
}

#[test]
fn a_primitive_coerces_to_a_wrapper_object() {
  assert_eq!(to_object(&str_expr("red")), Some(ObjectCoercion::Wrapper));
  assert_eq!(to_object(&num_expr(10.0)), Some(ObjectCoercion::Wrapper));
  assert_eq!(to_object(&bool_expr(true)), Some(ObjectCoercion::Wrapper));
  // The numeric globals arrive as identifiers and box like the numbers they
  // are, unlike the `undefined` that arrives the same way.
  assert_eq!(to_object(&ident_expr("NaN")), Some(ObjectCoercion::Wrapper));
  assert_eq!(
    to_object(&ident_expr("Infinity")),
    Some(ObjectCoercion::Wrapper)
  );
}

/// An expression the evaluator reduced to no value at all — neither a literal,
/// nor one of the surviving globals, nor an object, array, or function.
fn this_expr() -> Expr {
  Expr::This(ThisExpr { span: DUMMY_SP })
}

fn binding_pat(name: &str) -> Pat {
  Pat::Ident(BindingIdent {
    id: Ident::new(name.into(), DUMMY_SP, Default::default()),
    type_ann: None,
  })
}

/// A one-parameter arrow returning `body` — a form the crate refuses, because a
/// default initialiser on that parameter would make the body depend on it.
fn parameterised_arrow(body: Expr) -> Expr {
  Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    params: vec![binding_pat("x")],
    body: Box::new(BlockStmtOrExpr::Expr(Box::new(body))),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
    ctxt: Default::default(),
  })
}

/// A block-bodied arrow, whose statements are more than the crate reads.
fn block_bodied_arrow() -> Expr {
  Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    params: vec![],
    body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt::default())),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
    ctxt: Default::default(),
  })
}

fn getter_prop(name: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
    key: ident_key(name),
    ..Default::default()
  })))
}

fn setter_prop(name: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Setter(SetterProp {
    key: ident_key(name),
    param: Box::new(binding_pat("value")),
    ..Default::default()
  })))
}

fn method_prop(name: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Method(MethodProp {
    key: ident_key(name),
    function: Box::default(),
  })))
}

fn shorthand_prop(name: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Shorthand(Ident::new(
    name.into(),
    DUMMY_SP,
    Default::default(),
  ))))
}

fn assign_prop(name: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Assign(AssignProp {
    span: DUMMY_SP,
    key: Ident::new(name.into(), DUMMY_SP, Default::default()),
    value: Box::new(value),
  })))
}

#[test]
fn an_expression_that_is_not_a_value_has_no_coercion_at_all() {
  // Unlike an identifier, which may still be one of the globals that survive
  // evaluation, this is not a value of any kind — so neither coercion has an
  // answer and its `ToObject` outcome cannot be read off it either.
  assert_eq!(to_js_string(&this_expr()), None);
  assert_eq!(to_js_number(&this_expr()), None);
  assert_eq!(to_object(&this_expr()), None);
}

#[test]
fn an_object_whose_own_method_cannot_be_applied_has_no_number_either() {
  // A number asks for `valueOf` first, so an unapplicable one refuses before
  // the `Object.prototype` default is ever reached.
  let not_callable = object_expr(vec![key_value_prop(
    ident_key("valueOf"),
    str_expr("notfn"),
  )]);

  assert_eq!(to_js_number(&not_callable), None);
}

#[test]
fn a_conversion_method_written_as_a_method_or_an_accessor_has_no_coercion() {
  // Each of these is an own `toString`, so the `Object.prototype` default no
  // longer applies — and none of them is a form this crate can apply.
  assert_eq!(
    to_js_string(&object_expr(vec![method_prop("toString")])),
    None
  );
  assert_eq!(
    to_js_string(&object_expr(vec![getter_prop("toString")])),
    None
  );
  assert_eq!(
    to_js_string(&object_expr(vec![setter_prop("toString")])),
    None
  );
}

#[test]
fn a_conversion_arrow_the_crate_cannot_read_has_no_coercion() {
  let parameterised = object_expr(vec![key_value_prop(
    ident_key("toString"),
    parameterised_arrow(str_expr("red")),
  )]);

  assert_eq!(to_js_string(&parameterised), None);

  let block_bodied = object_expr(vec![key_value_prop(
    ident_key("toString"),
    block_bodied_arrow(),
  )]);

  assert_eq!(to_js_string(&block_bodied), None);
}

#[test]
fn a_shorthand_or_assignment_property_names_a_method_the_same_way() {
  // Neither carries a key node, so both are named from the identifier they
  // were written as — and neither is a form the coercion can apply.
  assert_eq!(
    to_js_string(&object_expr(vec![shorthand_prop("toString")])),
    None
  );
  assert_eq!(
    to_js_string(&object_expr(vec![assign_prop("toString", str_expr("red"))])),
    None
  );
  // Under any other name they leave the default in place.
  assert_eq!(
    to_js_string(&object_expr(vec![shorthand_prop("a")])).as_deref(),
    Some(OBJECT_TO_STRING)
  );
}

#[test]
fn a_key_that_is_not_a_name_is_not_a_conversion_method() {
  // A numeric key is readable — it is not the computed key `Symbol.toPrimitive`
  // is spelled with — but it names neither method, so the default stands.
  let numeric = object_expr(vec![key_value_prop(
    PropName::Num(Number {
      span: DUMMY_SP,
      value: 1.0,
      raw: None,
    }),
    str_expr("a"),
  )]);

  assert_eq!(to_js_string(&numeric).as_deref(), Some(OBJECT_TO_STRING));

  let big_int_key = object_expr(vec![key_value_prop(
    PropName::BigInt(BigInt {
      span: DUMMY_SP,
      value: Box::new(1.into()),
      raw: None,
    }),
    str_expr("a"),
  )]);

  assert_eq!(
    to_js_string(&big_int_key).as_deref(),
    Some(OBJECT_TO_STRING)
  );
}

#[test]
fn a_value_of_no_readable_kind_has_no_object_coercion() {
  // Which of the three outcomes applies cannot be read off this, so the caller
  // deopts rather than picking one.
  assert_eq!(to_object(&ident_expr("someBinding")), None);
  // An array is an object however its elements were written, so a spread does
  // not make its kind unreadable the way it makes its string form unknowable.
  assert_eq!(
    to_object(&spread_array_expr(array_expr(vec![]))),
    Some(ObjectCoercion::Identity)
  );
}

/// `void 0` — the third spelling of `undefined`, and the only one that is an
/// operator rather than a literal or a name.
fn void_expr(operand: Expr) -> Expr {
  Expr::Unary(UnaryExpr {
    span: DUMMY_SP,
    op: UnaryOp::Void,
    arg: Box::new(operand),
  })
}

#[test]
fn the_falsy_primitives_are_the_whole_of_the_falsy_list() {
  assert_eq!(to_js_boolean(&str_expr("")), Some(false));
  assert_eq!(to_js_boolean(&num_expr(0.0)), Some(false));
  assert_eq!(to_js_boolean(&num_expr(-0.0)), Some(false));
  assert_eq!(to_js_boolean(&bool_expr(false)), Some(false));
  assert_eq!(to_js_boolean(&null_expr()), Some(false));
  assert_eq!(to_js_boolean(&ident_expr("undefined")), Some(false));
  // `NaN` reaches the coercion two ways — as the global it was written as, and
  // as the number a fold arrived at — and is falsy by both routes. The second
  // is the one an inequality against zero answers wrongly.
  assert_eq!(to_js_boolean(&ident_expr("NaN")), Some(false));
  assert_eq!(to_js_boolean(&num_expr(f64::NAN)), Some(false));
  assert_eq!(to_js_boolean(&void_expr(num_expr(0.0))), Some(false));
}

#[test]
fn a_primitive_that_is_not_on_that_list_is_truthy() {
  // `'0'` and `'false'` are the two that catch a reader out: a non-empty
  // string is truthy whatever it spells.
  assert_eq!(to_js_boolean(&str_expr("0")), Some(true));
  assert_eq!(to_js_boolean(&str_expr("false")), Some(true));
  assert_eq!(to_js_boolean(&num_expr(-1.0)), Some(true));
  assert_eq!(to_js_boolean(&num_expr(f64::INFINITY)), Some(true));
  assert_eq!(to_js_boolean(&bool_expr(true)), Some(true));
  assert_eq!(to_js_boolean(&ident_expr("Infinity")), Some(true));
}

#[test]
fn the_two_spellings_of_a_value_agree_on_its_boolean_and_its_nullishness() {
  // `void x` and `NaN` each reach a coercion two ways, and a caller that folds
  // `??` on one spelling and refuses `||` on the other would disagree about a
  // value the language does not.
  for expr in [void_expr(num_expr(0.0)), ident_expr("undefined")] {
    assert_eq!(to_js_boolean(&expr), Some(false));
    assert!(is_nullish(&expr));
  }

  for expr in [num_expr(f64::NAN), ident_expr("NaN")] {
    assert_eq!(to_js_boolean(&expr), Some(false));
    assert!(!is_nullish(&expr));
  }
}

#[test]
fn zero_is_the_only_falsy_big_integer() {
  assert_eq!(to_js_boolean(&big_int_expr(0)), Some(false));
  assert_eq!(to_js_boolean(&big_int_expr(1)), Some(true));
  assert_eq!(to_js_boolean(&big_int_expr(-1)), Some(true));
}

#[test]
fn every_object_is_truthy_however_empty_it_is() {
  assert_eq!(to_js_boolean(&empty_object_expr()), Some(true));
  assert_eq!(to_js_boolean(&array_expr(vec![])), Some(true));
  assert_eq!(to_js_boolean(&regex_expr("", "")), Some(true));
  assert_eq!(to_js_boolean(&arrow_expr()), Some(true));
}

#[test]
fn an_own_conversion_method_does_not_enter_into_the_boolean() {
  // `ToBoolean` is the one coercion that never reaches `ToPrimitive`, so an
  // object whose `toString` answers the empty string is still truthy — where
  // its string coercion is the falsy value that method returns.
  let overriding = object_expr(vec![key_value_prop(
    ident_key("toString"),
    returning_arrow(str_expr("")),
  )]);

  assert_eq!(to_js_string(&overriding).as_deref(), Some(""));
  assert_eq!(to_js_boolean(&overriding), Some(true));
}

#[test]
fn an_object_with_no_string_form_still_has_a_boolean() {
  // A spread makes the elements unnameable and so the join unknowable, but an
  // array is truthy without any of them being read.
  let spread = spread_array_expr(array_expr(vec![]));

  assert_eq!(to_js_string(&spread), None);
  assert_eq!(to_js_boolean(&spread), Some(true));
}

#[test]
fn a_value_of_no_readable_kind_has_no_boolean() {
  // Which side of the falsy list this falls on cannot be read off it, so the
  // caller deopts rather than picking one.
  assert_eq!(to_js_boolean(&ident_expr("someBinding")), None);
  assert_eq!(to_js_boolean(&this_expr()), None);
}

#[test]
fn the_nullish_values_are_null_undefined_and_void() {
  assert!(is_nullish(&null_expr()));
  assert!(is_nullish(&ident_expr("undefined")));
  // `void` yields `undefined` whatever it is applied to, so the operand is
  // never read.
  assert!(is_nullish(&void_expr(num_expr(0.0))));
  assert!(is_nullish(&void_expr(str_expr("red"))));
}

#[test]
fn a_falsy_value_that_is_not_nullish_says_so() {
  // The distinction `??` rests on, and the one `||` does not draw: each of
  // these is falsy and none of them is nullish.
  assert!(!is_nullish(&str_expr("")));
  assert!(!is_nullish(&num_expr(0.0)));
  assert!(!is_nullish(&bool_expr(false)));
  assert!(!is_nullish(&ident_expr("NaN")));
}

#[test]
fn a_value_this_crate_cannot_read_is_not_nullish() {
  // Both spellings of nullish are syntax the predicate always recognises, so
  // anything else is answered rather than refused.
  assert!(!is_nullish(&ident_expr("someBinding")));
  assert!(!is_nullish(&this_expr()));
  assert!(!is_nullish(&empty_object_expr()));
  assert!(!is_nullish(&arrow_expr()));
}

// ==================== the global set, read as a set ====================

/// The three names the language spells as an identifier rather than as a
/// literal, and the only three.
///
/// Asked as a set rather than through a coercion because that is how the
/// evaluator asks: its reference-resolution chain has to know whether a name
/// *could* be one of these before it can decide whether something in scope took
/// the name over, and its object coercion answers that all three carry no own
/// properties. Neither wants a string back.
#[test]
fn the_three_globals_are_spelled_as_identifiers() {
  assert!(is_global_spelled_as_an_identifier(&ident("undefined")));
  assert!(is_global_spelled_as_an_identifier(&ident("NaN")));
  assert!(is_global_spelled_as_an_identifier(&ident("Infinity")));
}

/// Nothing else is, including every neighbouring spelling. The comparison is on
/// the whole symbol, so a difference of case or of one character is an ordinary
/// binding name -- and a binding is exactly what the caller must not mistake for
/// the global.
#[test]
fn no_other_name_is_a_global_spelled_as_an_identifier() {
  for name in [
    "nan",
    "NAN",
    "NaNa",
    "aNaN",
    "infinity",
    "INFINITY",
    "Undefined",
    "undefined_",
    "_undefined",
    "undefined2",
    // Other globals, which are spelled as identifiers but are not values a
    // coercion can read.
    "Math",
    "String",
    "Number",
    "globalThis",
    // Spellings no source can produce, which the predicate still must not
    // accept: a name carrying whitespace, one carrying a zero-width space, and
    // the empty symbol.
    "NaN ",
    " NaN",
    "NaN\u{200b}",
    "",
  ] {
    assert!(
      !is_global_spelled_as_an_identifier(&ident(name)),
      "expected `{}` not to be a global spelled as an identifier",
      name
    );
  }
}

/// The predicate and the coercions read one set, which is the whole reason the
/// set is written down once. A name the predicate accepts has a string form; one
/// it rejects is a binding this crate cannot read, and answers no string at all.
///
/// Asserted as a pair, per name, so a fourth global added to one reader and not
/// the other fails here rather than at whichever call site notices first.
#[test]
fn the_predicate_and_the_string_coercion_read_the_same_set() {
  for (name, expected) in [
    ("undefined", "undefined"),
    ("NaN", "NaN"),
    ("Infinity", "Infinity"),
  ] {
    assert!(is_global_spelled_as_an_identifier(&ident(name)), "{}", name);
    assert_eq!(to_js_string(&ident_expr(name)).as_deref(), Some(expected));
  }

  for name in ["someBinding", "nan", "Math"] {
    assert!(
      !is_global_spelled_as_an_identifier(&ident(name)),
      "{}",
      name
    );
    assert_eq!(to_js_string(&ident_expr(name)), None, "{}", name);
  }
}

/// Every value here was read out of `node -e 'console.log(x | 0)'`, which is
/// `ToInt32` spelled the shortest way, rather than derived from the
/// specification text.
#[test]
fn to_int32_wraps_into_the_signed_32_bit_range() {
  // The reason this function exists: a 64-bit negation answers -4294967297 for
  // `~[4294967296]`, where JavaScript answers -1.
  assert_eq!(to_int32(4_294_967_296.0), 0);
  assert_eq!(to_int32(2_147_483_648.0), -2_147_483_648);
  assert_eq!(to_int32(-2_147_483_649.0), 2_147_483_647);
  assert_eq!(to_int32(3_000_000_000.0), -1_294_967_296);
  assert_eq!(to_int32(1e21), -559_939_584);

  // Truncation is toward zero, not flooring.
  assert_eq!(to_int32(1.9), 1);
  assert_eq!(to_int32(-1.9), -1);

  // The values with no integer to wrap all answer zero rather than refusing.
  assert_eq!(to_int32(f64::NAN), 0);
  assert_eq!(to_int32(f64::INFINITY), 0);
  assert_eq!(to_int32(f64::NEG_INFINITY), 0);
  assert_eq!(to_int32(0.0), 0);
  assert_eq!(to_int32(-0.0), 0);

  // Inside the range, it is the identity on integers.
  assert_eq!(to_int32(2_147_483_647.0), 2_147_483_647);
  assert_eq!(to_int32(-2_147_483_648.0), -2_147_483_648);
  assert_eq!(to_int32(-1.0), -1);
}

// ── global_identifier_to_value ───────────────────────────────────────

/// The two numeric globals answer with the numbers they *are*, and `undefined`
/// answers with itself.
///
/// The split is not cosmetic. A consumer that inspects the expression's shape
/// rather than coercing it — style-value validation is the one that does, since
/// it admits a number and refuses an identifier — sees `NaN` as a number here,
/// so `height: [NaN, '2px']` is accepted exactly as `height: [0/0, '2px']` is.
/// `undefined` has no numeric reading to answer with, so it stands.
#[test]
fn global_identifier_to_value_answers_numbers_for_the_two_numeric_globals() {
  match global_identifier_to_value(&ident("NaN")) {
    Some(Expr::Lit(Lit::Num(number))) => {
      assert!(number.value.is_nan(), "NaN is the value, not the name");
      assert_eq!(
        number.raw.as_deref(),
        Some("NaN"),
        "and it keeps the text it was authored with"
      );
    },
    other => panic!("expected NaN to answer a number, got {:?}", other),
  }

  match global_identifier_to_value(&ident("Infinity")) {
    Some(Expr::Lit(Lit::Num(number))) => {
      assert_eq!(number.value, f64::INFINITY);
      assert_eq!(number.raw.as_deref(), Some("Infinity"));
    },
    other => panic!("expected Infinity to answer a number, got {:?}", other),
  }
}

/// `undefined` has no literal spelling, so the identifier is the answer. The
/// span comes back with it, which is what lets a caller report against the
/// reference it read.
#[test]
fn global_identifier_to_value_answers_undefined_with_itself() {
  match global_identifier_to_value(&ident("undefined")) {
    Some(Expr::Ident(answered)) => assert_eq!(answered.sym.as_ref(), "undefined"),
    other => panic!("expected undefined to answer itself, got {:?}", other),
  }
}

/// `None` for every other name, which is what lets a caller use this as the set
/// as well as the coercion — and it agrees with the predicate that publishes
/// the set on its own.
#[test]
fn global_identifier_to_value_declines_every_other_name() {
  for name in ["Number", "nan", "NAN", "infinity", "undefined_", "x", ""] {
    assert!(
      global_identifier_to_value(&ident(name)).is_none(),
      "`{}` is not one of the three globals",
      name
    );
    assert!(
      !is_global_spelled_as_an_identifier(&ident(name)),
      "`{}` must agree with the predicate over the same set",
      name
    );
  }

  for name in ["undefined", "NaN", "Infinity"] {
    assert!(
      global_identifier_to_value(&ident(name)).is_some(),
      "`{}` is one of the three",
      name
    );
    assert!(is_global_spelled_as_an_identifier(&ident(name)));
  }
}

/// The `raw` text is the point of `number_spelled_as`, not the value: asked to
/// print a `Number` node holding `NaN` with no raw text, the emitter writes
/// `0 / 0`, and `Infinity` becomes a numeral no author wrote. Both evaluate
/// correctly either way, so this pins the text a reader diffs.
#[test]
fn the_numeric_globals_carry_the_text_they_were_authored_with() {
  let raw_of = |name: &str| match global_identifier_to_value(&ident(name)) {
    Some(Expr::Lit(Lit::Num(number))) => number.raw.map(|raw| raw.to_string()),
    other => panic!("expected a number for `{}`, got {:?}", name, other),
  };

  assert_eq!(raw_of("NaN").as_deref(), Some("NaN"));
  assert_eq!(raw_of("Infinity").as_deref(), Some("Infinity"));
}
