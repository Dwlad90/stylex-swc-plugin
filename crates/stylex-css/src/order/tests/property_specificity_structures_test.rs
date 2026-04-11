use stylex_structures::{order::Order, order_pair::OrderPair};

use crate::order::structures::property_specificity_order::PropertySpecificityOrder;

// ── Aliases found via get_expansion_fn ──────────────────────────────

#[test]
fn get_expansion_fn_for_alias_block_size() {
  let func = PropertySpecificityOrder::get_expansion_fn("blockSize").unwrap();
  let result = func(Some("100px".into())).unwrap();
  assert_eq!(
    result,
    vec![OrderPair("height".into(), Some("100px".into()))]
  );
}

#[test]
fn get_expansion_fn_for_alias_inline_size() {
  let func = PropertySpecificityOrder::get_expansion_fn("inlineSize").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("width".into(), None)]);
}

#[test]
fn get_expansion_fn_for_alias_margin_block_start() {
  let func = PropertySpecificityOrder::get_expansion_fn("marginBlockStart").unwrap();
  let result = func(Some("5px".into())).unwrap();
  assert_eq!(
    result,
    vec![OrderPair("marginTop".into(), Some("5px".into()))]
  );
}

#[test]
fn get_expansion_fn_for_alias_overflow_block() {
  let func = PropertySpecificityOrder::get_expansion_fn("overflowBlock").unwrap();
  let result = func(None).unwrap();
  assert_eq!(result, vec![OrderPair("overflowY".into(), None)]);
}

// ── Shorthands found via get_expansion_fn ───────────────────────────

#[test]
fn get_expansion_fn_for_shorthand_animation() {
  let func = PropertySpecificityOrder::get_expansion_fn("animation").unwrap();
  let result = func(None);
  assert!(result.is_err());
}

#[test]
fn get_expansion_fn_for_shorthand_border() {
  let func = PropertySpecificityOrder::get_expansion_fn("border").unwrap();
  let result = func(None);
  assert!(result.is_err());
}

#[test]
fn get_expansion_fn_for_shorthand_background() {
  let func = PropertySpecificityOrder::get_expansion_fn("background").unwrap();
  let result = func(None);
  assert!(result.is_err());
}

// ── Unknown returns None ────────────────────────────────────────────

#[test]
fn get_expansion_fn_unknown_returns_none() {
  assert!(PropertySpecificityOrder::get_expansion_fn("color").is_none());
}

#[test]
fn get_expansion_fn_empty_returns_none() {
  assert!(PropertySpecificityOrder::get_expansion_fn("").is_none());
}

#[test]
fn get_expansion_fn_special_chars_returns_none() {
  assert!(PropertySpecificityOrder::get_expansion_fn("@#$").is_none());
}

// ── Priority: alias found before shorthand ──────────────────────────

#[test]
fn alias_has_priority_for_inset_block_start() {
  let func = PropertySpecificityOrder::get_expansion_fn("insetBlockStart").unwrap();
  let result = func(Some("20px".into())).unwrap();
  assert_eq!(result, vec![OrderPair("top".into(), Some("20px".into()))]);
}
