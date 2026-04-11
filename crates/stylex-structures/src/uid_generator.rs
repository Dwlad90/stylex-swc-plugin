use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use rustc_hash::FxHashMap;
use stylex_enums::counter_mode::CounterMode;
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::Ident;

use once_cell::sync::Lazy;

// Global counters map, protected by Mutex for thread safety
static GLOBAL_COUNTERS: Lazy<Mutex<FxHashMap<String, AtomicUsize>>> =
  Lazy::new(|| Mutex::new(FxHashMap::default()));

// Thread-local counters for test isolation
thread_local! {
  static THREAD_LOCAL_COUNTERS: std::cell::RefCell<FxHashMap<String, usize>> =
    std::cell::RefCell::new(FxHashMap::default());
}

pub struct UidGenerator {
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
        let mut counters = match GLOBAL_COUNTERS.lock() {
          Ok(c) => c,
          Err(e) => stylex_panic!("GLOBAL_COUNTERS mutex poisoned: {}", e),
        };
        counters
          .entry(prefix.to_string())
          .or_insert_with(|| AtomicUsize::new(1));
        drop(counters);
      },
      CounterMode::Local | CounterMode::ThreadLocal | CounterMode::_ThreadUnique => {
        // These modes don't need global counter initialization
      },
    }

    Self {
      prefix: prefix.to_string(),
      mode,
      local_counter: AtomicUsize::new(1),
    }
  }

  pub fn clear(&mut self) {
    match self.mode {
      CounterMode::_Global => {
        let mut counters = match GLOBAL_COUNTERS.lock() {
          Ok(c) => c,
          Err(e) => stylex_panic!("GLOBAL_COUNTERS mutex poisoned: {}", e),
        };
        counters.remove(&self.prefix);
      },
      CounterMode::Local => {
        self.local_counter.store(1, Ordering::SeqCst);
      },
      CounterMode::ThreadLocal => {
        THREAD_LOCAL_COUNTERS.with(|counters| {
          counters.borrow_mut().remove(&self.prefix);
        });
      },
      CounterMode::_ThreadUnique => {
        // Thread unique mode doesn't maintain persistent counters
      },
    }
  }

  pub fn generate(&self) -> String {
    match self.mode {
      CounterMode::_Global => {
        let counters = match GLOBAL_COUNTERS.lock() {
          Ok(c) => c,
          Err(e) => stylex_panic!("GLOBAL_COUNTERS mutex poisoned: {}", e),
        };
        let counter = match counters.get(&self.prefix) {
          Some(c) => c,
          None => stylex_panic!(
            "Counter for prefix '{}' not found in GLOBAL_COUNTERS",
            self.prefix
          ),
        };
        let count = counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}{}", self.prefix, count_string)
      },
      CounterMode::Local => {
        let count = self.local_counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}{}", self.prefix, count_string)
      },
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
      },
      CounterMode::_ThreadUnique => {
        let thread_id = thread::current().id();
        let count = self.local_counter.fetch_add(1, Ordering::SeqCst);

        let count_string = if count < 2 {
          String::default()
        } else {
          count.to_string()
        };

        format!("_{}_{:?}{}", self.prefix, thread_id, count_string)
      },
    }
  }

  /// Generates a unique identifier.
  pub fn generate_ident(&self) -> Ident {
    let unique_name = self.generate();

    Ident::from(unique_name.as_str())
  }
}

