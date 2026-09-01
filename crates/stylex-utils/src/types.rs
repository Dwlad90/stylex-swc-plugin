//! Reading the static type of a value as a string.

use std::any::type_name;

/// The compile-time name of `T`, for a comparison or a message that has to say
/// which type a value has.
pub fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

#[cfg(test)]
#[path = "tests/types_test.rs"]
mod tests;
