use swc_core::{
  common::{BytePos, DUMMY_SP, Span},
  ecma::ast::{BindingIdent, Expr, Ident, Lit, Number, ParenExpr, Pat, Str, VarDeclarator},
};

use stylex_ast::ast::convertors::{get_expr_from_var_decl, normalize_expr};

// ---------------------------------------------------------------------------
// get_expr_from_var_decl
// ---------------------------------------------------------------------------
mod get_expr_from_var_decl_tests {
  use super::*;

  fn make_var_declarator(init: Option<Box<Expr>>) -> VarDeclarator {
    VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(BindingIdent {
        id: Ident::new("x".into(), DUMMY_SP, Default::default()),
        type_ann: None,
      }),
      init,
      definite: false,
    }
  }

  #[test]
  fn returns_init_expression_number() {
    let decl = make_var_declarator(Some(Box::new(Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value: 42.0,
      raw: None,
    })))));

    let result = get_expr_from_var_decl(&decl);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
      _ => panic!("Expected numeric literal"),
    }
  }

  #[test]
  fn returns_init_expression_string() {
    let decl = make_var_declarator(Some(Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "hello".into(),
      raw: None,
    })))));

    let result = get_expr_from_var_decl(&decl);

    match result {
      Expr::Lit(Lit::Str(s)) => assert_eq!(&*s.value, "hello"),
      _ => panic!("Expected string literal"),
    }
  }

  #[test]
  #[should_panic(expected = "Variable declaration must be initialized")]
  fn panics_when_no_init() {
    let decl = make_var_declarator(None);
    get_expr_from_var_decl(&decl);
  }
}

// ---------------------------------------------------------------------------
// normalize_expr
// ---------------------------------------------------------------------------
mod normalize_expr_tests {
  use super::*;

  fn make_num_expr(value: f64) -> Expr {
    Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value,
      raw: None,
    }))
  }

  fn wrap_in_paren(expr: Expr) -> Expr {
    Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(expr),
    })
  }

  #[test]
  fn non_paren_expression_returned_as_is() {
    let expr = make_num_expr(42.0);
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
      _ => panic!("Expected numeric literal"),
    }
  }

  #[test]
  fn parenthesized_expression_is_unwrapped() {
    let inner = make_num_expr(99.0);
    let expr = wrap_in_paren(inner);
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 99.0),
      _ => panic!("Expected numeric literal after unwrapping paren"),
    }
  }

  #[test]
  fn nested_parens_unwrapped_recursively() {
    let inner = make_num_expr(7.0);
    let expr = wrap_in_paren(wrap_in_paren(wrap_in_paren(inner)));
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 7.0),
      _ => panic!("Expected numeric literal after unwrapping nested parens"),
    }
  }

  #[test]
  fn span_is_preserved_for_non_paren() {
    let original_span = Span::new(BytePos(10), BytePos(20));
    let expr = Expr::Lit(Lit::Num(Number {
      span: original_span,
      value: 1.0,
      raw: None,
    }));
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Num(n)) => {
        assert_eq!(
          n.span, original_span,
          "normalize_expr must preserve the span of a non-paren expression"
        );
      },
      _ => panic!("Expected numeric literal"),
    }
  }

  #[test]
  fn string_literal_passes_through() {
    let expr = Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "test".into(),
      raw: None,
    }));
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Str(s)) => assert_eq!(&*s.value, "test"),
      _ => panic!("Expected string literal"),
    }
  }

  #[test]
  fn paren_wrapping_string_unwrapped() {
    let inner = Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "wrapped".into(),
      raw: None,
    }));
    let expr = wrap_in_paren(inner);
    let result = normalize_expr(&expr);

    match result {
      Expr::Lit(Lit::Str(s)) => assert_eq!(&*s.value, "wrapped"),
      _ => panic!("Expected string literal after unwrapping paren"),
    }
  }
}
