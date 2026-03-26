use stylex_types::structures::order_pair::OrderPair;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Shorthands;

impl Shorthands {
  #[allow(dead_code)]
  fn all(_: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err("all is not supported".into())
  }
  fn animation(_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err("animation is not supported".into())
  }
  fn background(_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err("background is not supported. Use background-color, border-image etc. instead".into())
  }
  fn border(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err("border is not supported. Use border-width, border-style and border-color instead".into())
  }
  fn border_inline(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderInline is not supported. Use borderInlineWidth, borderInlineStyle and borderInlineColor instead".into(),
    )
  }
  // @Deprecated
  fn border_block(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderBlock is not supported. Use borderBlockWidth, borderBlockStyle and borderBlockColor instead".into(),
    )
  }

  // @Deprecated
  fn border_top(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderTop is not supported. Use borderTopWidth, borderTopStyle and borderTopColor instead"
        .into(),
    )
  }
  // @Deprecated
  fn border_inline_end(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderInlineEnd is not supported. Use borderInlineEndWidth, borderInlineEndStyle and borderInlineEndColor instead".into(),
    )
  }
  // @Deprecated
  fn border_right(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderRight is not supported. Use borderRightWidth, borderRightStyle and borderRightColor instead".into(),
    )
  }
  // @Deprecated
  fn border_bottom(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderBottom is not supported. Use borderBottomWidth, borderBottomStyle and borderBottomColor instead".into(),
    )
  }
  // @Deprecated
  fn border_inline_start(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(
      "borderInlineStart is not supported. Use borderInlineStartWidth, borderInlineStartStyle and borderInlineStartColor instead".into(),
    )
  }
  // @Deprecated
  fn border_left(_raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err(format!(
      "{}{}{}",
      "`borderLeft` is not supported.",
      "You could use `borderLeftWidth`, `borderLeftStyle` and `borderLeftColor`,",
      "but it is preferable to use `borderInlineStartWidth`, `borderInlineStartStyle` and `borderInlineStartColor`."
    ))
  }

  pub fn get(name: &str) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    match name {
      "animation" => Some(Shorthands::animation),
      "background" => Some(Shorthands::background),
      "border" => Some(Shorthands::border),
      "border_inline" => Some(Shorthands::border_inline),
      "border_block" => Some(Shorthands::border_block),
      "border_top" => Some(Shorthands::border_top),
      "border_inline_end" => Some(Shorthands::border_inline_end),
      "border_right" => Some(Shorthands::border_right),
      "border_bottom" => Some(Shorthands::border_bottom),
      "border_inline_start" => Some(Shorthands::border_inline_start),
      "border_left" => Some(Shorthands::border_left),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Aliases;

impl Aliases {
  fn block_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("height".to_string(), val)])
  }
  fn inline_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("width".to_string(), val)])
  }
  fn min_block_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("minHeight".to_string(), val)])
  }
  fn min_inline_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("minWidth".to_string(), val)])
  }
  fn max_block_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("maxHeight".to_string(), val)])
  }
  fn max_inline_size(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("maxWidth".to_string(), val)])
  }

  fn border_horizontal_width(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineWidth".to_string(), val)])
  }
  fn border_horizontal_style(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineStyle".to_string(), val)])
  }
  fn border_horizontal_color(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineColor".to_string(), val)])
  }
  fn border_vertical_width(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBlockWidth".to_string(), val)])
  }
  fn border_vertical_style(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBlockStyle".to_string(), val)])
  }
  fn border_vertical_color(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBlockColor".to_string(), val)])
  }

  fn border_block_start_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopColor".to_string(), value)])
  }
  fn border_block_end_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomColor".to_string(), value)])
  }
  fn border_block_start_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopStyle".to_string(), value)])
  }
  fn border_block_end_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomStyle".to_string(), value)])
  }
  fn border_block_start_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopWidth".to_string(), value)])
  }
  fn border_block_end_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomWidth".to_string(), value)])
  }

  fn border_start_color(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineStartColor".to_string(), val)])
  }
  fn border_end_color(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineEndColor".to_string(), val)])
  }
  fn border_start_style(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineStartStyle".to_string(), val)])
  }
  fn border_end_style(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineEndStyle".to_string(), val)])
  }
  fn border_start_width(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineStartWidth".to_string(), val)])
  }
  fn border_end_width(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderInlineEndWidth".to_string(), val)])
  }

  fn border_top_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderStartStartRadius".to_string(), value)])
  }
  fn border_top_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderStartEndRadius".to_string(), value)])
  }
  fn border_bottom_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderEndStartRadius".to_string(), value)])
  }
  fn border_bottom_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderEndEndRadius".to_string(), value)])
  }

  fn contain_intrinsic_block_size(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("containIntrinsicHeight".to_string(), value)])
  }
  fn contain_intrinsic_inline_size(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("containIntrinsicWidth".to_string(), value)])
  }

  fn margin_block_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginTop".to_string(), value)])
  }
  fn margin_block_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginBottom".to_string(), value)])
  }
  fn margin_start(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginInlineStart".to_string(), val)])
  }
  fn margin_end(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginInlineEnd".to_string(), val)])
  }
  fn margin_horizontal(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginInline".to_string(), val)])
  }
  fn margin_vertical(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginBlock".to_string(), val)])
  }

  fn overflow_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("overflowY".to_string(), value)])
  }
  fn overflow_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("overflowX".to_string(), value)])
  }

  fn padding_block_start(raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingTop".to_string(), raw_value)])
  }
  fn padding_block_end(raw_value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingBottom".to_string(), raw_value)])
  }
  fn padding_start(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingInlineStart".to_string(), val)])
  }
  fn padding_end(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingInlineEnd".to_string(), val)])
  }
  fn padding_horizontal(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingInline".to_string(), val)])
  }
  fn padding_vertical(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingBlock".to_string(), val)])
  }

  fn scroll_margin_block_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("scrollMarginTop".to_string(), value)])
  }
  fn scroll_margin_block_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("scrollMarginBottom".to_string(), value)])
  }

  fn inset_block_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("top".to_string(), value)])
  }
  fn inset_block_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("bottom".to_string(), value)])
  }
  fn start(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("insetInlineStart".to_string(), val)])
  }
  fn end(val: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("insetInlineEnd".to_string(), val)])
  }

  pub fn get(name: &str) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    match name {
      // @Deprecated
      "borderHorizontal" => Shorthands::get("borderHorizontal"),
      // @Deprecated
      "borderVertical" => Shorthands::get("borderVertical"),
      // @Deprecated
      "borderBlockStart" => Shorthands::get("borderBlockStart"),
      // @Deprecated
      "borderEnd" => Shorthands::get("borderEnd"),
      // @Deprecated
      "borderBlockEnd" => Shorthands::get("borderBlockEnd"),
      // @Deprecated
      "borderStart" => Shorthands::get("borderStart"),

      "blockSize" => Some(Aliases::block_size),
      "inlineSize" => Some(Aliases::inline_size),
      "minBlockSize" => Some(Aliases::min_block_size),
      "minInlineSize" => Some(Aliases::min_inline_size),
      "maxBlockSize" => Some(Aliases::max_block_size),
      "maxInlineSize" => Some(Aliases::max_inline_size),
      "borderHorizontalWidth" => Some(Aliases::border_horizontal_width),
      "borderHorizontalStyle" => Some(Aliases::border_horizontal_style),
      "borderHorizontalColor" => Some(Aliases::border_horizontal_color),
      "borderVerticalWidth" => Some(Aliases::border_vertical_width),
      "borderVerticalStyle" => Some(Aliases::border_vertical_style),
      "borderVerticalColor" => Some(Aliases::border_vertical_color),
      "borderBlockStartColor" => Some(Aliases::border_block_start_color),
      "borderBlockEndColor" => Some(Aliases::border_block_end_color),
      "borderBlockStartStyle" => Some(Aliases::border_block_start_style),
      "borderBlockEndStyle" => Some(Aliases::border_block_end_style),
      "borderBlockStartWidth" => Some(Aliases::border_block_start_width),
      "borderBlockEndWidth" => Some(Aliases::border_block_end_width),
      "borderStartColor" => Some(Aliases::border_start_color),
      "borderEndColor" => Some(Aliases::border_end_color),
      "borderStartStyle" => Some(Aliases::border_start_style),
      "borderEndStyle" => Some(Aliases::border_end_style),
      "borderStartWidth" => Some(Aliases::border_start_width),
      "borderEndWidth" => Some(Aliases::border_end_width),
      "borderTopStartRadius" => Some(Aliases::border_top_start_radius),
      "borderTopEndRadius" => Some(Aliases::border_top_end_radius),
      "borderBottomStartRadius" => Some(Aliases::border_bottom_start_radius),
      "borderBottomEndRadius" => Some(Aliases::border_bottom_end_radius),
      "containIntrinsicBlockSize" => Some(Aliases::contain_intrinsic_block_size),
      "containIntrinsicInlineSize" => Some(Aliases::contain_intrinsic_inline_size),
      "marginBlockStart" => Some(Aliases::margin_block_start),
      "marginBlockEnd" => Some(Aliases::margin_block_end),
      "marginStart" => Some(Aliases::margin_start),
      "marginEnd" => Some(Aliases::margin_end),
      "marginHorizontal" => Some(Aliases::margin_horizontal),
      "marginVertical" => Some(Aliases::margin_vertical),
      "overflowBlock" => Some(Aliases::overflow_block),
      "overflowInline" => Some(Aliases::overflow_inline),
      "paddingBlockStart" => Some(Aliases::padding_block_start),
      "paddingBlockEnd" => Some(Aliases::padding_block_end),
      "paddingStart" => Some(Aliases::padding_start),
      "paddingEnd" => Some(Aliases::padding_end),
      "paddingHorizontal" => Some(Aliases::padding_horizontal),
      "paddingVertical" => Some(Aliases::padding_vertical),
      "scrollMarginBlockStart" => Some(Aliases::scroll_margin_block_start),
      "scrollMarginBlockEnd" => Some(Aliases::scroll_margin_block_end),
      "insetBlockStart" => Some(Aliases::inset_block_start),
      "insetBlockEnd" => Some(Aliases::inset_block_end),
      "start" => Some(Aliases::start),
      "end" => Some(Aliases::end),
      _ => None,
    }
  }
}
