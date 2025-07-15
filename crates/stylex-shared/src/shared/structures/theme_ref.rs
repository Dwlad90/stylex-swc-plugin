use rustc_hash::FxHashMap;

use crate::shared::{
  constants::common::VAR_GROUP_HASH_KEY,
  utils::common::{create_hash, gen_file_based_identifier},
};

use super::state_manager::StateManager;

#[derive(Debug, Clone)]
pub struct ThemeRef {
  file_name: String,
  export_name: String,
  class_name_prefix: String,
  map: FxHashMap<String, String>,
}

impl ThemeRef {
  pub(crate) fn new(file_name: String, export_name: String, class_name_prefix: String) -> Self {
    Self {
      file_name,
      export_name,
      class_name_prefix,
      map: FxHashMap::default(),
    }
  }

  pub(crate) fn get(&mut self, key: &str, state: &StateManager) -> String {
    if key.starts_with("--") {
      let css_key = format!("var({})", key);
      return css_key;
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
        return var_name;
      }

      format!("var(--{})", var_name)
    });

    entry.to_string()
  }

  fn _set(&self, key: &str, value: &str) {
    panic!(
      "Cannot set value {} to key {} in theme {}",
      value, key, self.file_name
    );
  }
}

impl PartialEq for ThemeRef {
  fn eq(&self, _other: &Self) -> bool {
    panic!("ThemeRef cannot be compared");
    // self.file_name == other.file_name && self.export_name == other.export_name
  }
}
