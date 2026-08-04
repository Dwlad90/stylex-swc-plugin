use super::*;

#[test]
fn source_maps_config_defaults_to_true() {
  assert!(matches!(
    source_maps_config(None),
    SourceMapsConfig::Bool(true)
  ));
}

#[test]
fn source_maps_config_maps_explicit_values() {
  assert!(matches!(
    source_maps_config(Some(&SourceMaps::True)),
    SourceMapsConfig::Bool(true)
  ));
  assert!(matches!(
    source_maps_config(Some(&SourceMaps::False)),
    SourceMapsConfig::Bool(false)
  ));

  match source_maps_config(Some(&SourceMaps::Inline)) {
    SourceMapsConfig::Str(value) => assert_eq!(value, "inline"),
    _ => panic!("expected inline source map string"),
  }
}

#[test]
fn source_maps_config_true_vs_none_both_enable() {
  // Both None and Some(True) should produce Bool(true)
  let from_none = source_maps_config(None);
  let from_true = source_maps_config(Some(&SourceMaps::True));
  assert!(matches!(from_none, SourceMapsConfig::Bool(true)));
  assert!(matches!(from_true, SourceMapsConfig::Bool(true)));
}

#[test]
fn source_maps_config_false_disables() {
  let config = source_maps_config(Some(&SourceMaps::False));
  assert!(matches!(config, SourceMapsConfig::Bool(false)));
}

#[test]
fn source_maps_config_inline_is_string() {
  let config = source_maps_config(Some(&SourceMaps::Inline));
  match config {
    SourceMapsConfig::Str(s) => assert_eq!(s, "inline"),
    other => panic!("expected Str(\"inline\"), got {:?}", other),
  }
}

#[test]
fn inline_sources_content_defaults_to_true() {
  assert!(resolve_inline_sources_content(None));
  assert!(resolve_inline_sources_content(Some(true)));
  assert!(!resolve_inline_sources_content(Some(false)));
}

#[test]
fn emit_source_map_columns_defaults_to_true() {
  assert!(resolve_emit_source_map_columns(None));
  assert!(resolve_emit_source_map_columns(Some(true)));
  assert!(!resolve_emit_source_map_columns(Some(false)));
}

const OURS: &str = "/abs/path/Button.tsx";
const AUTHORED: &str = "export const a = 1;\n";

/// Build a map whose `sources`/`sourcesContent` are exactly as given. A `None`
/// content is serialized as JSON `null`, the shape earlier tooling emits when
/// it knows a source but not its text.
fn input_map(sources: &[&str], sources_content: &[Option<&str>]) -> swc_sourcemap::SourceMap {
  fn json_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('\n', "\\n"))
  }

  fn json_array(items: impl Iterator<Item = String>) -> String {
    format!("[{}]", items.collect::<Vec<_>>().join(","))
  }

  let sources = json_array(sources.iter().map(|source| json_string(source)));
  let sources_content = json_array(
    sources_content
      .iter()
      .map(|content| content.map_or_else(|| "null".to_string(), json_string)),
  );

  let json = format!(
    r#"{{"version":3,"sources":{},"sourcesContent":{},"names":[],"mappings":""}}"#,
    sources, sources_content
  );

  match swc_sourcemap::SourceMap::from_slice(json.as_bytes()) {
    Ok(map) => map,
    Err(err) => panic!("failed to build test source map: {}", err),
  }
}

fn contents_of(map: &swc_sourcemap::SourceMap) -> Vec<Option<String>> {
  (0..map.get_source_count())
    .map(|idx| map.get_source_contents(idx).map(|src| src.to_string()))
    .collect()
}

#[test]
fn backfill_fills_a_lone_source_whatever_its_path_spelling() {
  // A single-source map is unambiguously ours even when the previous tool
  // spelled the path relatively or behind a `webpack://` scheme.
  for spelling in ["Button.tsx", "./Button.tsx", "webpack://app/./Button.tsx"] {
    let mut map = input_map(&[spelling], &[None]);

    backfill_source_contents(&mut map, OURS, AUTHORED);

    assert_eq!(contents_of(&map), vec![Some(AUTHORED.to_string())]);
  }
}

#[test]
fn backfill_keeps_existing_content() {
  // The incoming map already resolves back to an earlier authored file; its
  // text wins over ours.
  let mut map = input_map(&[OURS], &[Some("// original\n")]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert_eq!(contents_of(&map), vec![Some("// original\n".to_string())]);
}

#[test]
fn backfill_only_touches_the_matching_source_in_a_multi_source_map() {
  let mut map = input_map(&["/abs/path/other.tsx", OURS], &[None, None]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert_eq!(contents_of(&map), vec![None, Some(AUTHORED.to_string())]);
}

#[test]
fn backfill_is_a_noop_when_no_source_matches() {
  // With several sources and no path match there is no way to tell which entry
  // the authored text belongs to — guessing would inline it under a foreign
  // file name.
  let mut map = input_map(&["/abs/path/a.tsx", "/abs/path/b.tsx"], &[None, None]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert_eq!(contents_of(&map), vec![None, None]);
}

#[test]
fn backfill_leaves_populated_neighbours_alone() {
  let mut map = input_map(&["/abs/path/other.tsx", OURS], &[Some("// other\n"), None]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert_eq!(
    contents_of(&map),
    vec![Some("// other\n".to_string()), Some(AUTHORED.to_string())]
  );
}

#[test]
fn backfill_handles_a_map_with_no_sources() {
  let mut map = input_map(&[], &[]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert!(contents_of(&map).is_empty());
}

#[test]
fn backfill_preserves_empty_string_content() {
  // An empty source file is legitimately empty content, not a missing entry.
  let mut map = input_map(&[OURS], &[Some("")]);

  backfill_source_contents(&mut map, OURS, AUTHORED);

  assert_eq!(contents_of(&map), vec![Some(String::new())]);
}

#[test]
fn backfill_is_idempotent() {
  let mut map = input_map(&[OURS], &[None]);

  backfill_source_contents(&mut map, OURS, AUTHORED);
  backfill_source_contents(&mut map, OURS, "// different\n");

  assert_eq!(contents_of(&map), vec![Some(AUTHORED.to_string())]);
}
