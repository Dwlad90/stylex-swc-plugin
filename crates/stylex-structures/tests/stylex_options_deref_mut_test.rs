//! Integration tests for runtime DerefMut coverage on StyleXOptions.

use stylex_structures::stylex_options::StyleXOptions;

/// Mutating core fields through StyleXOptions should use its DerefMut implementation.
#[test]
fn stylex_options_supports_deref_mut_to_core() {
  let mut options = StyleXOptions::default();

  options.dev = true;
  options.test = true;

  assert!(options.core.dev);
  assert!(options.core.test);
}
