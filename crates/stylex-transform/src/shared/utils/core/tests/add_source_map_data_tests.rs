use std::sync::Arc;

use swc_core::common::{BytePos, FileName, SourceMap as SwcSourceMap, Span};
use swc_core::ecma::ast::{Expr, IdentName, KeyValueProp, Lit, PropName, Str};
use swc_sourcemap::SourceMapBuilder;

use super::original_position_from_input_source_map;
use crate::shared::structures::state_manager::StateManager;

const INPUT_CODE: &str = "\
const styles = create({
  root: { color: 'red' },
  other: { display: 'flex' },
});
";

const UNICODE_INPUT_CODE: &str = "\
const styles = create({
  emoji: '🚀', root: { color: 'red' },
});
";

fn key_value_prop_at(code_offset: usize, key: &str) -> (KeyValueProp, u32) {
  let lo = code_offset as u32;

  (
    KeyValueProp {
      key: PropName::Ident(IdentName::new(
        key.into(),
        Span::new(BytePos(lo + 1), BytePos(lo + 1 + key.len() as u32)),
      )),
      value: Box::new(Expr::Lit(Lit::Str(Str {
        span: swc_core::common::DUMMY_SP,
        value: "unused".into(),
        raw: None,
      }))),
    },
    lo,
  )
}

fn state_with_input(code: &str) -> StateManager {
  let cm = SwcSourceMap::default();
  let source_file = cm.new_source_file(
    Arc::new(FileName::Custom("input_source_map_fixture.tsx".to_string())),
    code.to_string(),
  );

  let mut state = StateManager::default();
  state.set_input_source_file(source_file);
  state
}

#[test]
fn maps_key_position_through_input_source_map() {
  // `other` sits on line index 2 (0-based), column 2 of the compiler input.
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let mut state = state_with_input(INPUT_CODE);

  // The input map records that input line 2 originates from line index 41 of
  // the original authored file (as if a macro loader shifted the code).
  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  let position = original_position_from_input_source_map(&style_node_path, &state);

  let position = match position {
    Some(position) => position,
    None => panic!("the key position must map back to the original source"),
  };

  assert_eq!(position.filename, "Original.tsx");
  assert_eq!(position.line_number, 42);
}

#[test]
fn maps_key_position_after_non_ascii_text_using_utf16_columns() {
  let key_offset = match UNICODE_INPUT_CODE.find("root") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "root");

  let mut state = state_with_input(UNICODE_INPUT_CODE);

  let line_start = match UNICODE_INPUT_CODE.rfind("  emoji") {
    Some(offset) => offset,
    None => panic!("fixture must contain the line start"),
  };
  let prefix = &UNICODE_INPUT_CODE[line_start..key_offset];
  let utf16_col = prefix.encode_utf16().count() as u32;
  let byte_col = prefix.len() as u32;

  assert_ne!(
    utf16_col, byte_col,
    "fixture must distinguish UTF-16 columns from byte offsets"
  );

  let mut builder = SourceMapBuilder::new(None);
  builder.add(
    1,
    utf16_col,
    10,
    0,
    Some("Original.tsx".into()),
    None,
    false,
  );
  builder.add(1, byte_col, 99, 0, Some("Wrong.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  let position = match original_position_from_input_source_map(&style_node_path, &state) {
    Some(position) => position,
    None => panic!("the key position must map using UTF-16 source-map columns"),
  };

  assert_eq!(position.filename, "Original.tsx");
  assert_eq!(position.line_number, 11);
}

#[test]
fn returns_none_without_input_source_map() {
  let key_offset = match INPUT_CODE.find("other") {
    Some(offset) => offset,
    None => panic!("fixture must contain the key"),
  };
  let (style_node_path, _) = key_value_prop_at(key_offset, "other");

  let state = state_with_input(INPUT_CODE);

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "without an input map the caller must fall back to source-text lookups"
  );
}

#[test]
fn returns_none_for_spans_outside_the_input_file() {
  let (style_node_path, _) = key_value_prop_at(1_000_000, "other");

  let mut state = state_with_input(INPUT_CODE);

  let mut builder = SourceMapBuilder::new(None);
  builder.add(2, 0, 41, 0, Some("Original.tsx".into()), None, false);
  state.set_input_source_map(Arc::new(builder.into_sourcemap()));

  assert_eq!(
    original_position_from_input_source_map(&style_node_path, &state)
      .map(|position| (position.filename, position.line_number)),
    None,
    "foreign spans must not resolve through the input file"
  );
}
