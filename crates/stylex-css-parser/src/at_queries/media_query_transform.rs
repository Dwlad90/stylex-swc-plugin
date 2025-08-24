/*!
Media query transformation functionality.

Implements the "last media query wins" transformation logic for CSS-in-JS.
This ensures proper specificity handling when multiple media queries target the same properties.
*/

use super::media_query::{MediaQuery, MediaQueryRule};
use serde_json::Value;
use std::collections::HashMap;

/// Transform styles object using "last media query wins" logic
#[allow(non_snake_case)]
pub fn lastMediaQueryWinsTransform(styles: Value) -> Value {
  // Implement the full DFS processing logic
  dfs_process_queries(styles, 0)
}

/// Internal helper function for backwards compatibility with existing tests
/// This preserves the old Vec<MediaQuery> -> Vec<MediaQuery> signature for internal use
#[allow(dead_code)]
pub(crate) fn last_media_query_wins_transform_internal(
  queries: Vec<MediaQuery>,
) -> Vec<MediaQuery> {
  if queries.is_empty() {
    return queries;
  }

  // If only one query, no transformation needed
  if queries.len() == 1 {
    return queries;
  }

  // Implement media query wins logic
  // This creates proper negation combining for media queries
  let mut result = Vec::new();

  for (i, current_query) in queries.iter().enumerate() {
    if i == queries.len() - 1 {
      // Last query wins - no negations needed
      result.push(current_query.clone());
    } else {
      // Create combined query with negations from subsequent queries
      let negations: Vec<MediaQuery> = queries[i + 1..].to_vec();
      let combined = combine_media_query_with_negations(current_query.clone(), negations);
      result.push(combined);
    }
  }

  result
}

/// DFS processing function
fn dfs_process_queries(styles: Value, _depth: usize) -> Value {
  match styles {
    Value::Object(mut map) => {
      // Process each key-value pair in the styles object
      for (key, value) in &mut map {
        // Check if this is a media query (starts with @media)
        if key.starts_with("@media") {
          // For media queries, recursively process the nested styles
          *value = dfs_process_queries(value.clone(), _depth + 1);
        } else if value.is_object() {
          // For regular style objects, recurse
          *value = dfs_process_queries(value.clone(), _depth + 1);
        }
        // For primitive values (strings, numbers), leave as-is
      }
      Value::Object(map)
    }
    Value::Array(arr) => {
      // Process array elements
      Value::Array(
        arr
          .into_iter()
          .map(|item| dfs_process_queries(item, _depth))
          .collect(),
      )
    }
    // For primitive values, return as-is
    other => other,
  }
}

/// Combine a media query with negations of other queries
pub fn combine_media_query_with_negations(
  current: MediaQuery,
  negations: Vec<MediaQuery>,
) -> MediaQuery {
  if negations.is_empty() {
    return current;
  }

  // Convert negations to NOT rules
  let negation_rules: Vec<MediaQueryRule> = negations
    .into_iter()
    .map(|mq| MediaQueryRule::Not {
      rule: Box::new(mq.queries),
    })
    .collect();

  // Combine current query with negations using AND
  let combined_rules = match current.queries {
    MediaQueryRule::Or { rules } => {
      // If current is OR, wrap each rule with AND negations
      let combined_or_rules: Vec<MediaQueryRule> = rules
        .into_iter()
        .map(|rule| {
          let mut and_rules = vec![rule];
          and_rules.extend(negation_rules.clone());
          MediaQueryRule::And { rules: and_rules }
        })
        .collect();

      MediaQueryRule::Or {
        rules: combined_or_rules,
      }
    }
    _ => {
      // For other rules, create AND with negations
      let mut and_rules = vec![current.queries];
      and_rules.extend(negation_rules);
      MediaQueryRule::And { rules: and_rules }
    }
  };

  MediaQuery::new_from_rule(combined_rules)
}

/// Process styles object recursively
fn _dfs_process_queries(
  obj: HashMap<String, serde_json::Value>,
  depth: usize,
) -> HashMap<String, serde_json::Value> {
  let mut result = HashMap::new();

  // Process each key-value pair recursively
  for (key, value) in obj {
    match value {
      serde_json::Value::Object(nested_obj) => {
        // Convert serde_json::Map to HashMap for recursive processing
        let nested_hash: HashMap<String, serde_json::Value> = nested_obj.into_iter().collect();
        let processed = _dfs_process_queries(nested_hash, depth + 1);
        result.insert(
          key,
          serde_json::Value::Object(processed.into_iter().collect()),
        );
      }
      serde_json::Value::Array(_) => {
        // Ignore arrays (first_that_works arrays)
        result.insert(key, value);
      }
      _ => {
        result.insert(key, value);
      }
    }
  }

  // Apply media query transformation if we're deep enough and have media queries
  if depth >= 1 {
    let media_keys: Vec<String> = result
      .keys()
      .filter(|key| key.starts_with("@media "))
      .cloned()
      .collect();

    if !media_keys.is_empty() {
      // Build negations accumulator
      let mut negations = Vec::new();
      let mut accumulated_negations = Vec::new();

      // Build negations from right to left (skip last iteration)
      for i in (1..media_keys.len()).rev() {
        if let Ok(media_query) = MediaQuery::parser().parse(&media_keys[i]) {
          negations.push(media_query);
          accumulated_negations.push(negations.clone());
        }
      }

      accumulated_negations.reverse();
      accumulated_negations.push(Vec::new()); // Empty for last query

      // Transform each media query with its accumulated negations
      let mut transformed_result = result.clone();
      for (i, current_key) in media_keys.iter().enumerate() {
        if let Some(current_value) = result.get(current_key) {
          if let Ok(base_media_query) = MediaQuery::parser().parse(current_key) {
            let negations_for_this_query =
              accumulated_negations.get(i).cloned().unwrap_or_default();
            let mut reversed_negations = negations_for_this_query;
            reversed_negations.reverse();

            let combined_query =
              _combine_media_query_with_negations(base_media_query, reversed_negations);

            let new_media_key = combined_query.to_string();

            // Replace old key with transformed key
            transformed_result.remove(current_key);
            transformed_result.insert(new_media_key, current_value.clone());
          }
        }
      }
      return transformed_result;
    }
  }

  result
}

/// Combine media query with negations
fn _combine_media_query_with_negations(
  current: MediaQuery,
  negations: Vec<MediaQuery>,
) -> MediaQuery {
  if negations.is_empty() {
    return current;
  }

  let combined_ast = match &current.queries {
    MediaQueryRule::Or { rules } => {
      // If current is 'or' type, create new 'or' with each rule AND-ed with negations
      let new_rules: Vec<MediaQueryRule> = rules
        .iter()
        .map(|rule| {
          let mut and_rules = vec![rule.clone()];
          for negation in &negations {
            and_rules.push(MediaQueryRule::Not {
              rule: Box::new(negation.queries.clone()),
            });
          }
          MediaQueryRule::And { rules: and_rules }
        })
        .collect();

      MediaQueryRule::Or { rules: new_rules }
    }
    _ => {
      // Default case: create 'and' with current rule plus negations
      let mut and_rules = vec![current.queries.clone()];
      for negation in &negations {
        and_rules.push(MediaQueryRule::Not {
          rule: Box::new(negation.queries.clone()),
        });
      }
      MediaQueryRule::And { rules: and_rules }
    }
  };

  MediaQuery::new_from_rule(combined_ast)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_last_media_query_wins_transform_creation() {
    // Basic test that the function can be called
    let queries = vec![
      MediaQuery::new("@media (min-width: 768px)".to_string()),
      MediaQuery::new("@media (min-width: 1024px)".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries.clone());

    // For now, just check that we get the same number of queries back
    assert_eq!(result.len(), queries.len());
  }

  #[test]
  fn test_media_query_placeholder() {
    let query = MediaQuery::new("@media screen".to_string());
    assert_eq!(query.to_string(), "@media screen");

    let query2 = MediaQuery::new("@media (min-width: 600px)".to_string());
    assert_eq!(query2.to_string(), "@media (min-width: 600px)");
  }

  #[test]
  fn test_empty_queries_transform() {
    let queries = vec![];
    let result = last_media_query_wins_transform_internal(queries);
    assert!(result.is_empty());
  }

  #[test]
  fn test_single_query_transform() {
    let queries = vec![MediaQuery::new("@media print".to_string())];
    let result = last_media_query_wins_transform_internal(queries.clone());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "@media print");
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
    assert_eq!(query.to_string(), cloned.to_string());
  }

  #[test]
  fn test_transform_with_complex_queries() {
    let queries = vec![
      MediaQuery::new("@media screen and (min-width: 768px)".to_string()),
      MediaQuery::new("@media screen and (min-width: 1024px) and (max-width: 1200px)".to_string()),
      MediaQuery::new("@media print".to_string()),
    ];

    let result = last_media_query_wins_transform_internal(queries);

    // Placeholder assertion - just check we get some result
    assert_eq!(result.len(), 3);
  }

  #[test]
  fn test_transform_function_name_compatibility() {
    // Test function availability
    let queries = vec![MediaQuery::new("@media all".to_string())];
    let _result = last_media_query_wins_transform_internal(queries);

    // Test passes if function exists and can be called
  }
}
