// Tests for the ECMAScript coercions.
// Source: crates/stylex-js/src/coercions.rs
//
// Expected values are what `String(x)` answers in a JavaScript runtime, which
// is what `@stylexjs/babel-plugin` folds a `String(x)` call to.

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
  assert_eq!(
    to_js_string(&Expr::Arrow(ArrowExpr {
      span: DUMMY_SP,
      params: vec![],
      body: Box::new(BlockStmtOrExpr::Expr(Box::new(num_expr(1.0)))),
      is_async: false,
      is_generator: false,
      type_params: None,
      return_type: None,
      ctxt: Default::default(),
    })),
    None
  );
}

#[test]
fn only_null_and_undefined_join_as_empty() {
  assert!(joins_as_empty(&null_expr()));
  assert!(joins_as_empty(&ident_expr("undefined")));
  assert!(!joins_as_empty(&str_expr("")));
  assert!(!joins_as_empty(&num_expr(0.0)));
  assert!(!joins_as_empty(&ident_expr("NaN")));
}
