/*!
Media query transformation functionality.

Implements the "last media query wins" transformation logic for CSS-in-JS.
This ensures proper specificity handling when multiple media queries target the same properties.

This implementation provides media query transformation:
1. DFS traversal of the style object
2. At depth >= 1, apply negation-based media query transformation
3. Use pure AST manipulation, not range-based logic
*/

use super::media_query::{
  MediaAndRules, MediaNotRule, MediaOrRules, MediaQuery, MediaQueryRule, MediaRuleValue,
};
use std::collections::HashMap;
use swc_core::atoms::Atom;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str};

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
pub fn last_media_query_wins_transform_internal(queries: Vec<MediaQuery>) -> Vec<MediaQuery> {
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
  let media_keys: Vec<String> = result
    .iter()
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

  // Check if all media queries are disjoint ranges - if so, just normalize syntax
  if are_media_queries_disjoint(&media_keys) {
    return normalize_media_query_syntax(result);
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

        result_map.insert(
          new_media_key,
          KeyValueProp {
            key: PropName::Str(Str {
              span: DUMMY_SP,
              value: Atom::from(combined_query.to_string()),
              raw: None,
            }),
            value: original_kv.value.clone(),
          },
        );
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
fn combine_media_query_with_negations(
  current: MediaQuery,
  negations: Vec<MediaQuery>,
) -> MediaQuery {
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
      let new_rules = or_rules
        .rules
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

/// Check if all media queries represent disjoint width/height ranges
fn are_media_queries_disjoint(media_keys: &[String]) -> bool {
  let mut ranges = Vec::new();

  for media_key in media_keys {
    if let Ok(mq) = MediaQuery::parser().parse_to_end(media_key) {
      if let Some(range) = extract_width_height_range(&mq) {
        ranges.push(range);
      } else {
        // If any query is not a simple width/height range, don't apply disjoint logic
        return false;
      }
    } else {
      return false;
    }
  }

  // Check if all ranges are disjoint (no overlaps)
  for i in 0..ranges.len() {
    for j in (i + 1)..ranges.len() {
      if ranges_overlap(&ranges[i], &ranges[j]) {
        return false;
      }
    }
  }

  true
}

/// Extract width/height range from a media query if it's a simple range
fn extract_width_height_range(mq: &MediaQuery) -> Option<(String, f32, f32)> {
  match &mq.queries {
    MediaQueryRule::And(and_rules) if and_rules.rules.len() == 2 => {
      let mut min_val = None;
      let mut max_val = None;
      let mut dimension = None;

      for rule in &and_rules.rules {
        if let MediaQueryRule::Pair(pair) = rule {
          if pair.key.starts_with("min-width") || pair.key.starts_with("max-width") {
            if dimension.is_none() {
              dimension = Some("width".to_string());
            } else if dimension.as_ref() != Some(&"width".to_string()) {
              return None; // Mixed dimensions
            }

            if let MediaRuleValue::Length(length) = &pair.value {
              if pair.key.starts_with("min-") {
                min_val = Some(length.value);
              } else {
                max_val = Some(length.value);
              }
            } else {
              return None; // Non-length value
            }
          } else if pair.key.starts_with("min-height") || pair.key.starts_with("max-height") {
            if dimension.is_none() {
              dimension = Some("height".to_string());
            } else if dimension.as_ref() != Some(&"height".to_string()) {
              return None; // Mixed dimensions
            }

            if let MediaRuleValue::Length(length) = &pair.value {
              if pair.key.starts_with("min-") {
                min_val = Some(length.value);
              } else {
                max_val = Some(length.value);
              }
            } else {
              return None; // Non-length value
            }
          } else {
            return None; // Not a width/height rule
          }
        } else {
          return None; // Not a simple pair rule
        }
      }

      if let (Some(dim), Some(min), Some(max)) = (dimension, min_val, max_val) {
        Some((dim, min, max))
      } else {
        None
      }
    }
    _ => None,
  }
}

/// Check if two ranges overlap
fn ranges_overlap(range1: &(String, f32, f32), range2: &(String, f32, f32)) -> bool {
  // Only compare ranges of the same dimension
  if range1.0 != range2.0 {
    return false;
  }

  let (_, min1, max1) = range1;
  let (_, min2, max2) = range2;

  // Two ranges [min1, max1] and [min2, max2] overlap if:
  // min1 <= max2 && min2 <= max1
  min1 <= max2 && min2 <= max1
}

/// Just normalize media query syntax without applying negation logic
fn normalize_media_query_syntax(result: Vec<KeyValueProp>) -> Vec<KeyValueProp> {
  result
    .into_iter()
    .map(|kv| {
      let key = key_value_to_str(&kv);
      if key.starts_with("@media ") {
        if let Ok(mq) = MediaQuery::parser().parse_to_end(&key) {
          let normalized_key = mq.to_string();
          KeyValueProp {
            key: PropName::Str(Str {
              span: DUMMY_SP,
              value: Atom::from(normalized_key),
              raw: None,
            }),
            value: kv.value,
          }
        } else {
          kv
        }
      } else {
        kv
      }
    })
    .collect()
}
