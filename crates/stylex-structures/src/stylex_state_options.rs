use std::ops::{Deref, DerefMut};

use serde::Deserialize;

use stylex_constants::constants::common::DEFAULT_INJECT_PATH;
use stylex_enums::style_resolution::StyleResolution;

use crate::{
  core_stylex_options::CoreStyleXOptions,
  named_import_source::{RuntimeInjection, RuntimeInjectionState},
  stylex_options::{CheckModuleResolution, StyleXOptions},
};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct StyleXStateOptions {
  #[serde(flatten)]
  pub core: CoreStyleXOptions,
  pub runtime_injection: Option<RuntimeInjectionState>,
}

impl Deref for StyleXStateOptions {
  type Target = CoreStyleXOptions;
  fn deref(&self) -> &Self::Target {
    &self.core
  }
}

impl DerefMut for StyleXStateOptions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.core
  }
}

impl StyleXStateOptions {
  pub fn with_class_name_prefix(mut self, prefix: impl Into<String>) -> Self {
    self.core.class_name_prefix = prefix.into();
    self
  }

  pub fn with_style_resolution(mut self, resolution: StyleResolution) -> Self {
    self.core.style_resolution = resolution;
    self
  }

  pub fn with_debug(mut self, debug: bool) -> Self {
    self.core.debug = debug;
    self
  }

  pub fn with_dev(mut self, dev: bool) -> Self {
    self.core.dev = dev;
    self
  }

  pub fn with_test(mut self, test: bool) -> Self {
    self.core.test = test;
    self
  }

  pub fn with_enable_debug_class_names(mut self, enabled: bool) -> Self {
    self.core.enable_debug_class_names = enabled;
    self
  }

  pub fn with_enable_debug_data_prop(mut self, enabled: bool) -> Self {
    self.core.enable_debug_data_prop = enabled;
    self
  }

  pub fn with_enable_dev_class_names(mut self, enabled: bool) -> Self {
    self.core.enable_dev_class_names = enabled;
    self
  }

  pub fn with_enable_font_size_px_to_rem(mut self, enabled: bool) -> Self {
    self.core.enable_font_size_px_to_rem = enabled;
    self
  }

  pub fn with_enable_logical_styles_polyfill(mut self, enabled: bool) -> Self {
    self.core.enable_logical_styles_polyfill = enabled;
    self
  }

  pub fn with_enable_minified_keys(mut self, enabled: bool) -> Self {
    self.core.enable_minified_keys = enabled;
    self
  }

  pub fn with_unstable_module_resolution(mut self, resolution: CheckModuleResolution) -> Self {
    self.core.unstable_module_resolution = resolution;
    self
  }

  pub fn with_runtime_injection(mut self, injection: Option<RuntimeInjectionState>) -> Self {
    self.runtime_injection = injection;
    self
  }
}

impl From<StyleXOptions> for StyleXStateOptions {
  fn from(options: StyleXOptions) -> Self {
    let runtime_injection = match options.runtime_injection {
      RuntimeInjection::Boolean(b) => {
        if b {
          Some(RuntimeInjectionState::Regular(
            DEFAULT_INJECT_PATH.to_string(),
          ))
        } else {
          None
        }
      },
      RuntimeInjection::Named(n) => Some(RuntimeInjectionState::Named(n)),
      RuntimeInjection::Regular(s) => Some(RuntimeInjectionState::Regular(s)),
    };

    StyleXStateOptions {
      core: options.core,
      runtime_injection,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stylex_state_options_default() {
    let opts = StyleXStateOptions::default();
    assert!(!opts.dev);
    assert!(!opts.test);
    assert!(!opts.debug);
    assert_eq!(opts.class_name_prefix, "x");
    assert!(opts.enable_minified_keys);
    assert!(opts.enable_debug_data_prop);
    assert!(opts.runtime_injection.is_none());
    assert_eq!(opts.sx_prop_name, Some("sx".to_string()));
  }

  #[test]
  fn check_module_resolution_default_is_commonjs() {
    let res = CheckModuleResolution::default();
    assert!(matches!(res, CheckModuleResolution::CommonJS(_)));
  }

  #[test]
  fn from_stylex_options_boolean_false_injection() {
    let opts = StyleXOptions {
      runtime_injection: RuntimeInjection::Boolean(false),
      ..StyleXOptions::default()
    };
    let state: StyleXStateOptions = opts.into();
    assert!(state.runtime_injection.is_none());
  }

  #[test]
  fn from_stylex_options_boolean_true_injection() {
    let opts = StyleXOptions {
      runtime_injection: RuntimeInjection::Boolean(true),
      ..StyleXOptions::default()
    };
    let state: StyleXStateOptions = opts.into();
    assert!(matches!(
      state.runtime_injection,
      Some(RuntimeInjectionState::Regular(_))
    ));
  }

  #[test]
  fn from_stylex_options_regular_injection() {
    let opts = StyleXOptions {
      runtime_injection: RuntimeInjection::Regular("custom/path".into()),
      ..StyleXOptions::default()
    };
    let state: StyleXStateOptions = opts.into();
    match state.runtime_injection {
      Some(RuntimeInjectionState::Regular(s)) => assert_eq!(s, "custom/path"),
      _ => panic!("Expected Regular injection"),
    }
  }

  #[test]
  fn from_stylex_options_named_injection() {
    let named = crate::named_import_source::NamedImportSource {
      r#as: "inject".into(),
      from: "pkg".into(),
    };
    let opts = StyleXOptions {
      runtime_injection: RuntimeInjection::Named(named),
      ..StyleXOptions::default()
    };
    let state: StyleXStateOptions = opts.into();
    assert!(matches!(
      state.runtime_injection,
      Some(RuntimeInjectionState::Named(_))
    ));
  }

  #[test]
  fn from_stylex_options_preserves_fields() {
    let opts = StyleXOptions {
      dev: true,
      test: true,
      debug: true,
      class_name_prefix: "y".to_string(),
      ..StyleXOptions::default()
    };
    let state: StyleXStateOptions = opts.into();
    assert!(state.dev);
    assert!(state.test);
    assert!(state.debug);
    assert_eq!(state.class_name_prefix, "y");
  }
}
