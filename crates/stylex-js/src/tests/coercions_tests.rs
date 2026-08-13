// Tests for the ECMAScript coercions.
// Source: crates/stylex-js/src/coercions.rs
//
// Expected values are what `String(x)` and `Number(x)` answer in a JavaScript
// runtime, which is what `@stylexjs/babel-plugin` folds those calls to.

use super::*;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    ArrayLit, ArrowExpr, BlockStmtOrExpr, Bool, ExprOrSpread, Ident, IdentName, KeyValueProp, Null,
    Number, ObjectLit, Prop, PropName, PropOrSpread, Str,
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
