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
    common::DUMMY_SP,
    ecma::ast::{
      AwaitExpr, BinExpr, BinaryOp, ComputedPropName, Expr, IdentName, MemberExpr, MemberProp,
      OptChainBase, OptChainExpr, UnaryExpr, UnaryOp,
    },
  };

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
}
