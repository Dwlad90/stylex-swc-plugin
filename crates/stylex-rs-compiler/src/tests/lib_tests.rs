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
