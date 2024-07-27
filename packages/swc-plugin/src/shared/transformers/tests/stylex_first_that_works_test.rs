#[cfg(test)]
mod stylex_first_that_works {
  use swc_core::ecma::ast::ExprOrSpread;
  use swc_core::ecma::ast::Expr;

  use crate::shared::{
    transformers::stylex_first_that_works::stylex_first_that_works,
    utils::ast::{
      convertors::string_to_expression,
      factories::{array_expression_factory, expr_or_spread_string_expression_factory},
    },
  };

  #[test]
  fn reverses_simple_array_of_values() {
    first_that_works_transform(
      vec![string_to_expression("a"), string_to_expression("b")],
      vec!["b", "a"],
    );

    first_that_works_transform(
      vec![
        string_to_expression("a"),
        string_to_expression("b"),
        string_to_expression("c"),
      ],
      vec!["c", "b", "a"],
    );
  }

  #[test]
  fn creates_fallbacks_for_variables() {
    first_that_works_transform_to_string(
      vec![
        string_to_expression("var(--accent)"),
        string_to_expression("blue"),
      ],
      "var(--accent, blue)",
    );
  }

  #[test]
  fn allow_variables_to_be_fallbacks_too() {
    first_that_works_transform(
      vec![
        string_to_expression("color-mix(in srgb, currentColor 20%, transparent)"),
        string_to_expression("var(--accent)"),
        string_to_expression("blue"),
      ],
      vec![
        "var(--accent, blue)",
        "color-mix(in srgb, currentColor 20%, transparent)",
      ],
    );
  }

  #[test]
  fn omit_all_but_first_fallback_after_the_last_variable() {
    first_that_works_transform(
      vec![
        string_to_expression("color-mix(in oklch, currentColor 20%, transparent)"),
        string_to_expression("color-mix(in srgb, currentColor 20%, transparent)"),
        string_to_expression("var(--accent)"),
        string_to_expression("var(--primary)"),
        string_to_expression("var(--secondary)"),
        string_to_expression("red"),
        string_to_expression("blue"),
        string_to_expression("green"),
      ],
      vec![
        "var(--accent, var(--primary, var(--secondary, red)))",
        "color-mix(in srgb, currentColor 20%, transparent)",
        "color-mix(in oklch, currentColor 20%, transparent)",
      ],
    );
  }
  fn first_that_works_transform(args: Vec<Expr>, expected_values: Vec<&str>) {
    let expected_args = expected_values
      .into_iter()
      .map(|val| Some(expr_or_spread_string_expression_factory(val)))
      .collect::<Vec<Option<ExprOrSpread>>>();

    let result = stylex_first_that_works(args.into_iter().collect());
    let expected_result = array_expression_factory(expected_args);

    assert_eq!(result, expected_result);
  }

  fn first_that_works_transform_to_string(args: Vec<Expr>, expected_value: &str) {
    let result = stylex_first_that_works(args.into_iter().collect());

    assert_eq!(result, string_to_expression(expected_value));
  }
}
