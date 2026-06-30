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
use swc_core::{
  atoms::Wtf8Atom,
  common::DUMMY_SP,
  ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str},
};

/// Helper function to extract key as string from KeyValueProp
fn key_value_to_str(key_value: &KeyValueProp) -> String {
  match &key_value.key {
    PropName::Str(s) => s.value.as_str().map(str::to_owned).unwrap_or_default(),
    PropName::Ident(id) => id.sym.to_string(),
    _ => String::new(),
  }
}

/// Main entry point - equivalent to lastMediaQueryWinsTransform in JS
pub fn last_media_query_wins_transform(styles: &[KeyValueProp]) -> Vec<KeyValueProp> {
  dfs_process_queries_with_depth(styles, 0)
}

/// Internal helper function for backwards compatibility with existing tests
/// This preserves the old `Vec<MediaQuery> -> Vec<MediaQuery>` signature for
/// internal use
pub fn last_media_query_wins_transform_internal(queries: Vec<MediaQuery>) -> Vec<MediaQuery> {
  // For now, just return the queries unchanged since the main tests are using
  // KeyValueProp The real transformation happens in
  // last_media_query_wins_transform with KeyValueProp input
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
      },
      Expr::Object(obj_lit) => {
        // Extract key-value pairs from the object. If the object contains
        // spreads/shorthands/methods, preserve it unchanged; silently dropping
        // those props would mutate user AST before the main StyleX validation
        // can report the unsupported non-static value.
        let mut key_values = Vec::with_capacity(obj_lit.props.len());
        let mut only_key_values = true;
        for obj_prop in &obj_lit.props {
          if let PropOrSpread::Prop(p) = obj_prop
            && let Prop::KeyValue(kv) = &**p
          {
            key_values.push(kv.clone());
          } else {
            only_key_values = false;
            break;
          }
        }

        if !only_key_values {
          result.push(prop.clone());
          continue;
        }

        // Recursively process the object at depth + 1
        let processed_values = dfs_process_queries_with_depth(&key_values, depth + 1);
        let transformed_obj = create_object_from_key_values(processed_values);

        result.push(KeyValueProp {
          key: prop.key.clone(),
          value: Box::new(Expr::Object(transformed_obj)),
        });
      },
      _ => {
        // Non-object values pass through unchanged
        result.push(prop.clone());
      },
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

  // Collect all media query key+prop pairs in declaration order.
  // Collecting the pair together avoids a second `.find()` scan later.
  let media_pairs: Vec<(String, KeyValueProp)> = result
    .iter()
    .filter_map(|kv| {
      let key = key_value_to_str(kv);
      if key.starts_with("@media ") {
        Some((key, kv.clone()))
      } else {
        None
      }
    })
    .collect();

  if media_pairs.len() <= 1 {
    return result;
  }

  let mut parsed_media_pairs = Vec::with_capacity(media_pairs.len());
  for (media_key, original_kv) in media_pairs {
    match MediaQuery::parser().parse_to_end(&media_key) {
      Ok(media_query) => parsed_media_pairs.push((media_key, original_kv, media_query)),
      Err(_) => {
        // Preserve the original AST. Dropping an invalid `@media` key here would
        // hide the real parser error from the later flattening/validation phase.
        return result;
      },
    }
  }

  // Check if all media queries are disjoint ranges - if so, just normalize syntax
  if are_media_queries_disjoint(&parsed_media_pairs) {
    return normalize_media_query_syntax(result);
  }

  // Build negations array - JS logic: for each media query, collect all later
  // queries in reverse declaration order.
  let mut accumulated_negations = vec![Vec::new(); parsed_media_pairs.len()];
  let mut later_negations = Vec::new();
  for i in (0..parsed_media_pairs.len()).rev() {
    accumulated_negations[i] = later_negations.clone();
    later_negations.push(parsed_media_pairs[i].2.clone());
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

  for (i, (_, original_kv, base_mq)) in parsed_media_pairs.into_iter().enumerate() {
    let mut reversed_negations = accumulated_negations[i].clone();
    reversed_negations.reverse();

    let combined_query = combine_media_query_with_negations(base_mq, reversed_negations);
    let new_media_key = combined_query.to_string();

    final_result.push(KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from(new_media_key),
        raw: None,
      }),
      value: original_kv.value,
    });
  }

  final_result
}

/// Combine media query with negations - matches JS
/// combineMediaQueryWithNegations exactly
fn combine_media_query_with_negations(
  current: MediaQuery,
  negations: Vec<MediaQuery>,
) -> MediaQuery {
  if negations.is_empty() {
    return current;
  }

  // Create NOT rules from negations - matches JS: negations.map((mq) => ({ type:
  // 'not', rule: mq.queries }))
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
    },
    other => {
      let mut rules = vec![other];
      rules.extend(not_rules);
      MediaQueryRule::And(MediaAndRules::new(rules))
    },
  };

  MediaQuery::new_from_rule(combined_ast)
}

/// Check if all media queries represent disjoint width/height ranges
fn are_media_queries_disjoint(media_pairs: &[(String, KeyValueProp, MediaQuery)]) -> bool {
  let mut ranges = Vec::new();

  for (_, _, media_query) in media_pairs {
    if let Some(range) = extract_width_height_range(media_query) {
      ranges.push(range);
    } else {
      // If any query is not a simple width/height range, don't apply disjoint logic
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
#[cfg_attr(coverage_nightly, coverage(off))]
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
    },
    _ => None,
  }
}

/// Check if two ranges overlap
#[cfg_attr(coverage_nightly, coverage(off))]
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
              value: Wtf8Atom::from(normalized_key),
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

#[cfg(test)]
#[path = "../tests/at_queries/media_query_transform_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/at_queries/media_query_transform_coverage_test.rs"]
mod media_query_transform_coverage_test;
