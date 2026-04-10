#[cfg(test)]
mod generate_ltr_tests {
  use crate::css::generate_ltr::generate_ltr;
  use stylex_enums::style_resolution::StyleResolution;
  use stylex_structures::{pair::Pair, stylex_state_options::StyleXStateOptions};

  fn default_options() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  fn legacy_options() -> StyleXStateOptions {
    StyleXStateOptions {
      style_resolution: StyleResolution::LegacyExpandShorthands,
      ..Default::default()
    }
  }

  fn legacy_logical_options() -> StyleXStateOptions {
    StyleXStateOptions {
      style_resolution: StyleResolution::LegacyExpandShorthands,
      enable_logical_styles_polyfill: true,
      ..Default::default()
    }
  }

  // ── property_to_ltr path (default non-legacy) ─────────────────

  #[test]
  fn passthrough_regular_property() {
    let pair = Pair::new("color".into(), "red".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "color");
    assert_eq!(result.value, "red");
  }

  #[test]
  fn logical_property_mapped_to_ltr() {
    let pair = Pair::new("margin-start".into(), "10px".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "margin-left");
    assert_eq!(result.value, "10px");
  }

  #[test]
  fn logical_property_margin_end() {
    let pair = Pair::new("margin-end".into(), "20px".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "margin-right");
    assert_eq!(result.value, "20px");
  }

  #[test]
  fn padding_start_mapped_to_ltr() {
    let pair = Pair::new("padding-start".into(), "5px".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "padding-left");
    assert_eq!(result.value, "5px");
  }

  // ── background-position handling ──────────────────────────────

  #[test]
  fn background_position_start_becomes_left() {
    let pair = Pair::new("background-position".into(), "start center".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "background-position");
    assert_eq!(result.value, "left center");
  }

  #[test]
  fn background_position_end_becomes_right() {
    let pair = Pair::new("background-position".into(), "end top".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "background-position");
    assert_eq!(result.value, "right top");
  }

  #[test]
  fn background_position_no_logical_values() {
    let pair = Pair::new("background-position".into(), "center bottom".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "center bottom");
  }

  // ── float and clear ───────────────────────────────────────────

  #[test]
  fn float_start_becomes_left() {
    let pair = Pair::new("float".into(), "start".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "left");
  }

  #[test]
  fn float_end_becomes_right() {
    let pair = Pair::new("float".into(), "end".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "right");
  }

  #[test]
  fn float_left_stays_left() {
    let pair = Pair::new("float".into(), "left".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "left");
  }

  #[test]
  fn clear_start_becomes_left() {
    let pair = Pair::new("clear".into(), "start".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "left");
  }

  #[test]
  fn clear_end_becomes_right() {
    let pair = Pair::new("clear".into(), "end".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "right");
  }

  #[test]
  fn clear_both_unchanged() {
    let pair = Pair::new("clear".into(), "both".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "both");
  }

  // ── LegacyExpandShorthands without polyfill ───────────────────

  #[test]
  fn legacy_no_polyfill_passthrough() {
    let pair = Pair::new("margin-start".into(), "10px".into());
    let result = generate_ltr(&pair, &legacy_options());
    // Without polyfill enabled, returns the pair as-is
    assert_eq!(result.key, "margin-start");
    assert_eq!(result.value, "10px");
  }

  // ── LegacyExpandShorthands with polyfill ──────────────────────

  #[test]
  fn legacy_polyfill_inline_property_mapped() {
    let pair = Pair::new("margin-inline-start".into(), "10px".into());
    let result = generate_ltr(&pair, &legacy_logical_options());
    assert_eq!(result.key, "margin-left");
    assert_eq!(result.value, "10px");
  }

  #[test]
  fn legacy_polyfill_inline_end_mapped() {
    let pair = Pair::new("margin-inline-end".into(), "10px".into());
    let result = generate_ltr(&pair, &legacy_logical_options());
    assert_eq!(result.key, "margin-right");
    assert_eq!(result.value, "10px");
  }

  #[test]
  fn legacy_polyfill_unknown_falls_through_to_property_to_ltr() {
    let pair = Pair::new("color".into(), "red".into());
    let result = generate_ltr(&pair, &legacy_logical_options());
    assert_eq!(result.key, "color");
    assert_eq!(result.value, "red");
  }

  // ── inline-start / inline-end ─────────────────────────────────

  #[test]
  fn float_inline_start_becomes_left() {
    let pair = Pair::new("float".into(), "inline-start".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "left");
  }

  #[test]
  fn float_inline_end_becomes_right() {
    let pair = Pair::new("float".into(), "inline-end".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "right");
  }

  #[test]
  fn background_position_inset_inline_start() {
    let pair = Pair::new(
      "background-position".into(),
      "insetInlineStart center".into(),
    );
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "left center");
  }

  #[test]
  fn background_position_inset_inline_end() {
    let pair = Pair::new(
      "background-position".into(),
      "insetInlineEnd center".into(),
    );
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.value, "right center");
  }

  // ── Unknown property not in PROPERTY_TO_LTR ───────────────────

  #[test]
  fn unknown_property_unchanged() {
    let pair = Pair::new("z-index".into(), "10".into());
    let result = generate_ltr(&pair, &default_options());
    assert_eq!(result.key, "z-index");
    assert_eq!(result.value, "10");
  }
}
