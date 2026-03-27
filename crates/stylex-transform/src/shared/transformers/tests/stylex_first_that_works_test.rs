#[cfg(test)]
mod stylex_first_that_works {
  use swc_core::ecma::ast::Expr;
  use swc_core::ecma::ast::ExprOrSpread;

  use crate::shared::structures::functions::FunctionMap;
  use crate::shared::structures::state_manager::StateManager;
  use crate::shared::transformers::stylex_first_that_works::stylex_first_that_works;
  use crate::shared::utils::ast::convertors::create_string_expr;
  use stylex_ast::ast::factories::{create_array_expression, create_string_expr_or_spread};

  #[test]
  fn reverses_simple_array_of_values() {
    first_that_works_transform(
      vec![create_string_expr("a"), create_string_expr("b")],
      vec!["b", "a"],
      &mut StateManager::default(),
      &FunctionMap::default(),
    );

    first_that_works_transform(
      vec![
        create_string_expr("a"),
        create_string_expr("b"),
        create_string_expr("c"),
      ],
      vec!["c", "b", "a"],
      &mut StateManager::default(),
      &FunctionMap::default(),
    );
  }

  #[test]
  fn creates_fallbacks_for_variables() {
    first_that_works_transform_to_string(
      vec![
        create_string_expr("var(--accent)"),
        create_string_expr("blue"),
      ],
      "var(--accent, blue)",
      &mut StateManager::default(),
      &FunctionMap::default(),
    );
  }

  #[test]
  fn allow_variables_to_be_fallbacks_too() {
    first_that_works_transform(
      vec![
        create_string_expr("color-mix(in srgb, currentColor 20%, transparent)"),
        create_string_expr("var(--accent)"),
        create_string_expr("blue"),
      ],
      vec![
        "var(--accent, blue)",
        "color-mix(in srgb, currentColor 20%, transparent)",
      ],
      &mut StateManager::default(),
      &FunctionMap::default(),
    );
  }

  #[test]
  fn omit_all_but_first_fallback_after_the_last_variable() {
    first_that_works_transform(
      vec![
        create_string_expr("color-mix(in oklch, currentColor 20%, transparent)"),
        create_string_expr("color-mix(in srgb, currentColor 20%, transparent)"),
        create_string_expr("var(--accent)"),
        create_string_expr("var(--primary)"),
        create_string_expr("var(--secondary)"),
        create_string_expr("red"),
        create_string_expr("blue"),
        create_string_expr("green"),
      ],
      vec![
        "var(--accent, var(--primary, var(--secondary, red)))",
        "color-mix(in srgb, currentColor 20%, transparent)",
        "color-mix(in oklch, currentColor 20%, transparent)",
      ],
      &mut StateManager::default(),
      &FunctionMap::default(),
    );
  }
  fn first_that_works_transform(
    args: Vec<Expr>,
    expected_values: Vec<&str>,
    state: &mut StateManager,
    functions: &FunctionMap,
  ) {
    let expected_args = expected_values
      .into_iter()
      .map(|val| Some(create_string_expr_or_spread(val)))
      .collect::<Vec<Option<ExprOrSpread>>>();

    let result = stylex_first_that_works(args.into_iter().collect(), state, functions);
    let expected_result = create_array_expression(expected_args);

    assert_eq!(result, expected_result);
  }

  fn first_that_works_transform_to_string(
    args: Vec<Expr>,
    expected_value: &str,
    state: &mut StateManager,
    functions: &FunctionMap,
  ) {
    let result = stylex_first_that_works(args.into_iter().collect(), state, functions);

    assert_eq!(result, create_string_expr(expected_value));
  }
}
