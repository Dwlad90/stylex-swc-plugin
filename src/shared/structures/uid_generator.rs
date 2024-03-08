use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};

use swc_core::{
    common::{Mark, DUMMY_SP},
    ecma::ast::Ident,
};

// lazy_static! {
//     static ref COUNTERS: DashMap<String, AtomicUsize> = DashMap::new();
// }

/// A thread-safe generator for unique identifiers.
pub(crate) struct UidGenerator {
    prefix: String,
    COUNTERS: DashMap<String, AtomicUsize>,
}

impl UidGenerator {
    /// Creates a new IdGenerator with the given prefix.
    pub fn new(prefix: &str) -> Self {
        let COUNTERS = DashMap::new();
        COUNTERS
            .entry(prefix.to_string())
            .or_insert_with(|| AtomicUsize::new(1));
        Self {
            prefix: prefix.to_string(),
            COUNTERS,
        }
    }
    pub fn clear(&self) {
        self.COUNTERS.remove(&self.prefix);
    }
    pub fn generate(&self) -> (String) {
        // let mark = Mark::fresh(Mark::root());
        let counter = self.COUNTERS.get(&self.prefix).unwrap();
        let count = counter.fetch_add(1, Ordering::SeqCst);
        let unique_name = format!(
            "_{}{}",
            self.prefix,
            if count < 2 {
                "".to_string()
            } else {
                count.to_string()
            }
        );
        (unique_name)
    }

    /// Generates a unique identifier.
    pub fn generate_ident(&self) -> Ident {
        let (unique_name) = self.generate();

        Ident::new(unique_name.into(), DUMMY_SP)
    }
}
