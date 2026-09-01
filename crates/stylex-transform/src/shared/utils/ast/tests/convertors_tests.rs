use crate::shared::structures::state::EvaluationState;
use stylex_ast::ast::convertors::{
  convert_concat_to_tpl_expr, convert_key_value_to_str, convert_simple_tpl_to_str_expr,
  convert_string_to_prop_name, convert_tpl_to_string_lit, create_ident_expr, create_number_expr,
  create_string_expr,
};
use stylex_state::{functions::FunctionMap, state_manager::StateManager};
use swc_core::{
  common::SyntaxContext,
  ecma::ast::{BinExpr, BinaryOp, Expr, Ident, IdentName, Lit, Str},
};

#[test]
fn string_to_prop_name_with_quotes() {
  let keys_with_quotes = vec!["2ip", "123", "1b3", "1bc", "2xl", "x*x", "x-x", "x,x"];

  for key in keys_with_quotes {
    assert!(
      convert_string_to_prop_name(key).is_str(),
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
      convert_string_to_prop_name(key).is_ident(),
      "Key '{}' should not be wrapped in quotes",
      key
    );
  }
}

#[test]
fn test_simple_tpl_to_string_without_expressions() {
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
  use swc_core::ecma::ast::{Tpl, TplElement};

  // Create a template literal with expressions: `hello ${name}`
  let tpl = Tpl {
    span: Default::default(),
    exprs: vec![Box::new(create_ident_expr("name"))],
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
  use swc_core::ecma::ast::{Tpl, TplElement};

  // Create a template with expressions
  let tpl = Tpl {
    span: Default::default(),
    exprs: vec![Box::new(create_ident_expr("value"))],
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

  let expr = Expr::Tpl(tpl);
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
  use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

  // Create: "hello".concat("world")
  let call_expr = CallExpr {
    span: Default::default(),
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: Default::default(),
      obj: Box::new(create_string_expr("hello")),
      prop: MemberProp::Ident(IdentName {
        span: Default::default(),
        sym: "concat".into(),
      }),
    }))),
    args: vec![ExprOrSpread {
      spread: None,
      expr: Box::new(create_string_expr("world")),
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
  use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

  // Create: "prefix".concat(var1, var2, var3)
  let call_expr = CallExpr {
    span: Default::default(),
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: Default::default(),
      obj: Box::new(create_string_expr("prefix")),
      prop: MemberProp::Ident(IdentName {
        span: Default::default(),
        sym: "concat".into(),
      }),
    }))),
    args: vec![
      ExprOrSpread {
        spread: None,
        expr: Box::new(create_ident_expr("var1")),
      },
      ExprOrSpread {
        spread: None,
        expr: Box::new(create_ident_expr("var2")),
      },
      ExprOrSpread {
        spread: None,
        expr: Box::new(create_ident_expr("var3")),
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
  use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

  // Create: "hello".split("world") - not a concat call
  let call_expr = CallExpr {
    span: Default::default(),
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: Default::default(),
      obj: Box::new(create_string_expr("hello")),
      prop: MemberProp::Ident(IdentName {
        span: Default::default(),
        sym: "split".into(), // Not "concat"
      }),
    }))),
    args: vec![ExprOrSpread {
      spread: None,
      expr: Box::new(create_string_expr("world")),
    }],
    ..Default::default()
  };

  let original_expr = Expr::Call(call_expr);
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
  // Test with a non-call expression (e.g., just a string)
  let expr = create_string_expr("hello");
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
  use swc_core::ecma::ast::{CallExpr, Callee, ExprOrSpread, MemberExpr, MemberProp};

  // Create: "prefix".concat(...args) - with spread argument
  let call_expr = CallExpr {
    span: Default::default(),
    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
      span: Default::default(),
      obj: Box::new(create_string_expr("prefix")),
      prop: MemberProp::Ident(IdentName {
        span: Default::default(),
        sym: "concat".into(),
      }),
    }))),
    args: vec![ExprOrSpread {
      spread: Some(Default::default()),
      expr: Box::new(create_ident_expr("args")),
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
      arg: Box::new(create_number_expr(val)),
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
// convert_key_value_to_str tests
// ──────────────────────────────────────────────

mod convert_key_value_to_str_tests {
  use super::*;
  use swc_core::ecma::ast::{ComputedPropName, IdentName, KeyValueProp, Number, PropName};

  fn make_kv(key: PropName) -> KeyValueProp {
    KeyValueProp {
      key,
      value: Box::new(create_number_expr(0.0)),
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
      expr: Box::new(create_string_expr("dynamic")),
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
  use stylex_state::common::fill_state_declarations;
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
      exprs: vec![Box::new(create_number_expr(42.0))],
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

    let decl = make_var_declarator("size", create_string_expr("16px"));
    fill_state_declarations(&mut traversal_state, &decl);

    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![Box::new(create_ident_expr("size"))],
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
      exprs: vec![Box::new(create_string_expr("world"))],
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
// ident_to_number tests
// ──────────────────────────────────────────────

mod ident_to_number_tests {
  use super::*;
  use crate::shared::utils::ast::convertors::ident_to_number;
  use stylex_state::common::fill_state_declarations;
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

    let decl = make_var_declarator("myNum", create_number_expr(42.0));
    fill_state_declarations(&mut traversal_state, &decl);

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
  use stylex_state::common::fill_state_declarations;
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
    let expr = create_number_expr(2.5);
    let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
    assert!((result - 2.5).abs() < f64::EPSILON);
  }

  #[test]
  fn ident_resolves_to_number() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();

    let decl = make_var_declarator("val", create_number_expr(99.0));
    fill_state_declarations(&mut traversal_state, &decl);

    let expr = create_ident_expr("val");
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
      arg: Box::new(create_number_expr(5.0)),
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
      left: Box::new(create_number_expr(3.0)),
      right: Box::new(create_number_expr(4.0)),
    });
    let result = expr_to_num(&expr, &mut state, &mut traversal_state, &fns).unwrap();
    assert_eq!(result, 7.0);
  }
}

// ──────────────────────────────────────────────
// convert_key_value_to_str - BigInt key
// ──────────────────────────────────────────────

mod convert_key_value_to_str_bigint_tests {
  use super::*;
  use swc_core::ecma::ast::{BigInt, KeyValueProp, PropName};

  #[test]
  fn bigint_key_returns_string() {
    let kv = KeyValueProp {
      key: PropName::BigInt(BigInt {
        span: Default::default(),
        value: Box::new(100u32.into()),
        raw: None,
      }),
      value: Box::new(create_number_expr(0.0)),
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
  use stylex_state::common::fill_state_declarations;
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
      left: Box::new(create_number_expr(3.0)),
      right: Box::new(create_number_expr(7.0)),
    });
    let decl = make_var_declarator("sum", bin_expr);
    fill_state_declarations(&mut traversal_state, &decl);
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
      arg: Box::new(create_number_expr(5.0)),
    });
    let decl = make_var_declarator("neg", unary_expr);
    fill_state_declarations(&mut traversal_state, &decl);
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
    let decl = make_var_declarator("s", create_string_expr("hello"));
    fill_state_declarations(&mut traversal_state, &decl);
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
// expr_tpl_to_string - bin expr and literal expressions
// ──────────────────────────────────────────────

mod expr_tpl_to_string_extended_tests {
  use super::*;
  use crate::shared::utils::ast::convertors::expr_tpl_to_string;
  use stylex_state::common::fill_state_declarations;
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
        left: Box::new(create_number_expr(3.0)),
        right: Box::new(create_number_expr(4.0)),
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
      exprs: vec![Box::new(create_number_expr(42.0))],
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
    let decl = make_var_declarator("unit", create_string_expr("em"));
    fill_state_declarations(&mut traversal_state, &decl);
    let tpl = Tpl {
      span: Default::default(),
      exprs: vec![Box::new(create_ident_expr("unit"))],
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
      left: Box::new(create_number_expr(3.0)),
      right: Box::new(create_number_expr(4.0)),
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
      left: Box::new(create_number_expr(3.0)),
      right: Box::new(create_number_expr(5.0)),
    };
    let result = transform_bin_expr_to_number(&bin, &mut state, &mut traversal_state, &fns);
    assert_eq!(result, 15.0);
  }
}

// ──────────────────────────────────────────────
// expr_to_num - reports an unsupported expr
// ──────────────────────────────────────────────

mod expr_to_num_unsupported_tests {
  use super::*;
  use crate::shared::utils::ast::convertors::expr_to_num;
  use swc_core::ecma::ast::ArrayLit;

  /// An expression with no numeric reading is reported through the `Result`
  /// this returns, not by aborting: `-[]` is ordinary JavaScript, and the
  /// evaluator has to be able to refuse to fold it.
  #[test]
  fn reports_an_array_expr_as_an_error() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let expr = Expr::Array(ArrayLit {
      span: Default::default(),
      elems: vec![],
    });

    let error = expr_to_num(&expr, &mut state, &mut traversal_state, &fns)
      .expect_err("an array has no numeric reading");

    assert!(
      error.to_string().contains("not a number"),
      "the error should say what it could not read, got: {}",
      error
    );
  }

  /// The same for a binary expression that folds to a string rather than a
  /// number — the arm one level in, which used to abort separately.
  #[test]
  fn reports_a_string_valued_binary_expr_as_an_error() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let expr = Expr::Bin(BinExpr {
      span: Default::default(),
      op: BinaryOp::Add,
      left: Box::new(create_string_expr("a")),
      right: Box::new(create_string_expr("b")),
    });

    assert!(
      expr_to_num(&expr, &mut state, &mut traversal_state, &fns).is_err(),
      "a concatenation has no numeric reading"
    );
  }
}

// ──────────────────────────────────────────────
// convert_key_value_to_str - computed non-literal panic
// ──────────────────────────────────────────────

mod convert_key_value_to_str_panic_tests {
  use super::*;
  use swc_core::ecma::ast::{ComputedPropName, KeyValueProp, PropName};

  #[test]
  #[should_panic]
  fn panics_for_computed_non_literal_key() {
    let kv = KeyValueProp {
      key: PropName::Computed(ComputedPropName {
        span: Default::default(),
        expr: Box::new(create_ident_expr("dynamic")),
      }),
      value: Box::new(create_number_expr(0.0)),
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
  use stylex_state::common::fill_state_declarations;
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
    let decl = make_var_declarator("val", create_number_expr(42.0));
    fill_state_declarations(&mut traversal_state, &decl);
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
// convert_unary_to_num - error branches
// ──────────────────────────────────────────────

mod convert_unary_to_num_error_tests {
  use super::*;
  use crate::shared::utils::ast::convertors::convert_unary_to_num;
  use swc_core::ecma::ast::{UnaryExpr, UnaryOp};

  #[test]
  fn minus_num_returns_value() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let unary = UnaryExpr {
      span: Default::default(),
      op: UnaryOp::Minus,
      arg: Box::new(create_number_expr(5.0)),
    };
    let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
    assert_eq!(result, -5.0);
  }

  #[test]
  fn plus_num_returns_value() {
    let mut state = EvaluationState::new();
    let mut traversal_state = StateManager::default();
    let fns = FunctionMap::default();
    let unary = UnaryExpr {
      span: Default::default(),
      op: UnaryOp::Plus,
      arg: Box::new(create_number_expr(5.0)),
    };
    let result = convert_unary_to_num(&unary, &mut state, &mut traversal_state, &fns);
    assert_eq!(result, 5.0);
  }
}
