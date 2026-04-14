//! Tests for AST convertor functions that transform between node types.

use crate::ast::{convertors::*, factories::*};
use swc_core::{atoms::Wtf8Atom, common::DUMMY_SP, ecma::ast::*};

#[test]
fn convert_lit_to_number_bool_true() {
  let lit = Lit::Bool(Bool {
    span: DUMMY_SP,
    value: true,
  });
  assert_eq!(convert_lit_to_number(&lit).unwrap(), 1.0);
}

#[test]
fn convert_lit_to_number_bool_false() {
  let lit = Lit::Bool(Bool {
    span: DUMMY_SP,
    value: false,
  });
  assert_eq!(convert_lit_to_number(&lit).unwrap(), 0.0);
}

#[test]
fn convert_lit_to_number_num() {
  let lit = create_number_lit(42.5);
  assert_eq!(convert_lit_to_number(&lit).unwrap(), 42.5);
}

#[test]
fn convert_lit_to_number_str_valid() {
  let lit = create_string_lit("123");
  assert_eq!(convert_lit_to_number(&lit).unwrap(), 123.0);
}

#[test]
fn convert_lit_to_number_str_invalid() {
  let lit = create_string_lit("abc");
  assert!(convert_lit_to_number(&lit).is_err());
}

#[test]
fn convert_lit_to_number_null_returns_err() {
  let lit = Lit::Null(swc_core::ecma::ast::Null { span: DUMMY_SP });
  assert!(convert_lit_to_number(&lit).is_err());
}

#[test]
fn convert_tpl_to_string_lit_simple() {
  let tpl = Tpl {
    span: DUMMY_SP,
    exprs: vec![],
    quasis: vec![TplElement {
      span: DUMMY_SP,
      tail: true,
      cooked: Some("hello".into()),
      raw: "hello".into(),
    }],
  };
  let result = convert_tpl_to_string_lit(&tpl);
  assert!(result.is_some());
  if let Some(Lit::Str(s)) = result {
    assert_eq!(s.value.as_str().unwrap(), "hello");
  }
}

#[test]
fn convert_tpl_to_string_lit_with_exprs_returns_none() {
  let tpl = Tpl {
    span: DUMMY_SP,
    exprs: vec![Box::new(create_number_expr(1.0))],
    quasis: vec![
      TplElement {
        span: DUMMY_SP,
        tail: false,
        cooked: Some("a".into()),
        raw: "a".into(),
      },
      TplElement {
        span: DUMMY_SP,
        tail: true,
        cooked: Some("b".into()),
        raw: "b".into(),
      },
    ],
  };
  assert!(convert_tpl_to_string_lit(&tpl).is_none());
}

#[test]
fn convert_simple_tpl_to_str_expr_converts() {
  let tpl = Expr::Tpl(Tpl {
    span: DUMMY_SP,
    exprs: vec![],
    quasis: vec![TplElement {
      span: DUMMY_SP,
      tail: true,
      cooked: Some("test".into()),
      raw: "test".into(),
    }],
  });
  let result = convert_simple_tpl_to_str_expr(tpl);
  assert!(matches!(result, Expr::Lit(Lit::Str(_))));
}

#[test]
fn convert_simple_tpl_to_str_expr_passthrough_non_tpl() {
  let expr = create_number_expr(5.0);
  let result = convert_simple_tpl_to_str_expr(expr.clone());
  assert!(matches!(result, Expr::Lit(Lit::Num(_))));
}

#[test]
fn convert_concat_to_tpl_expr_passthrough_non_call() {
  let expr = create_number_expr(1.0);
  let result = convert_concat_to_tpl_expr(expr);
  assert!(matches!(result, Expr::Lit(Lit::Num(_))));
}

#[test]
fn convert_string_to_prop_name_simple_ident() {
  let result = convert_string_to_prop_name("color");
  assert!(matches!(result, PropName::Ident(_)));
}

#[test]
fn convert_string_to_prop_name_needs_quoting() {
  let result = convert_string_to_prop_name("background-color");
  assert!(matches!(result, PropName::Str(_)));
}

#[test]
fn convert_atom_to_string_valid() {
  let atom: Wtf8Atom = "hello".into();
  assert_eq!(convert_atom_to_string(&atom), "hello");
}

#[test]
fn convert_wtf8_to_atom_valid() {
  let atom: Wtf8Atom = "test".into();
  let result = convert_wtf8_to_atom(&atom);
  assert_eq!(result.as_ref(), "test");
}

#[test]
fn convert_str_lit_to_string_valid() {
  let s = Str {
    span: DUMMY_SP,
    value: "abc".into(),
    raw: None,
  };
  assert_eq!(convert_str_lit_to_string(&s), "abc");
}

#[test]
fn convert_str_lit_to_atom_valid() {
  let s = Str {
    span: DUMMY_SP,
    value: "xyz".into(),
    raw: None,
  };
  let atom = convert_str_lit_to_atom(&s);
  assert_eq!(atom.as_ref(), "xyz");
}

#[test]
fn extract_tpl_cooked_value_valid() {
  let elem = TplElement {
    span: DUMMY_SP,
    tail: true,
    cooked: Some("cooked".into()),
    raw: "cooked".into(),
  };
  assert_eq!(extract_tpl_cooked_value(&elem), "cooked");
}

#[test]
fn convert_atom_to_str_ref_valid() {
  let atom: Wtf8Atom = "ref".into();
  assert_eq!(convert_atom_to_str_ref(&atom), "ref");
}

#[test]
fn convert_lit_to_string_str() {
  let lit = create_string_lit("foo");
  assert_eq!(convert_lit_to_string(&lit), Some("foo".to_string()));
}

#[test]
fn convert_lit_to_string_num() {
  let lit = create_number_lit(42.0);
  assert_eq!(convert_lit_to_string(&lit), Some("42".to_string()));
}

#[test]
fn convert_lit_to_string_bigint() {
  let big = BigInt {
    span: DUMMY_SP,
    value: Box::new(99i64.into()),
    raw: None,
  };
  let lit = Lit::BigInt(big);
  assert_eq!(convert_lit_to_string(&lit), Some("99".to_string()));
}

#[test]
fn convert_lit_to_string_null_returns_none() {
  let lit = Lit::Null(swc_core::ecma::ast::Null { span: DUMMY_SP });
  assert!(convert_lit_to_string(&lit).is_none());
}

#[test]
fn extract_str_lit_ref_str() {
  let lit = create_string_lit("bar");
  assert_eq!(extract_str_lit_ref(&lit), Some("bar"));
}

#[test]
fn extract_str_lit_ref_num_returns_none() {
  let lit = create_number_lit(1.0);
  assert!(extract_str_lit_ref(&lit).is_none());
}

#[test]
fn create_number_expr_produces_num_lit() {
  let expr = create_number_expr(std::f64::consts::PI);
  match expr {
    Expr::Lit(Lit::Num(n)) => assert!((n.value - std::f64::consts::PI).abs() < f64::EPSILON),
    _ => panic!("Expected Num lit"),
  }
}

#[test]
fn create_string_expr_produces_str_lit() {
  let expr = create_string_expr("hello");
  match expr {
    Expr::Lit(Lit::Str(s)) => assert_eq!(s.value.as_str().unwrap(), "hello"),
    _ => panic!("Expected Str lit"),
  }
}

#[test]
fn create_bool_expr_produces_bool_lit() {
  assert!(matches!(
    create_bool_expr(true),
    Expr::Lit(Lit::Bool(Bool { value: true, .. }))
  ));
  assert!(matches!(
    create_bool_expr(false),
    Expr::Lit(Lit::Bool(Bool { value: false, .. }))
  ));
}

#[test]
fn create_ident_expr_produces_ident() {
  let expr = create_ident_expr("myVar");
  assert!(matches!(expr, Expr::Ident(_)));
}

#[test]
fn create_null_expr_produces_null() {
  assert!(matches!(create_null_expr(), Expr::Lit(Lit::Null(_))));
}

#[test]
fn expand_shorthand_prop_converts() {
  let ident = Ident::from("x");
  let mut prop = Box::new(Prop::Shorthand(ident));
  expand_shorthand_prop(&mut prop);
  assert!(matches!(*prop, Prop::KeyValue(_)));
}

#[test]
fn expand_shorthand_prop_noop_for_kv() {
  let kv = Prop::from(KeyValueProp {
    key: PropName::Ident(swc_core::ecma::ast::IdentName::new("a".into(), DUMMY_SP)),
    value: Box::new(create_number_expr(1.0)),
  });
  let mut prop = Box::new(kv);
  expand_shorthand_prop(&mut prop);
  assert!(matches!(*prop, Prop::KeyValue(_)));
}

#[test]
fn concat_call_to_template_literal_non_concat_returns_none() {
  use swc_core::{
    common::SyntaxContext,
    ecma::ast::{Callee, MemberExpr, MemberProp},
  };

  let call = CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(create_string_expr("base")),
      prop: MemberProp::Ident(swc_core::ecma::ast::IdentName::new(
        "slice".into(),
        DUMMY_SP,
      )),
    }))),
    args: vec![],
    type_args: None,
    ctxt: SyntaxContext::empty(),
  };
  assert!(concat_call_to_template_literal(&call).is_none());
}

#[test]
fn convert_concat_to_tpl_expr_converts_concat_call() {
  use swc_core::{
    common::SyntaxContext,
    ecma::ast::{Callee, MemberExpr, MemberProp},
  };

  let call = CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(create_string_expr("base")),
      prop: MemberProp::Ident(swc_core::ecma::ast::IdentName::new(
        "concat".into(),
        DUMMY_SP,
      )),
    }))),
    args: vec![ExprOrSpread {
      spread: None,
      expr: Box::new(create_string_expr("tail")),
    }],
    type_args: None,
    ctxt: SyntaxContext::empty(),
  };

  let result = convert_concat_to_tpl_expr(Expr::Call(call));
  assert!(matches!(result, Expr::Tpl(_)));
}

#[test]
fn create_big_int_expr_produces_big_int_lit() {
  let big = BigInt {
    span: DUMMY_SP,
    value: Box::new(42i64.into()),
    raw: None,
  };
  assert!(matches!(
    create_big_int_expr(big),
    Expr::Lit(Lit::BigInt(_))
  ));
}
