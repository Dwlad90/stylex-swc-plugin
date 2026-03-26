use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_f64(value: f64) -> u64 {
  let bits = value.to_bits();
  let mut hasher = DefaultHasher::new();
  bits.hash(&mut hasher);
  hasher.finish()
}
