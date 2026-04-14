use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

/// Hashes a float value by converting to its bit representation first.
pub fn hash_f64(value: f64) -> u64 {
  let bits = value.to_bits();
  let mut hasher = DefaultHasher::new();
  bits.hash(&mut hasher);
  hasher.finish()
}

/// Creates a base-36 hash of a string using murmur2.
pub fn create_hash(value: &str) -> String {
  radix_fmt::radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}

/// Deterministic hash using `DefaultHasher` (SipHash-based).
pub fn stable_hash<T: Hash>(t: &T) -> u64 {
  let mut hasher = DefaultHasher::new();
  t.hash(&mut hasher);
  hasher.finish()
}

/// Creates a short base-62 hash of a string using murmur2.
pub fn create_short_hash(value: &str) -> String {
  let hash = murmur2::murmur2(value.as_bytes(), 1) % (62u32.pow(5));
  base62::encode(hash)
}

#[cfg(test)]
#[path = "tests/hash_test.rs"]
mod tests;
