use std::collections::HashMap;

use crate::shared::utils::common::{create_hash, gen_file_based_identifier};

use super::state_manager::StateManager;

#[derive(Debug, Clone)]
pub struct ThemeRef {
  file_name: String,
  export_name: String,
  state: StateManager,
  map: HashMap<String, String>,
}

impl ThemeRef {
  pub(crate) fn new(file_name: String, export_name: String, state: StateManager) -> Self {
    Self {
      file_name,
      export_name,
      state,
      map: HashMap::new(),
    }
  }

  pub(crate) fn get(&mut self, key: &str) -> (String, &StateManager) {
    if key.starts_with("--") {
      let css_key = format!("var({})", key);
      return (css_key, &self.state);
    }
    let entry = self.map.entry(key.to_string()).or_insert_with(|| {
      let str_to_hash = gen_file_based_identifier(
        &self.file_name,
        &self.export_name,
        if key == "__themeName__" {
          None
        } else {
          Some(key)
        },
      );

      let var_name = format!(
        "{}{}",
        self.state.options.class_name_prefix,
        create_hash(&str_to_hash)
      );

      format!("var(--{})", var_name)
    });

    (entry.to_string(), &self.state)
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
