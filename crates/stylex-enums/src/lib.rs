pub mod aliases;
pub mod core;
pub mod counter_mode;
pub mod css_syntax;
pub mod import_path_resolution;
pub mod js;
pub mod misc;
pub mod property_validation_mode;
pub mod style_resolution;
pub mod style_vars_to_keep;
pub mod sx_prop_name_param;
pub mod theme_ref;
pub mod top_level_expression;
pub mod value_with_default;

/// Backward-compatible re-export module.
pub mod data_structures {
  pub use crate::css_syntax;
  pub use crate::import_path_resolution;
  pub use crate::style_vars_to_keep;
  pub use crate::top_level_expression;
  pub use crate::value_with_default;
}
