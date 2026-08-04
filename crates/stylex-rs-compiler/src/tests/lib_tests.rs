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
fn clear_source_contents_handles_a_map_with_no_sources() {
  let mut map = input_map(&[], &[]);

  clear_source_contents(&mut map);

  assert!(contents_of(&map).is_empty());
}

#[test]
fn clear_source_contents_removes_every_populated_entry() {
  let mut map = input_map(
    &["/abs/path/a.tsx", "/abs/path/b.tsx"],
    &[Some("// a\n"), Some("")],
  );

  clear_source_contents(&mut map);

  assert_eq!(contents_of(&map), vec![None, None]);
}

// ── source_names_file ───────────────────────────────────────────────
// Upstream tooling names the same file in several spellings; each of these
// must resolve to the file being compiled, and near-misses must not.

const COMPILED: &str = "/project/app/page.tsx";

#[test]
fn source_names_file_accepts_every_spelling_of_the_same_file() {
  for source in [
    "/project/app/page.tsx",
    "app/page.tsx",
    "./app/page.tsx",
    "page.tsx",
    "./page.tsx",
    "file:///project/app/page.tsx",
  ] {
    assert!(
      source_names_file(source, Path::new(COMPILED)),
      "expected `{}` to name `{}`",
      source,
      COMPILED
    );
  }
}

#[test]
fn source_names_file_rejects_a_different_file() {
  for source in [
    // A different file entirely.
    "/project/app/layout.tsx",
    // Shares a suffix of the *string* but not of the path components — the
    // case a naive `ends_with` on `&str` would wrongly accept.
    "my-page.tsx",
    "/project/other/page.tsx",
    // Longer than the compiled path, so it cannot be a suffix of it.
    "/deeper/project/app/page.tsx",
    // A `webpack://` URL keeps its namespace segment and stays unmatched.
    "webpack://_N_E/./app/page.tsx",
    "",
  ] {
    assert!(
      !source_names_file(source, Path::new(COMPILED)),
      "expected `{}` not to name `{}`",
      source,
      COMPILED
    );
  }
}

// ── backfill_source_contents ────────────────────────────────────────

#[test]
fn backfill_seeds_the_entry_naming_the_compiled_file() {
  let mut map = input_map(&["page.tsx"], &[None]);

  backfill_source_contents(&mut map, Path::new(COMPILED), "// authored\n");

  assert_eq!(contents_of(&map), vec![Some("// authored\n".to_string())]);
}

#[test]
fn backfill_leaves_existing_upstream_text_alone() {
  let mut map = input_map(&["page.tsx"], &[Some("// from an earlier loader\n")]);

  backfill_source_contents(&mut map, Path::new(COMPILED), "// this loader's input\n");

  assert_eq!(
    contents_of(&map),
    vec![Some("// from an earlier loader\n".to_string())]
  );
}

#[test]
fn backfill_skips_sources_that_name_other_files() {
  // The chain resolves back to files this compiler never saw. Their text is
  // not ours to invent.
  let mut map = input_map(
    &["/project/app/other.tsx", "/project/lib/util.ts"],
    &[None, None],
  );

  backfill_source_contents(&mut map, Path::new(COMPILED), "// this loader's input\n");

  assert_eq!(contents_of(&map), vec![None, None]);
}

#[test]
fn backfill_writes_nothing_when_two_entries_claim_the_file() {
  // `page.tsx` and `app/page.tsx` both resolve to the compiled file. Neither
  // can be filled with confidence, so neither is touched.
  let mut map = input_map(&["page.tsx", "app/page.tsx"], &[None, None]);

  backfill_source_contents(&mut map, Path::new(COMPILED), "// this loader's input\n");

  assert_eq!(contents_of(&map), vec![None, None]);
}

#[test]
fn backfill_fills_only_the_matching_entry_of_a_multi_source_map() {
  let mut map = input_map(&["/project/lib/util.ts", "app/page.tsx"], &[None, None]);

  backfill_source_contents(&mut map, Path::new(COMPILED), "// authored\n");

  assert_eq!(
    contents_of(&map),
    vec![None, Some("// authored\n".to_string())]
  );
}

#[test]
fn backfill_handles_a_map_with_no_sources() {
  let mut map = input_map(&[], &[]);

  backfill_source_contents(&mut map, Path::new(COMPILED), "// authored\n");

  assert!(contents_of(&map).is_empty());
}
