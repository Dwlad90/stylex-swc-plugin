#[cfg(test)]
mod tests {
  use crate::shared::{
    structures::{
      functions::FunctionMap, state_manager::StateManager, stylex_options::StyleXOptions,
    },
    utils::{
      ast::{
        convertors::{
          bool_to_expression, null_to_expression, number_to_expression, string_to_expression,
        },
        factories::ident_factory,
      },
      js::evaluate::evaluate,
    },
  };
  use swc_core::{
    common::{Globals, GLOBALS, DUMMY_SP},
    ecma::ast::{
      ArrayLit, AwaitExpr, BinExpr, BinaryOp, CallExpr, Callee, ComputedPropName, Expr,
      ExprOrSpread, IdentName, KeyValueProp, MemberExpr, MemberProp, ObjectLit, OptChainBase,
      OptChainExpr, Prop, PropName, PropOrSpread, Regex, UnaryExpr, UnaryOp,
    },
  };
  use swc_core::common::util::take::Take;

  // ==================== HELPER FUNCTIONS ====================

  // Helper: Create undefined expression
  fn make_undefined_expr() -> Expr {
    Expr::Ident(ident_factory("undefined"))
  }

  // Helper: Create identifier expression
  fn make_ident_expr(name: &str) -> Expr {
    Expr::Ident(ident_factory(name))
  }

  // Helper: Create regular member expression
  fn make_member_expr(obj: Expr, prop: &str) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(obj),
      prop: MemberProp::Ident(IdentName::new(prop.into(), DUMMY_SP)),
    })
  }

  // Helper: Create optional member expression
  fn make_optional_member_expr(obj: Expr, prop: &str) -> Expr {
    Expr::OptChain(OptChainExpr {
      span: DUMMY_SP,
      optional: true,
      base: Box::new(OptChainBase::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(obj),
        prop: MemberProp::Ident(IdentName::new(prop.into(), DUMMY_SP)),
      })),
    })
  }

  // Helper: Create unary expression (e.g., -5, !true, +x)
  fn make_unary_expr(op: UnaryOp, arg: Expr) -> Expr {
    Expr::Unary(UnaryExpr {
      span: DUMMY_SP,
      op,
      arg: Box::new(arg),
    })
  }

  // Helper: Create binary expression (e.g., 5 + 3, x && y)
  fn make_binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Bin(BinExpr {
      span: DUMMY_SP,
      left: Box::new(left),
      op,
      right: Box::new(right),
    })
  }

  // Helper: Create await expression
  fn make_await_expr(arg: Expr) -> Expr {
    Expr::Await(AwaitExpr {
      span: DUMMY_SP,
      arg: Box::new(arg),
    })
  }

  fn evaluate_expr(expr: &Expr) -> (bool, bool) {
    let mut state_manager = StateManager::new(StyleXOptions::default());
    let fns = FunctionMap::default();
    let result = evaluate(expr, &mut state_manager, &fns);
    (result.confident, result.value.is_some())
  }

  // ==================== OPTIONAL CHAINING TESTS ====================

  #[test]
  fn test_optional_chaining_with_null_returns_none() {
    let opt_chain = make_optional_member_expr(null_to_expression(), "prop");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    assert!(!has_value, "Optional chaining with null should return None");
  }

  #[test]
  fn test_optional_chaining_with_undefined_returns_none() {
    let opt_chain = make_optional_member_expr(make_undefined_expr(), "prop");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    assert!(
      !has_value,
      "Optional chaining with undefined should return None"
    );
  }

  #[test]
  fn test_optional_chaining_with_variable_not_confident() {
    let opt_chain = make_optional_member_expr(make_ident_expr("obj"), "prop");
    let (confident, _has_value) = evaluate_expr(&opt_chain);
    assert!(
      !confident,
      "Optional chaining with variable should not be confident"
    );
  }

  #[test]
  fn test_optional_chaining_null_no_panic() {
    let opt_chain = make_optional_member_expr(null_to_expression(), "nonexistent");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    assert!(!has_value, "Should short-circuit without panic");
  }

  #[test]
  fn test_optional_chaining_with_nested_member() {
    let opt_chain = make_optional_member_expr(make_ident_expr("obj"), "nested");
    let (confident, _has_value) = evaluate_expr(&opt_chain);
    assert!(!confident, "Variable reference should not be confident");
  }

  #[test]
  fn test_optional_chaining_null_short_circuit() {
    let opt_chain = make_optional_member_expr(null_to_expression(), "complexProp");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    assert!(!has_value, "Should short-circuit on null");
  }

  #[test]
  fn test_optional_chaining_multiple_levels() {
    let chain = make_optional_member_expr(
      make_optional_member_expr(make_optional_member_expr(make_ident_expr("obj"), "a"), "b"),
      "c",
    );
    let (confident, _has_value) = evaluate_expr(&chain);
    assert!(!confident, "Variable reference should not be confident");
  }

  #[test]
  fn test_optional_chaining_undefined_short_circuits() {
    let opt_chain = make_optional_member_expr(make_undefined_expr(), "prop");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    assert!(!has_value, "Should short-circuit on undefined");
  }

  #[test]
  fn test_optional_chaining_computed_property() {
    let opt_chain = Expr::OptChain(OptChainExpr {
      span: DUMMY_SP,
      optional: true,
      base: Box::new(OptChainBase::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(make_ident_expr("obj")),
        prop: MemberProp::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: Box::new(string_to_expression("prop")),
        }),
      })),
    });
    let (confident, _has_value) = evaluate_expr(&opt_chain);
    assert!(!confident, "Variable reference should not be confident");
  }

  #[test]
  fn test_optional_vs_regular_member_access() {
    let regular = make_member_expr(make_ident_expr("obj"), "prop");
    let optional = make_optional_member_expr(make_ident_expr("obj"), "prop");

    let (regular_confident, _) = evaluate_expr(&regular);
    let (optional_confident, _) = evaluate_expr(&optional);

    assert!(
      !regular_confident,
      "Regular member with variable should not be confident"
    );
    assert!(
      !optional_confident,
      "Optional member with variable should not be confident"
    );
  }

  // ==================== AWAIT EXPRESSION TESTS ====================

  #[test]
  fn test_await_with_variable() {
    let await_expr = make_await_expr(make_ident_expr("someVar"));
    let (confident, _has_value) = evaluate_expr(&await_expr);
    assert!(!confident, "Variable reference should not be confident");
  }

  #[test]
  fn test_await_with_null() {
    let await_expr = make_await_expr(null_to_expression());
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(has_value, "Null literal should evaluate to a value");
  }

  #[test]
  fn test_await_with_number() {
    let await_expr = make_await_expr(number_to_expression(42.0));
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(has_value, "Number literal should evaluate to a value");
  }

  #[test]
  fn test_await_with_string() {
    let await_expr = make_await_expr(string_to_expression("hello"));
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(has_value, "String literal should evaluate to a value");
  }

  #[test]
  fn test_await_with_boolean() {
    let await_expr = make_await_expr(bool_to_expression(true));
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(has_value, "Boolean literal should evaluate to a value");
  }

  #[test]
  fn test_await_removes_await_keyword() {
    // Test that await evaluates its argument (not the await keyword itself)
    let await_expr = make_await_expr(number_to_expression(123.0));
    let (_confident, has_value) = evaluate_expr(&await_expr);
    // Should have a value since we evaluate the number
    assert!(has_value, "Await should evaluate its argument");
  }

  #[test]
  fn test_await_with_optional_chaining() {
    // Test: await obj?.prop
    let optional_member = make_optional_member_expr(make_ident_expr("obj"), "prop");
    let await_expr = make_await_expr(optional_member);
    let (confident, _has_value) = evaluate_expr(&await_expr);
    assert!(!confident, "Variable in await should not be confident");
  }

  #[test]
  fn test_await_with_null_optional_chaining() {
    // Test: await null?.prop
    let optional_member = make_optional_member_expr(null_to_expression(), "prop");
    let await_expr = make_await_expr(optional_member);
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(!has_value, "Short-circuit should propagate through await");
  }

  // ==================== LITERAL EXPRESSION TESTS ====================

  #[test]
  fn test_null_literal_evaluation() {
    let null_expr = null_to_expression();
    let (confident, has_value) = evaluate_expr(&null_expr);
    assert!(confident, "Null literal should be confident");
    assert!(has_value, "Null literal should have a value");
  }

  #[test]
  fn test_undefined_identifier_evaluation() {
    let undef_expr = make_undefined_expr();
    let (confident, has_value) = evaluate_expr(&undef_expr);
    assert!(confident, "Undefined identifier should be confident");
    assert!(has_value, "Undefined identifier should have a value");
  }

  #[test]
  fn test_number_literal_evaluation() {
    let num_expr = number_to_expression(42.0);
    let (confident, has_value) = evaluate_expr(&num_expr);
    assert!(confident, "Number literal should be confident");
    assert!(has_value, "Number literal should have a value");
  }

  #[test]
  fn test_string_literal_evaluation() {
    let str_expr = string_to_expression("test");
    let (confident, has_value) = evaluate_expr(&str_expr);
    assert!(confident, "String literal should be confident");
    assert!(has_value, "String literal should have a value");
  }

  #[test]
  fn test_boolean_literal_evaluation() {
    let bool_expr = bool_to_expression(true);
    let (confident, has_value) = evaluate_expr(&bool_expr);
    assert!(confident, "Boolean literal should be confident");
    assert!(has_value, "Boolean literal should have a value");
  }

  #[test]
  fn test_variable_reference_not_confident() {
    let var_expr = make_ident_expr("myVar");
    let (confident, _has_value) = evaluate_expr(&var_expr);
    assert!(!confident, "Variable reference should not be confident");
  }

  // ==================== UNARY EXPRESSION TESTS ====================

  #[test]
  fn test_unary_minus_number() {
    let unary_expr = make_unary_expr(UnaryOp::Minus, number_to_expression(5.0));
    let (confident, has_value) = evaluate_expr(&unary_expr);
    assert!(confident, "Unary minus on number should be confident");
    assert!(has_value, "Unary minus should have a value");
  }

  #[test]
  fn test_unary_plus_number() {
    let unary_expr = make_unary_expr(UnaryOp::Plus, number_to_expression(5.0));
    let (confident, has_value) = evaluate_expr(&unary_expr);
    assert!(confident, "Unary plus on number should be confident");
    assert!(has_value, "Unary plus should have a value");
  }

  #[test]
  fn test_unary_not_boolean() {
    let unary_expr = make_unary_expr(UnaryOp::Bang, bool_to_expression(true));
    let (confident, has_value) = evaluate_expr(&unary_expr);
    assert!(confident, "Unary not on boolean should be confident");
    assert!(has_value, "Unary not should have a value");
  }

  #[test]
  fn test_unary_minus_variable() {
    let unary_expr = make_unary_expr(UnaryOp::Minus, make_ident_expr("x"));
    let (confident, _has_value) = evaluate_expr(&unary_expr);
    assert!(
      !confident,
      "Unary minus on variable should not be confident"
    );
  }

  // ==================== BINARY EXPRESSION TESTS ====================

  #[test]
  fn test_binary_addition_numbers() {
    let bin_expr = make_binary_expr(
      number_to_expression(5.0),
      BinaryOp::Add,
      number_to_expression(3.0),
    );
    let (confident, has_value) = evaluate_expr(&bin_expr);
    assert!(confident, "Binary addition of numbers should be confident");
    assert!(has_value, "Binary addition should have a value");
  }

  #[test]
  fn test_binary_subtraction_numbers() {
    let bin_expr = make_binary_expr(
      number_to_expression(5.0),
      BinaryOp::Sub,
      number_to_expression(3.0),
    );
    let (confident, has_value) = evaluate_expr(&bin_expr);
    assert!(
      confident,
      "Binary subtraction of numbers should be confident"
    );
    assert!(has_value, "Binary subtraction should have a value");
  }

  #[test]
  fn test_binary_multiplication_numbers() {
    let bin_expr = make_binary_expr(
      number_to_expression(5.0),
      BinaryOp::Mul,
      number_to_expression(3.0),
    );
    let (confident, has_value) = evaluate_expr(&bin_expr);
    assert!(
      confident,
      "Binary multiplication of numbers should be confident"
    );
    assert!(has_value, "Binary multiplication should have a value");
  }

  #[test]
  fn test_binary_with_variable() {
    let bin_expr = make_binary_expr(
      make_ident_expr("x"),
      BinaryOp::Add,
      number_to_expression(5.0),
    );
    let (confident, _has_value) = evaluate_expr(&bin_expr);
    assert!(
      !confident,
      "Binary expression with variable should not be confident"
    );
  }

  #[test]
  fn test_binary_logical_and() {
    let bin_expr = make_binary_expr(
      bool_to_expression(true),
      BinaryOp::LogicalAnd,
      bool_to_expression(false),
    );
    let (_confident, has_value) = evaluate_expr(&bin_expr);
    assert!(
      has_value || !_confident,
      "Logical AND should either have value or not be confident"
    );
  }

  #[test]
  fn test_binary_logical_or() {
    let bin_expr = make_binary_expr(
      bool_to_expression(true),
      BinaryOp::LogicalOr,
      bool_to_expression(false),
    );
    let (_confident, has_value) = evaluate_expr(&bin_expr);
    assert!(
      has_value || !_confident,
      "Logical OR should either have value or not be confident"
    );
  }

  // ==================== MEMBER EXPRESSION TESTS ====================

  #[test]
  fn test_member_access_with_variable() {
    let member_expr = make_member_expr(make_ident_expr("obj"), "prop");
    let (confident, _has_value) = evaluate_expr(&member_expr);
    assert!(
      !confident,
      "Member access on variable should not be confident"
    );
  }

  #[test]
  fn test_member_access_nested() {
    let nested = make_member_expr(make_member_expr(make_ident_expr("obj"), "a"), "b");
    let (confident, _has_value) = evaluate_expr(&nested);
    assert!(
      !confident,
      "Nested member access on variable should not be confident"
    );
  }

  // ==================== COMPOSITE TESTS ====================

  #[test]
  fn test_nested_optional_chaining() {
    let nested = make_optional_member_expr(
      make_optional_member_expr(make_ident_expr("obj"), "prop1"),
      "prop2",
    );
    let (confident, _has_value) = evaluate_expr(&nested);
    assert!(
      !confident,
      "Nested optional chains should not be confident with variables"
    );
  }

  #[test]
  fn test_await_then_optional_chain() {
    let optional = make_optional_member_expr(null_to_expression(), "prop");
    let await_expr = make_await_expr(optional);
    let (_confident, has_value) = evaluate_expr(&await_expr);
    assert!(
      !has_value,
      "Optional chain short-circuit should propagate through await"
    );
  }

  #[test]
  fn test_unary_on_optional_chain() {
    let optional = make_optional_member_expr(make_ident_expr("obj"), "prop");
    let unary = make_unary_expr(UnaryOp::Bang, optional);
    let (confident, _has_value) = evaluate_expr(&unary);
    assert!(
      !confident,
      "Unary on optional chain should not be confident with variable"
    );
  }

  #[test]
  fn test_binary_on_optional_chains() {
    let opt1 = make_optional_member_expr(make_ident_expr("a"), "x");
    let opt2 = make_optional_member_expr(make_ident_expr("b"), "y");
    let binary = make_binary_expr(opt1, BinaryOp::Add, opt2);
    let (confident, _has_value) = evaluate_expr(&binary);
    assert!(
      !confident,
      "Binary on optional chains should not be confident with variables"
    );
  }

  #[test]
  fn test_multiple_levels_of_optional_chaining_with_null_at_end() {
    let null_at_end = make_optional_member_expr(null_to_expression(), "prop");
    let (_confident, has_value) = evaluate_expr(&null_at_end);
    assert!(!has_value, "Should handle null at any level");
  }

  #[test]
  fn test_await_with_multiple_chained_operations() {
    // await (await someVar)
    let inner_await = make_await_expr(make_ident_expr("someVar"));
    let outer_await = make_await_expr(inner_await);
    let (confident, _has_value) = evaluate_expr(&outer_await);
    assert!(
      !confident,
      "Nested awaits with variables should not be confident"
    );
  }

  #[test]
  fn test_complex_expression_await_optional_unary() {
    // Complex: await !(obj?.prop)
    let optional = make_optional_member_expr(make_ident_expr("obj"), "prop");
    let unary = make_unary_expr(UnaryOp::Bang, optional);
    let await_expr = make_await_expr(unary);
    let (confident, _has_value) = evaluate_expr(&await_expr);
    assert!(
      !confident,
      "Complex expression should not be confident with variables"
    );
  }

  #[test]
  fn test_null_coalescing_like_pattern() {
    // Pattern: null?.prop (similar to nullish coalescing behavior)
    let optional = make_optional_member_expr(null_to_expression(), "prop");
    let (_confident, has_value) = evaluate_expr(&optional);
    assert!(!has_value, "Null coalescing pattern should short-circuit");
  }

  // ==================== LITERAL MEMBER ACCESS TESTS ====================

  #[test]
  fn test_null_literal_member_access() {
    // When accessing a member on a literal, the literal itself is evaluated
    let member_expr = make_member_expr(null_to_expression(), "prop");
    let (_confident, has_value) = evaluate_expr(&member_expr);
    // The literal is evaluable, so has_value should be true (the literal value)
    assert!(
      has_value,
      "Member access on null literal should return the literal value"
    );
  }

  #[test]
  fn test_number_literal_member_access() {
    // When accessing a member on a number literal, the number itself is returned
    let member_expr = make_member_expr(number_to_expression(42.0), "prop");
    let (_confident, has_value) = evaluate_expr(&member_expr);
    // The literal is evaluable
    assert!(
      has_value,
      "Member access on number literal should return the literal value"
    );
  }

  #[test]
  fn test_boolean_literal_member_access() {
    // When accessing a member on a boolean literal, the boolean itself is returned
    let member_expr = make_member_expr(bool_to_expression(true), "prop");
    let (_confident, has_value) = evaluate_expr(&member_expr);
    // The literal is evaluable
    assert!(
      has_value,
      "Member access on boolean literal should return the literal value"
    );
  }

  #[test]
  fn test_string_literal_member_access() {
    // When accessing a member on a string literal, the string itself is returned
    let member_expr = make_member_expr(string_to_expression("test"), "prop");
    let (_confident, has_value) = evaluate_expr(&member_expr);
    // The literal is evaluable
    assert!(
      has_value,
      "Member access on string literal should return the literal value"
    );
  }

  #[test]
  fn test_literal_with_optional_chaining() {
    // Optional chaining on a literal number: the number literal is returned
    let opt_chain = make_optional_member_expr(number_to_expression(5.0), "prop");
    let (_confident, has_value) = evaluate_expr(&opt_chain);
    // The number is a literal so it can be evaluated
    assert!(
      has_value,
      "Optional member access on number literal should return the literal value"
    );
  }

  // ==================== REGEX EXPRESSION TESTS ====================


  // Helper: Create regex literal expression
  fn make_regex_expr(pattern: &str, flags: &str) -> Expr {
    Expr::Lit(swc_core::ecma::ast::Lit::Regex(Regex {
      span: DUMMY_SP,
      exp: pattern.into(),
      flags: flags.into(),
    }))
  }

  // Helper: Create call expression
  fn make_call_expr(callee: Expr, args: Vec<Expr>) -> Expr {
    Expr::Call(CallExpr {
      span: DUMMY_SP,
      callee: Callee::Expr(Box::new(callee)),
      args: args
        .into_iter()
        .map(|arg| ExprOrSpread {
          spread: None,
          expr: Box::new(arg),
        })
        .collect(),
      type_args: None,
      ..CallExpr::dummy()
    })
  }

  #[test]
  fn test_regex_literal_evaluation() {
    let regex_expr = make_regex_expr("test", "g");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(confident, "Regex literal should be confident");
    assert!(has_value, "Regex literal should have a value");
  }

  #[test]
  fn test_regex_with_escaped_chars() {
    let regex_expr = make_regex_expr("\\/regex", "");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(
      confident,
      "Regex with escaped characters should be confident"
    );
    assert!(
      has_value,
      "Regex with escaped characters should have a value"
    );
  }

  #[test]
  fn test_regex_with_flags() {
    let regex_expr = make_regex_expr("test", "gi");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(confident, "Regex with flags should be confident");
    assert!(has_value, "Regex with flags should have a value");
  }

  #[test]
  fn test_regex_without_flags() {
    let regex_expr = make_regex_expr("^hello.*world$", "");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(confident, "Regex without flags should be confident");
    assert!(has_value, "Regex without flags should have a value");
  }

  #[test]
  fn test_regex_test_method_with_variable() {
    // Test: /regex/.test(someVar)
    let regex_expr = make_regex_expr("test", "");
    let member_expr = make_member_expr(regex_expr, "test");
    let call_expr = make_call_expr(member_expr, vec![make_ident_expr("someVar")]);
    let (confident, _has_value) = evaluate_expr(&call_expr);
    assert!(
      !confident,
      "Regex.test() with variable should not be confident"
    );
  }

  #[test]
  fn test_regex_test_method_with_literal() {
    // Test: /regex/.test("literal")
    let regex_expr = make_regex_expr("test", "");
    let member_expr = make_member_expr(regex_expr, "test");
    let call_expr = make_call_expr(member_expr, vec![string_to_expression("literal")]);
    let (confident, _has_value) = evaluate_expr(&call_expr);
    assert!(
      !confident,
      "Regex.test() cannot be statically evaluated, should not be confident"
    );
  }

  #[test]
  fn test_regex_exec_method() {
    // Test: /regex/.exec(someVar)
    let regex_expr = make_regex_expr("test", "");
    let member_expr = make_member_expr(regex_expr, "exec");
    let call_expr = make_call_expr(member_expr, vec![make_ident_expr("someVar")]);
    let (confident, _has_value) = evaluate_expr(&call_expr);
    assert!(
      !confident,
      "Regex.exec() should not be confident (cannot be statically evaluated)"
    );
  }

  #[test]
  fn test_regex_match_method() {
    // Test: /regex/.match()
    let regex_expr = make_regex_expr("test", "");
    let member_expr = make_member_expr(regex_expr, "match");
    let call_expr = make_call_expr(member_expr, vec![]);
    let (confident, _has_value) = evaluate_expr(&call_expr);
    assert!(
      !confident,
      "Regex.match() should not be confident (cannot be statically evaluated)"
    );
  }

  #[test]
  fn test_complex_regex_pattern() {
    let regex_expr = make_regex_expr("^[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}$", "");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(confident, "Complex regex pattern should be confident");
    assert!(has_value, "Complex regex pattern should have a value");
  }

  #[test]
  fn test_regex_with_unicode_flag() {
    let regex_expr = make_regex_expr("\\p{Emoji}", "u");
    let (confident, has_value) = evaluate_expr(&regex_expr);
    assert!(confident, "Regex with unicode flag should be confident");
    assert!(has_value, "Regex with unicode flag should have a value");
  }

  #[test]
  fn test_regex_test_method_with_nullish_coalescing() {
    // Test: /regex/.test(someVar ?? '')
    let regex_expr = make_regex_expr("\\/test", "");
    let member_expr = make_member_expr(regex_expr, "test");
    // Using binary expression for nullish coalescing
    let nullish = make_binary_expr(
      make_ident_expr("pattern"),
      BinaryOp::NullishCoalescing,
      string_to_expression(""),
    );
    let call_expr = make_call_expr(member_expr, vec![nullish]);
    let (confident, _has_value) = evaluate_expr(&call_expr);
    assert!(
      !confident,
      "Regex.test() with nullish coalescing should not be confident"
    );
  }

  // ==================== PANIC CONTEXT TESTS ====================
  // These tests verify that panics in the member expression evaluation path
  // include useful context (e.g., property names) rather than generic messages.


  // Helper: Create array literal expression
  fn make_array_expr(elems: Vec<Expr>) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: elems
        .into_iter()
        .map(|e| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(e),
          })
        })
        .collect(),
    })
  }

  // Helper: evaluate with SWC GLOBALS set (needed for panic_with_context! code paths)
  fn evaluate_expr_with_globals(expr: &Expr) -> (bool, bool) {
    let globals = Globals::default();
    GLOBALS.set(&globals, || evaluate_expr(expr))
  }

  #[test]
  fn test_unsupported_array_method_panic_includes_method_name() {
    // Calling an unsupported method on an array literal (e.g., [1].unsupported())
    // should panic with a message that includes the method name.
    // This validates that panic_with_context! is used in the member call evaluation path.
    let array = make_array_expr(vec![number_to_expression(1.0)]);
    let member = make_member_expr(array, "unsupported");
    let call = make_call_expr(member, vec![]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
      evaluate_expr_with_globals(&call);
    }));

    assert!(result.is_err(), "Should panic for unsupported array method");
    let panic_msg = result
      .unwrap_err()
      .downcast_ref::<String>()
      .cloned()
      .unwrap_or_default();
    assert!(
      panic_msg.contains("unsupported"),
      "Panic message should contain the method name 'unsupported', got: {}",
      panic_msg
    );
  }

  #[test]
  fn test_unsupported_string_method_panic_includes_method_name() {
    // Calling an unsupported method on a string literal (e.g., "hello".unsupported())
    // should panic with a message that includes the method name.
    let string = string_to_expression("hello");
    let member = make_member_expr(string, "unsupported");
    let call = make_call_expr(member, vec![]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
      evaluate_expr_with_globals(&call);
    }));

    assert!(result.is_err(), "Should panic for unsupported string method");
    let panic_msg = result
      .unwrap_err()
      .downcast_ref::<String>()
      .cloned()
      .unwrap_or_default();
    assert!(
      panic_msg.contains("unsupported"),
      "Panic message should contain the method name 'unsupported', got: {}",
      panic_msg
    );
  }

  #[test]
  fn test_supported_array_methods_no_panic() {
    // Calling supported methods like .join() should not panic during evaluation.
    // (.map() and .filter() need callback args, but .join() can work without)
    let array = make_array_expr(vec![
      number_to_expression(1.0),
      number_to_expression(2.0),
    ]);
    let member = make_member_expr(array, "join");
    let call = make_call_expr(member, vec![string_to_expression(",")]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
      evaluate_expr_with_globals(&call);
    }));

    assert!(
      result.is_ok(),
      "Supported array method .join() should not panic"
    );
  }

  // ==================== ERROR REASON KEY-PATH TESTS ====================
  // These tests verify that when evaluating an object expression with a
  // non-static value, the error reason includes the property key name.

  fn evaluate_expr_full(expr: &Expr) -> (bool, Option<String>) {
    let mut state_manager = StateManager::new(StyleXOptions::default());
    let fns = FunctionMap::default();
    let result = evaluate(expr, &mut state_manager, &fns);
    (result.confident, result.reason)
  }

  #[test]
  fn test_object_eval_failure_reason_includes_property_key() {
    // Evaluating { color: someVar } should produce a reason that includes "color"
    let obj = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(IdentName::new("color".into(), DUMMY_SP)),
        value: Box::new(make_ident_expr("someVar")),
      })))],
    });

    let (confident, reason) = evaluate_expr_full(&obj);
    assert!(!confident, "Object with variable value should not be confident");
    let reason_str = reason.expect("Should have a reason for the failure");
    assert!(
      reason_str.contains("color"),
      "Reason should contain the property key 'color', got: {}",
      reason_str
    );
  }

  #[test]
  fn test_object_eval_failure_reason_includes_nested_key() {
    // Evaluating { backgroundColor: someVar } should include "backgroundColor" in reason
    let obj = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(IdentName::new("backgroundColor".into(), DUMMY_SP)),
        value: Box::new(make_ident_expr("dynamicValue")),
      })))],
    });

    let (confident, reason) = evaluate_expr_full(&obj);
    assert!(!confident, "Object with variable value should not be confident");
    let reason_str = reason.expect("Should have a reason for the failure");
    assert!(
      reason_str.contains("backgroundColor"),
      "Reason should contain the property key 'backgroundColor', got: {}",
      reason_str
    );
  }
}
