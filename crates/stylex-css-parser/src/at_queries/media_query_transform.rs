/*!
Media query transformation functionality.

Implements the "last media query wins" transformation logic for CSS-in-JS.
This ensures proper specificity handling when multiple media queries target the same properties.

This implementation provides media query transformation:
1. DFS traversal of the style object
2. At depth >= 1, apply negation-based media query transformation
3. Use pure AST manipulation, not range-based logic
*/

use super::media_query::{MediaQuery, MediaQueryRule, MediaNotRule, MediaAndRules, MediaOrRules};
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str};
use swc_core::atoms::Atom;
use std::collections::HashMap;

/// Helper function to extract key as string from KeyValueProp
fn key_value_to_str(key_value: &KeyValueProp) -> String {
  match &key_value.key {
    PropName::Str(s) => s.value.to_string(),
    PropName::Ident(id) => id.sym.to_string(),
    _ => String::new(),
  }
}

/// Main entry point - equivalent to lastMediaQueryWinsTransform in JS
pub fn last_media_query_wins_transform(styles: &[KeyValueProp]) -> Vec<KeyValueProp> {
    dfs_process_queries_with_depth(styles, 0)
}

/// Internal helper function for backwards compatibility with existing tests
/// This preserves the old Vec<MediaQuery> -> Vec<MediaQuery> signature for internal use
pub fn last_media_query_wins_transform_internal(
  queries: Vec<MediaQuery>,
) -> Vec<MediaQuery> {
  // For now, just return the queries unchanged since the main tests are using KeyValueProp
  // The real transformation happens in last_media_query_wins_transform with KeyValueProp input
  queries
}

/// Helper function to create ObjectLit from key-value pairs
fn create_object_from_key_values(key_values: Vec<KeyValueProp>) -> ObjectLit {
  let props = key_values
    .into_iter()
    .map(|kv| PropOrSpread::Prop(Box::new(Prop::KeyValue(kv))))
    .collect();

  ObjectLit {
    span: DUMMY_SP,
    props,
  }
}

/// DFS traversal with depth tracking - matches JS dfsProcessQueries exactly
fn dfs_process_queries_with_depth(obj: &[KeyValueProp], depth: u32) -> Vec<KeyValueProp> {
  let mut result = Vec::new();

  for prop in obj {
    match &*prop.value {
      Expr::Array(_) => {
        // Ignore `firstThatWorks` arrays - pass through unchanged
        result.push(prop.clone());
      }
      Expr::Object(obj_lit) => {
        // Extract key-value pairs from the object
        let mut key_values = Vec::new();
        for obj_prop in &obj_lit.props {
          if let PropOrSpread::Prop(p) = obj_prop {
            if let Prop::KeyValue(kv) = &**p {
              key_values.push(kv.clone());
            }
          }
        }

        // Recursively process the object at depth + 1
        let processed_values = dfs_process_queries_with_depth(&key_values, depth + 1);
        let transformed_obj = create_object_from_key_values(processed_values);

        result.push(KeyValueProp {
          key: prop.key.clone(),
          value: Box::new(Expr::Object(transformed_obj)),
        });
      }
      _ => {
        // Non-object values pass through unchanged
        result.push(prop.clone());
      }
    }
  }

  // Apply media query transformation if at depth >= 1
  if depth >= 1 {
    transform_media_queries_in_result(result)
  } else {
    result
  }
}

/// Transform media queries in the result object - matches JS logic exactly
fn transform_media_queries_in_result(result: Vec<KeyValueProp>) -> Vec<KeyValueProp> {
  // Check if we have any media queries
  let has_media_queries = result.iter().any(|kv| {
    let key = key_value_to_str(kv);
    key.starts_with("@media ")
  });

  if !has_media_queries {
    return result;
  }

  // Collect all media query keys
  let media_keys: Vec<String> = result.iter()
    .filter_map(|kv| {
      let key = key_value_to_str(kv);
      if key.starts_with("@media ") {
        Some(key)
      } else {
        None
      }
    })
    .collect();

  if media_keys.len() <= 1 {
    return result;
  }

  // Build negations array - JS logic: for i from length-1 down to 1
  let mut negations = Vec::new();
  let mut accumulated_negations: Vec<Vec<MediaQuery>> = Vec::new();

  for i in (1..media_keys.len()).rev() {
    if let Ok(mq) = MediaQuery::parser().parse_to_end(&media_keys[i]) {
      negations.push(mq);
      accumulated_negations.push(negations.clone());
    }
  }
  accumulated_negations.reverse();
  accumulated_negations.push(Vec::new()); // Empty negations for the last query

  // Transform each media query
  let mut result_map = HashMap::new();

  // First, build a map of existing non-media properties
  for kv in &result {
    let key = key_value_to_str(kv);
    if !key.starts_with("@media ") {
      result_map.insert(key, kv.clone());
    }
  }

  // Process media queries with negations
  for (i, media_key) in media_keys.iter().enumerate() {
    if let Some(original_kv) = result.iter().find(|kv| key_value_to_str(kv) == *media_key) {
      if let Ok(base_mq) = MediaQuery::parser().parse_to_end(media_key) {
        let mut reversed_negations = accumulated_negations[i].clone();
        reversed_negations.reverse();

        let combined_query = combine_media_query_with_negations(base_mq, reversed_negations);
        let new_media_key = combined_query.to_string();

        result_map.insert(new_media_key, KeyValueProp {
          key: PropName::Str(Str {
            span: DUMMY_SP,
            value: Atom::from(combined_query.to_string()),
            raw: None,
          }),
          value: original_kv.value.clone(),
        });
      }
    }
  }

  // Convert back to Vec, preserving order (non-media first, then media)
  let mut final_result = Vec::new();

  // Add non-media properties first
  for kv in &result {
    let key = key_value_to_str(kv);
    if !key.starts_with("@media ") {
      final_result.push(kv.clone());
    }
  }

  // Add transformed media queries
  for media_key in &media_keys {
    if let Ok(base_mq) = MediaQuery::parser().parse_to_end(media_key) {
      let i = media_keys.iter().position(|k| k == media_key).unwrap();
      let mut reversed_negations = accumulated_negations[i].clone();
      reversed_negations.reverse();

      let combined_query = combine_media_query_with_negations(base_mq, reversed_negations);
      let new_media_key = combined_query.to_string();

      if let Some(original_kv) = result.iter().find(|kv| key_value_to_str(kv) == *media_key) {
        final_result.push(KeyValueProp {
          key: PropName::Str(Str {
            span: DUMMY_SP,
            value: Atom::from(new_media_key),
            raw: None,
          }),
          value: original_kv.value.clone(),
        });
      }
    }
  }

  final_result
}

/// Combine media query with negations - matches JS combineMediaQueryWithNegations exactly
fn combine_media_query_with_negations(current: MediaQuery, negations: Vec<MediaQuery>) -> MediaQuery {
  if negations.is_empty() {
    return current;
  }

  // Create NOT rules from negations - matches JS: negations.map((mq) => ({ type: 'not', rule: mq.queries }))
  let not_rules: Vec<MediaQueryRule> = negations
    .into_iter()
    .map(|mq| MediaQueryRule::Not(MediaNotRule::new(mq.queries)))
    .collect();

  // Combine media query with negations
  let combined_ast = match current.queries {
    MediaQueryRule::Or(or_rules) => {
      let new_rules = or_rules.rules
        .into_iter()
        .map(|rule| {
          let mut and_rules = vec![rule];
          and_rules.extend(not_rules.clone());
          MediaQueryRule::And(MediaAndRules::new(and_rules))
        })
        .collect();
      MediaQueryRule::Or(MediaOrRules::new(new_rules))
    }
    other => {
      let mut rules = vec![other];
      rules.extend(not_rules);
      MediaQueryRule::And(MediaAndRules::new(rules))
    }
  };

  MediaQuery::new_from_rule(combined_ast)
}


