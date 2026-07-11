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
  let result = convert_simple_tpl_to_str_expr(expr);
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

fn ident_member_prop(name: &str) -> MemberProp {
  MemberProp::Ident(create_ident_name(name))
}

fn computed_str_member_prop(value: &str) -> MemberProp {
  create_computed_member_prop(create_string_expr(value))
}

fn computed_num_member_prop(value: f64, raw: Option<&str>) -> MemberProp {
  create_computed_member_prop(Expr::Lit(Lit::Num(Number {
    span: DUMMY_SP,
    value,
    raw: raw.map(|raw| raw.into()),
  })))
}

fn computed_bigint_member_prop(value: i64) -> MemberProp {
  create_computed_member_prop(Expr::Lit(Lit::BigInt(BigInt {
    span: DUMMY_SP,
    value: Box::new(value.into()),
    raw: None,
  })))
}

fn computed_tpl_member_prop(value: &str) -> MemberProp {
  create_computed_member_prop(Expr::Tpl(Tpl {
    span: DUMMY_SP,
    exprs: vec![],
    quasis: vec![TplElement {
      span: DUMMY_SP,
      tail: true,
      cooked: Some(value.into()),
      raw: value.into(),
    }],
  }))
}

#[test]
fn convert_member_prop_to_string_ident_and_computed_str() {
  assert_eq!(
    convert_member_prop_to_string(&ident_member_prop("flex")).as_deref(),
    Some("flex")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_str_member_prop("calc(100% - 20px)")).as_deref(),
    Some("calc(100% - 20px)")
  );
}

#[test]
fn convert_member_prop_to_string_numeric_uses_parsed_value_not_raw() {
  // Renders from the parsed value, ignoring the raw source token.
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(16.0, Some("0x10"))).as_deref(),
    Some("16")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1.5, None)).as_deref(),
    Some("1.5")
  );
}

#[test]
fn convert_member_prop_to_string_numeric_zero() {
  // JS `String(0)` (and `String(-0)`) is "0"; the zero fast-path returns it
  // without going through exponential rendering.
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(0.0, None)).as_deref(),
    Some("0")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(-0.0, None)).as_deref(),
    Some("0")
  );
}

#[test]
fn convert_member_prop_to_string_bigint() {
  // A computed BigInt key (`obj[123n]`) renders as its decimal digits.
  assert_eq!(
    convert_member_prop_to_string(&computed_bigint_member_prop(123)).as_deref(),
    Some("123")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_bigint_member_prop(-45)).as_deref(),
    Some("-45")
  );
}

#[test]
fn convert_member_prop_to_string_numeric_js_edge_values() {
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(f64::NAN, None)).as_deref(),
    Some("NaN")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(f64::INFINITY, None)).as_deref(),
    Some("Infinity")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(f64::NEG_INFINITY, None)).as_deref(),
    Some("-Infinity")
  );
}

#[test]
fn convert_member_prop_to_string_numeric_renders_shortest_decimal() {
  // Rendered via Rust's shortest round-tripping `f64` `Display`: always a valid
  // JS literal, but plain decimal rather than JS's exponential spelling for
  // extreme magnitudes. Each output still parses back to the same `f64`.
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1e-7, None)).as_deref(),
    Some("0.0000001")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1.23e-7, None)).as_deref(),
    Some("0.000000123")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1e-6, None)).as_deref(),
    Some("0.000001")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1e20, None)).as_deref(),
    Some("100000000000000000000")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(1e21, None)).as_deref(),
    Some("1000000000000000000000")
  );
  assert_eq!(
    convert_member_prop_to_string(&computed_num_member_prop(-1e21, None)).as_deref(),
    Some("-1000000000000000000000")
  );
}

#[test]
fn convert_member_prop_to_string_unwraps_static_parens_and_templates() {
  let parenthesized_str = create_computed_member_prop(Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(create_string_expr("wrapped")),
  }));
  assert_eq!(
    convert_member_prop_to_string(&parenthesized_str).as_deref(),
    Some("wrapped")
  );

  let parenthesized_num = create_computed_member_prop(Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(create_number_expr(123.0)),
  }));
  assert_eq!(
    convert_member_prop_to_string(&parenthesized_num).as_deref(),
    Some("123")
  );

  assert_eq!(
    convert_member_prop_to_string(&computed_tpl_member_prop("template-key")).as_deref(),
    Some("template-key")
  );
}

#[test]
fn convert_member_prop_to_string_returns_none_for_non_literal_and_private() {
  let computed_ident = create_computed_member_prop(create_ident_expr("dynamic"));
  assert_eq!(convert_member_prop_to_string(&computed_ident), None);

  let private = MemberProp::PrivateName(PrivateName {
    span: DUMMY_SP,
    name: "secret".into(),
  });
  assert_eq!(convert_member_prop_to_string(&private), None);
}

#[test]
fn normalize_expr_unwraps_nested_parens() {
  let inner = create_string_expr("x");
  let expr = Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    })),
  });
  assert!(matches!(normalize_expr(&expr), Expr::Lit(Lit::Str(_))));
}

#[test]
fn normalize_expr_passthrough_non_paren() {
  let expr = create_string_expr("x");
  assert!(matches!(normalize_expr(&expr), Expr::Lit(Lit::Str(_))));
}

#[test]
fn normalize_expr_mut_allows_mutating_inner_expr() {
  let mut expr = Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(create_string_expr("before")),
  });
  *normalize_expr_mut(&mut expr) = create_string_expr("after");
  match &expr {
    Expr::Paren(paren) => match paren.expr.as_ref() {
      Expr::Lit(Lit::Str(s)) => assert_eq!(s.value.as_str(), Some("after")),
      other => panic!("Expected string literal inside paren, got {other:?}"),
    },
    other => panic!("Expected paren wrapper to remain, got {other:?}"),
  }
}

#[test]
fn normalize_expr_handles_deeply_nested_parens_without_recursion() {
  let mut expr = create_string_expr("before");
  for _ in 0..16_384 {
    expr = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(expr),
    });
  }

  assert!(matches!(normalize_expr(&expr), Expr::Lit(Lit::Str(_))));
  *normalize_expr_mut(&mut expr) = create_string_expr("after");

  // Consume the chain iteratively as well so test cleanup does not recurse.
  while let Expr::Paren(paren) = expr {
    expr = *paren.expr;
  }
  assert!(matches!(expr, Expr::Lit(Lit::Str(_))));
}

#[test]
fn get_expr_from_var_decl_returns_initializer() {
  let decl = create_var_declarator(create_ident("a"), create_string_expr("v"));
  assert!(matches!(
    get_expr_from_var_decl(&decl),
    Expr::Lit(Lit::Str(_))
  ));
}

#[test]
#[should_panic(expected = "Variable declaration must be initialized")]
fn get_expr_from_var_decl_panics_without_initializer() {
  let decl = VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(create_binding_ident(create_ident("a"))),
    init: None,
    definite: false,
  };
  get_expr_from_var_decl(&decl);
}
