#[cfg(test)]
mod converting_pre_rule_to_css {
  use crate::shared::structures::{
    injectable_style::InjectableStyle,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRuleValue, StylesPreRule},
    state_manager::StateManager,
    stylex_options::StyleResolution,
  };

  pub(super) fn get_state() -> StateManager {
    let mut state_manager = StateManager::default();

    state_manager.options.class_name_prefix = "x".to_string();
    state_manager.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state_manager.options.runtime_injection = None;
    state_manager.options.use_rem_for_font_size = true;
    state_manager.options.dev = false;
    state_manager.options.test = false;

    state_manager
  }

  #[test]
  fn should_convert_a_pre_rule_to_css() {
    let result = StylesPreRule::new("color", PreRuleValue::String("red".to_string()), None, None)
      .compiled(&get_state());

    assert_eq!(
      result,
      CompiledResult::ComputedStyles(vec![ComputedStyle(
        "x1e2nbdu".to_string(),
        InjectableStyle {
          ltr: ".x1e2nbdu{color:red}".to_string(),
          rtl: None,
          priority: Some(3000.0)
        }
      )])
    );
  }
}
