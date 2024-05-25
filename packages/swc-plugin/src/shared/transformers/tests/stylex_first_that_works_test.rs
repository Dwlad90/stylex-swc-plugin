#[cfg(test)]
mod stylex_first_that_works {
  use swc_core::ecma::ast::ExprOrSpread;

  use crate::shared::{
    transformers::stylex_first_that_works::stylex_first_that_works,
    utils::ast::{
      convertors::string_to_expression,
      factories::{array_expression_factory, expr_or_spread_string_expression_factory},
    },
  };

  #[test]
  fn reverses_simple_array_of_values() {
    let args = vec![
      string_to_expression("a"),
      string_to_expression("b"),
      string_to_expression("c"),
    ];

    let expected_args = vec!["c", "b", "a"]
      .into_iter()
      .map(|val| Option::Some(expr_or_spread_string_expression_factory(val)))
      .collect::<Vec<Option<ExprOrSpread>>>();

    let result = stylex_first_that_works(args.into_iter().flatten().collect());
    let expected_result = array_expression_factory(expected_args).unwrap();

    assert_eq!(result, expected_result);
  }
}
