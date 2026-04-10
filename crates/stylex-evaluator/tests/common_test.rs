use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    BinaryOp, BindingIdent, Expr, Ident, Lit, Number, ParenExpr, Pat, Str, VarDeclarator,
  },
};

use stylex_evaluator::common::{
  evaluate_bin_expr, get_expr_from_var_decl, normalize_expr, resolve_node_package_path,
};

// ---------------------------------------------------------------------------
// evaluate_bin_expr
// ---------------------------------------------------------------------------
mod evaluate_bin_expr_tests {
  use super::*;

  // --- Arithmetic operators ---

  #[test]
  fn addition() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 5.0, 3.0), 8.0);
  }

  #[test]
  fn subtraction() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, 10.0, 4.0), 6.0);
  }

  #[test]
  fn multiplication() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 3.0, 7.0), 21.0);
  }

  #[test]
  fn division() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Div, 15.0, 5.0), 3.0);
  }

  #[test]
  fn modulo() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mod, 17.0, 5.0), 2.0);
  }

  #[test]
  fn exponentiation() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 2.0, 10.0), 1024.0);
  }

  // --- Bitwise operators ---

  #[test]
  fn bitwise_or() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitOr, 5.0, 3.0), 7.0);
  }

  #[test]
  fn bitwise_and() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitAnd, 5.0, 3.0), 1.0);
  }

  #[test]
  fn bitwise_xor() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitXor, 5.0, 3.0), 6.0);
  }

  #[test]
  fn left_shift() {
    assert_eq!(evaluate_bin_expr(BinaryOp::LShift, 1.0, 4.0), 16.0);
  }

  #[test]
  fn right_shift() {
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, 16.0, 2.0), 4.0);
  }

  #[test]
  fn zero_fill_right_shift_positive() {
    assert_eq!(evaluate_bin_expr(BinaryOp::ZeroFillRShift, 16.0, 2.0), 4.0);
  }

  #[test]
  fn zero_fill_right_shift_negative() {
    // In Rust 2024, `-1.0f64 as u64` saturates to 0 (not wrapping like JS).
    // So `-1.0 >>> 0` evaluates to `0.0` rather than JS's `4294967295`.
    let result = evaluate_bin_expr(BinaryOp::ZeroFillRShift, -1.0, 0.0);
    assert_eq!(result, 0.0);
  }

  // --- Edge cases ---

  #[test]
  fn division_by_zero_yields_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, 1.0, 0.0);
    assert!(result.is_infinite() && result.is_sign_positive());
  }

  #[test]
  fn division_negative_by_zero_yields_neg_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, -1.0, 0.0);
    assert!(result.is_infinite() && result.is_sign_negative());
  }

  #[test]
  fn zero_divided_by_zero_is_nan() {
    let result = evaluate_bin_expr(BinaryOp::Div, 0.0, 0.0);
    assert!(result.is_nan());
  }

  #[test]
  fn modulo_by_zero_is_nan() {
    let result = evaluate_bin_expr(BinaryOp::Mod, 5.0, 0.0);
    assert!(result.is_nan());
  }

  #[test]
  fn addition_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, -3.0, -7.0), -10.0);
  }

  #[test]
  fn subtraction_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, -3.0, -7.0), 4.0);
  }

  #[test]
  fn multiplication_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, -3.0, 7.0), -21.0);
  }

  #[test]
  fn addition_with_zero() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 0.0, 0.0), 0.0);
  }

  #[test]
  fn multiplication_with_zero() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 42.0, 0.0), 0.0);
  }

  #[test]
  fn exponentiation_zero_power() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 5.0, 0.0), 1.0);
  }

  #[test]
  fn exponentiation_negative_exponent() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 2.0, -1.0), 0.5);
  }

  #[test]
  fn right_shift_negative_number() {
    // -16 >> 2 == -4 in two's complement
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, -16.0, 2.0), -4.0);
  }

  #[test]
  #[should_panic(expected = "Unsupported binary operator")]
  fn unsupported_operator_panics() {
    // EqEq is not handled by evaluate_bin_expr
    evaluate_bin_expr(BinaryOp::EqEq, 1.0, 1.0);
  }
}

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
    let mut expr = make_num_expr(42.0);
    let result = normalize_expr(&mut expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
      _ => panic!("Expected numeric literal"),
    }
  }

  #[test]
  fn parenthesized_expression_is_unwrapped() {
    let inner = make_num_expr(99.0);
    let mut expr = wrap_in_paren(inner);
    let result = normalize_expr(&mut expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 99.0),
      _ => panic!("Expected numeric literal after unwrapping paren"),
    }
  }

  #[test]
  fn nested_parens_unwrapped_recursively() {
    let inner = make_num_expr(7.0);
    let mut expr = wrap_in_paren(wrap_in_paren(wrap_in_paren(inner)));
    let result = normalize_expr(&mut expr);

    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 7.0),
      _ => panic!("Expected numeric literal after unwrapping nested parens"),
    }
  }

  #[test]
  fn span_is_dropped_for_non_paren() {
    let mut expr = make_num_expr(1.0);
    let result = normalize_expr(&mut expr);

    match result {
      Expr::Lit(Lit::Num(n)) => {
        assert_eq!(n.span, DUMMY_SP, "Span should be DUMMY_SP after drop_span");
      },
      _ => panic!("Expected numeric literal"),
    }
  }

  #[test]
  fn string_literal_passes_through() {
    let mut expr = Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "test".into(),
      raw: None,
    }));
    let result = normalize_expr(&mut expr);

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
    let mut expr = wrap_in_paren(inner);
    let result = normalize_expr(&mut expr);

    match result {
      Expr::Lit(Lit::Str(s)) => assert_eq!(&*s.value, "wrapped"),
      _ => panic!("Expected string literal after unwrapping paren"),
    }
  }
}

// ---------------------------------------------------------------------------
// resolve_node_package_path
// ---------------------------------------------------------------------------
mod resolve_node_package_path_tests {
  use super::*;

  #[test]
  fn nonexistent_package_returns_err() {
    let result = resolve_node_package_path("this-package-does-not-exist-abc123xyz");
    assert!(result.is_err());
  }

  #[test]
  fn empty_package_name_returns_err() {
    let result = resolve_node_package_path("");
    assert!(result.is_err());
  }

  #[test]
  fn error_message_contains_package_name() {
    let pkg = "nonexistent-pkg-xyz";
    let result = resolve_node_package_path(pkg);
    match result {
      Err(msg) => assert!(
        msg.contains(pkg),
        "Error message should contain the package name, got: {msg}"
      ),
      Ok(_) => panic!("Expected Err for nonexistent package"),
    }
  }

  #[test]
  fn scoped_nonexistent_package_returns_err() {
    let result = resolve_node_package_path("@nonexistent-scope/nonexistent-pkg");
    assert!(result.is_err());
  }
}
