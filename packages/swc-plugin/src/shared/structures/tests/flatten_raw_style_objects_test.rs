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
      state_manager::StateManager,
      stylex_options::StyleResolution,
    },
    utils::{
      common::{create_array, key_value_creator, string_to_expression},
      css::stylex::flatten_raw_style_object::flatten_raw_style_object,
    },
  };

  pub(super) fn get_state() -> StateManager {
    let mut state_manager = StateManager::default();

    state_manager.options.class_name_prefix = "x".to_string();
    state_manager.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state_manager.options.runtime_injection = Option::None;
    state_manager.options.use_rem_for_font_size = true;
    state_manager.options.dev = false;
    state_manager.options.test = false;

    state_manager
  }

  pub(super) fn pre_rule_factory(key: &str, value: &str) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::String(value.to_string()),
      Option::None,
      Option::None,
    ))
  }

  pub(super) fn pre_rule_set_factory(values: &[PreRules]) -> PreRules {
    PreRuleSet::create(values.to_vec())
  }

  pub(super) fn null_rule_factory() -> PreRules {
    PreRules::NullPreRule(NullPreRule::default())
  }

  pub(super) fn pre_rule_with_pseudos_factory(
    key: &str,
    value: &str,
    pseudos: &[&str],
    at_rules: &[&str],
  ) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::String(value.to_string()),
      Option::Some(pseudos.iter().map(|s| s.to_string()).collect()),
      Option::Some(at_rules.iter().map(|s| s.to_string()).collect()),
    ))
  }

  pub(super) fn pre_rule_vec_factory(key: &str, value: &[&str]) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::Vec(value.iter().map(|x| x.to_string()).collect()),
      Option::None,
      Option::None,
    ))
  }

  pub(super) fn pre_rule_vec_with_pseudos_factory(
    key: &str,
    value: &[&str],
    pseudos: &[&str],
    at_rules: &[&str],
  ) -> PreRules {
    PreRules::StylesPreRule(StylesPreRule::new(
      key,
      PreRuleValue::Vec(value.iter().map(|x| x.to_string()).collect()),
      Option::Some(pseudos.iter().map(|s| s.to_string()).collect()),
      Option::Some(at_rules.iter().map(|s| s.to_string()).collect()),
    ))
  }

  #[test]
  fn converts_style_to_class_name() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator("color", string_to_expression("red").unwrap()),
        key_value_creator("marginStart", string_to_expression("10").unwrap()),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert("color".to_string(), pre_rule_factory("color", "red"));
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10"),
    );
    expected_result.insert("marginLeft".to_string(), null_rule_factory());
    expected_result.insert("marginRight".to_string(), null_rule_factory());

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_shorthands() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "margin",
        string_to_expression("10").unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert("marginTop".to_string(), pre_rule_factory("marginTop", "10"));
    expected_result.insert("marginEnd".to_string(), pre_rule_factory("marginEnd", "10"));
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10"),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10"),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_simple_shorthands_extended() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator("margin", string_to_expression("10").unwrap()),
        key_value_creator("marginBottom", string_to_expression("20").unwrap()),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert("marginTop".to_string(), pre_rule_factory("marginTop", "10"));
    expected_result.insert("marginEnd".to_string(), pre_rule_factory("marginEnd", "10"));
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10"),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "10"),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "20"),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_shorthands_with_space_separated_values() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator("margin", string_to_expression("10px 20px").unwrap()),
        key_value_creator("borderColor", string_to_expression("red").unwrap()),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 8);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_factory("marginTop", "10px"),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "20px"),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_factory("marginBottom", "10px"),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "20px"),
    );

    expected_result.insert(
      "borderTopColor".to_string(),
      pre_rule_factory("borderTopColor", "red"),
    );
    expected_result.insert(
      "borderEndColor".to_string(),
      pre_rule_factory("borderEndColor", "red"),
    );
    expected_result.insert(
      "borderBottomColor".to_string(),
      pre_rule_factory("borderBottomColor", "red"),
    );
    expected_result.insert(
      "borderStartColor".to_string(),
      pre_rule_factory("borderStartColor", "red"),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn should_expand_shorthands_with_fallbacks() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "margin",
        Expr::Array(
          create_array(&[
            string_to_expression("10vh 20px").unwrap(),
            string_to_expression("10dvh 20px").unwrap(),
          ])
          .unwrap(),
        ),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

   // dbg!(&result);
    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_vec_factory("marginTop", &["10vh", "10dvh"]),
    );
    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_factory("marginEnd", "20px"),
    );
    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_vec_factory("marginBottom", &["10vh", "10dvh"]),
    );
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "20px"),
    );

    assert_eq!(result, expected_result)
  }
}

#[cfg(test)]
mod nested_objects {
  use indexmap::IndexMap;

  use crate::shared::{
    structures::{
      functions::FunctionMap,
      tests::flatten_raw_style_objects_test::flatten_style_object_with_legacy_shorthand_expansion::{
        get_state, null_rule_factory, pre_rule_factory, pre_rule_set_factory,
        pre_rule_with_pseudos_factory,
      },
    },
    utils::{
      common::{key_value_creator, prop_or_spread_string_creator, string_to_expression},
      css::{
        factories::object_expression_factory,
        stylex::flatten_raw_style_object::flatten_raw_style_object,
      },
    },
  };

  #[test]
  fn legacy_pseudo_classes() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator("color", string_to_expression("blue").unwrap()),
        key_value_creator("marginStart", string_to_expression("0").unwrap()),
        key_value_creator(
          ":hover",
          object_expression_factory(vec![
            prop_or_spread_string_creator("color", "red"),
            prop_or_spread_string_creator("marginStart", "10"),
          ])
          .unwrap(),
        ),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

   // dbg!(&result);
    assert_eq!(result.len(), 8);

    let mut expected_result = IndexMap::new();

    expected_result.insert("color".to_string(), pre_rule_factory("color", "blue"));
    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_factory("marginStart", "0"),
    );
    expected_result.insert("marginLeft".to_string(), null_rule_factory());
    expected_result.insert("marginRight".to_string(), null_rule_factory());
    expected_result.insert(
      ":hover_color".to_string(),
      pre_rule_with_pseudos_factory("color", "red", &[":hover"], &[]),
    );

    expected_result.insert(
      ":hover_marginStart".to_string(),
      pre_rule_with_pseudos_factory("marginStart", "10", &[":hover"], &[]),
    );

    expected_result.insert(":hover_marginLeft".to_string(), null_rule_factory());

    expected_result.insert(":hover_marginRight".to_string(), null_rule_factory());

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_classes() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "blue"),
            prop_or_spread_string_creator(":hover", "red"),
          ])
          .unwrap(),
        ),
        key_value_creator(
          "marginStart",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "0"),
            prop_or_spread_string_creator(":hover", "10"),
          ])
          .unwrap(),
        ),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0"),
        pre_rule_with_pseudos_factory("marginStart", "10", &[":hover"], &[]),
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
        key_value_creator(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "blue"),
            prop_or_spread_string_creator(":hover", "red"),
          ])
          .unwrap(),
        ),
        key_value_creator(
          "margin",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "0"),
            prop_or_spread_string_creator(":hover", "10"),
          ])
          .unwrap(),
        ),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 5);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "0"),
        pre_rule_with_pseudos_factory("marginTop", "10", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "0"),
        pre_rule_with_pseudos_factory("marginEnd", "10", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "0"),
        pre_rule_with_pseudos_factory("marginBottom", "10", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0"),
        pre_rule_with_pseudos_factory("marginStart", "10", &[":hover"], &[]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_classes_with_complex_shorthands() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "blue"),
            prop_or_spread_string_creator(":hover", "red"),
          ])
          .unwrap(),
        ),
        key_value_creator(
          "margin",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "1px 2px 3px 4px"),
            prop_or_spread_string_creator(":hover", "10px 20px"),
          ])
          .unwrap(),
        ),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 5);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "1px"),
        pre_rule_with_pseudos_factory("marginTop", "10px", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "2px"),
        pre_rule_with_pseudos_factory("marginEnd", "20px", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "3px"),
        pre_rule_with_pseudos_factory("marginBottom", "10px", &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "4px"),
        pre_rule_with_pseudos_factory("marginStart", "20px", &[":hover"], &[]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn modern_pseudo_and_at_rules() {
    let result = flatten_raw_style_object(
      &[
        key_value_creator(
          "color",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "blue"),
            prop_or_spread_string_creator(":hover", "red"),
            prop_or_spread_string_creator("@media (min-width: 300px)", "green"),
          ])
          .unwrap(),
        ),
        key_value_creator(
          "marginStart",
          object_expression_factory(vec![
            prop_or_spread_string_creator("default", "0"),
            prop_or_spread_string_creator(":hover", "10"),
          ])
          .unwrap(),
        ),
      ],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &[]),
        pre_rule_with_pseudos_factory("color", "green", &[], &["@media (min-width: 300px)"]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "0"),
        pre_rule_with_pseudos_factory("marginStart", "10", &[":hover"], &[]),
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
      functions::FunctionMap,
      tests::flatten_raw_style_objects_test::flatten_style_object_with_legacy_shorthand_expansion::{
        get_state, pre_rule_factory, pre_rule_set_factory, pre_rule_vec_with_pseudos_factory,
        pre_rule_with_pseudos_factory,
      },
    },
    utils::{
      common::{
        key_value_creator, prop_or_spread_array_string_creator, prop_or_spread_expr_creator,
        prop_or_spread_string_creator,
      },
      css::{
        factories::object_expression_factory,
        stylex::flatten_raw_style_object::flatten_raw_style_object,
      },
    },
  };

  #[test]
  fn fallback_styles_within_nested_objects() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "margin",
        object_expression_factory(vec![
          prop_or_spread_string_creator("default", "1px 2px 3px 4px"),
          prop_or_spread_array_string_creator(":hover", &["10px 20px", "1dvh 2dvh"]),
        ])
        .unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 4);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "marginTop".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginTop", "1px"),
        pre_rule_vec_with_pseudos_factory("marginTop", &["10px", "1dvh"], &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginEnd".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginEnd", "2px"),
        pre_rule_vec_with_pseudos_factory("marginEnd", &["20px", "2dvh"], &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginBottom".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginBottom", "3px"),
        pre_rule_vec_with_pseudos_factory("marginBottom", &["10px", "1dvh"], &[":hover"], &[]),
      ]),
    );

    expected_result.insert(
      "marginStart".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("marginStart", "4px"),
        pre_rule_vec_with_pseudos_factory("marginStart", &["20px", "2dvh"], &[":hover"], &[]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_within_a_media_query_legacy_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "@media (min-width: 300px)",
        object_expression_factory(vec![prop_or_spread_expr_creator(
          ":hover",
          vec![prop_or_spread_string_creator("color", "red")],
        )])
        .unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "@media (min-width: 300px)_:hover_color".to_string(),
      pre_rule_set_factory(&[pre_rule_with_pseudos_factory(
        "color",
        "red",
        &[":hover"],
        &["@media (min-width: 300px)"],
      )]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_with_a_pseudo_within_a_media_query_legacy_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "@media (min-width: 300px)",
        object_expression_factory(vec![prop_or_spread_expr_creator(
          ":hover",
          vec![
            prop_or_spread_string_creator("color", "pink"),
            prop_or_spread_expr_creator(
              ":active",
              vec![prop_or_spread_string_creator("color", "red")],
            ),
          ],
        )])
        .unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 2);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "@media (min-width: 300px)_:hover_color".to_string(),
      pre_rule_set_factory(&[pre_rule_with_pseudos_factory(
        "color",
        "pink",
        &[":hover"],
        &["@media (min-width: 300px)"],
      )]),
    );

    expected_result.insert(
      "@media (min-width: 300px)_:hover_:active_color".to_string(),
      pre_rule_set_factory(&[pre_rule_with_pseudos_factory(
        "color",
        "red",
        &[":hover", ":active"],
        &["@media (min-width: 300px)"],
      )]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn pseudo_within_a_media_query_modern_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "color",
        object_expression_factory(vec![
          prop_or_spread_string_creator("default", "blue"),
          prop_or_spread_expr_creator(
            "@media (min-width: 300px)",
            vec![prop_or_spread_string_creator(":hover", "red")],
          ),
        ])
        .unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &["@media (min-width: 300px)"]),
      ]),
    );

    assert_eq!(result, expected_result)
  }

  #[test]
  fn extra_deep_pseudo_within_a_media_query_modern_syntax() {
    let result = flatten_raw_style_object(
      &[key_value_creator(
        "color",
        object_expression_factory(vec![
          prop_or_spread_string_creator("default", "blue"),
          prop_or_spread_expr_creator(
            "@media (min-width: 300px)",
            vec![prop_or_spread_expr_creator(
              ":hover",
              vec![
                prop_or_spread_string_creator("default", "red"),
                prop_or_spread_string_creator(":active", "maroon"),
              ],
            )],
          ),
        ])
        .unwrap(),
      )],
      &mut vec![],
      &mut vec![],
      &mut get_state(),
      &FunctionMap::default(),
    );

    assert_eq!(result.len(), 1);

    let mut expected_result = IndexMap::new();

    expected_result.insert(
      "color".to_string(),
      pre_rule_set_factory(&[
        pre_rule_factory("color", "blue"),
        pre_rule_with_pseudos_factory("color", "red", &[":hover"], &["@media (min-width: 300px)"]),
        pre_rule_with_pseudos_factory(
          "color",
          "maroon",
          &[":hover", ":active"],
          &["@media (min-width: 300px)"],
        ),
      ]),
    );

    assert_eq!(result, expected_result)
  }
}
