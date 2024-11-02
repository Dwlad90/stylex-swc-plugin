use std::sync::atomic::{AtomicUsize, Ordering};

use rustc_hash::FxHashMap;
use swc_core::ecma::ast::Ident;

use crate::shared::utils::ast::factories::ident_factory;

/// A thread-safe generator for unique identifiers.
pub(crate) struct UidGenerator {
  prefix: String,
  counters: FxHashMap<String, AtomicUsize>,
}

impl UidGenerator {
  /// Creates a new IdGenerator with the given prefix.
  pub fn new(prefix: &str) -> Self {
    let mut counters = FxHashMap::default();

    counters
      .entry(prefix.to_string())
      .or_insert_with(|| AtomicUsize::new(1));
    Self {
      prefix: prefix.to_string(),
      counters,
    }
  }
  pub fn _clear(&mut self) {
    self.counters.remove(&self.prefix);
  }
  pub fn generate(&self) -> String {
    let counter = self.counters.get(&self.prefix).unwrap();
    let count = counter.fetch_add(1, Ordering::SeqCst);

    let count_string = if count < 2 {
      String::default()
    } else {
      count.to_string()
    };

    let unique_name = format!("_{}{}", self.prefix, count_string);
    unique_name
  }

  /// Generates a unique identifier.
  pub fn generate_ident(&self) -> Ident {
    let unique_name = self.generate();

    ident_factory(unique_name.as_str())
  }
}
