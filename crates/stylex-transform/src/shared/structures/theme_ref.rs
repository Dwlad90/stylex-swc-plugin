use std::sync::Arc;

use rustc_hash::FxHashMap;
use stylex_macros::stylex_panic;
use stylex_utils::hash::create_hash;

use crate::shared::utils::common::gen_file_based_identifier;
use stylex_constants::constants::common::VAR_GROUP_HASH_KEY;
use stylex_enums::theme_ref::ThemeRefResult;

use super::state_manager::StateManager;

#[derive(Debug, Clone)]
pub struct ThemeRef {
  file_name: String,
  export_name: String,
  class_name_prefix: String,
  map: FxHashMap<String, Arc<str>>,
}

impl ThemeRef {
  pub(crate) fn new(
    file_name: impl Into<String>,
    export_name: impl Into<String>,
    class_name_prefix: impl Into<String>,
  ) -> Self {
    Self {
      file_name: file_name.into(),
      export_name: export_name.into(),
      class_name_prefix: class_name_prefix.into(),
      map: FxHashMap::default(),
    }
  }

  pub(crate) fn get(&mut self, key: &str, state: &StateManager) -> ThemeRefResult {
    if key == "__IS_PROXY" {
      return ThemeRefResult::Proxy;
    }

    if key == "toString" {
      let value = format!(
        "{}{}",
        state.options.class_name_prefix,
        create_hash(&gen_file_based_identifier(
          &self.file_name,
          &self.export_name,
          None
        ))
      );
      return ThemeRefResult::ToString(value);
    }

    if key.starts_with("--") {
      return ThemeRefResult::CssVar(Arc::from(format!("var({})", key).as_str()));
    }

    let entry = self.map.entry(key.to_string()).or_insert_with(|| {
      let str_to_hash = gen_file_based_identifier(
        &self.file_name,
        &self.export_name,
        if key == VAR_GROUP_HASH_KEY {
          None
        } else {
          Some(key)
        },
      );

      let debug = state.options.debug;
      let enable_debug_class_names = state.options.enable_debug_class_names;

      let var_safe_key = if key == VAR_GROUP_HASH_KEY {
        String::new()
      } else {
        let mut safe = if key.chars().next().unwrap_or('\0').is_ascii_digit() {
          format!("_{}", key)
        } else {
          key.to_string()
        }
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

        safe.push('-');

        safe
      };

      let var_name = if debug && enable_debug_class_names {
        format!(
          "{}{}{}",
          var_safe_key,
          self.class_name_prefix,
          create_hash(&str_to_hash)
        )
      } else {
        format!("{}{}", self.class_name_prefix, create_hash(&str_to_hash))
      };

      if key == VAR_GROUP_HASH_KEY {
        return Arc::from(var_name.as_str());
      }

      Arc::from(format!("var(--{})", var_name).as_str())
    });

    ThemeRefResult::CssVar(Arc::clone(entry))
  }

  #[cfg_attr(coverage_nightly, coverage(off))]
  fn _set(&self, key: &str, value: &str) {
    stylex_panic!(
      "Cannot set value {} to key {} in theme {}",
      value,
      key,
      self.file_name
    );
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for ThemeRef {
  fn eq(&self, _other: &Self) -> bool {
    stylex_panic!("Theme references cannot be compared directly.");
    // self.file_name == other.file_name && self.export_name ==
    // other.export_name
  }
}
