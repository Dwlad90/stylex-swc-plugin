/*!
Media query transform tests.
*/

use crate::at_queries::{
  media_query::MediaQuery, media_query_transform::last_media_query_wins_transform_internal,
};

#[cfg(test)]
mod media_query_transformer {
  use super::*;

  #[test]
  fn basic_usage_multiple_widths() {
    // Expected transformations:
    // '@media (max-width: 1440px)' -> '@media (min-width: 1024.01px) and (max-width: 1440px)'
    // '@media (max-width: 1024px)' -> '@media (min-width: 768.01px) and (max-width: 1024px)'
    // '@media (max-width: 768px)' -> '@media (max-width: 768px)' (unchanged - wins)
    let queries = vec![
      MediaQuery::new("@media (max-width: 1440px)".to_string()),
      MediaQuery::new("@media (max-width: 1024px)".to_string()),
      MediaQuery::new("@media (max-width: 768px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries.clone());

    // Basic verification that transformation occurred
    assert_eq!(result.len(), 3);

    // The last query should remain unchanged (wins)
    assert_eq!(result[2].to_string(), "@media (max-width: 768px)");

    // First query should be transformed to avoid overlap with later queries using NOT clauses
    let first_result = result[0].to_string();
    assert!(
      first_result.contains("max-width") && first_result.contains("1440"),
      "First query should preserve original max-width constraint: {}",
      first_result
    );
    assert!(
      first_result.contains("not")
        && (first_result.contains("1024") || first_result.contains("768")),
      "First query should use NOT clauses to avoid overlap with later queries: {}",
      first_result
    );

    // Second query should be transformed to avoid overlap with third query using NOT clauses
    let second_result = result[1].to_string();
    assert!(
      second_result.contains("max-width") && second_result.contains("1024"),
      "Second query should preserve original max-width constraint: {}",
      second_result
    );
    assert!(
      second_result.contains("not") && second_result.contains("768"),
      "Second query should use NOT clauses to avoid overlap with later queries: {}",
      second_result
    );

    // Verify all transformed queries are valid media queries
    for (i, transformed) in result.iter().enumerate() {
      assert!(
        transformed.to_string().starts_with("@media"),
        "Transformed query {} should be valid media query: {}",
        i,
        transformed
      );
    }
  }

  #[test]
  fn does_not_modify_single_queries() {
    let single_query = vec![MediaQuery::new("@media (max-width: 1440px)".to_string())];
    let result = last_media_query_wins_transform_internal(single_query.clone());

    // Single queries should remain unchanged
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), single_query[0].to_string());
  }

  #[test]
  fn empty_queries() {
    let empty_queries: Vec<MediaQuery> = vec![];
    let result = last_media_query_wins_transform_internal(empty_queries);
    assert!(result.is_empty());
  }

  #[test]
  fn preserves_last_query() {
    let queries = vec![
      MediaQuery::new("@media (min-width: 768px)".to_string()),
      MediaQuery::new("@media (min-width: 1024px)".to_string()),
      MediaQuery::new("@media (max-width: 1200px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries.clone());

    // Last query should be preserved exactly (wins)
    assert_eq!(
      result.last().unwrap().to_string(),
      queries.last().unwrap().to_string()
    );
  }

  #[test]
  fn transform_with_complex_queries() {
    let queries = vec![
      MediaQuery::new("@media screen and (min-width: 768px)".to_string()),
      MediaQuery::new("@media screen and (min-width: 1024px) and (max-width: 1200px)".to_string()),
      MediaQuery::new("@media print".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries);
    assert_eq!(result.len(), 3);

    // All queries should be preserved in some form
    for query in &result {
      assert!(!query.to_string().is_empty());
    }
  }

  #[test]
  fn transform_with_mixed_types() {
    // Test mixed query types (width, height, orientation, etc.)
    let mixed_queries = vec![
      MediaQuery::new("@media (max-width: 1024px)".to_string()),
      MediaQuery::new("@media (orientation: landscape)".to_string()),
      MediaQuery::new("@media (max-height: 600px)".to_string()),
      MediaQuery::new("@media (prefers-color-scheme: dark)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(mixed_queries.clone());
    assert_eq!(result.len(), mixed_queries.len());

    // Since these are different query types (width, orientation, height, color-scheme),
    // they shouldn't conflict with each other much, but the transform function still applies
    assert!(
      result.iter().all(|q| !q.to_string().is_empty()),
      "All transformed queries should be non-empty"
    );
  }

  #[test]
  fn duplicate_queries() {
    let duplicate_queries = vec![
      MediaQuery::new("@media (max-width: 768px)".to_string()),
      MediaQuery::new("@media (max-width: 768px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(duplicate_queries.clone());
    assert_eq!(result.len(), duplicate_queries.len());

    // Last duplicate should remain unchanged
    assert_eq!(result[1].to_string(), duplicate_queries[1].to_string());
  }

  #[test]
  fn transform_maintains_media_prefix() {
    let queries = vec![
      MediaQuery::new("@media (min-width: 600px)".to_string()),
      MediaQuery::new("@media (min-width: 900px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries);

    // All results should maintain @media prefix
    for query in &result {
      assert!(
        query.to_string().starts_with("@media"),
        "Query should start with @media: {}",
        query
      );
    }
  }

  #[test]
  fn function_name_availability() {
    let queries = vec![MediaQuery::new("@media all".to_string())];
    let _result = last_media_query_wins_transform_internal(queries);

    // Test passes if function exists and can be called
  }

  #[test]
  fn lots_of_max_widths() {
    // Expected transformations:
    // '@media (max-width: 1440px)' -> '@media (min-width: 1024.01px) and (max-width: 1440px)'
    // '@media (max-width: 1024px)' -> '@media (min-width: 768.01px) and (max-width: 1024px)'
    // '@media (max-width: 768px)' -> '@media (min-width: 458.01px) and (max-width: 768px)'
    // '@media (max-width: 458px)' -> '@media (max-width: 458px)' (unchanged - wins)
    let queries = vec![
      MediaQuery::new("@media (max-width: 1440px)".to_string()),
      MediaQuery::new("@media (max-width: 1024px)".to_string()),
      MediaQuery::new("@media (max-width: 768px)".to_string()),
      MediaQuery::new("@media (max-width: 458px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries.clone());
    assert_eq!(result.len(), 4);

    // Last query (wins) should remain unchanged
    assert_eq!(
      result[3].to_string(),
      "@media (max-width: 458px)",
      "Last query should remain unchanged"
    );

    // Each earlier query should be transformed to avoid overlapping with later ones using NOT clauses
    let first_result = result[0].to_string();
    assert!(
      first_result.contains("max-width") && first_result.contains("1440"),
      "First query should preserve original max-width constraint: {}",
      first_result
    );
    assert!(
      first_result.contains("not")
        && (first_result.contains("1024")
          || first_result.contains("768")
          || first_result.contains("458")),
      "First query should use NOT clauses to avoid overlap with later queries: {}",
      first_result
    );

    let second_result = result[1].to_string();
    assert!(
      second_result.contains("max-width") && second_result.contains("1024"),
      "Second query should preserve original max-width constraint: {}",
      second_result
    );
    assert!(
      second_result.contains("not")
        && (second_result.contains("768") || second_result.contains("458")),
      "Second query should use NOT clauses to avoid overlap with later queries: {}",
      second_result
    );

    let third_result = result[2].to_string();
    assert!(
      third_result.contains("max-width") && third_result.contains("768"),
      "Third query should preserve original max-width constraint: {}",
      third_result
    );
    assert!(
      third_result.contains("not") && third_result.contains("458"),
      "Third query should use NOT clauses to avoid overlap with last query: {}",
      third_result
    );
  }
}
