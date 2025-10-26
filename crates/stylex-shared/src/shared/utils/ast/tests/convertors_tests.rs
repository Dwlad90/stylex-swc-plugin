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
    ecma::ast::{BinExpr, BinaryOp, Expr, Ident, IdentName, Lit, Str},
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

  #[test]
  fn test_simple_tpl_to_string_without_expressions() {
    use crate::shared::utils::ast::convertors::simple_tpl_to_string;
    use swc_core::ecma::ast::{Tpl, TplElement};

    // Create a simple template literal: `hello world`
    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![],
      quasis: vec![TplElement {
        span: Default::default(),
        tail: true,
        cooked: Some("hello world".into()),
        raw: "hello world".into(),
      }],
    };

    let result = simple_tpl_to_string(&tpl);
    assert!(result.is_some(), "Should convert simple template to string");

    if let Some(Lit::Str(str_lit)) = result {
      assert_eq!(str_lit.value.as_ref(), "hello world");
    } else {
      panic!("Expected Lit::Str");
    }
  }

  #[test]
  fn test_simple_tpl_to_string_with_expressions() {
    use crate::shared::utils::ast::convertors::simple_tpl_to_string;
    use swc_core::ecma::ast::{Tpl, TplElement};

    // Create a template literal with expressions: `hello ${name}`
    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![Box::new(make_ident_expr("name"))],
      quasis: vec![
        TplElement {
          span: Default::default(),
          tail: false,
          cooked: Some("hello ".into()),
          raw: "hello ".into(),
        },
        TplElement {
          span: Default::default(),
          tail: true,
          cooked: Some("".into()),
          raw: "".into(),
        },
      ],
    };

    let result = simple_tpl_to_string(&tpl);
    assert!(
      result.is_none(),
      "Should not convert template with expressions"
    );
  }

  #[test]
  fn test_convert_simple_tpl_to_str_expr() {
    use crate::shared::utils::ast::convertors::convert_simple_tpl_to_str_expr;
    use swc_core::ecma::ast::{Tpl, TplElement};

    // Create a simple template literal
    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![],
      quasis: vec![TplElement {
        span: Default::default(),
        tail: true,
        cooked: Some("var(--font-geist-sans), sans-serif".into()),
        raw: "var(--font-geist-sans), sans-serif".into(),
      }],
    };

    let expr = Expr::Tpl(tpl);
    let result = convert_simple_tpl_to_str_expr(expr);

    match result {
      Expr::Lit(Lit::Str(str_lit)) => {
        assert_eq!(str_lit.value.as_ref(), "var(--font-geist-sans), sans-serif");
      }
      _ => panic!("Expected Expr::Lit(Lit::Str)"),
    }
  }

  #[test]
  fn test_convert_simple_tpl_to_str_expr_with_expressions() {
    use crate::shared::utils::ast::convertors::convert_simple_tpl_to_str_expr;
    use swc_core::ecma::ast::{Tpl, TplElement};

    // Create a template with expressions
    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![Box::new(make_ident_expr("value"))],
      quasis: vec![
        TplElement {
          span: Default::default(),
          tail: false,
          cooked: Some("prefix ".into()),
          raw: "prefix ".into(),
        },
        TplElement {
          span: Default::default(),
          tail: true,
          cooked: Some(" suffix".into()),
          raw: " suffix".into(),
        },
      ],
    };

    let expr = Expr::Tpl(tpl.clone());
    let result = convert_simple_tpl_to_str_expr(expr);

    // Should remain as Tpl since it has expressions
    match result {
      Expr::Tpl(_) => {
        // This is expected
      }
      _ => panic!("Expected Expr::Tpl to remain unchanged"),
    }
  }

  #[test]
  fn test_convert_concat_to_tpl_expr_simple() {
    use crate::shared::utils::ast::convertors::convert_concat_to_tpl_expr;
    use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

    // Create: "hello".concat("world")
    let call_expr = CallExpr {
      span: Default::default(),
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: Default::default(),
        obj: Box::new(make_str_expr("hello")),
        prop: MemberProp::Ident(IdentName {
          span: Default::default(),
          sym: "concat".into(),
        }),
      }))),
      args: vec![ExprOrSpread {
        spread: None,
        expr: Box::new(make_str_expr("world")),
      }],
      ..Default::default()
    };

    let expr = Expr::Call(call_expr);
    let result = convert_concat_to_tpl_expr(expr);

    // Should be converted to template literal: `hello${world}`
    match result {
      Expr::Tpl(tpl) => {
        assert_eq!(tpl.quasis.len(), 2, "Should have 2 quasis");
        assert_eq!(tpl.exprs.len(), 1, "Should have 1 expression");
        assert_eq!(
          tpl.quasis[0].cooked.as_ref().unwrap().as_ref(),
          "hello",
          "First quasi should be 'hello'"
        );
      }
      _ => panic!("Expected Expr::Tpl"),
    }
  }

  #[test]
  fn test_convert_concat_to_tpl_expr_multiple_args() {
    use crate::shared::utils::ast::convertors::convert_concat_to_tpl_expr;
    use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

    // Create: "prefix".concat(var1, var2, var3)
    let call_expr = CallExpr {
      span: Default::default(),
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: Default::default(),
        obj: Box::new(make_str_expr("prefix")),
        prop: MemberProp::Ident(IdentName {
          span: Default::default(),
          sym: "concat".into(),
        }),
      }))),
      args: vec![
        ExprOrSpread {
          spread: None,
          expr: Box::new(make_ident_expr("var1")),
        },
        ExprOrSpread {
          spread: None,
          expr: Box::new(make_ident_expr("var2")),
        },
        ExprOrSpread {
          spread: None,
          expr: Box::new(make_ident_expr("var3")),
        },
      ],
      ..Default::default()
    };

    let expr = Expr::Call(call_expr);
    let result = convert_concat_to_tpl_expr(expr);

    // Should be converted to template literal: `prefix${var1}${var2}${var3}`
    match result {
      Expr::Tpl(tpl) => {
        assert_eq!(tpl.quasis.len(), 4, "Should have 4 quasis");
        assert_eq!(tpl.exprs.len(), 3, "Should have 3 expressions");
        assert_eq!(
          tpl.quasis[0].cooked.as_ref().unwrap().as_ref(),
          "prefix",
          "First quasi should be 'prefix'"
        );
        assert!(tpl.quasis[3].tail, "Last quasi should have tail=true");
      }
      _ => panic!("Expected Expr::Tpl"),
    }
  }

  #[test]
  fn test_convert_concat_to_tpl_expr_not_concat_method() {
    use crate::shared::utils::ast::convertors::convert_concat_to_tpl_expr;
    use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

    // Create: "hello".split("world") - not a concat call
    let call_expr = CallExpr {
      span: Default::default(),
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: Default::default(),
        obj: Box::new(make_str_expr("hello")),
        prop: MemberProp::Ident(IdentName {
          span: Default::default(),
          sym: "split".into(), // Not "concat"
        }),
      }))),
      args: vec![ExprOrSpread {
        spread: None,
        expr: Box::new(make_str_expr("world")),
      }],
      ..Default::default()
    };

    let original_expr = Expr::Call(call_expr.clone());
    let result = convert_concat_to_tpl_expr(original_expr);

    // Should remain as CallExpr since it's not concat
    match result {
      Expr::Call(_) => {
        // This is expected - should remain unchanged
      }
      _ => panic!("Expected Expr::Call to remain unchanged"),
    }
  }

  #[test]
  fn test_convert_concat_to_tpl_expr_non_call_expr() {
    use crate::shared::utils::ast::convertors::convert_concat_to_tpl_expr;

    // Test with a non-call expression (e.g., just a string)
    let expr = make_str_expr("hello");
    let result = convert_concat_to_tpl_expr(expr);

    // Should remain as string literal
    match result {
      Expr::Lit(Lit::Str(str_lit)) => {
        assert_eq!(str_lit.value.as_ref(), "hello");
      }
      _ => panic!("Expected Expr::Lit(Lit::Str) to remain unchanged"),
    }
  }

  #[test]
  fn test_convert_concat_to_tpl_expr_with_spread() {
    use crate::shared::utils::ast::convertors::convert_concat_to_tpl_expr;
    use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

    // Create: "prefix".concat(...args) - with spread argument
    let call_expr = CallExpr {
      span: Default::default(),
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: Default::default(),
        obj: Box::new(make_str_expr("prefix")),
        prop: MemberProp::Ident(IdentName {
          span: Default::default(),
          sym: "concat".into(),
        }),
      }))),
      args: vec![ExprOrSpread {
        spread: Some(Default::default()),
        expr: Box::new(make_ident_expr("args")),
      }],
      ..Default::default()
    };

    let expr = Expr::Call(call_expr);
    let result = convert_concat_to_tpl_expr(expr);

    // Should still convert but skip spread arguments
    match result {
      Expr::Tpl(tpl) => {
        assert_eq!(
          tpl.quasis.len(),
          1,
          "Should have 1 quasi (spread args are skipped)"
        );
        assert_eq!(
          tpl.exprs.len(),
          0,
          "Should have 0 expressions (spread args are skipped)"
        );
      }
      _ => panic!("Expected Expr::Tpl"),
    }
  }
}
