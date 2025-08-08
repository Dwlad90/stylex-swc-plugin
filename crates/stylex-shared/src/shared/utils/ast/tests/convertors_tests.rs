#[cfg(test)]
mod tests {
  use crate::shared::structures::functions::FunctionMap;
  use crate::shared::structures::state::EvaluationState;
  use crate::shared::structures::state_manager::StateManager;
  use crate::shared::{
    enums::misc::BinaryExprType,
    utils::ast::convertors::{binary_expr_to_num, binary_expr_to_string, string_to_prop_name},
  };
  use swc_core::{
    common::SyntaxContext,
    ecma::ast::{BinExpr, BinaryOp, Expr, Ident, Lit, Str},
  };

  fn make_num_expr(val: f64) -> Expr {
    Expr::Lit(Lit::Num(swc_core::ecma::ast::Number {
      value: val,
      span: Default::default(),
      raw: None,
    }))
  }
  fn make_str_expr(val: &str) -> Expr {
    Expr::Lit(Lit::Str(Str {
      value: val.into(),
      span: Default::default(),
      raw: None,
    }))
  }
  fn make_ident_expr(name: &str) -> Expr {
    Expr::Ident(Ident {
      span: Default::default(),
      sym: name.into(),
      optional: false,
      ctxt: SyntaxContext::empty(),
    })
  }

  #[test]
  fn string_to_prop_name_with_quotes() {
    let keys_with_quotes = vec!["2ip", "123", "1b3", "1bc", "2xl", "x*x", "x-x", "x,x"];

    for key in keys_with_quotes {
      assert!(
        string_to_prop_name(key).unwrap().is_str(),
        "Key '{}' should be wrapped in quotes",
        key
      );
    }
  }

  #[test]
  fn string_to_prop_name_without_quotes() {
    let keys_without_quotes = vec![
      "_abc_",
      "_ABC_",
      "$123AB",
      "$abc_",
      "$abc$",
      "$ABC$",
      "$ABC123",
      "abc_",
      "abc",
      "ABC",
      "abc$",
      "break",
      "case",
      "catch",
      "class",
      "const",
      "continue",
      "debugger",
      "default",
      "delete",
      "do",
      "else",
      "export",
      "extends",
      "false",
      "finally",
      "for",
      "function",
      "if",
      "import",
      "in",
      "instanceof",
      "new",
      "null",
      "return",
      "super",
      "switch",
      "this",
      "throw",
      "true",
      "try",
      "typeof",
      "var",
      "void",
      "while",
      "with",
      "x_x",
      "x$x",
      "xl",
    ];

    for key in keys_without_quotes {
      assert!(
        string_to_prop_name(key).unwrap().is_ident(),
        "Key '{}' should not be wrapped in quotes",
        key
      );
    }
  }

  #[test]
  fn test_binary_expr_to_num_arithmetic() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(10.0));
    let right = Box::new(make_num_expr(2.0));
    let ops = [
      BinaryOp::Add,
      BinaryOp::Sub,
      BinaryOp::Mul,
      BinaryOp::Div,
      BinaryOp::Mod,
      BinaryOp::Exp,
    ];
    let expected = [12.0, 8.0, 20.0, 5.0, 0.0, 100.0];
    for (op, exp) in ops.iter().zip(expected.iter()) {
      let bin = BinExpr {
        op: *op,
        left: left.clone(),
        right: right.clone(),
        span: Default::default(),
      };
      let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).unwrap();
      match res {
        BinaryExprType::Number(n) => assert_eq!(n, *exp),
        _ => panic!("Expected number result"),
      }
    }
  }

  #[test]
  fn test_binary_expr_to_num_comparison() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(10.0));
    let right = Box::new(make_num_expr(2.0));
    let cases = [
      (BinaryOp::Lt, 0.0),
      (BinaryOp::LtEq, 0.0),
      (BinaryOp::Gt, 1.0),
      (BinaryOp::GtEq, 1.0),
      (BinaryOp::EqEq, 0.0),
      (BinaryOp::NotEq, 1.0),
      (BinaryOp::EqEqEq, 0.0),
      (BinaryOp::NotEqEq, 1.0),
    ];
    for (op, exp) in cases.iter() {
      let bin = BinExpr {
        op: *op,
        left: left.clone(),
        right: right.clone(),
        span: Default::default(),
      };
      let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).unwrap();
      match res {
        BinaryExprType::Number(n) => assert_eq!(n, *exp),
        _ => panic!("Expected number result"),
      }
    }
  }

  #[test]
  fn test_binary_expr_to_num_bitwise() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(6.0));
    let right = Box::new(make_num_expr(3.0));
    let cases = [
      (BinaryOp::BitAnd, 2.0),
      (BinaryOp::BitOr, 7.0),
      (BinaryOp::BitXor, 5.0),
      (BinaryOp::RShift, 0.0),
      (BinaryOp::LShift, 48.0),
      (BinaryOp::ZeroFillRShift, 0.0),
    ];
    for (op, exp) in cases.iter() {
      let bin = BinExpr {
        op: *op,
        left: left.clone(),
        right: right.clone(),
        span: Default::default(),
      };
      let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).unwrap();
      match res {
        BinaryExprType::Number(n) => assert_eq!(n, *exp),
        _ => panic!("Expected number result"),
      }
    }
  }

  #[test]
  fn test_binary_expr_to_num_logical() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(0.0));
    let right = Box::new(make_num_expr(5.0));
    let bin_or = BinExpr {
      op: BinaryOp::LogicalOr,
      left: left.clone(),
      right: right.clone(),
      span: Default::default(),
    };
    let res_or = binary_expr_to_num(&bin_or, &mut state, &mut traversal_state, &fns).unwrap();
    match res_or {
      BinaryExprType::Number(n) => assert_eq!(n, 5.0),
      _ => panic!("Expected number result"),
    }
    let left = Box::new(make_num_expr(2.0));
    let right = Box::new(make_num_expr(0.0));
    let bin_and = BinExpr {
      op: BinaryOp::LogicalAnd,
      left: left.clone(),
      right: right.clone(),
      span: Default::default(),
    };
    let res_and = binary_expr_to_num(&bin_and, &mut state, &mut traversal_state, &fns).unwrap();
    match res_and {
      BinaryExprType::Number(n) => assert_eq!(n, 0.0),
      _ => panic!("Expected number result"),
    }
    let left = Box::new(make_num_expr(0.0));
    let right = Box::new(make_num_expr(7.0));
    let bin_nullish = BinExpr {
      op: BinaryOp::NullishCoalescing,
      left: left.clone(),
      right: right.clone(),
      span: Default::default(),
    };
    let res_nullish =
      binary_expr_to_num(&bin_nullish, &mut state, &mut traversal_state, &fns).unwrap();
    match res_nullish {
      BinaryExprType::Number(n) => assert_eq!(n, 7.0),
      _ => panic!("Expected number result"),
    }
  }

  #[test]
  fn test_binary_expr_to_string_add() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_str_expr("foo"));
    let right = Box::new(make_str_expr("bar"));
    let bin = BinExpr {
      op: BinaryOp::Add,
      left,
      right,
      span: Default::default(),
    };
    let res = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns).unwrap();
    match res {
      BinaryExprType::String(s) => assert_eq!(s, "foobar"),
      _ => panic!("Expected string result"),
    }
  }

  #[test]
  #[should_panic]
  fn test_binary_expr_to_string_non_add() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_str_expr("foo"));
    let right = Box::new(make_str_expr("bar"));
    let bin = BinExpr {
      op: BinaryOp::Sub,
      left,
      right,
      span: Default::default(),
    };
    let _ = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
  }

  #[test]
  fn test_binary_expr_to_num_in_operator() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(10.0));
    let right_zero = Box::new(make_num_expr(0.0));
    let right_non_zero = Box::new(make_num_expr(1.0));

    let bin_zero = BinExpr {
      op: BinaryOp::In,
      left: left.clone(),
      right: right_zero,
      span: Default::default(),
    };
    let res_zero = binary_expr_to_num(&bin_zero, &mut state, &mut traversal_state, &fns).unwrap();
    match res_zero {
      BinaryExprType::Number(n) => assert_eq!(n, 1.0),
      _ => panic!("Expected number result"),
    }

    let bin_non_zero = BinExpr {
      op: BinaryOp::In,
      left,
      right: right_non_zero,
      span: Default::default(),
    };
    let res_non_zero =
      binary_expr_to_num(&bin_non_zero, &mut state, &mut traversal_state, &fns).unwrap();
    match res_non_zero {
      BinaryExprType::Number(n) => assert_eq!(n, 0.0),
      _ => panic!("Expected number result"),
    }
  }

  #[test]
  fn test_binary_expr_to_num_instanceof_operator() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(10.0));
    let right_zero = Box::new(make_num_expr(0.0));
    let right_non_zero = Box::new(make_num_expr(2.0));

    let bin_zero = BinExpr {
      op: BinaryOp::InstanceOf,
      left: left.clone(),
      right: right_zero,
      span: Default::default(),
    };
    let res_zero = binary_expr_to_num(&bin_zero, &mut state, &mut traversal_state, &fns).unwrap();
    match res_zero {
      BinaryExprType::Number(n) => assert_eq!(n, 1.0),
      _ => panic!("Expected number result"),
    }

    let bin_non_zero = BinExpr {
      op: BinaryOp::InstanceOf,
      left,
      right: right_non_zero,
      span: Default::default(),
    };
    let res_non_zero =
      binary_expr_to_num(&bin_non_zero, &mut state, &mut traversal_state, &fns).unwrap();
    match res_non_zero {
      BinaryExprType::Number(n) => assert_eq!(n, 0.0),
      _ => panic!("Expected number result"),
    }
  }

  #[test]
  fn test_binary_expr_add_strings_returns_string() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_str_expr("foo"));
    let right = Box::new(make_str_expr("bar"));
    let bin = BinExpr {
      op: BinaryOp::Add,
      left,
      right,
      span: Default::default(),
    };
    let res = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns).unwrap();
    match res {
      BinaryExprType::String(s) => assert_eq!(s, "foobar"),
      _ => panic!("Expected string result from string addition in num evaluator"),
    }
  }

  #[test]
  fn test_binary_expr_to_num_left_unresolved_returns_err() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_ident_expr("x"));
    let right = Box::new(make_num_expr(1.0));
    let bin = BinExpr {
      op: BinaryOp::Add,
      left,
      right,
      span: Default::default(),
    };
    let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns);
    assert!(
      res.is_err(),
      "Expected error when left side is unresolved and state is not confident"
    );
  }

  #[test]
  fn test_binary_expr_to_num_logical_or_with_unresolved_right_returns_left() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_num_expr(3.0));
    let right = Box::new(make_ident_expr("unknown"));
    let bin = BinExpr {
      op: BinaryOp::LogicalOr,
      left,
      right: right.clone(),
      span: Default::default(),
    };
    let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).unwrap();

    match res {
      BinaryExprType::Number(n) => assert_eq!(n, 3.0),
      _ => panic!(
        "Expected number result equal to left operand when right is unresolved for LogicalOr"
      ),
    }

    let left = Box::new(make_num_expr(0.0));

    let bin = BinExpr {
      op: BinaryOp::LogicalOr,
      left,
      right,
      span: Default::default(),
    };

    let res = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns);

    assert!(
      res.is_err(),
      "Expected error when left side is unresolved and state is not confident"
    );
  }

  #[test]
  fn test_binary_expr_to_string_right_unresolved_returns_null_on_add() {
    let mut state = EvaluationState::new();
    // Force non-confident path on unresolved right
    state.confident = false;
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_str_expr("foo"));
    let right = Box::new(make_ident_expr("bar"));
    let bin = BinExpr {
      op: BinaryOp::Add,
      left,
      right,
      span: Default::default(),
    };
    let res = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    assert!(
      res.is_err(),
      "Expected error when right side is unresolved and op is Add in string evaluator"
    );
  }

  #[test]
  fn test_binary_expr_to_string_right_unresolved_logical_or_returns_left() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let left = Box::new(make_str_expr("foo"));
    let right = Box::new(make_ident_expr("baz"));
    let bin = BinExpr {
      op: BinaryOp::LogicalOr,
      left,
      right,
      span: Default::default(),
    };
    let res = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns).unwrap();
    match res {
      BinaryExprType::String(s) => assert_eq!(s, "foo"),
      _ => panic!("Expected left string when right is unresolved and op is LogicalOr"),
    }
  }
}
