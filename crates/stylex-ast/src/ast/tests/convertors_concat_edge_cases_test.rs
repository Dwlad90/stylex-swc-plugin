//! Edge-case coverage for concat/template conversion helpers.

use crate::ast::convertors::{
  concat_call_to_template_literal, convert_concat_to_tpl_expr, convert_simple_tpl_to_str_expr,
  create_number_expr, create_string_expr,
};
use swc_core::common::{DUMMY_SP, SyntaxContext};
use swc_core::ecma::ast::{
  CallExpr, Callee, Expr, ExprOrSpread, MemberExpr, MemberProp, Tpl, TplElement,
};

/// Interpolated template literals should be returned unchanged.
#[test]
fn convert_simple_tpl_to_str_expr_keeps_interpolated_templates() {
  let tpl_expr = Expr::Tpl(Tpl {
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
  });

  let result = convert_simple_tpl_to_str_expr(tpl_expr);
  assert!(matches!(result, Expr::Tpl(_)));
}

/// Non-concat call expressions should pass through without transformation.
#[test]
fn convert_concat_to_tpl_expr_keeps_non_concat_calls() {
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

  let result = convert_concat_to_tpl_expr(Expr::Call(call));
  assert!(matches!(result, Expr::Call(_)));
}

/// Spread arguments are intentionally ignored during concat -> template conversion.
#[test]
fn concat_call_to_template_literal_skips_spread_arguments() {
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
    args: vec![
      ExprOrSpread {
        spread: Some(DUMMY_SP),
        expr: Box::new(create_number_expr(1.0)),
      },
      ExprOrSpread {
        spread: None,
        expr: Box::new(create_string_expr("tail")),
      },
    ],
    type_args: None,
    ctxt: SyntaxContext::empty(),
  };

  let result = concat_call_to_template_literal(&call);
  let Expr::Tpl(tpl) = result.expect("concat call should convert") else {
    unreachable!();
  };

  assert_eq!(tpl.exprs.len(), 1);
}
