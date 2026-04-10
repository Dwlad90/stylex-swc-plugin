#[cfg(test)]
mod create_hash_tests {
  use crate::hash::create_hash;

  #[test]
  fn returns_consistent_hash() {
    let hash1 = create_hash("hello");
    let hash2 = create_hash("hello");
    assert_eq!(hash1, hash2);
  }

  #[test]
  fn different_inputs_produce_different_hashes() {
    assert_ne!(create_hash("hello"), create_hash("world"));
  }

  #[test]
  fn returns_non_empty_string() {
    assert!(!create_hash("test").is_empty());
  }

  #[test]
  fn handles_empty_string() {
    let hash = create_hash("");
    assert!(!hash.is_empty());
  }

  #[test]
  fn handles_unicode_input() {
    let hash = create_hash("日本語");
    assert!(!hash.is_empty());
  }

  #[test]
  fn handles_long_input() {
    let long = "a".repeat(10000);
    let hash = create_hash(&long);
    assert!(!hash.is_empty());
  }
}

#[cfg(test)]
mod stable_hash_tests {
  use crate::hash::stable_hash;

  #[test]
  fn returns_consistent_hash_for_same_value() {
    assert_eq!(stable_hash(&42u64), stable_hash(&42u64));
  }

  #[test]
  fn different_values_produce_different_hashes() {
    assert_ne!(stable_hash(&1u64), stable_hash(&2u64));
  }

  #[test]
  fn works_with_strings() {
    assert_eq!(stable_hash(&"test"), stable_hash(&"test"));
    assert_ne!(stable_hash(&"a"), stable_hash(&"b"));
  }

  #[test]
  fn works_with_tuples() {
    assert_eq!(stable_hash(&(1, 2)), stable_hash(&(1, 2)));
    assert_ne!(stable_hash(&(1, 2)), stable_hash(&(2, 1)));
  }
}

#[cfg(test)]
mod create_short_hash_tests {
  use crate::hash::create_short_hash;

  #[test]
  fn returns_consistent_hash() {
    assert_eq!(create_short_hash("hello"), create_short_hash("hello"));
  }

  #[test]
  fn different_inputs_produce_different_hashes() {
    assert_ne!(create_short_hash("hello"), create_short_hash("world"));
  }

  #[test]
  fn returns_non_empty_string() {
    assert!(!create_short_hash("test").is_empty());
  }

  #[test]
  fn produces_short_output() {
    // base62 encoded, mod 62^5, should be at most 5 chars
    assert!(create_short_hash("test").len() <= 5);
  }
}

#[cfg(test)]
mod hash_f64_tests {
  use crate::hash::hash_f64;

  #[test]
  fn returns_consistent_hash_for_same_value() {
    assert_eq!(hash_f64(1.23456), hash_f64(1.23456));
  }

  #[test]
  fn different_values_produce_different_hashes() {
    assert_ne!(hash_f64(1.0), hash_f64(2.0));
  }

  #[test]
  fn zero_and_neg_zero_differ() {
    // In IEEE 754, 0.0 and -0.0 have different bit patterns
    assert_ne!(hash_f64(0.0), hash_f64(-0.0));
  }

  #[test]
  fn handles_special_values() {
    let _ = hash_f64(f64::INFINITY);
    let _ = hash_f64(f64::NEG_INFINITY);
    let _ = hash_f64(f64::NAN);
  }
}
