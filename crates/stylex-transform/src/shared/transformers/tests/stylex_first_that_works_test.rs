#[cfg(test)]
mod stylex_first_that_works {
  use swc_core::ecma::ast::{Expr, ExprOrSpread};

  use crate::shared::{
    structures::{functions::FunctionMap, state_manager::StateManager},
    transformers::stylex_first_that_works::stylex_first_that_works,
    utils::ast::convertors::create_string_expr,
  };
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

/// The ordering arithmetic on its own, over positions rather than over values.
///
/// Asked here as well as through the transform because two callers share it —
/// the evaluator's expression path and the compile-time engine's — and a case
/// that reads awkwardly as a declaration reads plainly as a list of booleans.
#[cfg(test)]
mod fallback_plan {
  use crate::shared::transformers::stylex_first_that_works::{
    css_variable_name, fold_fallback_chain, plan_fallbacks,
  };

  /// The plan for `is_var`, as `(chain, rest)`, or `None` where there is no
  /// variable to build a chain from.
  fn plan(is_var: &[bool]) -> Option<(Vec<usize>, Vec<usize>)> {
    plan_fallbacks(is_var.len(), |index| is_var[index])
      .map(|fallbacks| (fallbacks.chain, fallbacks.rest))
  }

  #[test]
  fn no_arguments_have_no_plan() {
    assert_eq!(plan(&[]), None);
  }

  #[test]
  fn arguments_with_no_variable_have_no_plan() {
    assert_eq!(plan(&[false]), None);
    assert_eq!(plan(&[false, false, false]), None);
  }

  #[test]
  fn one_variable_is_a_chain_of_one() {
    assert_eq!(plan(&[true]), Some((vec![0], vec![])));
  }

  #[test]
  fn the_chain_runs_from_the_first_variable_to_the_value_after_it() {
    assert_eq!(plan(&[true, false]), Some((vec![1, 0], vec![])));
    assert_eq!(plan(&[true, true, false]), Some((vec![2, 1, 0], vec![])));
  }

  #[test]
  fn the_arguments_before_the_first_variable_follow_the_chain_reversed() {
    assert_eq!(plan(&[false, true]), Some((vec![1], vec![0])));
    assert_eq!(
      plan(&[false, false, true, false]),
      Some((vec![3, 2], vec![1, 0]))
    );
  }

  /// Everything past the value the chain stops on is dropped, which is the one
  /// part of this arithmetic an author is likely to be surprised by.
  #[test]
  fn the_arguments_past_the_chains_end_are_dropped() {
    assert_eq!(
      plan(&[false, true, true, false, false, false]),
      Some((vec![3, 2, 1], vec![0]))
    );
  }

  #[test]
  fn only_variables_run_the_chain_to_the_end() {
    assert_eq!(plan(&[true, true, true]), Some((vec![2, 1, 0], vec![])));
  }

  /// A thousand variables, which is past anything a stylesheet writes and still
  /// one walk with no recursion in it.
  #[test]
  fn a_very_long_argument_list_plans_in_one_walk() {
    let is_var = vec![true; 1000];
    let Some((chain, rest)) = plan(&is_var) else {
      panic!("expected a chain over every argument");
    };

    assert_eq!(chain.len(), 1000);
    assert_eq!(chain.first(), Some(&999));
    assert_eq!(chain.last(), Some(&0));
    assert!(rest.is_empty());
  }

  #[test]
  fn an_empty_chain_folds_to_nothing() {
    assert_eq!(fold_fallback_chain(Vec::new()), "");
  }

  /// One part that is a value stays that value; one that is a variable name is
  /// wrapped, since a name is not a reference until it is.
  #[test]
  fn one_part_folds_to_itself_or_to_a_reference() {
    assert_eq!(fold_fallback_chain(vec!["red".to_string()]), "red");
    assert_eq!(fold_fallback_chain(vec!["--x".to_string()]), "var(--x)");
  }

  /// An empty first part leaves the chain empty, so the part after it is still
  /// the innermost — which is why `firstThatWorks('var(--p)', '')` answers the
  /// bare reference.
  #[test]
  fn an_empty_part_does_not_start_the_chain() {
    assert_eq!(
      fold_fallback_chain(vec![String::new(), "--p".to_string()]),
      "var(--p)"
    );
  }

  #[test]
  fn later_parts_wrap_the_chain_so_far() {
    assert_eq!(
      fold_fallback_chain(vec!["blue".to_string(), "--accent".to_string()]),
      "var(--accent, blue)"
    );

    assert_eq!(
      fold_fallback_chain(vec![
        "red".to_string(),
        "--c".to_string(),
        "--b".to_string(),
        "--a".to_string(),
      ]),
      "var(--a, var(--b, var(--c, red)))"
    );
  }

  /// A bare reference reads as the name inside it; anything else reads as no
  /// variable at all, which is what keeps the name from ever being sliced out of
  /// text that has none.
  #[test]
  fn only_a_bare_single_reference_reads_as_a_variable() {
    assert_eq!(css_variable_name("var(--x)"), Some("--x"));
    assert_eq!(css_variable_name("var(--token_1)"), Some("--token_1"));
    assert_eq!(css_variable_name("var(--a-b_c1)"), Some("--a-b_c1"));

    assert_eq!(css_variable_name("var(--x, red)"), None);
    assert_eq!(css_variable_name("var(x)"), None);
    assert_eq!(css_variable_name("var()"), None);
    assert_eq!(css_variable_name("--x"), None);
    assert_eq!(css_variable_name(""), None);
    assert_eq!(css_variable_name(")"), None);
    assert_eq!(css_variable_name("VAR(--x)"), None);
    assert_eq!(css_variable_name(" var(--x)"), None);
    assert_eq!(css_variable_name("var(--é)"), None);
  }
}
