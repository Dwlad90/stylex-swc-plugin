use crate::shared::{
  structures::order_pair::OrderPair, utils::css::stylex::split_css_value::split_value,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

#[allow(dead_code)]
impl Shorthands {
  fn border(raw_value: Option<String>) -> Vec<OrderPair> {
    return vec![
      OrderPair("borderTop".to_string(), raw_value.clone()),
      OrderPair("borderEnd".to_string(), raw_value.clone()),
      OrderPair("borderBottom".to_string(), raw_value.clone()),
      OrderPair("borderStart".to_string(), raw_value),
    ];
  }

  fn border_color(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("borderTopColor".to_string(), Some(top)),
      OrderPair("borderEndColor".to_string(), Some(right)),
      OrderPair("borderBottomColor".to_string(), Some(bottom)),
      OrderPair("borderStartColor".to_string(), Some(left)),
    ];
  }
  fn border_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    return vec![
      OrderPair("borderStart".to_string(), raw_value.clone()),
      OrderPair("borderEnd".to_string(), raw_value),
    ];
  }
  fn border_style(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("borderTopStyle".to_string(), Some(top)),
      OrderPair("borderEndStyle".to_string(), Some(right)),
      OrderPair("borderBottomStyle".to_string(), Some(bottom)),
      OrderPair("borderStartStyle".to_string(), Some(left)),
    ];
  }
  fn border_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    return vec![
      OrderPair("borderTop".to_string(), raw_value.clone()),
      OrderPair("borderBottom".to_string(), raw_value),
    ];
  }
  fn border_width(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("borderTopWidth".to_string(), Some(top)),
      OrderPair("borderEndWidth".to_string(), Some(right)),
      OrderPair("borderBottomWidth".to_string(), Some(bottom)),
      OrderPair("borderStartWidth".to_string(), Some(left)),
    ];
  }

  fn border_horizontal_color(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderStartColor".to_string(), raw_value.clone()),
      OrderPair("borderEndColor".to_string(), raw_value),
    ];

    result
  }
  fn border_horizontal_style(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderStartStyle".to_string(), raw_value.clone()),
      OrderPair("borderEndStyle".to_string(), raw_value),
    ];

    result
  }
  fn border_horizontal_width(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderStartWidth".to_string(), raw_value.clone()),
      OrderPair("borderEndWidth".to_string(), raw_value),
    ];

    result
  }
  fn border_vertical_color(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTopColor".to_string(), raw_value.clone()),
      OrderPair("borderBottomColor".to_string(), raw_value),
    ];

    result
  }
  fn border_vertical_style(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTopStyle".to_string(), raw_value.clone()),
      OrderPair("borderBottomStyle".to_string(), raw_value),
    ];

    result
  }
  fn border_vertical_width(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTopWidth".to_string(), raw_value.clone()),
      OrderPair("borderBottomWidth".to_string(), raw_value),
    ];

    result
  }

  fn border_radius(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("borderTopStartRadius".to_string(), Some(top)),
      OrderPair("borderTopEndRadius".to_string(), Some(right)),
      OrderPair("borderBottomEndRadius".to_string(), Some(bottom)),
      OrderPair("borderBottomStartRadius".to_string(), Some(left)),
    ];
  }

  fn inset(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("top".to_string(), raw_value.clone()),
      OrderPair("end".to_string(), raw_value.clone()),
      OrderPair("bottom".to_string(), raw_value.clone()),
      OrderPair("start".to_string(), raw_value),
    ];

    result
  }
  fn inset_inline(raw_value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::start(raw_value.clone()));
    result.extend(Shorthands::end(raw_value));

    result
  }
  fn inset_block(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("top".to_string(), raw_value.clone()),
      OrderPair("bottom".to_string(), raw_value),
    ];

    result
  }
  fn start(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("start".to_string(), raw_value),
      OrderPair("left".to_string(), None),
      OrderPair("right".to_string(), None),
    ];

    result
  }
  fn end(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("end".to_string(), raw_value),
      OrderPair("left".to_string(), None),
      OrderPair("right".to_string(), None),
    ];

    result
  }
  fn left(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("left".to_string(), raw_value),
      OrderPair("start".to_string(), None),
      OrderPair("end".to_string(), None),
    ];

    result
  }
  fn right(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("right".to_string(), raw_value),
      OrderPair("start".to_string(), None),
      OrderPair("end".to_string(), None),
    ];

    result
  }

  fn gap(raw_value: Option<String>) -> Vec<OrderPair> {
    let (row, column, _, _) = split_value(raw_value);

    return vec![
      OrderPair("rowGap".to_string(), Some(row)),
      OrderPair("columnGap".to_string(), Some(column)),
    ];
  }
  fn margin(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("marginTop".to_string(), Some(top)),
      OrderPair("marginEnd".to_string(), Some(right)),
      OrderPair("marginBottom".to_string(), Some(bottom)),
      OrderPair("marginStart".to_string(), Some(left)),
    ];
  }
  fn margin_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::margin_start(raw_value.clone()));
    result.extend(Shorthands::margin_end(raw_value));

    result
  }
  fn margin_start(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginStart".to_string(), raw_value),
      OrderPair("marginLeft".to_string(), None),
      OrderPair("marginRight".to_string(), None),
    ];

    result
  }
  fn margin_end(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginEnd".to_string(), raw_value),
      OrderPair("marginLeft".to_string(), None),
      OrderPair("marginRight".to_string(), None),
    ];

    result
  }
  fn margin_left(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginLeft".to_string(), raw_value),
      OrderPair("marginStart".to_string(), None),
      OrderPair("marginEnd".to_string(), None),
    ];

    result
  }
  fn margin_right(raw_value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginRight".to_string(), raw_value),
      OrderPair("marginStart".to_string(), None),
      OrderPair("marginEnd".to_string(), None),
    ];

    result
  }
  fn margin_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    return vec![
      OrderPair("marginTop".to_string(), raw_value.clone()),
      OrderPair("marginBottom".to_string(), raw_value),
    ];
  }

  fn overflow(raw_value: Option<String>) -> Vec<OrderPair> {
    let (x, y, _, _) = split_value(raw_value);
    return vec![
      OrderPair("overflowX".to_string(), Some(x)),
      OrderPair("overflowY".to_string(), Some(y)),
    ];
  }
  fn padding(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value(raw_value); // Assuming split_value returns a Vec<String>

    return vec![
      OrderPair("paddingTop".to_string(), Some(top)),
      OrderPair("paddingEnd".to_string(), Some(right)),
      OrderPair("paddingBottom".to_string(), Some(bottom)),
      OrderPair("paddingStart".to_string(), Some(left)),
    ];
  }
  fn padding_horizontal(val: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::padding_start(val.clone()));
    result.extend(Shorthands::padding_end(val));

    result
  }
  fn padding_start(val: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingStart".to_string(), val),
      OrderPair("paddingLeft".to_string(), None),
      OrderPair("paddingRight".to_string(), None),
    ];

    result
  }
  fn padding_end(val: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingEnd".to_string(), val),
      OrderPair("paddingLeft".to_string(), None),
      OrderPair("paddingRight".to_string(), None),
    ];

    result
  }
  fn padding_left(val: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingLeft".to_string(), val),
      OrderPair("paddingStart".to_string(), None),
      OrderPair("paddingEnd".to_string(), None),
    ];

    result
  }
  fn padding_right(val: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingRight".to_string(), val),
      OrderPair("paddingStart".to_string(), None),
      OrderPair("paddingEnd".to_string(), None),
    ];

    result
  }
  fn padding_vertical(val: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingTop".to_string(), val.clone()),
      OrderPair("paddingBottom".to_string(), val),
    ];

    result
  }

  pub(crate) fn get(name: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name.as_str() {
      "border" => Some(Shorthands::border),
      "borderColor" => Some(Shorthands::border_color),
      "borderHorizontal" => Some(Shorthands::border_horizontal),
      "borderStyle" => Some(Shorthands::border_style),
      "borderVertical" => Some(Shorthands::border_vertical),
      "borderWidth" => Some(Shorthands::border_width),
      "borderHorizontalColor" => Some(Shorthands::border_horizontal_color),
      "borderHorizontalStyle" => Some(Shorthands::border_horizontal_style),
      "borderHorizontalWidth" => Some(Shorthands::border_horizontal_width),
      "borderVerticalColor" => Some(Shorthands::border_vertical_color),
      "borderVerticalStyle" => Some(Shorthands::border_vertical_style),
      "borderVerticalWidth" => Some(Shorthands::border_vertical_width),
      "borderRadius" => Some(Shorthands::border_radius),
      "inset" => Some(Shorthands::inset),
      "insetInline" => Some(Shorthands::inset_inline),
      "insetBlock" => Some(Shorthands::inset_block),
      "start" => Some(Shorthands::start),
      "end" => Some(Shorthands::end),
      "left" => Some(Shorthands::left),
      "right" => Some(Shorthands::right),
      "gap" => Some(Shorthands::gap),
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

      _ => Option::None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Aliases;

#[allow(dead_code)]
impl Aliases {
  fn inset_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("top".to_string(), val)]
  }
  fn inset_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("bottom".to_string(), val)]
  }

  fn block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("height".to_string(), val)]
  }
  fn inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("width".to_string(), val)]
  }
  fn min_block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("minHeight".to_string(), val)]
  }
  fn min_inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("minWidth".to_string(), val)]
  }
  fn max_block_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("maxHeight".to_string(), val)]
  }
  fn max_inline_size(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("maxWidth".to_string(), val)]
  }

  fn border_block_start_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopWidth".to_string(), val)]
  }
  fn border_block_start_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopStyle".to_string(), val)]
  }
  fn border_block_start_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopColor".to_string(), val)]
  }
  fn border_block_end_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomWidth".to_string(), val)]
  }
  fn border_block_end_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomStyle".to_string(), val)]
  }
  fn border_block_end_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomColor".to_string(), val)]
  }

  fn border_inline_start_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderStartWidth".to_string(), val)]
  }
  fn border_inline_start_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderStartStyle".to_string(), val)]
  }
  fn border_inline_start_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderStartColor".to_string(), val)]
  }
  fn border_inline_end_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndWidth".to_string(), val)]
  }
  fn border_inline_end_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndStyle".to_string(), val)]
  }
  fn border_inline_end_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndColor".to_string(), val)]
  }
  fn border_start_start_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopStartRadius".to_string(), val)]
  }
  fn border_start_end_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderTopEndRadius".to_string(), val)]
  }
  fn border_end_start_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomStartRadius".to_string(), val)]
  }
  fn border_end_end_radius(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderBottomEndRadius".to_string(), val)]
  }

  fn grid_row_gap(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("rowGap".to_string(), value)]
  }
  fn grid_column_gap(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("columnGap".to_string(), value)]
  }

  fn margin_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginTop".to_string(), val)]
  }
  fn margin_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginBottom".to_string(), val)]
  }
  fn margin_inline_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginStart".to_string(), val)]
  }
  fn margin_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginEnd".to_string(), val)]
  }

  fn overflow_block(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("overflowY".to_string(), value)]
  }
  fn overflow_inline(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("overflowX".to_string(), value)]
  }

  fn padding_block_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingTop".to_string(), val)]
  }
  fn padding_block_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingBottom".to_string(), val)]
  }

  fn padding_inline_start(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingStart".to_string(), val)]
  }
  fn padding_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingEnd".to_string(), val)]
  }

  fn scroll_margin_block_start(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginTop".to_string(), value)]
  }
  fn scroll_margin_block_end(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginBottom".to_string(), value)]
  }

  pub(crate) fn get(name: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name.as_str() {
      "insetBlockStart" => Some(Aliases::inset_block_start),
      "insetBlockEnd" => Some(Aliases::inset_block_end),
      "insetInlineStart" => Shorthands::get("start".to_string()),
      "insetInlineEnd" => Shorthands::get("end".to_string()),
      "blockSize" => Some(Aliases::block_size),
      "inlineSize" => Some(Aliases::inline_size),
      "minBlockSize" => Some(Aliases::min_block_size),
      "minInlineSize" => Some(Aliases::min_inline_size),
      "maxBlockSize" => Some(Aliases::max_block_size),
      "maxInlineSize" => Some(Aliases::max_inline_size),
      "borderBlockWidth" => Shorthands::get("borderVerticalWidth".to_string()),
      "borderBlockStyle" => Shorthands::get("borderVerticalStyle".to_string()),
      "borderBlockColor" => Shorthands::get("borderVerticalColor".to_string()),
      "borderBlockStartWidth" => Some(Aliases::border_block_start_width),
      "borderBlockStartStyle" => Some(Aliases::border_block_start_style),
      "borderBlockStartColor" => Some(Aliases::border_block_start_color),
      "borderBlockEndWidth" => Some(Aliases::border_block_end_width),
      "borderBlockEndStyle" => Some(Aliases::border_block_end_style),
      "borderBlockEndColor" => Some(Aliases::border_block_end_color),
      "borderInlineWidth" => Shorthands::get("borderHorizontalWidth".to_string()),
      "borderInlineStyle" => Shorthands::get("borderHorizontalStyle".to_string()),
      "borderInlineColor" => Shorthands::get("borderHorizontalColor".to_string()),
      "borderInlineStartWidth" => Some(Aliases::border_inline_start_width),
      "borderInlineStartStyle" => Some(Aliases::border_inline_start_style),
      "borderInlineStartColor" => Some(Aliases::border_inline_start_color),
      "borderInlineEndWidth" => Some(Aliases::border_inline_end_width),
      "borderInlineEndStyle" => Some(Aliases::border_inline_end_style),
      "borderInlineEndColor" => Some(Aliases::border_inline_end_color),
      "borderStartStartRadius" => Some(Aliases::border_start_start_radius),
      "borderStartEndRadius" => Some(Aliases::border_start_end_radius),
      "borderEndStartRadius" => Some(Aliases::border_end_start_radius),
      "borderEndEndRadius" => Some(Aliases::border_end_end_radius),
      "gridGap" => Shorthands::get("gap".to_string()),
      "gridRowGap" => Some(Aliases::grid_row_gap),
      "gridColumnGap" => Some(Aliases::grid_column_gap),
      "marginBlock" => Shorthands::get("marginVertical".to_string()),
      "marginBlockStart" => Some(Aliases::margin_block_start),
      "marginBlockEnd" => Some(Aliases::margin_block_end),
      "marginInline" => Shorthands::get("marginHorizontal".to_string()),
      "marginInlineStart" => Some(Aliases::margin_inline_start),
      "marginInlineEnd" => Some(Aliases::margin_inline_end),
      "overflowBlock" => Some(Aliases::overflow_block),
      "overflowInline" => Some(Aliases::overflow_inline),
      "paddingBlock" => Shorthands::get("paddingVertical".to_string()),
      "paddingBlockStart" => Some(Aliases::padding_block_start),
      "paddingBlockEnd" => Some(Aliases::padding_block_end),
      "paddingInline" => Shorthands::get("paddingHorizontal".to_string()),
      "paddingInlineStart" => Some(Aliases::padding_inline_start),
      "paddingInlineEnd" => Some(Aliases::padding_inline_end),
      "scrollMarginBlockStart" => Some(Aliases::scroll_margin_block_start),
      "scrollMarginBlockEnd" => Some(Aliases::scroll_margin_block_end),
      _ => Option::None,
    }
  }
}
