/*!
Validation media query tests.
*/

use crate::at_queries::{validate_media_query, MediaQueryErrors};

#[cfg(test)]
mod media_query_parser_with_simplified_errors {
  use super::*;

  #[test]
  fn throws_syntax_error_for_empty_or_incomplete_conditions() {
    let syntax_error_cases = vec![
      "@media",
      "@media ",
      "@media ()",
      "@media not (min-width: )",
      "@media (width:)",
      "@media (min-width:)",
      "@media (max-width: )",
      "@media and",
    ];

    for invalid_query in syntax_error_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn throws_syntax_error_for_malformed_expressions_or_invalid_operators() {
    let malformed_cases = vec![
      "@media (min-width: 700px and max-width: 767px)",
      "@media (min-width:445px; max-width:768px)",
      "@media (width > )",
      "@media ( > 600px)",
      "@media (600px > width) or",
      "@media (width < )",
      "@media (width <=)",
      "@media (>= width)",
      "@media (300px < width < )",
    ];

    for invalid_query in malformed_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn syntax_error_for_invalid_colon_or_value() {
    let invalid_cases = vec![
      "@media (width :)",
      "@media (: 600px)",
      "@media (width: #$%)",
      "@media (width: [])",
    ];

    for invalid_query in invalid_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn syntax_error_for_invalid_var_usage() {
    let invalid_var_cases = vec![
      "@media (min-width: var(--test))",
      "@media (min-width: var(--foo) and (max-width: 700px))",
      "@media (min-width: var(foo) and (max-width: 700px))",
    ];

    for invalid_query in invalid_var_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn syntax_error_for_invalid_symbols() {
    let invalid_symbol_cases = vec!["@media (width @ 600px)"];

    for invalid_query in invalid_symbol_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn syntax_error_for_misused_logical_operators() {
    let logical_error_cases = vec![
      "@media ((width: 600px) and)",
      "@media and (min-width: 600px)",
      "@media or (max-width: 1200px)",
      "@media (color) and",
    ];

    for invalid_query in logical_error_cases {
      let result = validate_media_query(invalid_query);
      assert!(result.is_err(), "Should fail for: {}", invalid_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::SYNTAX_ERROR,
        "Should return SYNTAX_ERROR for: {}",
        invalid_query
      );
    }
  }

  #[test]
  fn unbalanced_parens() {
    let unbalanced_cases = vec![
      "@media (width: 600px",
      "@media screen and (color",
      "@media not (min-resolution: 300dpi",
      "@media (orientation: portrait",
      "@media ((min-width: 300px) and (max-width: 1000px)",
      "@media (hover: hover) and (pointer: fine",
      "@media (width: calc(100% - 50px)",
      "@media (aspect-ratio: (16/9",
      "@media screen and ((min-width: 640px)",
      "@media ((prefers-color-scheme: dark)",
    ];

    for unbalanced_query in unbalanced_cases {
      let result = validate_media_query(unbalanced_query);
      assert!(result.is_err(), "Should fail for: {}", unbalanced_query);
      assert_eq!(
        result.unwrap_err(),
        MediaQueryErrors::UNBALANCED_PARENS,
        "Should return UNBALANCED_PARENS for: {}",
        unbalanced_query
      );
    }
  }

  #[test]
  fn valid_queries_pass_validation() {
    let valid_queries = vec![
      "@media screen",
      "@media print",
      "@media (min-width: 600px)",
      "@media (max-width: 1200px)",
      "@media (orientation: landscape)",
      "@media screen and (min-width: 600px)",
      "@media (min-width: 600px) and (max-width: 1200px)",
      "@media (color)",
      "@media (aspect-ratio: 16/9)",
      "@media not screen",
      "@media only print",
    ];

    for valid_query in valid_queries {
      let result = validate_media_query(valid_query);
      assert!(result.is_ok(), "Should succeed for: {}", valid_query);

      // Verify the parsed MediaQuery has content
      let media_query = result.unwrap();
      assert!(!media_query.to_string().is_empty());
    }
  }

  #[test]
  fn balanced_parentheses_checker() {
    use crate::at_queries::media_query::MediaQuery;

    // Test balanced cases
    let balanced_cases = vec![
      "@media screen",
      "@media (min-width: 600px)",
      "@media (min-width: 600px) and (max-width: 1200px)",
      "@media ((min-width: 400px) and (max-width: 800px))",
      "@media screen and (color)",
    ];

    for case in balanced_cases {
      assert!(
        MediaQuery::has_balanced_parens(case),
        "Should be balanced: {}",
        case
      );
    }

    // Test unbalanced cases
    let unbalanced_cases = vec![
      "@media (min-width: 600px",
      "@media screen and (color",
      "@media ((min-width: 400px) and (max-width: 800px)",
      "@media (width: calc(100% - 50px)",
    ];

    for case in unbalanced_cases {
      assert!(
        !MediaQuery::has_balanced_parens(case),
        "Should be unbalanced: {}",
        case
      );
    }
  }
}
