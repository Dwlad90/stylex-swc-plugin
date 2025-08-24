/*!
CSS Custom Identifier type parsing.

Handles custom identifiers that exclude CSS reserved keywords.
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// List of reserved CSS keywords that cannot be used as custom identifiers
const RESERVED_KEYWORDS: &[&str] = &[
  "unset",
  "initial",
  "inherit",
  "default",
  "none",
  "auto",
  "normal",
  "hidden",
  "visible",
  "revert",
  "revert-layer",
];

/// CSS Custom Identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomIdentifier {
  pub value: String,
}

impl CustomIdentifier {
  /// Create a new CustomIdentifier
  pub fn new(value: String) -> Self {
    Self { value }
  }

  /// Check if a string is a reserved keyword
  pub fn is_reserved_keyword(value: &str) -> bool {
    RESERVED_KEYWORDS.contains(&value.to_lowercase().as_str())
  }

  /// Check if a string is a valid custom identifier
  pub fn is_valid_custom_ident(value: &str) -> bool {
    !Self::is_reserved_keyword(value) && !value.is_empty()
  }

  /// Get the list of reserved keywords
  pub fn reserved_keywords() -> &'static [&'static str] {
    RESERVED_KEYWORDS
  }

  /// Parser for CSS custom identifiers
  pub fn parser() -> TokenParser<CustomIdentifier> {
    TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            value
          } else {
            unreachable!()
          }
        },
        Some("extract_ident"),
      )
      .where_fn(
        |value| Self::is_valid_custom_ident(value),
        Some("valid_custom_ident"),
      )
      .map(CustomIdentifier::new, Some("to_custom_identifier"))
  }
}

impl Display for CustomIdentifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_custom_identifier_creation() {
    let ident = CustomIdentifier::new("my-custom-id".to_string());
    assert_eq!(ident.value, "my-custom-id");
  }

  #[test]
  fn test_custom_identifier_display() {
    let ident = CustomIdentifier::new("button-style".to_string());
    assert_eq!(ident.to_string(), "button-style");
  }

  #[test]
  fn test_reserved_keywords() {
    // Test all reserved keywords
    assert!(CustomIdentifier::is_reserved_keyword("unset"));
    assert!(CustomIdentifier::is_reserved_keyword("initial"));
    assert!(CustomIdentifier::is_reserved_keyword("inherit"));
    assert!(CustomIdentifier::is_reserved_keyword("default"));
    assert!(CustomIdentifier::is_reserved_keyword("none"));
    assert!(CustomIdentifier::is_reserved_keyword("auto"));
    assert!(CustomIdentifier::is_reserved_keyword("normal"));
    assert!(CustomIdentifier::is_reserved_keyword("hidden"));
    assert!(CustomIdentifier::is_reserved_keyword("visible"));
    assert!(CustomIdentifier::is_reserved_keyword("revert"));
    assert!(CustomIdentifier::is_reserved_keyword("revert-layer"));

    // Test case insensitivity
    assert!(CustomIdentifier::is_reserved_keyword("UNSET"));
    assert!(CustomIdentifier::is_reserved_keyword("Initial"));
    assert!(CustomIdentifier::is_reserved_keyword("INHERIT"));

    // Test non-reserved words
    assert!(!CustomIdentifier::is_reserved_keyword("my-custom"));
    assert!(!CustomIdentifier::is_reserved_keyword("button"));
    assert!(!CustomIdentifier::is_reserved_keyword("primary"));
  }

  #[test]
  fn test_valid_custom_ident() {
    // Valid custom identifiers
    assert!(CustomIdentifier::is_valid_custom_ident("my-button"));
    assert!(CustomIdentifier::is_valid_custom_ident("primaryColor"));
    assert!(CustomIdentifier::is_valid_custom_ident("grid-area-1"));
    assert!(CustomIdentifier::is_valid_custom_ident("_underscore"));
    assert!(CustomIdentifier::is_valid_custom_ident("kebab-case"));
    assert!(CustomIdentifier::is_valid_custom_ident("camelCase"));

    // Invalid custom identifiers (reserved keywords)
    assert!(!CustomIdentifier::is_valid_custom_ident("inherit"));
    assert!(!CustomIdentifier::is_valid_custom_ident("initial"));
    assert!(!CustomIdentifier::is_valid_custom_ident("unset"));
    assert!(!CustomIdentifier::is_valid_custom_ident("none"));
    assert!(!CustomIdentifier::is_valid_custom_ident("auto"));

    // Invalid: empty string
    assert!(!CustomIdentifier::is_valid_custom_ident(""));

    // Test case insensitivity for reserved keywords
    assert!(!CustomIdentifier::is_valid_custom_ident("INHERIT"));
    assert!(!CustomIdentifier::is_valid_custom_ident("Initial"));
  }

  #[test]
  fn test_custom_identifier_parser_creation() {
    // Basic test that parser can be created
    let _parser = CustomIdentifier::parser();
  }

  #[test]
  fn test_custom_identifier_equality() {
    let ident1 = CustomIdentifier::new("my-id".to_string());
    let ident2 = CustomIdentifier::new("my-id".to_string());
    let ident3 = CustomIdentifier::new("other-id".to_string());

    assert_eq!(ident1, ident2);
    assert_ne!(ident1, ident3);
  }

  #[test]
  fn test_reserved_keywords_list() {
    let keywords = CustomIdentifier::reserved_keywords();

    // Should contain all expected reserved keywords
    assert!(keywords.contains(&"unset"));
    assert!(keywords.contains(&"initial"));
    assert!(keywords.contains(&"inherit"));
    assert!(keywords.contains(&"default"));
    assert!(keywords.contains(&"none"));
    assert!(keywords.contains(&"auto"));
    assert!(keywords.contains(&"normal"));
    assert!(keywords.contains(&"hidden"));
    assert!(keywords.contains(&"visible"));
    assert!(keywords.contains(&"revert"));
    assert!(keywords.contains(&"revert-layer"));

    // Should have exactly 11 CSS-wide keywords
    assert_eq!(keywords.len(), 11);
  }

  #[test]
  fn test_custom_identifier_common_cases() {
    // Test common CSS custom identifier patterns
    let animation_name = CustomIdentifier::new("slideIn".to_string());
    assert_eq!(animation_name.to_string(), "slideIn");

    let grid_area = CustomIdentifier::new("main-content".to_string());
    assert_eq!(grid_area.to_string(), "main-content");

    let counter_name = CustomIdentifier::new("section-counter".to_string());
    assert_eq!(counter_name.to_string(), "section-counter");

    // All should be valid
    assert!(CustomIdentifier::is_valid_custom_ident("slideIn"));
    assert!(CustomIdentifier::is_valid_custom_ident("main-content"));
    assert!(CustomIdentifier::is_valid_custom_ident("section-counter"));
  }
}
