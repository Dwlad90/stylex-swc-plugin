/*!
CSS Alpha Value type parsing.

Handles alpha channel values for colors, accepting both numbers (0-1) and percentages (0-100%).
Mirrors: packages/style-value-parser/src/css-types/alpha-value.js
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// CSS Alpha value (0.0 to 1.0)
/// Mirrors: AlphaValue class in alpha-value.js
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue {
    pub value: f32, // Always stored as 0.0 to 1.0
}

impl AlphaValue {
    /// Create a new AlphaValue
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    /// Create from percentage (0-100 range)
    pub fn from_percentage(percentage: f32) -> Self {
        Self {
            value: percentage / 100.0,
        }
    }

    /// Check if the alpha value is valid (0.0 to 1.0)
    pub fn is_valid(value: f32) -> bool {
        value >= 0.0 && value <= 1.0
    }

    /// Parser for CSS alpha values
    /// Mirrors: AlphaValue.parser
    pub fn parser() -> TokenParser<AlphaValue> {
        // Parser for percentage values (converted to 0-1 range)
        let percentage_parser = TokenParser::<SimpleToken>::token(
            SimpleToken::Percentage(0.0),
            Some("Percentage")
        )
        .map(
            |token| {
                if let SimpleToken::Percentage(value) = token {
                    // Handle sign and convert percentage to 0-1 range
                    AlphaValue::from_percentage(value as f32)
                } else {
                    unreachable!()
                }
            },
            Some("percentage_to_alpha"),
        );

        // Parser for number values (0-1 range)
        let number_parser = TokenParser::<SimpleToken>::token(
            SimpleToken::Number(0.0),
            Some("Number")
        )
        .map(
            |token| {
                if let SimpleToken::Number(value) = token {
                    // Handle sign
                    AlphaValue::new(value as f32)
                } else {
                    unreachable!()
                }
            },
            Some("number_to_alpha"),
        );

        // Combine both parsers - percentage first, then number
        TokenParser::one_of(vec![percentage_parser, number_parser])
    }
}

impl Display for AlphaValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_value_creation() {
        let alpha = AlphaValue::new(0.5);
        assert_eq!(alpha.value, 0.5);
    }

    #[test]
    fn test_alpha_value_from_percentage() {
        let alpha_50 = AlphaValue::from_percentage(50.0);
        assert_eq!(alpha_50.value, 0.5);

        let alpha_100 = AlphaValue::from_percentage(100.0);
        assert_eq!(alpha_100.value, 1.0);

        let alpha_0 = AlphaValue::from_percentage(0.0);
        assert_eq!(alpha_0.value, 0.0);

        let alpha_75 = AlphaValue::from_percentage(75.0);
        assert_eq!(alpha_75.value, 0.75);
    }

    #[test]
    fn test_alpha_value_display() {
        let alpha_half = AlphaValue::new(0.5);
        assert_eq!(alpha_half.to_string(), "0.5");

        let alpha_full = AlphaValue::new(1.0);
        assert_eq!(alpha_full.to_string(), "1");

        let alpha_zero = AlphaValue::new(0.0);
        assert_eq!(alpha_zero.to_string(), "0");

        let alpha_quarter = AlphaValue::new(0.25);
        assert_eq!(alpha_quarter.to_string(), "0.25");
    }

    #[test]
    fn test_alpha_value_validation() {
        assert!(AlphaValue::is_valid(0.0));
        assert!(AlphaValue::is_valid(0.5));
        assert!(AlphaValue::is_valid(1.0));

        // Edge cases - technically these might be invalid in CSS but we test the function
        assert!(!AlphaValue::is_valid(-0.1));
        assert!(!AlphaValue::is_valid(1.1));
        assert!(!AlphaValue::is_valid(2.0));
    }

    #[test]
    fn test_alpha_value_parser_creation() {
        // Basic test that parser can be created
        let _parser = AlphaValue::parser();
    }

    #[test]
    fn test_alpha_value_equality() {
        let alpha1 = AlphaValue::new(0.5);
        let alpha2 = AlphaValue::new(0.5);
        let alpha3 = AlphaValue::new(0.75);

        assert_eq!(alpha1, alpha2);
        assert_ne!(alpha1, alpha3);
    }

    #[test]
    fn test_alpha_value_common_values() {
        // Test common alpha values
        let transparent = AlphaValue::new(0.0);
        assert_eq!(transparent.value, 0.0);

        let half_transparent = AlphaValue::new(0.5);
        assert_eq!(half_transparent.value, 0.5);

        let opaque = AlphaValue::new(1.0);
        assert_eq!(opaque.value, 1.0);

        // From percentages
        let from_percent_50 = AlphaValue::from_percentage(50.0);
        assert_eq!(from_percent_50.value, 0.5);
        assert_eq!(from_percent_50, half_transparent);
    }

    #[test]
    fn test_alpha_value_precision() {
        // Test decimal precision is maintained
        let precise = AlphaValue::new(0.123456);
        assert!((precise.value - 0.123456).abs() < 0.000001);

        let from_percent = AlphaValue::from_percentage(12.3456);
        assert!((from_percent.value - 0.123456).abs() < 0.000001);
    }
}
