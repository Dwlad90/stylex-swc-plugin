/*!
CSS Number type parsing.

Handles pure number values without units.
Provides number parsing functionality for CSS numeric values.
*/

pub use crate::css_types::common_types::Number;

pub use Number as CssNumber;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number_reexport() {
    let num = Number::new(42.5);
    assert_eq!(num.value, 42.5);
    assert_eq!(num.to_string(), "42.5");
  }

  #[test]
  fn test_css_number_alias() {
    let num = CssNumber::new(10.0);
    assert_eq!(num.value, 10.0);
  }
}
