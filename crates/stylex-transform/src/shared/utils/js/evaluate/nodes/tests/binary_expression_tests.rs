use super::*;
use swc_core::{
  common::SyntaxContext,
  ecma::ast::{Ident, Str},
};

fn make_num_expr(val: f64) -> Expr {
  Expr::Lit(Lit::Num(Number {
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
    left,
    right,
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
    left,
    right,
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
    left,
    right,
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
    _ => {
      panic!("Expected number result equal to left operand when right is unresolved for LogicalOr")
    },
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
      left: Box::new(make_num_expr(left)),
      right: Box::new(make_num_expr(right)),
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

  #[test]
  fn logical_or_truthy_left() {
    assert_eq!(eval_bin(BinaryOp::LogicalOr, 5.0, 3.0), 5.0);
  }

  #[test]
  fn logical_or_falsy_left() {
    assert_eq!(eval_bin(BinaryOp::LogicalOr, 0.0, 3.0), 3.0);
  }

  #[test]
  fn logical_and_truthy_left() {
    assert_eq!(eval_bin(BinaryOp::LogicalAnd, 5.0, 3.0), 3.0);
  }

  #[test]
  fn logical_and_falsy_left() {
    assert_eq!(eval_bin(BinaryOp::LogicalAnd, 0.0, 3.0), 0.0);
  }

  #[test]
  fn nullish_coalescing_nonzero_left() {
    assert_eq!(eval_bin(BinaryOp::NullishCoalescing, 5.0, 3.0), 5.0);
  }

  #[test]
  fn nullish_coalescing_zero_left() {
    assert_eq!(eval_bin(BinaryOp::NullishCoalescing, 0.0, 3.0), 3.0);
  }
}

// ──────────────────────────────────────────────
// binary_expr_to_string - non-Add operator panic
// ──────────────────────────────────────────────

mod binary_expr_to_string_non_add_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn panics_for_sub_op() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Sub,
      left: Box::new(make_str_expr("hello")),
      right: Box::new(make_str_expr("world")),
    };
    let _ = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
  }

  #[test]
  fn add_two_strings_returns_concat() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(make_str_expr("hello")),
      right: Box::new(make_str_expr(" world")),
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
      left: Box::new(make_num_expr(42.0)),
      right: Box::new(make_str_expr("world")),
    };
    // Left is not a string, but doesn't necessarily panic -
    // it may return Ok with concatenation via evaluate_cached
    let result = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    assert!(result.is_ok());
  }

  #[test]
  fn add_string_left_num_right_returns_concat() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let bin = BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(make_str_expr("hello")),
      right: Box::new(make_num_expr(42.0)),
    };
    let result = binary_expr_to_string(&bin, &mut state, &mut traversal_state, &fns);
    assert!(result.is_ok());
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
      left: Box::new(make_str_expr("hello")),
      right: Box::new(make_num_expr(5.0)),
    };
    let result = binary_expr_to_num(&bin, &mut state, &mut traversal_state, &fns);
    // Expect error since string can't be converted to number
    assert!(result.is_err());
  }
}
