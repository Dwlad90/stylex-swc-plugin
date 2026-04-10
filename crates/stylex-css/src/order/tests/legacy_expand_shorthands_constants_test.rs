use crate::order::constants::legacy_expand_shorthands_order::{Aliases, Shorthands};
use stylex_structures::order_pair::OrderPair;

// ── Shorthands::get ─────────────────────────────────────────────────

#[test]
fn shorthands_get_border_expands() {
  let func = Shorthands::get("border").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0].0, "borderTop");
}

#[test]
fn shorthands_get_border_with_value() {
  let func = Shorthands::get("border").unwrap();
  let result = func(Some("1px solid red".into())).unwrap();
  assert_eq!(result[0].1, Some("1px solid red".to_string()));
}

#[test]
fn shorthands_get_border_color() {
  let func = Shorthands::get("borderColor").unwrap();
  let result = func(Some("red green blue yellow".into())).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0], OrderPair("borderTopColor".into(), Some("red".into())));
}

#[test]
fn shorthands_get_border_style() {
  let func = Shorthands::get("borderStyle").unwrap();
  let result = func(Some("solid".into())).unwrap();
  assert_eq!(result.len(), 4);
}

#[test]
fn shorthands_get_border_width() {
  let func = Shorthands::get("borderWidth").unwrap();
  let result = func(Some("1px 2px".into())).unwrap();
  assert_eq!(result.len(), 4);
}

#[test]
fn shorthands_get_border_horizontal() {
  let func = Shorthands::get("borderHorizontal").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
  assert_eq!(result[0].0, "borderInlineStart");
  assert_eq!(result[1].0, "borderInlineEnd");
}

#[test]
fn shorthands_get_border_vertical() {
  let func = Shorthands::get("borderVertical").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_horizontal_color() {
  let func = Shorthands::get("borderHorizontalColor").unwrap();
  let result = func(Some("red".into())).unwrap();
  assert_eq!(result.len(), 2);
  assert_eq!(result[0].0, "borderInlineStartColor");
}

#[test]
fn shorthands_get_border_inline_width() {
  let func = Shorthands::get("borderInlineWidth").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_vertical_width() {
  let func = Shorthands::get("borderVerticalWidth").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_radius() {
  let func = Shorthands::get("borderRadius").unwrap();
  let result = func(Some("4px".into())).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0].0, "borderStartStartRadius");
}

#[test]
fn shorthands_get_contain_intrinsic_size() {
  let func = Shorthands::get("containIntrinsicSize").unwrap();
  let result = func(Some("300px 200px".into())).unwrap();
  assert_eq!(result.len(), 2);
  assert_eq!(result[0].0, "containIntrinsicWidth");
  assert_eq!(result[1].0, "containIntrinsicHeight");
}

#[test]
fn shorthands_get_contain_intrinsic_size_auto() {
  let func = Shorthands::get("containIntrinsicSize").unwrap();
  let result = func(Some("auto 300px".into())).unwrap();
  assert_eq!(result.len(), 2);
  let width = result[0].1.as_deref().unwrap_or("");
  assert!(width.contains("auto"));
}

#[test]
fn shorthands_get_inset() {
  let func = Shorthands::get("inset").unwrap();
  let result = func(Some("10px 20px 30px 40px".into())).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0], OrderPair("top".into(), Some("10px".into())));
}

#[test]
fn shorthands_get_inset_inline() {
  let func = Shorthands::get("insetInline").unwrap();
  let result = func(Some("10px 20px".into())).unwrap();
  assert!(result.len() >= 4); // start + end each expand to 3
}

#[test]
fn shorthands_get_inset_block() {
  let func = Shorthands::get("insetBlock").unwrap();
  let result = func(Some("10px 20px".into())).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_start() {
  let func = Shorthands::get("start").unwrap();
  let result = func(Some("10px".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(result[0], OrderPair("insetInlineStart".into(), Some("10px".into())));
}

#[test]
fn shorthands_get_end() {
  let func = Shorthands::get("end").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(result[0].0, "insetInlineEnd");
}

#[test]
fn shorthands_get_left() {
  let func = Shorthands::get("left").unwrap();
  let result = func(Some("5px".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(result[0], OrderPair("left".into(), Some("5px".into())));
}

#[test]
fn shorthands_get_right() {
  let func = Shorthands::get("right").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_gap() {
  let func = Shorthands::get("gap").unwrap();
  let result = func(Some("10px 20px".into())).unwrap();
  assert_eq!(result.len(), 2);
  assert_eq!(result[0], OrderPair("rowGap".into(), Some("10px".into())));
  assert_eq!(result[1], OrderPair("columnGap".into(), Some("20px".into())));
}

#[test]
fn shorthands_get_margin() {
  let func = Shorthands::get("margin").unwrap();
  let result = func(Some("10px".into())).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0].0, "marginTop");
}

#[test]
fn shorthands_get_margin_horizontal() {
  let func = Shorthands::get("marginHorizontal").unwrap();
  let result = func(Some("10px 20px".into())).unwrap();
  // margin_start(3) + margin_end(3) = 6
  assert_eq!(result.len(), 6);
}

#[test]
fn shorthands_get_margin_start() {
  let func = Shorthands::get("marginStart").unwrap();
  let result = func(Some("10px".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(
    result[0],
    OrderPair("marginInlineStart".into(), Some("10px".into()))
  );
}

#[test]
fn shorthands_get_margin_end() {
  let func = Shorthands::get("marginEnd").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_margin_left() {
  let func = Shorthands::get("marginLeft").unwrap();
  let result = func(Some("5px".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(result[0], OrderPair("marginLeft".into(), Some("5px".into())));
}

#[test]
fn shorthands_get_margin_right() {
  let func = Shorthands::get("marginRight").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_margin_vertical() {
  let func = Shorthands::get("marginVertical").unwrap();
  let result = func(Some("10px".into())).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_overflow() {
  let func = Shorthands::get("overflow").unwrap();
  let result = func(Some("hidden scroll".into())).unwrap();
  assert_eq!(result.len(), 2);
  assert_eq!(result[0], OrderPair("overflowX".into(), Some("hidden".into())));
}

#[test]
fn shorthands_get_padding() {
  let func = Shorthands::get("padding").unwrap();
  let result = func(Some("10px 20px 30px 40px".into())).unwrap();
  assert_eq!(result.len(), 4);
  assert_eq!(result[0], OrderPair("paddingTop".into(), Some("10px".into())));
}

#[test]
fn shorthands_get_padding_horizontal() {
  let func = Shorthands::get("paddingHorizontal").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 6);
}

#[test]
fn shorthands_get_padding_start() {
  let func = Shorthands::get("paddingStart").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_padding_end() {
  let func = Shorthands::get("paddingEnd").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_padding_left() {
  let func = Shorthands::get("paddingLeft").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_padding_right() {
  let func = Shorthands::get("paddingRight").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn shorthands_get_padding_vertical() {
  let func = Shorthands::get("paddingVertical").unwrap();
  let result = func(Some("8px 12px".into())).unwrap();
  assert_eq!(result.len(), 2);
}

// ── listStyle ───────────────────────────────────────────────────────

#[test]
fn shorthands_get_list_style_none() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
  assert!(result.iter().all(|p| p.1.is_none()));
}

#[test]
fn shorthands_get_list_style_single_type() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("disc".into())).unwrap();
  assert_eq!(result.len(), 3);
  let type_val = &result[0];
  assert_eq!(type_val.0, "listStyleType");
  assert_eq!(type_val.1, Some("disc".into()));
}

#[test]
fn shorthands_get_list_style_position() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("inside".into())).unwrap();
  let pos = &result[1];
  assert_eq!(pos.0, "listStylePosition");
  assert_eq!(pos.1, Some("inside".into()));
}

#[test]
fn shorthands_get_list_style_global_inherit() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("inherit".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert!(result.iter().all(|p| p.1 == Some("inherit".into())));
}

#[test]
fn shorthands_get_list_style_none_value() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("none".into())).unwrap();
  assert_eq!(result.len(), 3);
  assert_eq!(result[0].1, Some("none".into()));
}

#[test]
fn shorthands_get_list_style_quoted_type() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("\"→\"".into())).unwrap();
  // Quoted string => listStyleType
  assert_eq!(result[0].0, "listStyleType");
}

#[test]
fn shorthands_get_list_style_type_and_position() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("disc inside".into())).unwrap();
  assert_eq!(result[0], OrderPair("listStyleType".into(), Some("disc".into())));
  assert_eq!(
    result[1],
    OrderPair("listStylePosition".into(), Some("inside".into()))
  );
}

// ── Border sub-types ────────────────────────────────────────────────

#[test]
fn shorthands_get_border_horizontal_style() {
  let func = Shorthands::get("borderHorizontalStyle").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_horizontal_width() {
  let func = Shorthands::get("borderHorizontalWidth").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_inline_color() {
  let func = Shorthands::get("borderInlineColor").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_inline_style() {
  let func = Shorthands::get("borderInlineStyle").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_vertical_color() {
  let func = Shorthands::get("borderVerticalColor").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn shorthands_get_border_vertical_style() {
  let func = Shorthands::get("borderVerticalStyle").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

// ── Unknown ─────────────────────────────────────────────────────────

#[test]
fn shorthands_get_unknown_returns_none() {
  assert!(Shorthands::get("nonexistent").is_none());
}

#[test]
fn shorthands_get_empty_returns_none() {
  assert!(Shorthands::get("").is_none());
}

#[test]
fn shorthands_get_special_chars_returns_none() {
  assert!(Shorthands::get("$%^").is_none());
}

// ── Aliases::get ────────────────────────────────────────────────────

#[test]
fn aliases_get_inset_block_start() {
  let func = Aliases::get("insetBlockStart").unwrap();
  let result = func(Some("10px".into())).unwrap();
  assert_eq!(result, vec![OrderPair("top".into(), Some("10px".into()))]);
}

#[test]
fn aliases_get_inset_block_end() {
  let func = Aliases::get("insetBlockEnd").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("bottom".into(), None)]);
}

#[test]
fn aliases_get_inset_inline_start_delegates() {
  let func = Aliases::get("insetInlineStart").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn aliases_get_inset_inline_end_delegates() {
  let func = Aliases::get("insetInlineEnd").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn aliases_get_block_size() {
  let func = Aliases::get("blockSize").unwrap();
  let result = func(Some("200px".into())).unwrap();
  assert_eq!(result, vec![OrderPair("height".into(), Some("200px".into()))]);
}

#[test]
fn aliases_get_inline_size() {
  let func = Aliases::get("inlineSize").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("width".into(), None)]);
}

#[test]
fn aliases_get_min_max_block_inline_size() {
  let min_block = Aliases::get("minBlockSize").unwrap()(None).unwrap();
  let min_inline = Aliases::get("minInlineSize").unwrap()(None).unwrap();
  let max_block = Aliases::get("maxBlockSize").unwrap()(None).unwrap();
  let max_inline = Aliases::get("maxInlineSize").unwrap()(None).unwrap();
  assert_eq!(min_block[0].0, "minHeight");
  assert_eq!(min_inline[0].0, "minWidth");
  assert_eq!(max_block[0].0, "maxHeight");
  assert_eq!(max_inline[0].0, "maxWidth");
}

#[test]
fn aliases_get_border_block_delegates() {
  for name in &[
    "borderBlockWidth",
    "borderBlockStyle",
    "borderBlockColor",
  ] {
    let func = Aliases::get(name).unwrap();
    let result = func(None).unwrap();
    assert_eq!(result.len(), 2);
  }
}

#[test]
fn aliases_get_border_inline_delegates() {
  for name in &[
    "borderInlineWidth",
    "borderInlineStyle",
    "borderInlineColor",
  ] {
    let func = Aliases::get(name).unwrap();
    let result = func(None).unwrap();
    assert_eq!(result.len(), 2);
  }
}

#[test]
fn aliases_get_border_start_end() {
  let start = Aliases::get("borderStart").unwrap()(None).unwrap();
  let end = Aliases::get("borderEnd").unwrap()(None).unwrap();
  assert_eq!(start[0].0, "borderInlineStart");
  assert_eq!(end[0].0, "borderInlineEnd");
}

#[test]
fn aliases_get_border_block_start_end_properties() {
  let bsw = Aliases::get("borderBlockStartWidth").unwrap()(None).unwrap();
  let bss = Aliases::get("borderBlockStartStyle").unwrap()(None).unwrap();
  let bsc = Aliases::get("borderBlockStartColor").unwrap()(None).unwrap();
  let bew = Aliases::get("borderBlockEndWidth").unwrap()(None).unwrap();
  let bes = Aliases::get("borderBlockEndStyle").unwrap()(None).unwrap();
  let bec = Aliases::get("borderBlockEndColor").unwrap()(None).unwrap();
  assert_eq!(bsw[0].0, "borderTopWidth");
  assert_eq!(bss[0].0, "borderTopStyle");
  assert_eq!(bsc[0].0, "borderTopColor");
  assert_eq!(bew[0].0, "borderBottomWidth");
  assert_eq!(bes[0].0, "borderBottomStyle");
  assert_eq!(bec[0].0, "borderBottomColor");
}

#[test]
fn aliases_get_border_radius_aliases() {
  let ss = Aliases::get("borderTopStartRadius").unwrap()(None).unwrap();
  let se = Aliases::get("borderTopEndRadius").unwrap()(None).unwrap();
  let es = Aliases::get("borderBottomStartRadius").unwrap()(None).unwrap();
  let ee = Aliases::get("borderBottomEndRadius").unwrap()(None).unwrap();
  assert_eq!(ss[0].0, "borderTopStartRadius");
  assert_eq!(se[0].0, "borderTopEndRadius");
  assert_eq!(es[0].0, "borderBottomStartRadius");
  assert_eq!(ee[0].0, "borderBottomEndRadius");
}

#[test]
fn aliases_get_grid_gap_delegates() {
  let func = Aliases::get("gridGap").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn aliases_get_grid_row_gap() {
  let func = Aliases::get("gridRowGap").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("rowGap".into(), None)]);
}

#[test]
fn aliases_get_grid_column_gap() {
  let func = Aliases::get("gridColumnGap").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("columnGap".into(), None)]);
}

#[test]
fn aliases_get_margin_block_start_end() {
  let start = Aliases::get("marginBlockStart").unwrap()(Some("5px".into())).unwrap();
  let end = Aliases::get("marginBlockEnd").unwrap()(None).unwrap();
  assert_eq!(start[0], OrderPair("marginTop".into(), Some("5px".into())));
  assert_eq!(end[0], OrderPair("marginBottom".into(), None));
}

#[test]
fn aliases_get_margin_inline_start_end() {
  let start = Aliases::get("marginInlineStart").unwrap()(None).unwrap();
  let end = Aliases::get("marginInlineEnd").unwrap()(None).unwrap();
  assert_eq!(start[0].0, "marginInlineStart");
  assert_eq!(end[0].0, "marginInlineEnd");
}

#[test]
fn aliases_get_margin_block_delegates() {
  let func = Aliases::get("marginBlock").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn aliases_get_margin_inline_delegates() {
  let func = Aliases::get("marginInline").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 6);
}

#[test]
fn aliases_get_overflow_block_inline() {
  let block = Aliases::get("overflowBlock").unwrap()(None).unwrap();
  let inline = Aliases::get("overflowInline").unwrap()(None).unwrap();
  assert_eq!(block[0].0, "overflowY");
  assert_eq!(inline[0].0, "overflowX");
}

#[test]
fn aliases_get_padding_block_start_end() {
  let start = Aliases::get("paddingBlockStart").unwrap()(None).unwrap();
  let end = Aliases::get("paddingBlockEnd").unwrap()(None).unwrap();
  assert_eq!(start[0].0, "paddingTop");
  assert_eq!(end[0].0, "paddingBottom");
}

#[test]
fn aliases_get_padding_inline_start_end() {
  let start = Aliases::get("paddingInlineStart").unwrap()(None).unwrap();
  let end = Aliases::get("paddingInlineEnd").unwrap()(None).unwrap();
  assert_eq!(start[0].0, "paddingInlineStart");
  assert_eq!(end[0].0, "paddingInlineEnd");
}

#[test]
fn aliases_get_padding_block_delegates() {
  let func = Aliases::get("paddingBlock").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 2);
}

#[test]
fn aliases_get_padding_inline_delegates() {
  let func = Aliases::get("paddingInline").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result.len(), 6);
}

#[test]
fn aliases_get_scroll_margin_block_start_end() {
  let start = Aliases::get("scrollMarginBlockStart").unwrap()(None).unwrap();
  let end = Aliases::get("scrollMarginBlockEnd").unwrap()(None).unwrap();
  assert_eq!(start[0].0, "scrollMarginTop");
  assert_eq!(end[0].0, "scrollMarginBottom");
}

#[test]
fn aliases_get_float_start() {
  let func = Aliases::get("float").unwrap();
  let result = func(Some("start".into())).unwrap();
  assert_eq!(result[0].0, "float");
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_float_inline_start() {
  let func = Aliases::get("float").unwrap();
  let result = func(Some("inline-start".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_float_end() {
  let func = Aliases::get("float").unwrap();
  let result = func(Some("end".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_float_inline_end() {
  let func = Aliases::get("float").unwrap();
  let result = func(Some("inline-end".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_float_left_passthrough() {
  let func = Aliases::get("float").unwrap();
  let result = func(Some("left".into())).unwrap();
  assert_eq!(result[0], OrderPair("float".into(), Some("left".into())));
}

#[test]
fn aliases_get_float_none() {
  let func = Aliases::get("float").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result[0], OrderPair("float".into(), None));
}

#[test]
fn aliases_get_clear_start() {
  let func = Aliases::get("clear").unwrap();
  let result = func(Some("start".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_clear_end() {
  let func = Aliases::get("clear").unwrap();
  let result = func(Some("end".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_clear_inline_start() {
  let func = Aliases::get("clear").unwrap();
  let result = func(Some("inline-start".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_clear_inline_end() {
  let func = Aliases::get("clear").unwrap();
  let result = func(Some("inline-end".into())).unwrap();
  assert!(result[0].1.as_deref().unwrap().contains("var("));
}

#[test]
fn aliases_get_clear_both_passthrough() {
  let func = Aliases::get("clear").unwrap();
  let result = func(Some("both".into())).unwrap();
  assert_eq!(result[0], OrderPair("clear".into(), Some("both".into())));
}

#[test]
fn aliases_get_clear_none() {
  let func = Aliases::get("clear").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result[0], OrderPair("clear".into(), None));
}

// ── Unknown ─────────────────────────────────────────────────────────

#[test]
fn aliases_get_unknown_returns_none() {
  assert!(Aliases::get("nonexistent").is_none());
}

#[test]
fn aliases_get_empty_returns_none() {
  assert!(Aliases::get("").is_none());
}

// ── Coverage: listStyle error paths ─────────────────────────────────

#[test]
fn shorthands_list_style_var_mixed_with_other() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("disc var(--foo)".into()));
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert!(err.contains("Invalid listStyle"));
}

#[test]
fn shorthands_list_style_duplicate_position() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("inside outside".into()));
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert!(err.contains("Invalid listStyle"));
}

#[test]
fn shorthands_list_style_duplicate_type() {
  let func = Shorthands::get("listStyle").unwrap();
  let result = func(Some("disc square".into()));
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert!(err.contains("Invalid listStyle"));
}

#[test]
fn shorthands_list_style_too_many_nones() {
  let func = Shorthands::get("listStyle").unwrap();
  // "none none none" → first none → type, second none → image, third none → error (duplicate image)
  let result = func(Some("none none none".into()));
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert!(err.contains("Invalid listStyle"));
}

#[test]
fn shorthands_list_style_global_mixed() {
  let func = Shorthands::get("listStyle").unwrap();
  // A global keyword mixed with other values should error
  let result = func(Some("disc inherit".into()));
  assert!(result.is_err());
}
