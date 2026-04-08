use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use rustc_hash::FxHashMap;
use stylex_macros::stylex_panic;
use stylex_utils::hash::create_hash;

use crate::shared::utils::common::gen_file_based_identifier;
use stylex_constants::constants::common::VAR_GROUP_HASH_KEY;
use stylex_enums::theme_ref::ThemeRefResult;

use super::state_manager::StateManager;

/// A reference to a `defineVars` group. Multiple `ThemeRef` values may
/// share the same underlying hash-map cache via `Rc<RefCell<…>>`, so that
/// repeated lookups of the same `key` (across `clone()`s of this struct, e.g.
/// from the `FunctionType::ThemeRefMapper` factory) reuse already-computed CSS
/// variable names.
///
/// `base_id` is the precomputed `"{file_name}//{export_name}"` prefix used by
/// `gen_file_based_identifier`. Caching it eliminates one `format!` allocation
/// per `get()` call.
#[derive(Debug, Clone)]
pub struct ThemeRef {
  class_name_prefix: String,
  /// Precomputed `"{file_name}//{export_name}"` prefix — the result of
  /// `gen_file_based_identifier(file_name, export_name, None)`.
  base_id: String,
  map: Rc<RefCell<FxHashMap<String, Arc<str>>>>,
}

impl ThemeRef {
  pub(crate) fn new(
    file_name: impl Into<String>,
    export_name: impl Into<String>,
    class_name_prefix: impl Into<String>,
  ) -> Self {
    let file_name = file_name.into();
    let export_name = export_name.into();
    let base_id = gen_file_based_identifier(&file_name, &export_name, None);

    Self {
      class_name_prefix: class_name_prefix.into(),
      base_id,
      map: Rc::new(RefCell::new(FxHashMap::default())),
    }
  }

  pub(crate) fn get(&mut self, key: &str, state: &StateManager) -> ThemeRefResult {
    if key == "__IS_PROXY" {
      return ThemeRefResult::Proxy;
    }

    if key == "toString" {
      // NOTE: hash the cached base id instead of recomputing the prefix.
      let value = format!(
        "{}{}",
        state.options.class_name_prefix,
        create_hash(&self.base_id)
      );
      return ThemeRefResult::ToString(value);
    }

    if key.starts_with("--") {
      return ThemeRefResult::CssVar(Arc::from(format!("var({})", key).as_str()));
    }

    // NOTE: Fast path: cache hit, no map-key allocation.
    if let Some(cached) = self.map.borrow().get(key) {
      return ThemeRefResult::CssVar(Arc::clone(cached));
    }

    // NOTE: derive the per-key identifier by concatenation rather than calling
    // `gen_file_based_identifier` (which would rebuild the `file//export` prefix).
    let str_to_hash: String = if key == VAR_GROUP_HASH_KEY {
      self.base_id.clone()
    } else {
      format!("{}.{}", self.base_id, key)
    };

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

    let value: Arc<str> = if key == VAR_GROUP_HASH_KEY {
      Arc::from(var_name.as_str())
    } else {
      Arc::from(format!("var(--{})", var_name).as_str())
    };

    self
      .map
      .borrow_mut()
      .insert(key.to_string(), Arc::clone(&value));

    ThemeRefResult::CssVar(value)
  }

  #[cfg_attr(coverage_nightly, coverage(off))]
  fn _set(&self, key: &str, value: &str) {
    stylex_panic!(
      "Cannot set value {} to key {} in theme {}",
      value,
      key,
      self.base_id
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
