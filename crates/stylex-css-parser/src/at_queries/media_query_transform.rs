/*!
Media query transformation functionality.

Implements the "last media query wins" transformation logic. This ensures proper
specificity handling when multiple media queries target the same properties.

This implementation provides media query transformation:
1. DFS traversal of the style object
2. At depth >= 1, apply negation-based media query transformation
3. Use pure AST manipulation, not range-based logic
*/

use super::media_query::{
  MediaAndRules, MediaNotRule, MediaOrRules, MediaQuery, MediaQueryRule, validate_media_query,
};
use indexmap::{IndexMap, map::Entry};
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

/// What identifies a property inside one object level.
///
/// The rewritten keys are held in a map, so this has to tell two properties
/// apart as reliably as a JavaScript object's own key does. A name the renderer
/// cannot spell -- numeric, bigint, computed -- comes back empty, and letting
/// every one of those share the empty string would merge properties that are
/// genuinely distinct. They get a key built from their position instead, with a
/// NUL byte in front so nothing an author could write can reach it.
fn map_key(key_value: &KeyValueProp, index: usize) -> String {
  match key_value_to_str(key_value) {
    name if !name.is_empty() => name,
    _ => format!("\0{index}"),
  }
}

/// Rewrite the `@media` keys of one object level so that a later query wins.
///
/// The keys live in an insertion-ordered map rather than a list, because the
/// reference implementation's `dfsProcessQueries` holds them in a plain
/// JavaScript object and rewrites each one with `delete result[old]` followed
/// by `result[new] = value`. Three consequences follow from that pair, and all
/// three are contract:
///
/// - deleting and re-adding moves a key to the end, so the rewritten media keys
///   end up after every other property, in their own declaration order
/// - assigning a key that is already present keeps that key's position and
///   replaces only its value, so two entries canonicalizing to one query text
///   leave one rule, at the earlier position, holding the later value
/// - the value is read from the map at the moment its key is rewritten, not
///   collected beforehand, so an earlier rewrite that landed on a later key is
///   what that later key then carries
///
/// The second is the one an author notices: one of their declarations is absent
/// from the output. That is faithful rather than accidental, and nothing is
/// reported for it, because the reference implementation reports nothing.
fn transform_media_queries_in_result(result: Vec<KeyValueProp>) -> Vec<KeyValueProp> {
  let is_media_key = |key: &str| key.starts_with("@media ");

  // Bail out before building the map so that a level with no media key is
  // handed back exactly as it arrived.
  if !result.iter().any(|kv| is_media_key(&key_value_to_str(kv))) {
    return result;
  }

  let mut entries: IndexMap<String, KeyValueProp> = IndexMap::with_capacity(result.len());
  for (index, kv) in result.into_iter().enumerate() {
    entries.insert(map_key(&kv, index), kv);
  }

  let media_keys: Vec<String> = entries
    .keys()
    .filter(|key| is_media_key(key))
    .cloned()
    .collect();

  let mut parsed_media_queries = Vec::with_capacity(media_keys.len());
  for media_key in &media_keys {
    // Validated rather than merely parsed, because the tokenizer synthesizes a
    // closing parenthesis at end of input: `(min-width: 100px` parses cleanly
    // here and would reach the stylesheet as a query the author never wrote.
    // The reference implementation's tokenizer synthesizes nothing, so its
    // parse fails outright on the same input -- the balanced-parenthesis check
    // is how the two arrive at the same refusal.
    match validate_media_query(media_key) {
      Ok(media_query) => parsed_media_queries.push(media_query),
      Err(_) => {
        // An unparseable query is a hard error, not something to pass through:
        // no later phase rejects it, so returning here emitted the broken query
        // verbatim into the stylesheet. The caller catches this and reports it
        // as invalid media query syntax.
        stylex_panic!("Invalid media query: {}", media_key);
      },
    }
  }

  // For each key, the queries that follow it, in declaration order. Built once
  // from the back so that the list of later queries grows by one per step
  // instead of being re-collected per key.
  let mut accumulated_negations = vec![Vec::new(); media_keys.len()];
  let mut later_queries = Vec::new();
  for i in (0..media_keys.len()).rev() {
    let mut in_order = later_queries.clone();
    in_order.reverse();
    accumulated_negations[i] = in_order;
    later_queries.push(parsed_media_queries[i].clone());
  }

  for (i, media_key) in media_keys.iter().enumerate() {
    let Some(current) = entries.shift_remove(media_key) else {
      continue;
    };

    let combined_query = combine_media_query_with_negations(
      parsed_media_queries[i].clone(),
      accumulated_negations[i].clone(),
    );
    let new_media_key = combined_query.to_string();

    match entries.entry(new_media_key.clone()) {
      // The key is already there: it keeps its position and takes this value,
      // and the declaration that put it there is gone from the output.
      Entry::Occupied(mut occupied) => occupied.get_mut().value = current.value,
      Entry::Vacant(vacant) => {
        vacant.insert(KeyValueProp {
          key: PropName::Str(Str {
            span: DUMMY_SP,
            value: Wtf8Atom::from(new_media_key),
            raw: None,
          }),
          value: current.value,
        });
      },
    }
  }

  entries.into_values().collect()
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
