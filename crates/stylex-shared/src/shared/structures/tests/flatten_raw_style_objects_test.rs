#[cfg(test)]
mod flatten_style_object_with_legacy_shorthand_expansion {
  use indexmap::IndexMap;
  use swc_core::ecma::ast::Expr;

  use crate::shared::{
    structures::{
      functions::FunctionMap,
      null_pre_rule::NullPreRule,
      pre_rule::{PreRuleValue, PreRules, StylesPreRule},
      pre_rule_set::PreRuleSet,
      state::EvaluationState,
      state_manager::StateManager,
      stylex_options::StyleResolution,
    },
    utils::{
      ast::{
        convertors::string_to_expression,
        factories::{create_array, key_value_factory},
      },
      core::flatten_raw_style_object::flatten_raw_style_object,
    },
  };

  pub(super) fn get_state() -> StateManager {
    let mut state_manager = StateManager::default();

    state_manager.options.class_name_prefix = "x".to_string();
    state_manager.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state_manager.options.runtime_injection = None;
    state_manager.options.dev = false;
    state_manager.options.test = false;
    state_manager.options.debug = false;

    state_manager
  }

  pub(super) fn pre_rule_factory(key: &str, value: &str, path_key: &[&str]) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::String(value.to_string()),
      Some(path_key.iter().map(|s| s.to_string()).collect()),
    ))
  }

  pub(super) fn pre_rule_set_factory(values: &[PreRules]) -> PreRules {
    PreRuleSet::create(values.to_vec())
  }

  pub(super) fn null_rule_factory() -> PreRules {
    PreRules::NullPreRule(NullPreRule::default())
  }

  pub(super) fn pre_rule_vec_factory(key: &str, value: &[&str], path_key: &[&str]) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::Vec(value.iter().map(|x| x.to_string()).collect()),
      Some(path_key.iter().map(|s| s.to_string()).collect()),
    ))
  }

  #[test]
  fn should_create_pre_rule_objects_for_simple_style_values() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory("color", string_to_expression("red")),
        key_value_factory("marginStart", string_to_expression("10")),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_factory("color", "red", &["color"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10", &["marginStart"]),
    );
    expected_result.insert("marginLeft".to_string(), null_rule_factory());
    expected_result.insert("marginRight".to_string(), null_rule_factory());

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_gap_values() {
    let result = flatten_raw_style_object(
      &[key_value_factory("gap", string_to_expression("10"))],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "rowGap".to_string(),
      pre_rule_factory("rowGap", "10", &["rowGap"]),
    );
    expected_result.insert(
      "columnGap".to_string(),
      pre_rule_factory("columnGap", "10", &["columnGap"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_contain_intrinsic_size_values() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "containIntrinsicSize",
        string_to_expression("10"),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "containIntrinsicWidth".to_string(),
      pre_rule_factory("containIntrinsicWidth", "10", &["containIntrinsicWidth"]),
    );
    expected_result.insert(
      "containIntrinsicHeight".to_string(),
      pre_rule_factory("containIntrinsicHeight", "10", &["containIntrinsicHeight"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_gap_with_space_separated_values() {
    let result = flatten_raw_style_object(
      &[key_value_factory("gap", string_to_expression("10px 20px"))],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "rowGap".to_string(),
      pre_rule_factory("rowGap", "10px", &["rowGap"]),
    );
    expected_result.insert(
      "columnGap".to_string(),
      pre_rule_factory("columnGap", "20px", &["columnGap"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_contain_intrinsic_size_with_space_separated_values() {
    let w = "containIntrinsicWidth";
    let h = "containIntrinsicHeight";

    let result = flatten_raw_style_object(
      &[key_value_factory(
        "containIntrinsicSize",
        string_to_expression("10px 20px"),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(w.to_string(), pre_rule_factory(w, "10px", &[w]));
    expected_result.insert(h.to_string(), pre_rule_factory(h, "20px", &[h]));

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_contain_intrinsic_size_with_space_separated_values_v2() {
    let w = "containIntrinsicWidth";
    let h = "containIntrinsicHeight";

    let result = flatten_raw_style_object(
      &[key_value_factory(
        "containIntrinsicSize",
        string_to_expression("auto 10px 20px"),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(w.to_string(), pre_rule_factory(w, "auto 10px", &[w]));
    expected_result.insert(h.to_string(), pre_rule_factory(h, "20px", &[h]));

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_contain_intrinsic_size_with_space_separated_values_v3() {
    let w = "containIntrinsicWidth";
    let h = "containIntrinsicHeight";

    let result = flatten_raw_style_object(
      &[key_value_factory(
        "containIntrinsicSize",
        string_to_expression("auto 10px auto 20px"),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(w.to_string(), pre_rule_factory(w, "auto 10px", &[w]));
    expected_result.insert(h.to_string(), pre_rule_factory(h, "auto 20px", &[h]));

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_shorthands() {
    let result = flatten_raw_style_object(
      &[key_value_factory("margin", string_to_expression("10"))],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_factory("marginTop", "10", &["marginTop"]),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "10", &["marginEnd"]),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10", &["marginBottom"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10", &["marginStart"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_shorthands_extended() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory("margin", string_to_expression("10")),
        key_value_factory("marginBottom", string_to_expression("20")),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_factory("marginTop", "10", &["marginTop"]),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "10", &["marginEnd"]),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10", &["marginBottom"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10", &["marginStart"]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "20", &["marginBottom"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_shorthands_with_space_separated_values() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory("margin", string_to_expression("10px 20px")),
        key_value_factory("borderColor", string_to_expression("red")),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 8);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_factory("marginTop", "10px", &["marginTop"]),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "20px", &["marginEnd"]),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10px", &["marginBottom"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "20px", &["marginStart"]),
    );

    expected_result.insert(
      "borderTopColor".to_string(),
      pre_rule_factory("borderTopColor", "red", &["borderTopColor"]),
    );
    expected_result.insert(
      "borderEndColor".to_string(),
      pre_rule_factory("borderEndColor", "red", &["borderEndColor"]),
    );
    expected_result.insert(
      "borderBottomColor".to_string(),
      pre_rule_factory("borderBottomColor", "red", &["borderBottomColor"]),
    );
    expected_result.insert(
      "borderStartColor".to_string(),
      pre_rule_factory("borderStartColor", "red", &["borderStartColor"]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_shorthands_with_fallbacks() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "margin",
        Expr::from(create_array(&[
          string_to_expression("10vh 20px"),
          string_to_expression("10dvh 20px"),
        ])),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_vec_factory("marginTop", &["10vh", "10dvh"], &["marginTop"]),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "20px", &["marginEnd"]),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_vec_factory("marginBottom", &["10vh", "10dvh"], &["marginBottom"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "20px", &["marginStart"]),
    );

    assert_eq!(result, expected_result)
  }
}

#[cfg(test)]
mod nested_objects {
  use indexmap::IndexMap;

  use crate::shared::{
    structures::{
      functions::FunctionMap, state::EvaluationState, tests::flatten_raw_style_objects_test::flatten_style_object_with_legacy_shorthand_expansion::{
        get_state, null_rule_factory, pre_rule_factory, pre_rule_set_factory,
      }
    },
    utils::{
      ast::{convertors::string_to_expression, factories::{key_value_factory, object_expression_factory, prop_or_spread_string_factory}, }, core::flatten_raw_style_object::flatten_raw_style_object,
    },
  };

  #[test]
  fn legacy_pseudo_classes() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory("color", string_to_expression("blue")),
        key_value_factory("marginStart", string_to_expression("0")),
        key_value_factory(
          ":hover",
          object_expression_factory(vec![
            prop_or_spread_string_factory("color", "red"),
            prop_or_spread_string_factory("marginStart", "10"),
          ]),
        ),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 8);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_factory("color", "blue", &["color"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "0", &["marginStart"]),
    );
    expected_result.insert("marginLeft".to_string(), null_rule_factory());
    expected_result.insert("marginRight".to_string(), null_rule_factory());
    expected_result.insert(
      ":hover_color".to_string(),
      pre_rule_factory("color", "red", &[":hover", "color"]),
    );

    expected_result.insert(
      ":hover_marginStart".to_string(),
      pre_rule_factory("marginStart", "10", &[":hover", "marginStart"]),
    );

    expected_result.insert(":hover_marginLeft".to_string(), null_rule_factory());

    expected_result.insert(":hover_marginRight".to_string(), null_rule_factory());

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_classes() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "blue"),
            prop_or_spread_string_factory(":hover", "red"),
          ]),
        ),
        key_value_factory(
          "marginStart",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "0"),
            prop_or_spread_string_factory(":hover", "10"),
          ]),
        ),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory("color", "red", &["color", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0", &["marginStart", "default"]),
        pre_rule_factory("marginStart", "10", &["marginStart", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginLeft".to_string(),
      pre_rule_set_factory(&[null_rule_factory(), null_rule_factory()]),
    );

    expected_result.insert(
      "marginRight".to_string(),
      pre_rule_set_factory(&[null_rule_factory(), null_rule_factory()]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_classes_with_shorthands() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "blue"),
            prop_or_spread_string_factory(":hover", "red"),
          ]),
        ),
        key_value_factory(
          "margin",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "0"),
            prop_or_spread_string_factory(":hover", "10"),
          ]),
        ),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 5);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory("color", "red", &["color", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "0", &["marginTop", "default"]),
        pre_rule_factory("marginTop", "10", &["marginTop", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "0", &["marginEnd", "default"]),
        pre_rule_factory("marginEnd", "10", &["marginEnd", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "0", &["marginBottom", "default"]),
        pre_rule_factory("marginBottom", "10", &["marginBottom", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0", &["marginStart", "default"]),
        pre_rule_factory("marginStart", "10", &["marginStart", ":hover"]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_classes_with_complex_shorthands() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "blue"),
            prop_or_spread_string_factory(":hover", "red"),
          ]),
        ),
        key_value_factory(
          "margin",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "1px 2px 3px 4px"),
            prop_or_spread_string_factory(":hover", "10px 20px"),
          ]),
        ),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 5);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory("color", "red", &["color", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "1px", &["marginTop", "default"]),
        pre_rule_factory("marginTop", "10px", &["marginTop", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "2px", &["marginEnd", "default"]),
        pre_rule_factory("marginEnd", "20px", &["marginEnd", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "3px", &["marginBottom", "default"]),
        pre_rule_factory("marginBottom", "10px", &["marginBottom", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "4px", &["marginStart", "default"]),
        pre_rule_factory("marginStart", "20px", &["marginStart", ":hover"]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_and_at_rules() {
    let result = flatten_raw_style_object(
      &[
        key_value_factory(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "blue"),
            prop_or_spread_string_factory(":hover", "red"),
            prop_or_spread_string_factory("@media (min-width: 300px)", "green"),
          ]),
        ),
        key_value_factory(
          "marginStart",
          object_expression_factory(vec![
            prop_or_spread_string_factory("default", "0"),
            prop_or_spread_string_factory(":hover", "10"),
          ]),
        ),
      ],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory("color", "red", &["color", ":hover"]),
        pre_rule_factory("color", "green", &["color", "@media (min-width: 300px)"]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0", &["marginStart", "default"]),
        pre_rule_factory("marginStart", "10", &["marginStart", ":hover"]),
      ]),
    );
    expected_result.insert(
      "marginLeft".to_string(),
      pre_rule_set_factory(&[null_rule_factory(), null_rule_factory()]),
    );

    expected_result.insert(
      "marginRight".to_string(),
      pre_rule_set_factory(&[null_rule_factory(), null_rule_factory()]),
    );

    assert_eq!(result, expected_result)
  }
}

#[cfg(test)]
mod multiple_levels_of_nesting {
  use indexmap::IndexMap;

  use crate::shared::{
    structures::{
      functions::FunctionMap, state::EvaluationState, tests::flatten_raw_style_objects_test::flatten_style_object_with_legacy_shorthand_expansion::{
        get_state, pre_rule_factory, pre_rule_set_factory, pre_rule_vec_factory,
      }
    },
    utils::{
       ast::factories::{key_value_factory, object_expression_factory, prop_or_spread_array_string_factory, prop_or_spread_expr_factory, prop_or_spread_string_factory}, core::flatten_raw_style_object::flatten_raw_style_object
    },
  };

  #[test]
  fn fallback_styles_within_nested_objects() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "margin",
        object_expression_factory(vec![
          prop_or_spread_string_factory("default", "1px 2px 3px 4px"),
          prop_or_spread_array_string_factory(":hover", &["10px 20px", "1dvh 2dvh"]),
        ]),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "1px", &["marginTop", "default"]),
        pre_rule_vec_factory("marginTop", &["10px", "1dvh"], &["marginTop", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "2px", &["marginEnd", "default"]),
        pre_rule_vec_factory("marginEnd", &["20px", "2dvh"], &["marginEnd", ":hover"]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "3px", &["marginBottom", "default"]),
        pre_rule_vec_factory(
          "marginBottom",
          &["10px", "1dvh"],
          &["marginBottom", ":hover"],
        ),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "4px", &["marginStart", "default"]),
        pre_rule_vec_factory("marginStart", &["20px", "2dvh"], &["marginStart", ":hover"]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_within_a_media_query_legacy_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "@media (min-width: 300px)",
        object_expression_factory(vec![prop_or_spread_expr_factory(
          ":hover",
          vec![prop_or_spread_string_factory("color", "red")],
        )]),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "@media (min-width: 300px)_:hover_color".to_string(),
      pre_rule_set_factory(&[pre_rule_factory(
        "color",
        "red",
        &["@media (min-width: 300px)", ":hover", "color"],
      )]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_with_a_pseudo_within_a_media_query_legacy_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "@media (min-width: 300px)",
        object_expression_factory(vec![prop_or_spread_expr_factory(
          ":hover",
          vec![
            prop_or_spread_string_factory("color", "pink"),
            prop_or_spread_expr_factory(
              ":active",
              vec![prop_or_spread_string_factory("color", "red")],
            ),
          ],
        )]),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "@media (min-width: 300px)_:hover_color".to_string(),
      pre_rule_set_factory(&[pre_rule_factory(
        "color",
        "pink",
        &["@media (min-width: 300px)", ":hover", "color"],
      )]),
    );

    expected_result.insert(
      "@media (min-width: 300px)_:hover_:active_color".to_string(),
      pre_rule_set_factory(&[pre_rule_factory(
        "color",
        "red",
        &["@media (min-width: 300px)", ":hover", ":active", "color"],
      )]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_within_a_media_query_modern_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "color",
        object_expression_factory(vec![
          prop_or_spread_string_factory("default", "blue"),
          prop_or_spread_expr_factory(
            "@media (min-width: 300px)",
            vec![prop_or_spread_string_factory(":hover", "red")],
          ),
        ]),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory(
          "color",
          "red",
          &["color", "@media (min-width: 300px)", ":hover"],
        ),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn extra_deep_pseudo_within_a_media_query_modern_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_factory(
        "color",
        object_expression_factory(vec![
          prop_or_spread_string_factory("default", "blue"),
          prop_or_spread_expr_factory(
            "@media (min-width: 300px)",
            vec![prop_or_spread_expr_factory(
              ":hover",
              vec![
                prop_or_spread_string_factory("default", "red"),
                prop_or_spread_string_factory(":active", "maroon"),
              ],
            )],
          ),
        ]),
      )],
      &mut vec![],
      &mut EvaluationState::new(),
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue", &["color", "default"]),
        pre_rule_factory(
          "color",
          "red",
          &["color", "@media (min-width: 300px)", ":hover", "default"],
        ),
        pre_rule_factory(
          "color",
          "maroon",
          &["color", "@media (min-width: 300px)", ":hover", ":active"],
        ),
      ]),
    );

    assert_eq!(result, expected_result)
  }
}
