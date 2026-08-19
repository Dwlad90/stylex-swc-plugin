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

// ── Every rejecting shorthand is reachable by its authored name ─────

#[test]
fn every_rejecting_shorthand_is_reachable_by_its_authored_name() {
  // `border` agreed with the table by accident — it is spelled the same in
  // both cases. The rest were unreachable, so a `borderTop: "none"` reached
  // the stylesheet as `border-top:none` and defeated the specificity model
  // this table exists to enforce.
  let names = [
    "all",
    "animation",
    "background",
    "border",
    "borderInline",
    "borderBlock",
    "borderTop",
    "borderInlineEnd",
    "borderRight",
    "borderBottom",
    "borderInlineStart",
    "borderLeft",
  ];

  for name in names {
    let func = PropertySpecificityOrder::get_expansion_fn(name)
      .unwrap_or_else(|| panic!("'{name}' should have an expansion fn"));

    match func(Some("1px solid red".into())) {
      Ok(pairs) => panic!("'{name}' should reject, expanded to {pairs:?}"),
      Err(error) => assert!(
        error.contains("is not supported"),
        "'{name}' reported {error:?}"
      ),
    }
  }
}

#[test]
fn a_snake_case_spelling_is_not_a_property_name() {
  // The keys are authored property names, never Rust identifiers. Pinning
  // this stops the two from being conflated again.
  for name in [
    "border_top",
    "border_inline",
    "border_block",
    "border_left",
    "border_right",
    "border_bottom",
    "border_inline_start",
    "border_inline_end",
  ] {
    assert!(
      PropertySpecificityOrder::get_expansion_fn(name).is_none(),
      "'{name}' is not a property an author can write"
    );
  }
}
