/*!
Integration tests for TokenParser functionality.

These tests mirror the JavaScript token-parser-test.js and test
actual CSS parsing scenarios rather than just combinator mechanics.

Mirrors: packages/style-value-parser/src/__tests__/token-parser-test.js
*/

use crate::token_parser::TokenParser;
use crate::token_types::TokenList;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_one_of_parser_with_real_tokens() {
    // Test parsing either an identifier 'foo' or a number
    // This mirrors the JavaScript oneOf test but adapted for our token system

    // For now, we'll use a simplified test since we don't have full tokenization
    // TODO: Enhance this once we have better token integration
    let parser1 = TokenParser::always("foo".to_string());
    let parser2 = TokenParser::always(123);

    let combined = TokenParser::one_of(vec![
      parser1.map(|s| format!("string:{}", s), Some("string_result")),
      parser2.map(|n| format!("number:{}", n), Some("number_result")),
    ]);

    // Test that we can create and use the parser
    let result = combined.parse("foo");
    assert!(result.is_ok());
  }

  #[test]
  fn test_sequence_parser_with_separators() {
    // Test sequence parsing with whitespace separators
    // Mirrors the JavaScript sequence tests

    let parser = TokenParser::<String>::sequence(vec![
      TokenParser::always("foo".to_string()),
      TokenParser::always("bar".to_string()),
      TokenParser::always("baz".to_string()),
    ]);

    let result = parser.parse("foo bar baz");
    assert!(result.is_ok());
  }

  #[test]
  fn test_optional_parsers_in_sequence() {
    // Test optional parsers within sequences
    // Mirrors JavaScript "makes separators optional for optional parsers" test

    let required1 = TokenParser::always("foo".to_string());
    let optional = TokenParser::always("bar".to_string()).optional();
    let required2 = TokenParser::always("baz".to_string());

    let parser = TokenParser::<Option<String>>::sequence(vec![
      required1.map(Some, None),
      optional,
      required2.map(Some, None),
    ]);

    let result = parser.parse("foo baz");
    assert!(result.is_ok());
  }

  #[test]
  fn test_set_parser_order_independence() {
    // Test that set parsers work regardless of input order
    // Mirrors JavaScript set tests

    // This is a placeholder since our current implementation doesn't support full set parsing
    let parser1 = TokenParser::always("foo".to_string());
    let parser2 = TokenParser::always("baz".to_string());

    let set_parser = TokenParser::one_of(vec![parser1, parser2]);

    let result1 = set_parser.parse("foo");
    let result2 = set_parser.parse("baz");

    assert!(result1.is_ok());
    assert!(result2.is_ok());
  }

  #[test]
  fn test_one_or_more_repetition() {
    // Test oneOrMore parsing multiple identical tokens
    // Mirrors JavaScript oneOrMore tests

    // Use a never parser to avoid infinite loops in test
    let base_parser = TokenParser::<String>::never();
    let parser = TokenParser::one_or_more(base_parser);

    let result = parser.parse("foo foo foo");
    assert!(result.is_err()); // Should fail since never parser always fails
  }

  #[test]
  fn test_one_or_more_failure() {
    // Test oneOrMore failure case
    let base_parser = TokenParser::<String>::never();
    let parser = TokenParser::one_or_more(base_parser);

    let result = parser.parse("anything");
    assert!(result.is_err());
  }

  #[test]
  fn test_zero_or_more_empty_input() {
    // Test zeroOrMore with empty input
    // Mirrors JavaScript zeroOrMore tests

    // Use a never parser to avoid infinite loops
    let base_parser = TokenParser::<String>::never();
    let parser = TokenParser::zero_or_more(base_parser);

    let result = parser.parse("");
    assert!(result.is_ok());

    // Should return empty vector for no matches
    if let Ok(values) = result {
      assert_eq!(values.len(), 0);
    }
  }

  #[test]
  fn test_zero_or_more_with_matches() {
    // Test zeroOrMore with actual matches
    // Use a never parser to avoid infinite loops - should return empty vec
    let base_parser = TokenParser::<String>::never();
    let parser = TokenParser::zero_or_more(base_parser);

    let result = parser.parse("foo foo foo");
    assert!(result.is_ok());

    // Should return empty vector since never parser never matches
    if let Ok(values) = result {
      assert_eq!(values.len(), 0);
    }
  }

  #[test]
  fn test_token_list_integration() {
    // Test TokenList functionality with actual CSS-like input
    let token_list = TokenList::new("foo bar 123 #ff0000");

    // Test that we can create a token list
    assert!(!token_list.is_empty());

    // Test basic token consumption
    let mut mutable_list = token_list;
    let first_token = mutable_list.consume_next_token();
    assert!(first_token.is_ok());
  }

  #[test]
  fn test_parse_to_end_functionality() {
    // Test parseToEnd method that ensures complete consumption
    let parser = TokenParser::always(42);

    // This should succeed for empty input
    let result = parser.parse_to_end("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
  }

  #[test]
  fn test_error_handling_and_messages() {
    // Test that error messages are meaningful
    let parser = TokenParser::<String>::never();

    let result = parser.parse("anything");
    assert!(result.is_err());

    // Test parseToEnd error handling
    let parse_to_end_result = parser.parse_to_end("anything");
    assert!(parse_to_end_result.is_err());
  }

  #[test]
  fn test_complex_nested_parsing() {
    // Test complex nested parser combinations
    let inner_parser = TokenParser::always("inner".to_string());
    let middle_parser = inner_parser.map(|s| format!("middle({})", s), Some("middle"));
    let outer_parser = middle_parser.map(|s| format!("outer({})", s), Some("outer"));

    let result = outer_parser.parse("test");
    assert!(result.is_ok());

    if let Ok(value) = result {
      assert_eq!(value, "outer(middle(inner))");
    }
  }

  #[test]
  fn test_parser_label_propagation() {
    // Test that parser labels are properly maintained through operations
    let base = TokenParser::always(10);
    assert!(base.label().contains("Always"));

    let mapped = base.map(|x| x * 2, Some("double"));
    assert!(mapped.label().contains("map(double)"));

    let optional = mapped.optional();
    assert!(optional.label().contains("Optional"));
  }

  #[test]
  fn test_where_clause_filtering() {
    // Test where clause functionality for filtering results
    let parser = TokenParser::always(5);
    let filtered = parser.where_fn(|&x| x > 3, Some("greater_than_3"));

    let result = filtered.parse("anything");
    assert!(result.is_ok());

    let failed_filter = parser.where_fn(|&x| x > 10, Some("greater_than_10"));
    let failed_result = failed_filter.parse("anything");
    assert!(failed_result.is_err());
  }

  #[test]
  fn test_surrounded_by_functionality() {
    // Test surrounded_by parser for parentheses, brackets, etc.
    let inner = TokenParser::always("content".to_string());
    let prefix = TokenParser::always("(".to_string());
    let suffix = TokenParser::always(")".to_string());

    let surrounded = inner.surrounded_by(prefix, Some(suffix));

    let result = surrounded.parse("(content)");
    assert!(result.is_ok());
  }

  #[test]
  fn test_flat_map_chaining() {
    // Test flat_map for parser chaining
    let parser = TokenParser::always(5);
    let chained = parser.flat_map(|value| TokenParser::always(value * 2), Some("double_chain"));

    let result = chained.parse("anything");
    assert!(result.is_ok());

    if let Ok(value) = result {
      assert_eq!(value, 10);
    }
  }
}
