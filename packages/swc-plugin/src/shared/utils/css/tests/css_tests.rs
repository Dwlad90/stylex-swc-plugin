#[cfg(test)]
mod common_css_tests {
  use crate::shared::{
    structures::state_manager::StateManager,
    utils::css::utils::{get_number_suffix, transform_value},
  };

  #[test]
  fn should_transform_css_property_value() {
    assert_eq!(
      transform_value("padding", "1", &StateManager::default()),
      "1px"
    );
  }

  #[test]
  fn should_return_correct_suffix() {
    assert_eq!(get_number_suffix("padding"), "px");
    assert_eq!(get_number_suffix("opacity"), "");
    assert_eq!(get_number_suffix("voiceDuration"), "ms");
  }
}
