/*!
Media query transformation functionality.

Implements the "last media query wins" transformation logic. This ensures proper
specificity handling when multiple media queries target the same properties.

This implementation provides media query transformation:
1. DFS traversal of the style object
2. At depth >= 1, apply negation-based media query transformation
3. Use pure AST manipulation, not range-based logic
*/

use super::media_query::{MediaAndRules, MediaNotRule, MediaOrRules, MediaQuery, MediaQueryRule};
use stylex_macros::stylex_panic;
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

/// Main entry point for the last-media-query-wins transform
pub fn last_media_query_wins_transform(styles: &[KeyValueProp]) -> Vec<KeyValueProp> {
  dfs_process_queries_with_depth(styles, 0)
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

/// DFS traversal with depth tracking
fn dfs_process_queries_with_depth(obj: &[KeyValueProp], depth: u32) -> Vec<KeyValueProp> {
  let mut result = Vec::new();

  for prop in obj {
    match &*prop.value {
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
        // Non-object values pass through unchanged, including the `firstThatWorks`
        // arrays this transform deliberately ignores.
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

/// Transform media queries in the result object
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

  let mut parsed_media_pairs = Vec::with_capacity(media_pairs.len());
  for (media_key, original_kv) in media_pairs {
    match MediaQuery::parser().parse_to_end(&media_key) {
      Ok(media_query) => parsed_media_pairs.push((original_kv, media_query)),
      Err(_) => {
        // An unparseable query is a hard error, not something to pass through:
        // no later phase rejects it, so returning here emitted the broken query
        // verbatim into the stylesheet. The caller catches this and reports it
        // as invalid media query syntax.
        stylex_panic!("Invalid media query: {}", media_key);
      },
    }
  }

  // Build negations array: for each media query, collect all later queries in
  // reverse declaration order.
  let mut accumulated_negations = vec![Vec::new(); parsed_media_pairs.len()];
  let mut later_negations = Vec::new();
  for i in (0..parsed_media_pairs.len()).rev() {
    accumulated_negations[i] = later_negations.clone();
    later_negations.push(parsed_media_pairs[i].1.clone());
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

  for (i, (original_kv, base_mq)) in parsed_media_pairs.into_iter().enumerate() {
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

/// Combine a media query with the negations of every query that follows it
fn combine_media_query_with_negations(
  current: MediaQuery,
  negations: Vec<MediaQuery>,
) -> MediaQuery {
  if negations.is_empty() {
    return current;
  }

  // Wrap each negated query in a `not` rule
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

#[cfg(test)]
#[path = "../tests/at_queries/media_query_transform_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/at_queries/media_query_transform_coverage_test.rs"]
mod media_query_transform_coverage_test;
