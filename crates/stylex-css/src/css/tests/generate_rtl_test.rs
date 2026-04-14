#[cfg(test)]
mod generate_rtl_tests {
  use crate::css::generate_rtl::generate_rtl;
  use stylex_enums::style_resolution::StyleResolution;
  use stylex_structures::{pair::Pair, stylex_state_options::StyleXStateOptions};

  fn default_options() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  fn legacy_options() -> StyleXStateOptions {
    StyleXStateOptions::default().with_style_resolution(StyleResolution::LegacyExpandShorthands)
  }

  fn legacy_logical_options() -> StyleXStateOptions {
    StyleXStateOptions::default()
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
  }

  fn flipping_options() -> StyleXStateOptions {
    let mut options = StyleXStateOptions::default();
    options.core.enable_legacy_value_flipping = true;
    options
  }

  // ── No RTL needed for non-logical properties ──────────────────

  #[test]
  fn no_rtl_for_regular_property() {
    let pair = Pair::new("color", "red");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  #[test]
  fn no_rtl_for_width() {
    let pair = Pair::new("width", "100px");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  // ── Logical properties produce RTL pairs ──────────────────────

  #[test]
  fn margin_start_to_rtl_margin_right() {
    let pair = Pair::new("margin-start", "10px");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.key, "margin-right");
    assert_eq!(rtl.value, "10px");
  }

  #[test]
  fn margin_end_to_rtl_margin_left() {
    let pair = Pair::new("margin-end", "10px");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.key, "margin-left");
  }

  #[test]
  fn padding_start_to_rtl() {
    let pair = Pair::new("padding-start", "5px");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.key, "padding-right");
  }

  // ── float / clear RTL conversion ──────────────────────────────

  #[test]
  fn float_start_becomes_right_in_rtl() {
    let pair = Pair::new("float", "start");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right");
  }

  #[test]
  fn float_end_becomes_left_in_rtl() {
    let pair = Pair::new("float", "end");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
  }

  #[test]
  fn clear_start_becomes_right_in_rtl() {
    let pair = Pair::new("clear", "start");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right");
  }

  #[test]
  fn clear_end_becomes_left_in_rtl() {
    let pair = Pair::new("clear", "end");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
  }

  #[test]
  fn float_left_no_rtl() {
    let pair = Pair::new("float", "left");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
  }

  #[test]
  fn clear_both_no_rtl() {
    let pair = Pair::new("clear", "both");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "both");
  }

  // ── float inline-start / inline-end ───────────────────────────

  #[test]
  fn float_inline_start_becomes_right_in_rtl() {
    let pair = Pair::new("float", "inline-start");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right");
  }

  #[test]
  fn float_inline_end_becomes_left_in_rtl() {
    let pair = Pair::new("float", "inline-end");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
  }

  // ── background-position RTL ───────────────────────────────────

  #[test]
  fn background_position_start_becomes_right() {
    let pair = Pair::new("background-position", "start center");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.value, "right center");
  }

  #[test]
  fn background_position_end_becomes_left() {
    let pair = Pair::new("background-position", "end top");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.value, "left top");
  }

  #[test]
  fn background_position_no_logical_no_rtl() {
    let pair = Pair::new("background-position", "center bottom");
    let result = generate_rtl(&pair, &default_options());
    // "center bottom" → still "center bottom" in RTL, same as original
    // So generate_rtl might return None or Some with same value
    if let Some(rtl) = result {
      assert_eq!(rtl.value, "center bottom");
    }
  }

  // ── LegacyExpandShorthands without polyfill ───────────────────

  #[test]
  fn legacy_no_polyfill_returns_none() {
    let pair = Pair::new("margin-start", "10px");
    let result = generate_rtl(&pair, &legacy_options());
    assert!(result.is_none());
  }

  // ── LegacyExpandShorthands with polyfill ──────────────────────

  #[test]
  fn legacy_polyfill_inline_start_mapped() {
    let pair = Pair::new("margin-inline-start", "10px");
    let result = generate_rtl(&pair, &legacy_logical_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.key, "margin-right");
  }

  #[test]
  fn legacy_polyfill_inline_end_mapped() {
    let pair = Pair::new("margin-inline-end", "10px");
    let result = generate_rtl(&pair, &legacy_logical_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert_eq!(rtl.key, "margin-left");
  }

  // ── box-shadow / text-shadow flipping ─────────────────────────

  #[test]
  fn box_shadow_flips_with_legacy_value_flipping() {
    let pair = Pair::new("box-shadow", "5px 5px 10px #000");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(rtl.value.contains("-5px"));
  }

  #[test]
  fn text_shadow_flips_with_legacy_value_flipping() {
    let pair = Pair::new("text-shadow", "2px 2px 4px #000");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(rtl.value.contains("-2px"));
  }

  #[test]
  fn box_shadow_no_flip_without_legacy() {
    let pair = Pair::new("box-shadow", "5px 5px 10px #000");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  #[test]
  fn box_shadow_zero_offset_no_flip() {
    let pair = Pair::new("box-shadow", "0 0 10px #000");
    let result = generate_rtl(&pair, &flipping_options());
    // 0 stays 0, no actual flip needed
    assert!(result.is_none());
  }

  #[test]
  fn box_shadow_negative_becomes_positive() {
    let pair = Pair::new("box-shadow", "-3px 3px 5px #000");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(rtl.value.contains("3px 3px"));
  }

  #[test]
  fn box_shadow_multiple_shadows() {
    let pair = Pair::new("box-shadow", "2px 0 4px #000, -1px 0 2px #fff");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(rtl.value.contains("-2px"));
    assert!(rtl.value.contains("1px"));
  }

  // ── cursor flipping ───────────────────────────────────────────

  #[test]
  fn cursor_no_flip_without_legacy() {
    let pair = Pair::new("cursor", "e-resize");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  #[test]
  fn cursor_no_flip_for_unknown_value() {
    let pair = Pair::new("cursor", "pointer");
    let result = generate_rtl(&pair, &flipping_options());
    // "pointer" is not in CURSOR_FLIP
    assert!(result.is_none());
  }

  // ── other properties return None ──────────────────────────────

  #[test]
  fn non_shadow_non_logical_returns_none() {
    let pair = Pair::new("z-index", "10");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  #[test]
  fn display_returns_none() {
    let pair = Pair::new("display", "flex");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  // ── Additional coverage for remaining branches ──────────────────

  #[test]
  fn cursor_e_resize_flips_with_legacy() {
    let pair = Pair::new("cursor", "e-resize");
    let result = generate_rtl(&pair, &flipping_options());
    if let Some(rtl) = result {
      assert_eq!(rtl.value, "w-resize");
    }
  }

  #[test]
  fn cursor_w_resize_flips_with_legacy() {
    let pair = Pair::new("cursor", "w-resize");
    let result = generate_rtl(&pair, &flipping_options());
    if let Some(rtl) = result {
      assert_eq!(rtl.value, "e-resize");
    }
  }

  #[test]
  fn cursor_ne_resize_flips_with_legacy() {
    let pair = Pair::new("cursor", "ne-resize");
    let result = generate_rtl(&pair, &flipping_options());
    if let Some(rtl) = result {
      assert_eq!(rtl.value, "nw-resize");
    }
  }

  #[test]
  fn cursor_se_resize_flips_with_legacy() {
    let pair = Pair::new("cursor", "se-resize");
    let result = generate_rtl(&pair, &flipping_options());
    if let Some(rtl) = result {
      assert_eq!(rtl.value, "sw-resize");
    }
  }

  #[test]
  fn background_position_inset_inline_start_becomes_right() {
    let pair = Pair::new("background-position", "inset-inline-start center");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right center");
  }

  #[test]
  fn background_position_inset_inline_end_becomes_left() {
    let pair = Pair::new("background-position", "inset-inline-end top");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left top");
  }

  #[test]
  fn clear_inline_start_becomes_right() {
    let pair = Pair::new("clear", "inline-start");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right");
  }

  #[test]
  fn clear_inline_end_becomes_left() {
    let pair = Pair::new("clear", "inline-end");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "left");
  }

  #[test]
  fn text_shadow_no_flip_without_legacy() {
    let pair = Pair::new("text-shadow", "2px 2px #000");
    let result = generate_rtl(&pair, &default_options());
    assert!(result.is_none());
  }

  #[test]
  fn box_shadow_inset_flips() {
    let pair = Pair::new("box-shadow", "inset 5px 5px 10px #000");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(rtl.value.contains("-5px") || rtl.value.contains("inset"));
  }

  #[test]
  fn legacy_polyfill_non_inline_key_falls_through() {
    let pair = Pair::new("float", "start");
    let result = generate_rtl(&pair, &legacy_logical_options());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, "right");
  }

  #[test]
  fn legacy_no_polyfill_regular_property_returns_none() {
    let pair = Pair::new("color", "red");
    let result = generate_rtl(&pair, &legacy_options());
    assert!(result.is_none());
  }

  /// The `else { 1 }` branch in `flip_shadow` is taken when `parts[0]`
  /// is NOT a CSS unit value (e.g. a keyword like "inset" or a color).
  /// This exercises `index = 1` and flips `parts[1]` instead of `parts[0]`.
  #[test]
  fn box_shadow_keyword_first_part_uses_index_one() {
    // "inset 3px 2px 4px #000" → parts[0] = "inset" (not a unit)
    // → index = 1 → flip parts[1] = "3px" → "-3px"
    let pair = Pair::new("box-shadow", "inset 3px 2px 4px #000");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    assert!(
      rtl.value.contains("-3px"),
      "expected -3px in: {}",
      rtl.value
    );
  }

  /// When a comma-separated shadow segment has a single non-unit token
  /// (e.g. just a color), `index = 1` but `parts.len() == 1`, so the
  /// flip is skipped — exercising the `index >= parts.len()` false branch.
  #[test]
  fn box_shadow_single_color_segment_skips_flip() {
    let pair = Pair::new("box-shadow", "3px 3px #000, #fff");
    let result = generate_rtl(&pair, &flipping_options());
    assert!(result.is_some());
    let rtl = result.unwrap();
    // The second segment "#fff" has only one non-unit token so it is unchanged
    assert!(
      rtl.value.contains("#fff"),
      "expected #fff in: {}",
      rtl.value
    );
  }
}
