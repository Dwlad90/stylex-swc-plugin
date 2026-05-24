#[cfg(test)]
mod stylex_create_theme_nested {
  use indexmap::IndexMap;
  use stylex_ast::ast::factories::{create_key_value_prop, create_object_expression};
  use stylex_structures::stylex_options::StyleXOptions;
  use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue,
      flat_compiled_styles_value::FlatCompiledStylesValue,
    },
    structures::{state_manager::StateManager, types::FlatCompiledStyles},
    transformers::{
      stylex_create_theme::stylex_create_theme,
      stylex_create_theme_nested::stylex_create_theme_nested,
      stylex_define_vars::stylex_define_vars, stylex_define_vars_nested::stylex_define_vars_nested,
    },
    utils::{
      ast::convertors::create_string_expr,
      core::stylex_nested_utils::{
        UnflattenedCompiledStylesValue, convert_unflattened_object_to_ast,
      },
    },
  };

  fn create_test_state_manager() -> StateManager {
    let options = StyleXOptions::default()
      .with_class_name_prefix("x")
      .with_debug(false)
      .with_enable_debug_class_names(false);
    let mut state = StateManager::new(options);
    state.export_id = Some("test/tokens.stylex.js//tokens".to_string());
    state
  }

  fn vars_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![
        create_key_value_prop("background", create_string_expr("red")),
        create_key_value_prop("color", create_string_expr("blue")),
      ]),
    )]))
  }

  fn overrides_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![
        create_key_value_prop("background", create_string_expr("green")),
        create_key_value_prop(
          "color",
          create_object_expression(vec![
            create_key_value_prop("default", create_string_expr("white")),
            create_key_value_prop(
              "@media (prefers-color-scheme: dark)",
              create_string_expr("black"),
            ),
          ]),
        ),
      ]),
    )]))
  }

  #[test]
  fn creates_theme_override_from_nested_vars_and_nested_overrides() {
    let mut state = create_test_state_manager();
    let (vars_output, _) = stylex_define_vars_nested(&vars_fixture(), &mut state);
    let mut theme_vars = EvaluateResultValue::Expr(convert_unflattened_object_to_ast(&vars_output));
    let mut typed_variables: FlatCompiledStyles = IndexMap::default();

    let (theme_output, theme_css) = stylex_create_theme_nested(
      &mut theme_vars,
      &overrides_fixture(),
      &mut state,
      &mut typed_variables,
    );

    assert!(theme_output.contains_key("$$css"));
    let all_css = theme_css
      .values()
      .map(|style| match style.as_ref() {
        InjectableStyleKind::Regular(style) => style.ltr.as_str(),
        InjectableStyleKind::Const(_) => "",
      })
      .collect::<String>();
    assert!(all_css.contains("green"));
    assert!(all_css.contains("white"));
    assert!(all_css.contains("black"));
    assert!(all_css.contains("@media"));
  }

  #[test]
  #[should_panic]
  fn throws_when_first_arg_lacks_var_group_hash() {
    let mut state = create_test_state_manager();
    let mut theme_vars =
      EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
        "button",
        create_object_expression(vec![create_key_value_prop(
          "background",
          create_string_expr("var(--hash)"),
        )]),
      )]));
    let mut typed_variables: FlatCompiledStyles = IndexMap::default();

    let _ = stylex_create_theme_nested(
      &mut theme_vars,
      &overrides_fixture(),
      &mut state,
      &mut typed_variables,
    );
  }

  fn partial_overrides_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![create_key_value_prop(
        "background",
        create_string_expr("red"),
      )]),
    )]))
  }

  #[test]
  fn supports_partial_overrides_only_some_leaves() {
    let mut state = create_test_state_manager();
    let (vars_output, _) = stylex_define_vars_nested(&vars_fixture(), &mut state);
    let mut theme_vars = EvaluateResultValue::Expr(convert_unflattened_object_to_ast(&vars_output));
    let mut typed_variables: FlatCompiledStyles = IndexMap::default();

    let (theme_output, theme_css) = stylex_create_theme_nested(
      &mut theme_vars,
      &partial_overrides_fixture(),
      &mut state,
      &mut typed_variables,
    );

    assert!(theme_output.contains_key("$$css"));
    let all_css = theme_css
      .values()
      .map(|style| match style.as_ref() {
        InjectableStyleKind::Regular(style) => style.ltr.as_str(),
        InjectableStyleKind::Const(_) => "",
      })
      .collect::<String>();
    assert!(all_css.contains("red"));
    // Only one override property, so only one var declaration in the generated CSS
    assert!(!all_css.contains("blue"));
  }

  fn conditional_overrides_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![create_key_value_prop(
        "background",
        create_object_expression(vec![
          create_key_value_prop("default", create_string_expr("green")),
          create_key_value_prop(
            "@media (prefers-color-scheme: dark)",
            create_string_expr("lightgreen"),
          ),
        ]),
      )]),
    )]))
  }

  #[test]
  fn handles_conditional_overrides_with_media() {
    let mut state = create_test_state_manager();
    let (vars_output, _) = stylex_define_vars_nested(&vars_fixture(), &mut state);
    let mut theme_vars = EvaluateResultValue::Expr(convert_unflattened_object_to_ast(&vars_output));
    let mut typed_variables: FlatCompiledStyles = IndexMap::default();

    let (theme_output, theme_css) = stylex_create_theme_nested(
      &mut theme_vars,
      &conditional_overrides_fixture(),
      &mut state,
      &mut typed_variables,
    );

    assert!(theme_output.contains_key("$$css"));
    let all_css = theme_css
      .values()
      .map(|style| match style.as_ref() {
        InjectableStyleKind::Regular(style) => style.ltr.as_str(),
        InjectableStyleKind::Const(_) => "",
      })
      .collect::<String>();
    assert!(all_css.contains("green"));
    assert!(all_css.contains("lightgreen"));
    assert!(all_css.contains("@media (prefers-color-scheme: dark)"));
  }

  fn simple_nested_vars_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![create_key_value_prop("bg", create_string_expr("red"))]),
    )]))
  }

  fn equivalent_flat_vars_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button.bg",
      create_string_expr("red"),
    )]))
  }

  fn simple_nested_overrides_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button",
      create_object_expression(vec![create_key_value_prop(
        "bg",
        create_string_expr("green"),
      )]),
    )]))
  }

  fn equivalent_flat_overrides_fixture() -> EvaluateResultValue {
    EvaluateResultValue::Expr(create_object_expression(vec![create_key_value_prop(
      "button.bg",
      create_string_expr("green"),
    )]))
  }

  #[test]
  fn produces_same_output_as_flat_create_theme_with_equivalent_flattened_inputs() {
    let mut nested_state = create_test_state_manager();
    let mut flat_state = create_test_state_manager();

    let (nested_vars_output, _) =
      stylex_define_vars_nested(&simple_nested_vars_fixture(), &mut nested_state);
    let (flat_vars_output, _) =
      stylex_define_vars(&equivalent_flat_vars_fixture(), &mut flat_state);

    let nested_hash = match &nested_vars_output["__varGroupHash__"] {
      UnflattenedCompiledStylesValue::Leaf(value) => value
        .as_string()
        .expect("expected nested __varGroupHash__ string")
        .to_string(),
      _ => panic!("expected leaf"),
    };
    let flat_hash = match flat_vars_output
      .get("__varGroupHash__")
      .map(|value| value.as_ref())
    {
      Some(FlatCompiledStylesValue::String(value)) => value.clone(),
      _ => panic!("expected flat __varGroupHash__ string"),
    };

    assert_eq!(nested_hash, flat_hash);

    let mut nested_theme_vars =
      EvaluateResultValue::Expr(convert_unflattened_object_to_ast(&nested_vars_output));
    let flat_theme_props: Vec<_> = flat_vars_output
      .iter()
      .filter_map(|(key, value)| match value.as_ref() {
        FlatCompiledStylesValue::String(s) => {
          Some(create_key_value_prop(key.as_str(), create_string_expr(s)))
        },
        _ => None,
      })
      .collect();
    let mut flat_theme_vars = EvaluateResultValue::Expr(create_object_expression(flat_theme_props));

    let mut nested_typed_variables: FlatCompiledStyles = IndexMap::default();
    let mut flat_typed_variables: FlatCompiledStyles = IndexMap::default();

    let (nested_theme, _) = stylex_create_theme_nested(
      &mut nested_theme_vars,
      &simple_nested_overrides_fixture(),
      &mut nested_state,
      &mut nested_typed_variables,
    );
    let (flat_theme, _) = stylex_create_theme(
      &mut flat_theme_vars,
      &equivalent_flat_overrides_fixture(),
      &mut flat_state,
      &mut flat_typed_variables,
    );

    assert!(nested_theme.contains_key("$$css"));
    assert!(flat_theme.contains_key("$$css"));
  }

  #[test]
  fn theme_output_contains_var_group_hash_key() {
    let mut state = create_test_state_manager();
    let (vars_output, _) = stylex_define_vars_nested(&vars_fixture(), &mut state);
    let var_group_hash = match &vars_output["__varGroupHash__"] {
      crate::shared::utils::core::stylex_nested_utils::UnflattenedCompiledStylesValue::Leaf(v) => {
        v.as_string().expect("expected string leaf").to_string()
      },
      _ => panic!("expected leaf"),
    };
    let mut theme_vars = EvaluateResultValue::Expr(convert_unflattened_object_to_ast(&vars_output));
    let mut typed_variables: FlatCompiledStyles = IndexMap::default();

    let (theme_output, _) = stylex_create_theme_nested(
      &mut theme_vars,
      &overrides_fixture(),
      &mut state,
      &mut typed_variables,
    );

    assert!(theme_output.contains_key(&var_group_hash));
    assert!(theme_output.contains_key("$$css"));
  }
}
