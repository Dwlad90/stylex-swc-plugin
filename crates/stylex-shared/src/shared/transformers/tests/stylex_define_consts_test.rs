#[cfg(test)]
mod stylex_define_consts {
  use swc_core::ecma::ast::PropOrSpread;

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue, injectable_style::InjectableStyleKind,
    },
    structures::{state_manager::StateManager, stylex_options::StyleXOptions},
    transformers::stylex_define_consts::stylex_define_consts,
    utils::{
      ast::{
        convertors::string_to_expression,
        factories::{object_expression_factory, prop_or_spread_expression_factory},
      },
      common::create_hash,
    },
  };

  fn create_test_state_manager(export_id: &str) -> StateManager {
    let mut state = StateManager::new(StyleXOptions {
      class_name_prefix: "x".to_string(),
      debug: false,
      enable_debug_class_names: false,
      ..Default::default()
    });
    state.export_id = Some(export_id.to_string());
    state
  }

  fn constants_factory(constants: &[(&str, &str)]) -> EvaluateResultValue {
    let props = constants
      .iter()
      .map(|(key, value)| prop_or_spread_expression_factory(key, string_to_expression(value)))
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(object_expression_factory(props))
  }

  fn get_const_hash(export_id: &str, key: &str, class_name_prefix: &str) -> String {
    format!(
      "{}{}",
      class_name_prefix,
      create_hash(&format!("{}.{}", export_id, key))
    )
  }

  #[test]
  fn returns_correct_structure_for_basic_constants() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state = create_test_state_manager(export_id);

    let constants =
      constants_factory(&[("sm", "(min-width: 768px)"), ("md", "(min-width: 1024px)")]);

    let (js_output, injectable_styles) = stylex_define_consts(&constants, &mut state);

    assert_eq!(js_output.len(), 2);
    assert_eq!(
      js_output.get("sm").unwrap().as_string().unwrap(),
      "(min-width: 768px)"
    );
    assert_eq!(
      js_output.get("md").unwrap().as_string().unwrap(),
      "(min-width: 1024px)"
    );

    let sm_hash = get_const_hash(export_id, "sm", "x");
    let md_hash = get_const_hash(export_id, "md", "x");

    assert!(injectable_styles.contains_key(&sm_hash));
    assert!(injectable_styles.contains_key(&md_hash));

    match injectable_styles.get(&sm_hash).unwrap().as_ref() {
      InjectableStyleKind::Const(style) => {
        assert_eq!(style.const_value, "(min-width: 768px)");
        assert_eq!(style.ltr, "");
        assert_eq!(style.rtl, None);
        assert_eq!(style.priority, Some(0.0));
      }
      _ => panic!("Expected InjectableStyleKind::Const"),
    }

    match injectable_styles.get(&md_hash).unwrap().as_ref() {
      InjectableStyleKind::Const(style) => {
        assert_eq!(style.const_value, "(min-width: 1024px)");
        assert_eq!(style.ltr, "");
        assert_eq!(style.rtl, None);
        assert_eq!(style.priority, Some(0.0));
      }
      _ => panic!("Expected InjectableStyleKind::Const"),
    }

    assert_ne!(sm_hash, md_hash);
  }

  #[test]
  fn handles_special_characters_in_keys() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state = create_test_state_manager(export_id);

    let constants = constants_factory(&[("max-width", "1200px"), ("font-size*large", "18px")]);

    let (js_output, injectable_styles) = stylex_define_consts(&constants, &mut state);

    assert_eq!(js_output.len(), 2);
    assert_eq!(
      js_output.get("max-width").unwrap().as_string().unwrap(),
      "1200px"
    );
    assert_eq!(
      js_output
        .get("font-size*large")
        .unwrap()
        .as_string()
        .unwrap(),
      "18px"
    );

    let max_width_hash = get_const_hash(export_id, "max-width", "x");
    let font_size_hash = get_const_hash(export_id, "font-size*large", "x");

    assert!(injectable_styles.contains_key(&max_width_hash));
    assert!(injectable_styles.contains_key(&font_size_hash));

    assert_ne!(max_width_hash, font_size_hash);
  }

  #[test]
  fn handles_numeric_keys() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state = create_test_state_manager(export_id);

    let constants = constants_factory(&[("1", "one"), ("2", "two")]);

    let (js_output, injectable_styles) = stylex_define_consts(&constants, &mut state);

    assert_eq!(js_output.len(), 2);
    assert_eq!(js_output.get("1").unwrap().as_string().unwrap(), "one");
    assert_eq!(js_output.get("2").unwrap().as_string().unwrap(), "two");

    let hash_1 = get_const_hash(export_id, "1", "x");
    let hash_2 = get_const_hash(export_id, "2", "x");

    assert!(injectable_styles.contains_key(&hash_1));
    assert!(injectable_styles.contains_key(&hash_2));

    assert_ne!(hash_1, hash_2);
  }

  #[test]
  fn generates_consistent_hashes_for_identical_constants() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state1 = create_test_state_manager(export_id);
    let mut state2 = create_test_state_manager(export_id);

    let constants = constants_factory(&[("padding", "10px")]);

    let (js_output1, styles1) = stylex_define_consts(&constants, &mut state1);
    let (js_output2, styles2) = stylex_define_consts(&constants, &mut state2);

    let key_hash = get_const_hash(export_id, "padding", "x");

    assert_eq!(js_output1, js_output2);

    assert!(styles1.contains_key(&key_hash));
    assert!(styles2.contains_key(&key_hash));

    match (
      styles1.get(&key_hash).unwrap().as_ref(),
      styles2.get(&key_hash).unwrap().as_ref(),
    ) {
      (InjectableStyleKind::Const(style1), InjectableStyleKind::Const(style2)) => {
        assert_eq!(style1.const_key, style2.const_key);
      }
      _ => panic!("Expected InjectableStyleKind::Const"),
    }
  }

  #[test]
  fn generates_different_hashes_for_different_constants() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state1 = create_test_state_manager(export_id);
    let mut state2 = create_test_state_manager(export_id);

    let constants1 = constants_factory(&[("padding", "10px")]);
    let constants2 = constants_factory(&[("margin", "10px")]);

    let (_, styles1) = stylex_define_consts(&constants1, &mut state1);
    let (_, styles2) = stylex_define_consts(&constants2, &mut state2);

    let padding_key = get_const_hash(export_id, "padding", "x");
    let margin_key = get_const_hash(export_id, "margin", "x");

    assert_ne!(padding_key, margin_key);

    assert!(styles1.contains_key(&padding_key));
    assert!(styles2.contains_key(&margin_key));

    match (
      styles1.get(&padding_key).unwrap().as_ref(),
      styles2.get(&margin_key).unwrap().as_ref(),
    ) {
      (InjectableStyleKind::Const(style1), InjectableStyleKind::Const(style2)) => {
        assert_ne!(style1.const_key, style2.const_key);
      }
      _ => panic!("Expected InjectableStyleKind::Const"),
    }
  }

  #[test]
  fn preserves_object_keys() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state = create_test_state_manager(export_id);

    let constants = constants_factory(&[("borderRadius", "8px"), ("colorPrimary", "#ff0000")]);

    let (js_output, _) = stylex_define_consts(&constants, &mut state);

    let keys: Vec<&String> = js_output.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&&"borderRadius".to_string()));
    assert!(keys.contains(&&"colorPrimary".to_string()));
  }

  #[test]
  #[should_panic(expected = "Keys in defineConsts() cannot start with \"--\".")]
  fn throws_an_error_for_keys_that_start_with_double_dash() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let mut state = create_test_state_manager(export_id);

    let constants = constants_factory(&[("--custom-var", "red")]);

    stylex_define_consts(&constants, &mut state);
  }
}
