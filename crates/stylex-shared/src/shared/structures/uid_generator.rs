use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use rustc_hash::FxHashMap;
use swc_core::ecma::ast::Ident;

use crate::shared::utils::ast::factories::ident_factory;

// Add once_cell for global static
use once_cell::sync::Lazy;

// Global counters map, protected by Mutex for thread safety
static GLOBAL_COUNTERS: Lazy<Mutex<FxHashMap<String, AtomicUsize>>> =
  Lazy::new(|| Mutex::new(FxHashMap::default()));

// Thread-local counters for test isolation
thread_local! {
  static THREAD_LOCAL_COUNTERS: std::cell::RefCell<FxHashMap<String, usize>> =
    std::cell::RefCell::new(FxHashMap::default());
}

/// Counter mode for UidGenerator
#[derive(Clone, Debug)]
pub enum CounterMode {
  /// Use global counters shared across all instances (default behavior).
  /// This ensures unique identifiers across the entire application.
  _Global,
  /// Use local counters specific to each instance (legacy behavior).
  /// Each UidGenerator instance maintains its own counter, which can lead to
  /// duplicate identifiers across different instances with the same prefix.
  Local,
  /// Use thread-local counters for test isolation.
  /// Each thread gets its own set of counters, perfect for parallel testing.
  ThreadLocal,
  /// Use a combination of thread ID and prefix for maximum uniqueness.
  /// This mode uses thread ID as part of the identifier generation.
  _ThreadUnique,
}

pub(crate) struct UidGenerator {
  prefix: String,
  mode: CounterMode,
  local_counter: AtomicUsize,
}

impl UidGenerator {
  /// Creates a new UidGenerator with the given prefix and counter mode.
  pub fn new(prefix: &str, mode: CounterMode) -> Self {
    match mode {
      CounterMode::_Global => {
        // Ensure the counter for this prefix exists in global counters
        let mut counters = GLOBAL_COUNTERS.lock().unwrap();
        counters
          .entry(prefix.to_string())
          .or_insert_with(|| AtomicUsize::new(1));
        drop(counters);
      }
      CounterMode::Local | CounterMode::ThreadLocal | CounterMode::_ThreadUnique => {
        // These modes don't need global counter initialization
      }
    }

    Self {
      prefix: prefix.to_string(),
      mode,
      local_counter: AtomicUsize::new(1),
    }
  }

  pub fn _clear(&mut self) {
    match self.mode {
      CounterMode::_Global => {
        let mut counters = GLOBAL_COUNTERS.lock().unwrap();
        counters.remove(&self.prefix);
      }
      CounterMode::Local => {
        self.local_counter.store(1, Ordering::SeqCst);
      }
      CounterMode::ThreadLocal => {
        THREAD_LOCAL_COUNTERS.with(|counters| {
          counters.borrow_mut().remove(&self.prefix);
        });
      }
      CounterMode::_ThreadUnique => {
        // Thread unique mode doesn't maintain persistent counters
      }
    }
  }

  pub fn generate(&self) -> String {
    match self.mode {
      CounterMode::_Global => {
        let counters = GLOBAL_COUNTERS.lock().unwrap();
        let counter = counters.get(&self.prefix).unwrap();
        let count = counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}{}", self.prefix, count_string)
      }
      CounterMode::Local => {
        let count = self.local_counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}{}", self.prefix, count_string)
      }
      CounterMode::ThreadLocal => {
        let count = THREAD_LOCAL_COUNTERS.with(|counters| {
          let mut counters = counters.borrow_mut();
          let counter = counters.entry(self.prefix.clone()).or_insert(1);
          let current_count = *counter;
          *counter += 1;
          current_count
        });

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}{}", self.prefix, count_string)
      }
      CounterMode::_ThreadUnique => {
        let thread_id = thread::current().id();
        let count = self.local_counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}_{:?}{}", self.prefix, thread_id, count_string)
      }
    }
  }

  /// Generates a unique identifier.
  pub fn generate_ident(&self) -> Ident {
    let unique_name = self.generate();

    ident_factory(unique_name.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_global_counter_consistency() {
    let gen1 = UidGenerator::new("test", CounterMode::_Global);
    let gen2 = UidGenerator::new("test", CounterMode::_Global);

    assert_eq!(gen1.generate(), "_test");
    assert_eq!(gen2.generate(), "_test2");
    assert_eq!(gen1.generate(), "_test3");
  }

  #[test]
  fn test_local_counter_isolation() {
    let gen1 = UidGenerator::new("test", CounterMode::Local);
    let gen2 = UidGenerator::new("test", CounterMode::Local);

    assert_eq!(gen1.generate(), "_test");
    assert_eq!(gen2.generate(), "_test"); // Same because local counters are independent
    assert_eq!(gen1.generate(), "_test2");
    assert_eq!(gen2.generate(), "_test2"); // Each maintains its own counter
  }

  #[test]
  fn test_thread_local_counter() {
    let gen1 = UidGenerator::new("test", CounterMode::ThreadLocal);
    let gen2 = UidGenerator::new("test", CounterMode::ThreadLocal);

    assert_eq!(gen1.generate(), "_test");
    assert_eq!(gen2.generate(), "_test2"); // Shared within same thread
    assert_eq!(gen1.generate(), "_test3");
  }

  #[test]
  fn test_thread_unique_identifiers() {
    let generator = UidGenerator::new("test", CounterMode::_ThreadUnique);
    let id1 = generator.generate();
    let id2 = generator.generate();

    // Both should contain thread ID and be unique
    assert!(id1.starts_with("_test_"));
    assert!(id2.starts_with("_test_"));
    assert_ne!(id1, id2);
  }

  #[test]
  fn test_parallel_thread_local_isolation() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let barrier = Arc::new(Barrier::new(2));
    let mut handles = vec![];

    for thread_num in 0..2 {
      let barrier = Arc::clone(&barrier);
      let handle = thread::spawn(move || {
        let generator = UidGenerator::new("test", CounterMode::ThreadLocal);

        // Wait for both threads to be ready
        barrier.wait();

        // Each thread should get the same sequence independently
        let results = (0..3).map(|_| generator.generate()).collect::<Vec<_>>();
        (thread_num, results)
      });
      handles.push(handle);
    }

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Both threads should generate the same sequence because they're isolated
    assert_eq!(results[0].1, vec!["_test", "_test2", "_test3"]);
    assert_eq!(results[1].1, vec!["_test", "_test2", "_test3"]);
  }
}
