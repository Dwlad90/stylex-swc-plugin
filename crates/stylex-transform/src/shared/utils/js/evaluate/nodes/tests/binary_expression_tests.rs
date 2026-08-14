use super::*;
use stylex_ast::ast::convertors::create_ident_expr;

#[test]
fn test_binary_expr_to_num_arithmetic() {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();
  let left = Box::new(create_number_expr(10.0));
  let right = Box::new(create_number_expr(2.0));
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
  let left = Box::new(create_number_expr(10.0));
  let right = Box::new(create_number_expr(2.0));
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
  let left = Box::new(create_number_expr(6.0));
  let right = Box::new(create_number_expr(3.0));
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
fn test_binary_expr_to_string_add() {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();
  let left = Box::new(create_string_expr("foo"));
  let right = Box::new(create_string_expr("bar"));
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
fn test_binary_expr_to_num_in_operator() {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();
  let left = Box::new(create_number_expr(10.0));
  let right_zero = Box::new(create_number_expr(0.0));
  let right_non_zero = Box::new(create_number_expr(1.0));

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
  let left = Box::new(create_number_expr(10.0));
  let right_zero = Box::new(create_number_expr(0.0));
  let right_non_zero = Box::new(create_number_expr(2.0));

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
  let left = Box::new(create_string_expr("foo"));
  let right = Box::new(create_string_expr("bar"));
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
  let left = Box::new(create_ident_expr("x"));
  let right = Box::new(create_number_expr(1.0));
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
fn test_binary_expr_to_string_right_unresolved_returns_null_on_add() {
  let mut state = EvaluationState::new();
  // Force non-confident path on unresolved right
  state.confident = false;
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();
  let left = Box::new(create_string_expr("foo"));
  let right = Box::new(create_ident_expr("bar"));
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

// ──────────────────────────────────────────────
// binary_expr_to_num - comparison operators
// ──────────────────────────────────────────────

mod binary_expr_to_num_comparison_tests {
  use super::*;

  fn eval_bin(op: BinaryOp, left: f64, right: f64) -> f64 {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op,
      left: Box::new(create_number_expr(left)),
      right: Box::new(create_number_expr(right)),
    };
    match binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).unwrap() {
      BinaryExprType::Number(n) => n,
      _ => panic!("Expected number"),
    }
  }

  #[test]
  fn eqeq_equal_returns_1() {
    assert_eq!(eval_bin(BinaryOp::EqEq, 5.0, 5.0), 1.0);
  }

  #[test]
  fn eqeq_not_equal_returns_0() {
    assert_eq!(eval_bin(BinaryOp::EqEq, 5.0, 3.0), 0.0);
  }

  #[test]
  fn noteq_different_returns_1() {
    assert_eq!(eval_bin(BinaryOp::NotEq, 5.0, 3.0), 1.0);
  }

  #[test]
  fn noteq_same_returns_0() {
    assert_eq!(eval_bin(BinaryOp::NotEq, 5.0, 5.0), 0.0);
  }

  #[test]
  fn eqeqeq_equal_returns_1() {
    assert_eq!(eval_bin(BinaryOp::EqEqEq, 5.0, 5.0), 1.0);
  }

  #[test]
  fn eqeqeq_not_equal_returns_0() {
    assert_eq!(eval_bin(BinaryOp::EqEqEq, 5.0, 3.0), 0.0);
  }

  #[test]
  fn noteqeq_different_returns_1() {
    assert_eq!(eval_bin(BinaryOp::NotEqEq, 5.0, 3.0), 1.0);
  }

  #[test]
  fn noteqeq_same_returns_0() {
    assert_eq!(eval_bin(BinaryOp::NotEqEq, 5.0, 5.0), 0.0);
  }

  #[test]
  fn gt_greater_returns_1() {
    assert_eq!(eval_bin(BinaryOp::Gt, 5.0, 3.0), 1.0);
  }

  #[test]
  fn gt_not_greater_returns_0() {
    assert_eq!(eval_bin(BinaryOp::Gt, 3.0, 5.0), 0.0);
  }

  #[test]
  fn gteq_equal_returns_1() {
    assert_eq!(eval_bin(BinaryOp::GtEq, 5.0, 5.0), 1.0);
  }

  #[test]
  fn gteq_less_returns_0() {
    assert_eq!(eval_bin(BinaryOp::GtEq, 3.0, 5.0), 0.0);
  }

  #[test]
  fn lt_less_returns_1() {
    assert_eq!(eval_bin(BinaryOp::Lt, 3.0, 5.0), 1.0);
  }

  #[test]
  fn lt_not_less_returns_0() {
    assert_eq!(eval_bin(BinaryOp::Lt, 5.0, 3.0), 0.0);
  }

  #[test]
  fn lteq_equal_returns_1() {
    assert_eq!(eval_bin(BinaryOp::LtEq, 5.0, 5.0), 1.0);
  }

  #[test]
  fn lteq_greater_returns_0() {
    assert_eq!(eval_bin(BinaryOp::LtEq, 5.0, 3.0), 0.0);
  }

  #[test]
  fn in_zero_right_returns_1() {
    assert_eq!(eval_bin(BinaryOp::In, 5.0, 0.0), 1.0);
  }

  #[test]
  fn in_nonzero_right_returns_0() {
    assert_eq!(eval_bin(BinaryOp::In, 5.0, 1.0), 0.0);
  }

  #[test]
  fn instanceof_zero_right_returns_1() {
    assert_eq!(eval_bin(BinaryOp::InstanceOf, 5.0, 0.0), 1.0);
  }

  #[test]
  fn instanceof_nonzero_right_returns_0() {
    assert_eq!(eval_bin(BinaryOp::InstanceOf, 5.0, 1.0), 0.0);
  }

  #[test]
  fn modulo_returns_remainder() {
    assert_eq!(eval_bin(BinaryOp::Mod, 10.0, 3.0), 1.0);
  }

  #[test]
  fn exp_returns_power() {
    assert_eq!(eval_bin(BinaryOp::Exp, 2.0, 3.0), 8.0);
  }

  #[test]
  fn zero_fill_rshift() {
    assert_eq!(eval_bin(BinaryOp::ZeroFillRShift, 8.0, 1.0), 4.0);
  }

  /// The three logical operators never reach this path — the node dispatches
  /// them elsewhere — and it refuses rather than coercing them to a number.
  #[test]
  fn logical_operators_are_refused() {
    for op in [
      BinaryOp::LogicalOr,
      BinaryOp::LogicalAnd,
      BinaryOp::NullishCoalescing,
    ] {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let bin = BinExpr {
        span: Default::default(),
        op,
        left: Box::new(create_number_expr(5.0)),
        right: Box::new(create_number_expr(3.0)),
      };

      assert!(
        binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns).is_err(),
        "expected {:?} to be refused by the number path",
        op
      );
    }
  }
}

// ──────────────────────────────────────────────
// binary_expr_to_string - non-Add operators
// ──────────────────────────────────────────────

mod binary_expr_to_string_non_add_tests {
  use super::*;

  /// Only `+` has a string result. Every other operator arrives here having
  /// already been refused by the number path, and is refused again so the
  /// caller deopts — rather than failing the build over an expression the
  /// language reads as a value.
  #[test]
  fn refuses_a_sub_op() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Sub,
      left: Box::new(create_string_expr("hello")),
      right: Box::new(create_string_expr("world")),
    };

    assert!(binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns).is_err());
  }

  #[test]
  fn add_two_strings_returns_concat() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(create_string_expr("hello")),
      right: Box::new(create_string_expr(" world")),
    };
    let result = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    assert!(result.is_ok());
    match result.unwrap() {
      BinaryExprType::String(s) => assert_eq!(s, "hello world"),
      _ => panic!("Expected string result"),
    }
  }

  #[test]
  fn add_num_left_with_string_right_concat() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(create_number_expr(42.0)),
      right: Box::new(create_string_expr("world")),
    };
    let result = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    match result.unwrap() {
      BinaryExprType::String(s) => assert_eq!(s, "42world"),
      _ => panic!("Expected string result"),
    }
  }

  #[test]
  fn add_string_left_num_right_returns_concat() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(create_string_expr("hello")),
      right: Box::new(create_number_expr(42.0)),
    };
    let result = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    match result.unwrap() {
      BinaryExprType::String(s) => assert_eq!(s, "hello42"),
      _ => panic!("Expected string result"),
    }
  }
}

// ──────────────────────────────────────────────
// binary_expr_to_num - with non-number result from bin
// ──────────────────────────────────────────────

mod binary_expr_to_num_error_tests {
  use super::*;

  #[test]
  fn str_operand_returns_error() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Sub,
      left: Box::new(create_string_expr("hello")),
      right: Box::new(create_number_expr(5.0)),
    };
    let result = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns);
    // Expect error since string can't be converted to number
    assert!(result.is_err());
  }
}
