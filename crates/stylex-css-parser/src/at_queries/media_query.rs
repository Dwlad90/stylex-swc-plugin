/*!
Media query parsing and representation.

Simplified implementation - will be enhanced later.
Mirrors: packages/style-value-parser/src/at-queries/media-query.js
*/

use crate::token_parser::TokenParser;
use std::fmt::{self, Display};

/// Simplified MediaQuery for now
/// TODO: Implement full media query parsing structure
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQuery {
    pub query_string: String,
}

/// Simplified MediaQueryRule type
/// TODO: Implement full rule type system
#[derive(Debug, Clone, PartialEq)]
pub enum MediaQueryRule {
    Placeholder(String),
}

impl MediaQuery {
    /// Create a new MediaQuery
    pub fn new(query_string: String) -> Self {
        Self { query_string }
    }

    /// Placeholder parser - will be enhanced
    pub fn parser() -> TokenParser<MediaQuery> {
        // TODO: Implement full media query parsing
        TokenParser::never()
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        self.query_string.clone()
    }
}

impl Display for MediaQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query_string)
    }
}

/// Validate media query string
pub fn validate_media_query(input: &str) -> Result<MediaQuery, String> {
    // Check for balanced parentheses
    if !has_balanced_parens(input) {
        return Err(crate::at_queries::messages::MediaQueryErrors::UNBALANCED_PARENS.to_string());
    }

    // For now, just create a MediaQuery with the input string
    Ok(MediaQuery::new(input.to_string()))
}

/// Check if parentheses are balanced
fn has_balanced_parens(input: &str) -> bool {
    let mut count = 0;
    for ch in input.chars() {
        match ch {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_query_creation() {
        let query = MediaQuery::new("@media screen".to_string());
        assert_eq!(query.to_string(), "@media screen");
    }

    #[test]
    fn test_media_query_display() {
        let query = MediaQuery::new("@media (min-width: 768px)".to_string());
        assert_eq!(format!("{}", query), "@media (min-width: 768px)");
    }

    #[test]
    fn test_has_balanced_parens() {
        assert!(has_balanced_parens("(min-width: 768px)"));
        assert!(has_balanced_parens("(min-width: 768px) and (max-width: 1200px)"));
        assert!(has_balanced_parens("screen"));
        assert!(has_balanced_parens(""));

        assert!(!has_balanced_parens("(min-width: 768px"));
        assert!(!has_balanced_parens("min-width: 768px)"));
        assert!(!has_balanced_parens("((min-width: 768px)"));
    }

    #[test]
    fn test_validate_media_query_success() {
        let result = validate_media_query("@media (min-width: 768px)");
        assert!(result.is_ok());

        let query = result.unwrap();
        assert_eq!(query.to_string(), "@media (min-width: 768px)");
    }

    #[test]
    fn test_validate_media_query_unbalanced_parens() {
        let result = validate_media_query("@media (min-width: 768px");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("parentheses"));
    }

    #[test]
    fn test_media_query_parser_creation() {
        // Test that parser can be created (even if it's a placeholder)
        let _parser = MediaQuery::parser();
    }

    #[test]
    fn test_media_query_equality() {
        let query1 = MediaQuery::new("@media screen".to_string());
        let query2 = MediaQuery::new("@media screen".to_string());
        let query3 = MediaQuery::new("@media print".to_string());

        assert_eq!(query1, query2);
        assert_ne!(query1, query3);
    }

    #[test]
    fn test_media_query_clone() {
        let query = MediaQuery::new("@media (orientation: landscape)".to_string());
        let cloned = query.clone();

        assert_eq!(query, cloned);
    }

    #[test]
    fn test_common_media_queries() {
        let queries = vec![
            "@media screen",
            "@media print",
            "@media (min-width: 768px)",
            "@media screen and (min-width: 768px)",
            "@media (min-width: 768px) and (max-width: 1024px)",
            "@media not screen",
            "@media only screen and (min-width: 768px)",
        ];

        for query_str in queries {
            let result = validate_media_query(query_str);
            assert!(result.is_ok(), "Failed to validate: {}", query_str);

            let query = result.unwrap();
            assert_eq!(query.to_string(), query_str);
        }
    }

    #[test]
    fn test_complex_parentheses() {
        let complex_query = "@media screen and ((min-width: 768px) and (max-width: 1024px))";
        let result = validate_media_query(complex_query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_unbalanced_parentheses() {
        let invalid_queries = vec![
            "@media ((min-width: 768px)",
            "@media (min-width: 768px))",
            "@media (((min-width: 768px)",
            "@media (min-width: 768px)))",
        ];

        for query_str in invalid_queries {
            let result = validate_media_query(query_str);
            assert!(result.is_err(), "Should have failed: {}", query_str);
        }
    }
}
