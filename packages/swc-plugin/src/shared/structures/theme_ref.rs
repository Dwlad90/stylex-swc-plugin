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

  pub(crate) fn get(&mut self, key: &str) -> (String, StateManager) {
    if let Some(value) = self.map.get(key) {
      return (value.clone(), self.state.clone());
    }

    // dbg!(&self.file_name);

    let str_to_hash = if key == "__themeName__" {
      gen_file_based_identifier(&self.file_name, &self.export_name, None)
    } else {
      gen_file_based_identifier(&self.file_name, &self.export_name, Option::Some(key))
    };
    // println!("!!!!!Themeref: str_to_hash: {}", &str_to_hash);

    let var_name = format!(
      "{}{}",
      self.state.options.class_name_prefix,
      create_hash(&str_to_hash)
    );
    // dbg!(&var_name);

    let value = format!("var(--{})", var_name);
    // println!("!!!!!Themeref: key: {}, str_to_hash: {}, value: {}, file_name: {}, export_name: {}", &key, &str_to_hash, &value, &self.file_name, &self.export_name);

    self.map.insert(key.to_string(), value.clone());
    (value, self.state.clone())
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
