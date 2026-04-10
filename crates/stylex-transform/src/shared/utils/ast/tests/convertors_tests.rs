#[cfg(test)]
mod tests {
  use crate::shared::structures::functions::FunctionMap;
  use crate::shared::structures::state::EvaluationState;
  use crate::shared::structures::state_manager::StateManager;
  use crate::shared::utils::ast::convertors::{
    binary_expr_to_num, binary_expr_to_string, convert_string_to_prop_name,
  };
  use stylex_enums::misc::BinaryExprType;
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
        convert_string_to_prop_name(key).unwrap().is_str(),
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
        convert_string_to_prop_name(key).unwrap().is_ident(),
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
    use crate::shared::utils::ast::convertors::convert_tpl_to_string_lit;
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

    let result = convert_tpl_to_string_lit(&tpl);
    assert!(result.is_some(), "Should convert simple template to string");

    if let Some(Lit::Str(str_lit)) = result {
      assert_eq!(
        str_lit.value.as_str().expect("Failed to get string Value"),
        "hello world"
      );
    } else {
      panic!("Expected Lit::Str");
    }
  }

  #[test]
  fn test_simple_tpl_to_string_with_expressions() {
    use crate::shared::utils::ast::convertors::convert_tpl_to_string_lit;
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

    let result = convert_tpl_to_string_lit(&tpl);
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
        assert_eq!(
          str_lit.value.as_str().expect("Failed to get string Value"),
          "var(--font-geist-sans), sans-serif"
        );
      },
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
      },
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
          tpl.quasis[0]
            .cooked
            .as_ref()
            .expect("Failed to get string value"),
          "hello",
          "First quasi should be 'hello'"
        );
      },
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
          tpl.quasis[0]
            .cooked
            .as_ref()
            .expect("Failed to get cooked value"),
          "prefix",
          "First quasi should be 'prefix'"
        );
        assert!(tpl.quasis[3].tail, "Last quasi should have tail=true");
      },
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
      },
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
        assert_eq!(
          str_lit.value.as_str().expect("Failed to get string value"),
          "hello"
        );
      },
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
      },
      _ => panic!("Expected Expr::Tpl"),
    }
  }

  // ──────────────────────────────────────────────
  // convert_unary_to_num tests
  // ──────────────────────────────────────────────

  mod convert_unary_to_num_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_unary_to_num;
    use swc_core::ecma::ast::{UnaryExpr, UnaryOp};

    fn make_unary(op: UnaryOp, val: f64) -> UnaryExpr {
      UnaryExpr {
        span: Default::default(),
        op,
        arg: Box::new(make_num_expr(val)),
      }
    }

    #[test]
    fn minus_negates_positive() {
      let unary = make_unary(UnaryOp::Minus, 5.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, -5.0);
    }

    #[test]
    fn minus_negates_negative() {
      let unary = make_unary(UnaryOp::Minus, -3.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 3.0);
    }

    #[test]
    fn minus_zero() {
      let unary = make_unary(UnaryOp::Minus, 0.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      // -0.0 == 0.0 in f64
      assert_eq!(result, 0.0);
    }

    #[test]
    fn plus_preserves_value() {
      let unary = make_unary(UnaryOp::Plus, 7.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 7.0);
    }

    #[test]
    fn plus_preserves_negative() {
      let unary = make_unary(UnaryOp::Plus, -4.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, -4.0);
    }

    #[test]
    fn minus_large_number() {
      let unary = make_unary(UnaryOp::Minus, 1e10);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, -1e10);
    }

    #[test]
    fn minus_fractional() {
      let unary = make_unary(UnaryOp::Minus, 0.5);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, -0.5);
    }

    #[test]
    #[should_panic]
    fn unsupported_op_panics() {
      let unary = make_unary(UnaryOp::TypeOf, 5.0);
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_ident_to_expr tests
  // ──────────────────────────────────────────────

  mod convert_ident_to_expr_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_ident_to_expr;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn resolves_ident_to_number_expr() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("myNum", make_num_expr(42.0));
      fill_state_declarations(&mut state, &decl);
      // Set count so reduce doesn't underflow
      state
        .var_decl_count_map
        .insert("myNum".into(), 2);

      let ident = Ident {
        span: Default::default(),
        sym: "myNum".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };

      let result = convert_ident_to_expr(&ident, &mut state, &fns);
      match result {
        Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
        _ => panic!("Expected number literal"),
      }
    }

    #[test]
    fn resolves_ident_to_string_expr() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("myStr", make_str_expr("hello"));
      fill_state_declarations(&mut state, &decl);
      state
        .var_decl_count_map
        .insert("myStr".into(), 2);

      let ident = Ident {
        span: Default::default(),
        sym: "myStr".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };

      let result = convert_ident_to_expr(&ident, &mut state, &fns);
      match result {
        Expr::Lit(Lit::Str(s)) => {
          assert_eq!(s.value.as_str().expect("Expected string"), "hello")
        },
        _ => panic!("Expected string literal"),
      }
    }

    #[test]
    #[should_panic]
    fn panics_for_undeclared_ident() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let ident = Ident {
        span: Default::default(),
        sym: "undeclared".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      convert_ident_to_expr(&ident, &mut state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_bool tests
  // ──────────────────────────────────────────────

  mod convert_expr_to_bool_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_bool;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{
      ArrayLit, BindingIdent, Bool, ClassExpr, Function, Null, ObjectLit, UnaryExpr,
      UnaryOp,
    };

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn bool_true_returns_true() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Lit(Lit::Bool(Bool {
        span: Default::default(),
        value: true,
      }));
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn bool_false_returns_false() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Lit(Lit::Bool(Bool {
        span: Default::default(),
        value: false,
      }));
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn nonzero_number_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_num_expr(42.0);
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn zero_number_is_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_num_expr(0.0);
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn nonempty_string_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_str_expr("hello");
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn empty_string_is_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_str_expr("");
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn null_is_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Lit(Lit::Null(Null {
        span: Default::default(),
      }));
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn array_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Array(ArrayLit {
        span: Default::default(),
        elems: vec![],
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn object_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Object(ObjectLit {
        span: Default::default(),
        props: vec![],
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn class_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Class(ClassExpr {
        ident: None,
        class: Box::new(swc_core::ecma::ast::Class {
          span: Default::default(),
          decorators: vec![],
          body: vec![],
          super_class: None,
          is_abstract: false,
          type_params: None,
          super_type_params: None,
          implements: vec![],
          ctxt: SyntaxContext::empty(),
        }),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn fn_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Fn(swc_core::ecma::ast::FnExpr {
        ident: None,
        function: Box::new(Function {
          params: vec![],
          decorators: vec![],
          span: Default::default(),
          body: None,
          is_generator: false,
          is_async: false,
          type_params: None,
          return_type: None,
          ctxt: SyntaxContext::empty(),
        }),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn void_unary_is_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Void,
        arg: Box::new(make_num_expr(0.0)),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn typeof_unary_is_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::TypeOf,
        arg: Box::new(make_num_expr(0.0)),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn bang_negates_truthy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Bang,
        arg: Box::new(Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: true,
        }))),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn bang_negates_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Bang,
        arg: Box::new(Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: false,
        }))),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn minus_unary_negates_bool() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Minus,
        arg: Box::new(Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: true,
        }))),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn plus_unary_negates_bool() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Plus,
        arg: Box::new(Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: true,
        }))),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn tilde_unary_negates_bool() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Tilde,
        arg: Box::new(Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: true,
        }))),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn ident_resolves_and_converts_to_bool() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator(
        "flag",
        Expr::Lit(Lit::Bool(Bool {
          span: Default::default(),
          value: true,
        })),
      );
      fill_state_declarations(&mut state, &decl);
      state.var_decl_count_map.insert("flag".into(), 2);

      let expr = make_ident_expr("flag");
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }
  }

  // ──────────────────────────────────────────────
  // convert_key_value_to_str tests
  // ──────────────────────────────────────────────

  mod convert_key_value_to_str_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_key_value_to_str;
    use swc_core::ecma::ast::{
      ComputedPropName, IdentName, KeyValueProp, Number, PropName,
    };

    fn make_kv(key: PropName) -> KeyValueProp {
      KeyValueProp {
        key,
        value: Box::new(make_num_expr(0.0)),
      }
    }

    #[test]
    fn ident_key_returns_name() {
      let kv = make_kv(PropName::Ident(IdentName {
        span: Default::default(),
        sym: "color".into(),
      }));
      let result = convert_key_value_to_str(&kv);
      assert_eq!(result, "color");
    }

    #[test]
    fn str_key_returns_value() {
      let kv = make_kv(PropName::Str(Str {
        span: Default::default(),
        value: "background-color".into(),
        raw: None,
      }));
      let result = convert_key_value_to_str(&kv);
      assert!(result.contains("background-color"));
    }

    #[test]
    fn num_key_returns_number_string() {
      let kv = make_kv(PropName::Num(Number {
        span: Default::default(),
        value: 42.0,
        raw: None,
      }));
      let result = convert_key_value_to_str(&kv);
      assert_eq!(result, "42");
    }

    #[test]
    fn computed_string_key_returns_value() {
      let kv = make_kv(PropName::Computed(ComputedPropName {
        span: Default::default(),
        expr: Box::new(make_str_expr("dynamic")),
      }));
      let result = convert_key_value_to_str(&kv);
      assert!(result.contains("dynamic"));
    }
  }

  // ──────────────────────────────────────────────
  // expr_tpl_to_string tests
  // ──────────────────────────────────────────────

  mod expr_tpl_to_string_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::expr_tpl_to_string;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, Tpl, TplElement};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn simple_template_without_expressions() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

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

      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "hello world");
    }

    #[test]
    fn template_with_literal_expression() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_num_expr(42.0))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("value is ".into()),
            raw: "value is ".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some(" px".into()),
            raw: " px".into(),
          },
        ],
      };

      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "value is 42 px");
    }

    #[test]
    fn template_with_ident_expression() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("size", make_str_expr("16px"));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state
        .var_decl_count_map
        .insert("size".into(), 2);

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_ident_expr("size"))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("font-size: ".into()),
            raw: "font-size: ".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("".into()),
            raw: "".into(),
          },
        ],
      };

      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "font-size: 16px");
    }

    #[test]
    fn template_with_string_literal_expression() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_str_expr("world"))],
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

      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "hello world");
    }
  }

  // ──────────────────────────────────────────────
  // handle_tpl_to_expression tests
  // ──────────────────────────────────────────────

  mod handle_tpl_to_expression_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::handle_tpl_to_expression;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, Tpl, TplElement};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn replaces_ident_with_var_decl_init() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("myVar", make_str_expr("replaced"));
      fill_state_declarations(&mut state, &decl);
      state.var_decl_count_map.insert("myVar".into(), 2);

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_ident_expr("myVar"))],
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

      let result = handle_tpl_to_expression(&tpl, &mut state, &fns);
      match result {
        Expr::Tpl(result_tpl) => {
          assert_eq!(result_tpl.exprs.len(), 1);
          // The expression should have been replaced with the var init
          match result_tpl.exprs[0].as_ref() {
            Expr::Lit(Lit::Str(s)) => {
              assert_eq!(
                s.value.as_str().expect("Expected string"),
                "replaced"
              )
            },
            _ => panic!("Expected string literal replacement"),
          }
        },
        _ => panic!("Expected Tpl expression"),
      }
    }

    #[test]
    fn non_ident_expressions_unchanged() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_num_expr(42.0))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("val: ".into()),
            raw: "val: ".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("".into()),
            raw: "".into(),
          },
        ],
      };

      let result = handle_tpl_to_expression(&tpl, &mut state, &fns);
      match result {
        Expr::Tpl(result_tpl) => {
          match result_tpl.exprs[0].as_ref() {
            Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
            _ => panic!("Expected numeric literal unchanged"),
          }
        },
        _ => panic!("Expected Tpl expression"),
      }
    }
  }

  // ──────────────────────────────────────────────
  // ident_to_number tests
  // ──────────────────────────────────────────────

  mod ident_to_number_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::ident_to_number;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn resolves_numeric_literal() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("myNum", make_num_expr(42.0));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state
        .var_decl_count_map
        .insert("myNum".into(), 2);

      let ident = Ident {
        span: Default::default(),
        sym: "myNum".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };

      let result = ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 42.0);
    }

    #[test]
    #[should_panic(expected = "not declared")]
    fn panics_for_undeclared_ident() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let ident = Ident {
        span: Default::default(),
        sym: "nonexistent".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };

      ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // expr_to_num additional tests
  // ──────────────────────────────────────────────

  mod expr_to_num_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::expr_to_num;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, UnaryExpr, UnaryOp};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn literal_number_returns_value() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_num_expr(2.5);
      let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
      assert!((result - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn ident_resolves_to_number() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();

      let decl = make_var_declarator("val", make_num_expr(99.0));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("val".into(), 2);

      let expr = make_ident_expr("val");
      let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
      assert_eq!(result, 99.0);
    }

    #[test]
    fn unary_minus_number() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Minus,
        arg: Box::new(make_num_expr(5.0)),
      });
      let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
      assert_eq!(result, -5.0);
    }

    #[test]
    fn bin_expr_addition() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Bin(BinExpr {
        span: Default::default(),
        op: BinaryOp::Add,
        left: Box::new(make_num_expr(3.0)),
        right: Box::new(make_num_expr(4.0)),
      });
      let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
      assert_eq!(result, 7.0);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_str tests
  // ──────────────────────────────────────────────

  mod convert_expr_to_str_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_str;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn string_literal_returns_string() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_str_expr("hello");
      let result = convert_expr_to_str(&expr, &mut state, &fns);
      assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn ident_resolves_to_string() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let decl = make_var_declarator("color", make_str_expr("red"));
      fill_state_declarations(&mut state, &decl);
      state.var_decl_count_map.insert("color".into(), 2);
      let expr = make_ident_expr("color");
      let result = convert_expr_to_str(&expr, &mut state, &fns);
      assert_eq!(result, Some("red".to_string()));
    }

    #[test]
    fn number_literal_returns_string() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = make_num_expr(42.0);
      let result = convert_expr_to_str(&expr, &mut state, &fns);
      assert_eq!(result, Some("42".to_string()));
    }
  }

  // ──────────────────────────────────────────────
  // convert_key_value_to_str - BigInt key
  // ──────────────────────────────────────────────

  mod convert_key_value_to_str_bigint_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_key_value_to_str;
    use swc_core::ecma::ast::{BigInt, KeyValueProp, PropName};

    #[test]
    fn bigint_key_returns_string() {
      let kv = KeyValueProp {
        key: PropName::BigInt(BigInt {
          span: Default::default(),
          value: Box::new(100u32.into()),
          raw: None,
        }),
        value: Box::new(make_num_expr(0.0)),
      };
      let result = convert_key_value_to_str(&kv);
      assert!(result.contains("100"));
    }
  }

  // ──────────────────────────────────────────────
  // ident_to_number - bin/unary declaration resolution
  // ──────────────────────────────────────────────

  mod ident_to_number_extended_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::ident_to_number;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, UnaryExpr, UnaryOp};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn resolves_ident_with_bin_expr_decl() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let bin_expr = Expr::Bin(BinExpr {
        span: Default::default(),
        op: BinaryOp::Add,
        left: Box::new(make_num_expr(3.0)),
        right: Box::new(make_num_expr(7.0)),
      });
      let decl = make_var_declarator("sum", bin_expr);
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("sum".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "sum".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      let result = ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 10.0);
    }

    #[test]
    fn resolves_ident_with_unary_expr_decl() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let unary_expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Minus,
        arg: Box::new(make_num_expr(5.0)),
      });
      let decl = make_var_declarator("neg", unary_expr);
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("neg".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "neg".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      let result = ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, -5.0);
    }

    #[test]
    #[should_panic]
    fn panics_for_undeclared_ident() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let ident = Ident {
        span: Default::default(),
        sym: "missing".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
    }

    #[test]
    #[should_panic]
    fn panics_for_non_number_decl() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let decl = make_var_declarator("s", make_str_expr("hello"));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("s".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "s".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // handle_tpl_to_expression tests
  // ──────────────────────────────────────────────

  mod handle_tpl_to_expression_extended_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::handle_tpl_to_expression;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, Tpl, TplElement};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn replaces_ident_with_var_decl_init_extended() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let decl = make_var_declarator("val", make_num_expr(42.0));
      fill_state_declarations(&mut state, &decl);
      state.var_decl_count_map.insert("val".into(), 2);

      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_ident_expr("val"))],
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
      let result = handle_tpl_to_expression(&tpl, &mut state, &fns);
      assert!(result.is_tpl());
    }

    #[test]
    fn non_ident_expressions_unchanged_extended() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_num_expr(10.0))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("a".into()),
            raw: "a".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("b".into()),
            raw: "b".into(),
          },
        ],
      };
      let result = handle_tpl_to_expression(&tpl, &mut state, &fns);
      assert!(result.is_tpl());
    }
  }

  // ──────────────────────────────────────────────
  // expr_tpl_to_string - bin expr and literal expressions
  // ──────────────────────────────────────────────

  mod expr_tpl_to_string_extended_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::expr_tpl_to_string;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::{BindingIdent, Tpl, TplElement};

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn template_with_bin_expr() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(Expr::Bin(BinExpr {
          span: Default::default(),
          op: BinaryOp::Add,
          left: Box::new(make_num_expr(3.0)),
          right: Box::new(make_num_expr(4.0)),
        }))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("result: ".into()),
            raw: "result: ".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("px".into()),
            raw: "px".into(),
          },
        ],
      };
      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "result: 7px");
    }

    #[test]
    fn template_with_number_literal() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_num_expr(42.0))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("".into()),
            raw: "".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("px".into()),
            raw: "px".into(),
          },
        ],
      };
      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "42px");
    }

    #[test]
    fn template_with_ident_resolving_to_string() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let decl = make_var_declarator("unit", make_str_expr("em"));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("unit".into(), 2);
      let tpl = Tpl {
        span: Default::default(),
        exprs: vec![Box::new(make_ident_expr("unit"))],
        quasis: vec![
          TplElement {
            span: Default::default(),
            tail: false,
            cooked: Some("10".into()),
            raw: "10".into(),
          },
          TplElement {
            span: Default::default(),
            tail: true,
            cooked: Some("".into()),
            raw: "".into(),
          },
        ],
      };
      let result = expr_tpl_to_string(&tpl, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, "10em");
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
  // transform_bin_expr_to_number tests
  // ──────────────────────────────────────────────

  mod transform_bin_expr_to_number_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::transform_bin_expr_to_number;

    #[test]
    fn add_two_numbers() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let bin = BinExpr {
        span: Default::default(),
        op: BinaryOp::Add,
        left: Box::new(make_num_expr(3.0)),
        right: Box::new(make_num_expr(4.0)),
      };
      let result = transform_bin_expr_to_number(&bin, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 7.0);
    }

    #[test]
    fn mul_two_numbers() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let bin = BinExpr {
        span: Default::default(),
        op: BinaryOp::Mul,
        left: Box::new(make_num_expr(3.0)),
        right: Box::new(make_num_expr(5.0)),
      };
      let result = transform_bin_expr_to_number(&bin, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 15.0);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_bool - additional branches
  // ──────────────────────────────────────────────

  mod convert_expr_to_bool_extra_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_bool;
    use swc_core::ecma::ast::{UnaryExpr, UnaryOp};

    #[test]
    fn minus_zero_is_truthy_by_negation() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // Minus applies !convert_expr_to_bool(arg), and 0 is falsy, so !false = true
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Minus,
        arg: Box::new(make_num_expr(0.0)),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn plus_zero_is_truthy_by_negation() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // Plus applies !convert_expr_to_bool(arg), and 0 is falsy, so !false = true
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Plus,
        arg: Box::new(make_num_expr(0.0)),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn tilde_of_neg1_is_falsy() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // ~(-1) = 0 → !convert_expr_to_bool(-1) = !true = false
      // But -1 is truthy (nonzero), so tilde inverts: !true = false
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Tilde,
        arg: Box::new(make_num_expr(1.0)),
      });
      assert!(!convert_expr_to_bool(&expr, &mut state, &fns));
    }

    #[test]
    fn nested_bang_double_negation() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // !!true == true
      let inner = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Bang,
        arg: Box::new(Expr::Lit(Lit::Bool(swc_core::ecma::ast::Bool {
          span: Default::default(),
          value: true,
        }))),
      });
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: UnaryOp::Bang,
        arg: Box::new(inner),
      });
      assert!(convert_expr_to_bool(&expr, &mut state, &fns));
    }
  }

  // ──────────────────────────────────────────────
  // convert_ident_to_expr tests
  // ──────────────────────────────────────────────

  mod convert_ident_to_expr_extended_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_ident_to_expr;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn resolves_ident_to_expr_value() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let decl = make_var_declarator("x", make_num_expr(42.0));
      fill_state_declarations(&mut state, &decl);
      state.var_decl_count_map.insert("x".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "x".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      let result = convert_ident_to_expr(&ident, &mut state, &fns);
      assert!(result.is_lit());
    }

    #[test]
    #[should_panic]
    fn panics_for_undeclared_ident_convert() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let ident = Ident {
        span: Default::default(),
        sym: "missing".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      convert_ident_to_expr(&ident, &mut state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_str - should_panic for unsupported expr
  // ──────────────────────────────────────────────

  mod convert_expr_to_str_panic_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_str;
    use swc_core::ecma::ast::ArrayLit;

    #[test]
    #[should_panic]
    fn panics_for_array_expr() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Array(ArrayLit {
        span: Default::default(),
        elems: vec![],
      });
      convert_expr_to_str(&expr, &mut state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // expr_to_num - should_panic for unsupported expr
  // ──────────────────────────────────────────────

  mod expr_to_num_panic_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::expr_to_num;
    use swc_core::ecma::ast::ArrayLit;

    #[test]
    #[should_panic]
    fn panics_for_array_expr() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Array(ArrayLit {
        span: Default::default(),
        elems: vec![],
      });
      let _ = expr_to_num(&expr, &mut state, &mut traversal_state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_bool - unsupported expr
  // ──────────────────────────────────────────────

  mod convert_expr_to_bool_unsupported_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_bool;
    use swc_core::ecma::ast::Tpl;

    #[test]
    #[should_panic]
    fn panics_for_template_literal() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Tpl(Tpl {
        span: Default::default(),
        exprs: vec![],
        quasis: vec![],
      });
      convert_expr_to_bool(&expr, &mut state, &fns);
    }

    #[test]
    #[should_panic]
    fn panics_for_unsupported_lit_type() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // Regex lit is a Lit that's not handled
      let expr = Expr::Lit(Lit::Regex(swc_core::ecma::ast::Regex {
        span: Default::default(),
        exp: ".*".into(),
        flags: "g".into(),
      }));
      convert_expr_to_bool(&expr, &mut state, &fns);
    }

    #[test]
    #[should_panic]
    fn panics_for_unsupported_unary_op() {
      use swc_core::ecma::ast::UnaryExpr;
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      let expr = Expr::Unary(UnaryExpr {
        span: Default::default(),
        op: swc_core::ecma::ast::UnaryOp::Delete,
        arg: Box::new(make_num_expr(1.0)),
      });
      convert_expr_to_bool(&expr, &mut state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_key_value_to_str - computed non-literal panic
  // ──────────────────────────────────────────────

  mod convert_key_value_to_str_panic_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_key_value_to_str;
    use swc_core::ecma::ast::{ComputedPropName, KeyValueProp, PropName};

    #[test]
    #[should_panic]
    fn panics_for_computed_non_literal_key() {
      let kv = KeyValueProp {
        key: PropName::Computed(ComputedPropName {
          span: Default::default(),
          expr: Box::new(make_ident_expr("dynamic")),
        }),
        value: Box::new(make_num_expr(0.0)),
      };
      convert_key_value_to_str(&kv);
    }
  }

  // ──────────────────────────────────────────────
  // ident_to_number - additional edge cases
  // ──────────────────────────────────────────────

  mod ident_to_number_edge_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::ident_to_number;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn resolves_ident_with_literal_string_number() {
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      // Declare val = 42 (as number literal)
      let decl = make_var_declarator("val", make_num_expr(42.0));
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("val".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "val".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      let result = ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
      assert_eq!(result, 42.0);
    }

    #[test]
    #[should_panic]
    fn panics_for_object_expr_decl() {
      use swc_core::ecma::ast::ObjectLit;
      let mut state = EvaluationState::new();
      let mut traversal_state = StateManager::default();
      let fns = FunctionMap::default();
      let obj_expr = Expr::Object(ObjectLit {
        span: Default::default(),
        props: vec![],
      });
      let decl = make_var_declarator("obj", obj_expr);
      fill_state_declarations(&mut traversal_state, &decl);
      traversal_state.var_decl_count_map.insert("obj".into(), 2);
      let ident = Ident {
        span: Default::default(),
        sym: "obj".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      };
      // This should panic with "Variable ... is not a number"
      ident_to_number(&ident, &mut state, &mut traversal_state, &fns);
    }
  }

  // ──────────────────────────────────────────────
  // convert_expr_to_str - ident resolving to ident chain
  // ──────────────────────────────────────────────

  mod convert_expr_to_str_ident_chain_tests {
    use super::*;
    use crate::shared::utils::ast::convertors::convert_expr_to_str;
    use crate::shared::utils::common::fill_state_declarations;
    use swc_core::ecma::ast::BindingIdent;

    fn make_var_declarator(name: &str, init: Expr) -> swc_core::ecma::ast::VarDeclarator {
      swc_core::ecma::ast::VarDeclarator {
        span: Default::default(),
        name: swc_core::ecma::ast::Pat::Ident(BindingIdent {
          id: Ident {
            span: Default::default(),
            sym: name.into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
          },
          type_ann: None,
        }),
        init: Some(Box::new(init)),
        definite: false,
      }
    }

    #[test]
    fn ident_resolves_through_chain_to_string() {
      let mut state = StateManager::default();
      let fns = FunctionMap::default();
      // inner = "red"
      let inner_decl = make_var_declarator("inner", make_str_expr("red"));
      fill_state_declarations(&mut state, &inner_decl);
      state.var_decl_count_map.insert("inner".into(), 3);
      // outer = inner (ident)
      let outer_decl = make_var_declarator("outer", make_ident_expr("inner"));
      fill_state_declarations(&mut state, &outer_decl);
      state.var_decl_count_map.insert("outer".into(), 2);

      let expr = make_ident_expr("outer");
      let result = convert_expr_to_str(&expr, &mut state, &fns);
      assert_eq!(result, Some("red".to_string()));
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
  }
}
