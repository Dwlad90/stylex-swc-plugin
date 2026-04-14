#[cfg(test)]
mod common_css_tests {
  use crate::shared::{
    structures::state_manager::StateManager,
    utils::css::common::{get_number_suffix, transform_value_cached},
  };

  #[test]
  fn should_transform_css_property_value() {
    assert_eq!(
      transform_value_cached("padding", "1", &mut StateManager::default()),
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
