use crate::shared::{structures::order_pair::OrderPair, utils::css::common::split_value_required};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

#[allow(dead_code)]
impl Shorthands {
  fn border(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTop".into(), raw_value.clone()),
      OrderPair("borderEnd".into(), raw_value.clone()),
      OrderPair("borderBottom".into(), raw_value.clone()),
      OrderPair("borderStart".into(), raw_value),
    ]
  }

  fn border_color(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopColor".into(), Some(top)),
      OrderPair("borderEndColor".into(), Some(right)),
      OrderPair("borderBottomColor".into(), Some(bottom)),
      OrderPair("borderStartColor".into(), Some(left)),
    ]
  }
  fn border_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderStart".into(), raw_value.clone()),
      OrderPair("borderEnd".into(), raw_value),
    ]
  }
  fn border_style(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopStyle".into(), Some(top)),
      OrderPair("borderEndStyle".into(), Some(right)),
      OrderPair("borderBottomStyle".into(), Some(bottom)),
      OrderPair("borderStartStyle".into(), Some(left)),
    ]
  }
  fn border_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTop".into(), raw_value.clone()),
      OrderPair("borderBottom".into(), raw_value),
    ]
  }
  fn border_width(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopWidth".into(), Some(top)),
      OrderPair("borderEndWidth".into(), Some(right)),
      OrderPair("borderBottomWidth".into(), Some(bottom)),
      OrderPair("borderStartWidth".into(), Some(left)),
    ]
  }

  fn border_horizontal_color(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderStartColor".into(), raw_value.clone()),
      OrderPair("borderEndColor".into(), raw_value),
    ]
  }
  fn border_horizontal_style(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderStartStyle".into(), raw_value.clone()),
      OrderPair("borderEndStyle".into(), raw_value),
    ]
  }
  fn border_horizontal_width(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderStartWidth".into(), raw_value.clone()),
      OrderPair("borderEndWidth".into(), raw_value),
    ]
  }
  fn border_vertical_color(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopColor".into(), raw_value.clone()),
      OrderPair("borderBottomColor".into(), raw_value),
    ]
  }
  fn border_vertical_style(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopStyle".into(), raw_value.clone()),
      OrderPair("borderBottomStyle".into(), raw_value),
    ]
  }
  fn border_vertical_width(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("borderTopWidth".into(), raw_value.clone()),
      OrderPair("borderBottomWidth".into(), raw_value),
    ]
  }

  fn border_radius(raw_value: Option<String>) -> Vec<OrderPair> {
    let (top, right, bottom, left) = split_value_required(raw_value.as_deref());

    vec![
      OrderPair("borderTopStartRadius".into(), Some(top)),
      OrderPair("borderTopEndRadius".into(), Some(right)),
      OrderPair("borderBottomEndRadius".into(), Some(bottom)),
      OrderPair("borderBottomStartRadius".into(), Some(left)),
    ]
  }

  fn inset(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("top".into(), raw_value.clone()),
      OrderPair("end".into(), raw_value.clone()),
      OrderPair("bottom".into(), raw_value.clone()),
      OrderPair("start".into(), raw_value),
    ]
  }
  fn inset_inline(raw_value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::start(raw_value.clone()));
    result.extend(Shorthands::end(raw_value));

    result
  }
  fn inset_block(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("top".into(), raw_value.clone()),
      OrderPair("bottom".into(), raw_value),
    ]
  }
  fn start(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("start".into(), raw_value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ]
  }
  fn end(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("end".into(), raw_value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ]
  }
  fn left(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("left".into(), raw_value),
      OrderPair("start".into(), None),
      OrderPair("end".into(), None),
    ]
  }
  fn right(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("right".into(), raw_value),
      OrderPair("start".into(), None),
      OrderPair("end".into(), None),
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
      OrderPair("marginEnd".into(), Some(right)),
      OrderPair("marginBottom".into(), Some(bottom)),
      OrderPair("marginStart".into(), Some(left)),
    ]
  }
  fn margin_horizontal(raw_value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::margin_start(raw_value.clone()));
    result.extend(Shorthands::margin_end(raw_value));

    result
  }
  fn margin_start(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginStart".into(), raw_value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ]
  }
  fn margin_end(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginEnd".into(), raw_value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ]
  }
  fn margin_left(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginLeft".into(), raw_value),
      OrderPair("marginStart".into(), None),
      OrderPair("marginEnd".into(), None),
    ]
  }
  fn margin_right(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginRight".into(), raw_value),
      OrderPair("marginStart".into(), None),
      OrderPair("marginEnd".into(), None),
    ]
  }
  fn margin_vertical(raw_value: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("marginTop".into(), raw_value.clone()),
      OrderPair("marginBottom".into(), raw_value),
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
      OrderPair("paddingEnd".into(), Some(right)),
      OrderPair("paddingBottom".into(), Some(bottom)),
      OrderPair("paddingStart".into(), Some(left)),
    ]
  }
  fn padding_horizontal(val: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![];

    result.extend(Shorthands::padding_start(val.clone()));
    result.extend(Shorthands::padding_end(val));

    result
  }
  fn padding_start(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingStart".into(), val),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ]
  }
  fn padding_end(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingEnd".into(), val),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ]
  }
  fn padding_left(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingLeft".into(), val),
      OrderPair("paddingStart".into(), None),
      OrderPair("paddingEnd".into(), None),
    ]
  }
  fn padding_right(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingRight".into(), val),
      OrderPair("paddingStart".into(), None),
      OrderPair("paddingEnd".into(), None),
    ]
  }
  fn padding_vertical(val: Option<String>) -> Vec<OrderPair> {
    vec![
      OrderPair("paddingTop".into(), val.clone()),
      OrderPair("paddingBottom".into(), val),
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
    vec![OrderPair("borderStartWidth".into(), val)]
  }
  fn border_inline_start_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderStartStyle".into(), val)]
  }
  fn border_inline_start_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderStartColor".into(), val)]
  }
  fn border_inline_end_width(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndWidth".into(), val)]
  }
  fn border_inline_end_style(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndStyle".into(), val)]
  }
  fn border_inline_end_color(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("borderEndColor".into(), val)]
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
    vec![OrderPair("marginStart".into(), val)]
  }
  fn margin_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("marginEnd".into(), val)]
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
    vec![OrderPair("paddingStart".into(), val)]
  }
  fn padding_inline_end(val: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("paddingEnd".into(), val)]
  }

  fn scroll_margin_block_start(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginTop".into(), value)]
  }
  fn scroll_margin_block_end(value: Option<String>) -> Vec<OrderPair> {
    vec![OrderPair("scrollMarginBottom".into(), value)]
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
      "borderBlockStartWidth" => Some(Aliases::border_block_start_width),
      "borderBlockStartStyle" => Some(Aliases::border_block_start_style),
      "borderBlockStartColor" => Some(Aliases::border_block_start_color),
      "borderBlockEndWidth" => Some(Aliases::border_block_end_width),
      "borderBlockEndStyle" => Some(Aliases::border_block_end_style),
      "borderBlockEndColor" => Some(Aliases::border_block_end_color),
      "borderInlineWidth" => Shorthands::get("borderHorizontalWidth"),
      "borderInlineStyle" => Shorthands::get("borderHorizontalStyle"),
      "borderInlineColor" => Shorthands::get("borderHorizontalColor"),
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
      _ => None,
    }
  }
}
