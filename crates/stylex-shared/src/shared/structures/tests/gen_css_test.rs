#[cfg(test)]
mod converting_pre_rule_to_css {
  use indexmap::IndexMap;

  use crate::shared::structures::{
    injectable_style::InjectableStyle,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRuleValue, StylesPreRule},
    state_manager::StateManager,
    stylex_options::StyleResolution,
    types::ClassesToOriginalPaths,
  };

  pub(super) fn get_state() -> StateManager {
    let mut state_manager = StateManager::default();

    state_manager.options.class_name_prefix = "x".to_string();
    state_manager.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state_manager.options.runtime_injection = None;
    state_manager.options.enable_font_size_px_to_rem = true;
    state_manager.options.dev = false;
    state_manager.options.test = false;
    state_manager.options.debug = false;

    state_manager
  }

  #[test]
  fn should_convert_a_pre_rule_to_css() {
    let result = StylesPreRule::new(
      "color",
      PreRuleValue::String("red".to_string()),
      Some(vec!["color".to_string()]),
    )
    .compiled(&mut get_state());

    let mut classes_to_original_paths: ClassesToOriginalPaths = IndexMap::new();
    classes_to_original_paths.insert("x1e2nbdu".to_string(), vec!["color".to_string()]);

    assert_eq!(
      result,
      CompiledResult::ComputedStyles(vec![ComputedStyle(
        "x1e2nbdu".to_string(),
        InjectableStyle {
          ltr: ".x1e2nbdu{color:red}".to_string(),
          rtl: None,
          priority: Some(3000.0)
        },
        classes_to_original_paths
      )])
    );
  }
}
