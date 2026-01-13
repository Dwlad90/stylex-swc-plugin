use crate::shared::{
  constants::common::{LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR},
  structures::order_pair::OrderPair,
  utils::css::{common::split_value_required, parser::parse_css},
};

/// Helper function to check if a string is a valid list-style-type value
/// Matches: [a-z-]+ or quoted strings like "..." or '...'
fn is_list_style_type(s: &str) -> bool {
  // Check if it's a quoted string (single or double quotes)
  if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
    return true;
  }
  
  // Check if it matches [a-z-]+ pattern (lowercase letters and hyphens only)
  !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase() || c == '-')
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

impl Shorthands {
  fn border(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTop".into(), raw_value.to_owned()),
      OrderPair("borderInlineEnd".into(), raw_value.to_owned()),
      OrderPair("borderBottom".into(), raw_value.to_owned()),
      OrderPair("borderInlineStart".into(), raw_value),
    ]
  }

  fn border_color(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopColor".into(), Some(top)),
      OrderPair("borderInlineEndColor".into(), Some(right)),
      OrderPair("borderBottomColor".into(), Some(bottom)),
      OrderPair("borderInlineStartColor".into(), Some(left)),
    ]
  }
  fn border_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStart".into(), raw_value.to_owned()),
      OrderPair("borderInlineEnd".into(), raw_value),
    ]
  }
  fn border_style(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopStyle".into(), Some(top)),
      OrderPair("borderInlineEndStyle".into(), Some(right)),
      OrderPair("borderBottomStyle".into(), Some(bottom)),
      OrderPair("borderInlineStartStyle".into(), Some(left)),
    ]
  }
  fn border_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTop".into(), raw_value.to_owned()),
      OrderPair("borderBottom".into(), raw_value),
    ]
  }
  fn border_width(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopWidth".into(), Some(top)),
      OrderPair("borderInlineEndWidth".into(), Some(right)),
      OrderPair("borderBottomWidth".into(), Some(bottom)),
      OrderPair("borderInlineStartWidth".into(), Some(left)),
    ]
  }

  fn border_horizontal_color(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartColor".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndColor".into(), raw_value),
    ]
  }
  fn border_horizontal_style(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartStyle".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndStyle".into(), raw_value),
    ]
  }
  fn border_horizontal_width(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartWidth".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndWidth".into(), raw_value),
    ]
  }
  fn border_inline_color(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartColor".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndColor".into(), raw_value),
    ]
  }
  fn border_inline_style(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartStyle".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndStyle".into(), raw_value),
    ]
  }
  fn border_inline_width(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderInlineStartWidth".into(), raw_value.to_owned()),
      OrderPair("borderInlineEndWidth".into(), raw_value),
    ]
  }
  fn border_vertical_color(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopColor".into(), raw_value.to_owned()),
      OrderPair("borderBottomColor".into(), raw_value),
    ]
  }
  fn border_vertical_style(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopStyle".into(), raw_value.to_owned()),
      OrderPair("borderBottomStyle".into(), raw_value),
    ]
  }
  fn border_vertical_width(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopWidth".into(), raw_value.to_owned()),
      OrderPair("borderBottomWidth".into(), raw_value),
    ]
  }

  fn border_radius(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderStartStartRadius".into(), Some(top)),
      OrderPair("borderStartEndRadius".into(), Some(right)),
      OrderPair("borderEndEndRadius".into(), Some(bottom)),
      OrderPair("borderEndStartRadius".into(), Some(left)),
    ]
  }

  fn contain_intrinsic_size(raw_value: Option<String>) -> Vec<OrderPair> {
    let parts = split_value_required(raw_value.as_deref());
    let parts = [parts.0, parts.1, parts.2, parts.3];

    let mut coll: Vec<String> = Vec::with_capacity(parts.len());

    for part in parts {
      if let Some(last_element) = coll.last()
        && last_element == "auto"
        && !part.is_empty()
      {
        coll.pop();
        coll.push(format!("auto {}", part));
        continue;
      }
      coll.push(part);
    }

    let width = coll.first().cloned().unwrap_or_default();
    let height = coll.get(1).cloned().unwrap_or_else(|| width.clone());

    vec![
      OrderPair("containIntrinsicWidth".into(), Some(width)),
      OrderPair("containIntrinsicHeight".into(), Some(height)),
    ]
  }

  fn inset(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("top".into(), Some(top)),
      OrderPair("insetInlineEnd".into(), Some(right)),
      OrderPair("bottom".into(), Some(bottom)),
      OrderPair("insetInlineStart".into(), Some(left)),
    ]
  }
  fn inset_inline(raw_value: Option<String>) -> Vec<OrderPair> {
    let (start, end, _, _) = split_value_required(raw_value.as_deref());

    let mut result = vec![];

    result.extend(Shorthands::start(Some(start)));
    result.extend(Shorthands::end(Some(end)));

    result
  }
  fn inset_block(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, bottom, _, _) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("top".into(), Some(top)),
      OrderPair("bottom".into(), Some(bottom)),
    ]
  }
  fn start(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("insetInlineStart".into(), raw_value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ]
  }
  fn end(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("insetInlineEnd".into(), raw_value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ]
  }
  fn left(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("left".into(), raw_value),
      OrderPair("insetInlineStart".into(), None),
      OrderPair("insetInlineEnd".into(), None),
    ]
  }
  fn right(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("right".into(), raw_value),
      OrderPair("insetInlineStart".into(), None),
      OrderPair("insetInlineEnd".into(), None),
    ]
  }

  fn gap(raw_value: Option<String>) -> Vec<OrderPair> {
    let (row, column, _, _) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("rowGap".into(), Some(row)),
      OrderPair("columnGap".into(), Some(column)),
    ]
  }
  fn margin(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("marginTop".into(), Some(top)),
      OrderPair("marginInlineEnd".into(), Some(right)),
      OrderPair("marginBottom".into(), Some(bottom)),
      OrderPair("marginInlineStart".into(), Some(left)),
    ]
  }
  fn margin_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    let (start, end, _, _) = split_value_required(raw_value.as_deref());

    let mut result = vec![];

    result.extend(Shorthands::margin_start(Some(start)));
    result.extend(Shorthands::margin_end(Some(end)));

    result
  }
  fn margin_start(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginInlineStart".into(), raw_value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ]
  }
  fn margin_end(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginInlineEnd".into(), raw_value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ]
  }
  fn margin_left(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginLeft".into(), raw_value),
      OrderPair("marginInlineStart".into(), None),
      OrderPair("marginInlineEnd".into(), None),
    ]
  }
  fn margin_right(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginRight".into(), raw_value),
      OrderPair("marginInlineStart".into(), None),
      OrderPair("marginInlineEnd".into(), None),
    ]
  }
  fn margin_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, bottom, _, _) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("marginTop".into(), Some(top)),
      OrderPair("marginBottom".into(), Some(bottom)),
    ]
  }

  fn list_style(raw_value: Option<String>) -> Vec<OrderPair> {
    // Handle None case
    if raw_value.is_none() {
      return vec![
        OrderPair("listStyleType".into(), None),
        OrderPair("listStylePosition".into(), None),
        OrderPair("listStyleImage".into(), None),
      ];
    }

    let raw_value_str = raw_value.as_ref().unwrap();
    let parts: Vec<String> = parse_css(raw_value_str);

    // Global values that must be the only value
    let list_style_global_values = ["inherit", "initial", "revert", "unset"];
    
    // Position values (unambiguous)
    let list_style_position_values = ["inside", "outside"];

    // Handle global keywords - must be the only value
    if parts.len() == 1 && list_style_global_values.contains(&parts[0].as_str()) {
      let global_value = Some(parts[0].clone());
      return vec![
        OrderPair("listStyleType".into(), global_value.clone()),
        OrderPair("listStylePosition".into(), global_value.clone()),
        OrderPair("listStyleImage".into(), global_value),
      ];
    }

    let mut image: Option<String> = None;
    let mut position: Option<String> = None;
    let mut list_type: Option<String> = None;
    let mut remaining_parts: Vec<String> = Vec::new();

    // First pass: assign values that can only belong to one property
    for part in &parts {
      // Check for global keywords mixed with other values (invalid)
      // and use of `var()` which can't be disambiguated
      if list_style_global_values.contains(&part.as_str()) || part.contains("var(--") {
        panic!("Invalid listStyle value: '{}'", raw_value_str);
      }
      // Check if it's a position value (unambiguous)
      else if list_style_position_values.contains(&part.as_str()) {
        if position.is_some() {
          panic!("Invalid listStyle value: '{}'", raw_value_str);
        }
        position = Some(part.clone());
      }
      // Check if it's a type value that's not 'none' (unambiguous)
      // Type values are: keywords (letters and hyphens) or quoted strings
      else if part != "none" && is_list_style_type(part) {
        if list_type.is_some() {
          panic!("Invalid listStyle value: '{}'", raw_value_str);
        }
        list_type = Some(part.clone());
      }
      // Keep ambiguous values for second pass
      else {
        remaining_parts.push(part.clone());
      }
    }

    // Second pass: handle remaining parts (including 'none' and image values)
    for part in remaining_parts {
      // If 'none' and type is not yet assigned, assign to type
      if part == "none" && list_type.is_none() {
        list_type = Some(part);
      }
      // Otherwise assign to image
      else {
        if image.is_some() {
          panic!("Invalid listStyle value: '{}'", raw_value_str);
        }
        image = Some(part);
      }
    }

    vec![
      OrderPair("listStyleType".into(), list_type),
      OrderPair("listStylePosition".into(), position),
      OrderPair("listStyleImage".into(), image),
    ]
  }

  fn overflow(raw_value: Option<String>) -> Vec<OrderPair> {
    let (x, y, _, _) = split_value_required(raw_value.as_deref());
    vec![
      OrderPair("overflowX".into(), Some(x)),
      OrderPair("overflowY".into(), Some(y)),
    ]
  }
  fn padding(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("paddingTop".into(), Some(top)),
      OrderPair("paddingInlineEnd".into(), Some(right)),
      OrderPair("paddingBottom".into(), Some(bottom)),
      OrderPair("paddingInlineStart".into(), Some(left)),
    ]
  }
  fn padding_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    let (start, end, _, _) = split_value_required(raw_value.as_deref());

    let mut result = vec![];

    result.extend(Shorthands::padding_start(Some(start)));
    result.extend(Shorthands::padding_end(Some(end)));

    result
  }
  fn padding_start(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingInlineStart".into(), val),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ]
  }
  fn padding_end(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingInlineEnd".into(), val),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ]
  }
  fn padding_left(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingLeft".into(), val),
      OrderPair("paddingInlineStart".into(), None),
      OrderPair("paddingInlineEnd".into(), None),
    ]
  }
  fn padding_right(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingRight".into(), val),
      OrderPair("paddingInlineStart".into(), None),
      OrderPair("paddingInlineEnd".into(), None),
    ]
  }
  fn padding_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, bottom, _, _) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("paddingTop".into(), Some(top)),
      OrderPair("paddingBottom".into(), Some(bottom)),
    ]
  }

  pub(crate) fn get(name: &str) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name {
      "border" => Some(Shorthands::border),
      "borderColor" => Some(Shorthands::border_color),
      "borderHorizontal" => Some(Shorthands::border_horizontal),
      "borderStyle" => Some(Shorthands::border_style),
      "borderVertical" => Some(Shorthands::border_vertical),
      "borderWidth" => Some(Shorthands::border_width),
      "borderHorizontalColor" => Some(Shorthands::border_horizontal_color),
      "borderHorizontalStyle" => Some(Shorthands::border_horizontal_style),
      "borderHorizontalWidth" => Some(Shorthands::border_horizontal_width),
      "borderInlineColor" => Some(Shorthands::border_inline_color),
      "borderInlineStyle" => Some(Shorthands::border_inline_style),
      "borderInlineWidth" => Some(Shorthands::border_inline_width),
      "borderVerticalColor" => Some(Shorthands::border_vertical_color),
      "borderVerticalStyle" => Some(Shorthands::border_vertical_style),
      "borderVerticalWidth" => Some(Shorthands::border_vertical_width),
      "borderRadius" => Some(Shorthands::border_radius),
      "containIntrinsicSize" => Some(Shorthands::contain_intrinsic_size),
      "inset" => Some(Shorthands::inset),
      "insetInline" => Some(Shorthands::inset_inline),
      "insetBlock" => Some(Shorthands::inset_block),
      "start" => Some(Shorthands::start),
      "end" => Some(Shorthands::end),
      "left" => Some(Shorthands::left),
      "right" => Some(Shorthands::right),
      "gap" => Some(Shorthands::gap),
      "listStyle" => Some(Shorthands::list_style),
      "margin" => Some(Shorthands::margin),
      "marginHorizontal" => Some(Shorthands::margin_horizontal),
      "marginStart" => Some(Shorthands::margin_start),
      "marginEnd" => Some(Shorthands::margin_end),
      "marginLeft" => Some(Shorthands::margin_left),
      "marginRight" => Some(Shorthands::margin_right),
      "marginVertical" => Some(Shorthands::margin_vertical),
      "overflow" => Some(Shorthands::overflow),
      "padding" => Some(Shorthands::padding),
      "paddingHorizontal" => Some(Shorthands::padding_horizontal),
      "paddingStart" => Some(Shorthands::padding_start),
      "paddingEnd" => Some(Shorthands::padding_end),
      "paddingLeft" => Some(Shorthands::padding_left),
      "paddingRight" => Some(Shorthands::padding_right),
      "paddingVertical" => Some(Shorthands::padding_vertical),

      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Aliases;

#[allow(dead_code)]
impl Aliases {
  fn inset_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("top".into(), val)]
  }
  fn inset_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("bottom".into(), val)]
  }

  fn block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("height".into(), val)]
  }
  fn inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("width".into(), val)]
  }
  fn min_block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("minHeight".into(), val)]
  }
  fn min_inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("minWidth".into(), val)]
  }
  fn max_block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("maxHeight".into(), val)]
  }
  fn max_inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("maxWidth".into(), val)]
  }
  fn border_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineStart".into(), val)]
  }
  fn border_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineEnd".into(), val)]
  }
  fn border_block_start_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopWidth".into(), val)]
  }
  fn border_block_start_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopStyle".into(), val)]
  }
  fn border_block_start_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopColor".into(), val)]
  }
  fn border_block_end_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomWidth".into(), val)]
  }
  fn border_block_end_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomStyle".into(), val)]
  }
  fn border_block_end_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomColor".into(), val)]
  }

  fn border_inline_start_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineStartWidth".into(), val)]
  }
  fn border_inline_start_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineStartStyle".into(), val)]
  }
  fn border_inline_start_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineStartColor".into(), val)]
  }
  fn border_inline_end_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineEndWidth".into(), val)]
  }
  fn border_inline_end_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineEndStyle".into(), val)]
  }
  fn border_inline_end_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderInlineEndColor".into(), val)]
  }
  fn border_start_start_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopStartRadius".into(), val)]
  }
  fn border_start_end_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopEndRadius".into(), val)]
  }
  fn border_end_start_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomStartRadius".into(), val)]
  }
  fn border_end_end_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomEndRadius".into(), val)]
  }

  fn grid_row_gap(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("rowGap".into(), value)]
  }
  fn grid_column_gap(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("columnGap".into(), value)]
  }

  fn margin_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginTop".into(), val)]
  }
  fn margin_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginBottom".into(), val)]
  }
  fn margin_inline_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginInlineStart".into(), val)]
  }
  fn margin_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginInlineEnd".into(), val)]
  }

  fn overflow_block(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("overflowY".into(), value)]
  }
  fn overflow_inline(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("overflowX".into(), value)]
  }

  fn padding_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingTop".into(), val)]
  }
  fn padding_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingBottom".into(), val)]
  }

  fn padding_inline_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingInlineStart".into(), val)]
  }
  fn padding_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingInlineEnd".into(), val)]
  }

  fn scroll_margin_block_start(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginTop".into(), value)]
  }
  fn scroll_margin_block_end(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginBottom".into(), value)]
  }

  fn float(value: Option<String>) -> Vec<OrderPair> {
    if let Some(ref val) = value {
      if val == "inline-start" || val == "start" {
        return vec![OrderPair(
          "float".into(),
          Some(format!("var({})", LOGICAL_FLOAT_START_VAR)),
        )];
      } else if val == "inline-end" || val == "end" {
        return vec![OrderPair(
          "float".into(),
          Some(format!("var({})", LOGICAL_FLOAT_END_VAR)),
        )];
      }
    }
    vec![OrderPair("float".into(), value)]
  }

  fn clear(value: Option<String>) -> Vec<OrderPair> {
    if let Some(ref val) = value {
      if val == "inline-start" || val == "start" {
        return vec![OrderPair(
          "clear".into(),
          Some(format!("var({})", LOGICAL_FLOAT_START_VAR)),
        )];
      } else if val == "inline-end" || val == "end" {
        return vec![OrderPair(
          "clear".into(),
          Some(format!("var({})", LOGICAL_FLOAT_END_VAR)),
        )];
      }
    }
    vec![OrderPair("clear".into(), value)]
  }

  pub(crate) fn get(name: &str) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name {
      "insetBlockStart" => Some(Aliases::inset_block_start),
      "insetBlockEnd" => Some(Aliases::inset_block_end),
      "insetInlineStart" => Shorthands::get("start"),
      "insetInlineEnd" => Shorthands::get("end"),
      "blockSize" => Some(Aliases::block_size),
      "inlineSize" => Some(Aliases::inline_size),
      "minBlockSize" => Some(Aliases::min_block_size),
      "minInlineSize" => Some(Aliases::min_inline_size),
      "maxBlockSize" => Some(Aliases::max_block_size),
      "maxInlineSize" => Some(Aliases::max_inline_size),
      "borderBlockWidth" => Shorthands::get("borderVerticalWidth"),
      "borderBlockStyle" => Shorthands::get("borderVerticalStyle"),
      "borderBlockColor" => Shorthands::get("borderVerticalColor"),
      "borderStart" => Some(Aliases::border_start),
      "borderEnd" => Some(Aliases::border_end),
      "borderBlockStartWidth" => Some(Aliases::border_block_start_width),
      "borderBlockStartStyle" => Some(Aliases::border_block_start_style),
      "borderBlockStartColor" => Some(Aliases::border_block_start_color),
      "borderBlockEndWidth" => Some(Aliases::border_block_end_width),
      "borderBlockEndStyle" => Some(Aliases::border_block_end_style),
      "borderBlockEndColor" => Some(Aliases::border_block_end_color),
      "borderInlineWidth" => Shorthands::get("borderInlineWidth"),
      "borderInlineStyle" => Shorthands::get("borderInlineStyle"),
      "borderInlineColor" => Shorthands::get("borderInlineColor"),
      "borderTopStartRadius" => Some(Aliases::border_start_start_radius),
      "borderTopEndRadius" => Some(Aliases::border_start_end_radius),
      "borderBottomStartRadius" => Some(Aliases::border_end_start_radius),
      "borderBottomEndRadius" => Some(Aliases::border_end_end_radius),
      "gridGap" => Shorthands::get("gap"),
      "gridRowGap" => Some(Aliases::grid_row_gap),
      "gridColumnGap" => Some(Aliases::grid_column_gap),
      "marginBlock" => Shorthands::get("marginVertical"),
      "marginBlockStart" => Some(Aliases::margin_block_start),
      "marginBlockEnd" => Some(Aliases::margin_block_end),
      "marginInline" => Shorthands::get("marginHorizontal"),
      "marginInlineStart" => Some(Aliases::margin_inline_start),
      "marginInlineEnd" => Some(Aliases::margin_inline_end),
      "overflowBlock" => Some(Aliases::overflow_block),
      "overflowInline" => Some(Aliases::overflow_inline),
      "paddingBlock" => Shorthands::get("paddingVertical"),
      "paddingBlockStart" => Some(Aliases::padding_block_start),
      "paddingBlockEnd" => Some(Aliases::padding_block_end),
      "paddingInline" => Shorthands::get("paddingHorizontal"),
      "paddingInlineStart" => Some(Aliases::padding_inline_start),
      "paddingInlineEnd" => Some(Aliases::padding_inline_end),
      "scrollMarginBlockStart" => Some(Aliases::scroll_margin_block_start),
      "scrollMarginBlockEnd" => Some(Aliases::scroll_margin_block_end),
      "float" => Some(Aliases::float),
      "clear" => Some(Aliases::clear),
      _ => None,
    }
  }
}
