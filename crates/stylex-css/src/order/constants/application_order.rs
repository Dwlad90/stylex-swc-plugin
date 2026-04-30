use stylex_structures::order_pair::OrderPair;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Shorthands;

impl Shorthands {
  fn infallible(result: Result<Vec<OrderPair>, String>) -> Vec<OrderPair> {
    match result {
      Ok(v) => v,
      Err(e) => unreachable!("infallible shorthand returned Err: {}", e),
    }
  }

  fn all(_: Option<String>) -> Result<Vec<OrderPair>, String> {
    Err("all is not supported".into())
  }
  fn animation(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![
      OrderPair("animation".into(), value),
      OrderPair("animationComposition".into(), None),
      OrderPair("animationName".into(), None),
      OrderPair("animationDuration".into(), None),
      OrderPair("animationTimingFunction".into(), None),
      OrderPair("animationDelay".into(), None),
      OrderPair("animationIterationCount".into(), None),
      OrderPair("animationDirection".into(), None),
      OrderPair("animationFillMode".into(), None),
      OrderPair("animationPlayState".into(), None),
      OrderPair("animationTimeline".into(), None),
    ];

    result.extend(Self::infallible(Shorthands::animation_range(None)));

    Ok(result)
  }

  fn animation_range(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("animationRange".into(), value),
      OrderPair("animationRangeEnd".into(), None),
      OrderPair("animationRangeStart".into(), None),
    ];

    Ok(result)
  }

  fn background(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![
      OrderPair("background".into(), value),
      OrderPair("backgroundAttachment".into(), None),
      OrderPair("backgroundClip".into(), None),
      OrderPair("backgroundColor".into(), None),
      OrderPair("backgroundImage".into(), None),
      OrderPair("backgroundOrigin".into(), None),
      OrderPair("backgroundRepeat".into(), None),
      OrderPair("backgroundSize".into(), None),
    ];

    result.extend(Self::infallible(Shorthands::background_position(None)));

    Ok(result)
  }

  fn background_position(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("backgroundPosition".into(), value),
      OrderPair("backgroundPositionX".into(), None),
      OrderPair("backgroundPositionY".into(), None),
    ];

    Ok(result)
  }

  // These will be removed later, matching the properties with React Native.
  // For now, we're compiling them to the React Native properties.
  fn border(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("border".into(), value)];

    result.extend(Self::infallible(Shorthands::border_width(None)));
    result.extend(Self::infallible(Shorthands::border_style(None)));
    result.extend(Self::infallible(Shorthands::border_color(None)));

    Ok(result)
  }
  fn border_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderInline".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_width(None)));
    result.extend(Self::infallible(Shorthands::border_inline_style(None)));
    result.extend(Self::infallible(Shorthands::border_inline_color(None)));

    Ok(result)
  }
  fn border_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderBlock".into(), value)];

    result.extend(Self::infallible(Shorthands::border_block_width(None)));
    result.extend(Self::infallible(Shorthands::border_block_style(None)));
    result.extend(Self::infallible(Shorthands::border_block_color(None)));

    Ok(result)
  }

  fn border_top(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderTop".into(), value),
      OrderPair("borderTopWidth".into(), None),
      OrderPair("borderTopStyle".into(), None),
      OrderPair("borderTopColor".into(), None),
    ];

    Ok(result)
  }
  // @Deprecated
  fn border_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderInlineEnd".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_end_width(None)));
    result.extend(Self::infallible(Shorthands::border_inline_end_style(None)));
    result.extend(Self::infallible(Shorthands::border_inline_end_color(None)));

    Ok(result)
  }
  // @Deprecated
  fn border_right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderRight".into(), value)];

    result.extend(Self::infallible(Shorthands::border_right_width(None)));
    result.extend(Self::infallible(Shorthands::border_right_style(None)));
    result.extend(Self::infallible(Shorthands::border_right_color(None)));

    Ok(result)
  }
  // @Deprecated
  fn border_bottom(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBottom".into(), value),
      OrderPair("borderBottomWidth".into(), None),
      OrderPair("borderBottomStyle".into(), None),
      OrderPair("borderBottomColor".into(), None),
    ];

    Ok(result)
  }
  // @Deprecated
  fn border_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderInlineStart".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_start_width(
      None,
    )));
    result.extend(Self::infallible(Shorthands::border_inline_start_style(
      None,
    )));
    result.extend(Self::infallible(Shorthands::border_inline_start_color(
      None,
    )));

    Ok(result)
  }
  // @Deprecated
  fn border_left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderLeft".into(), value)];

    result.extend(Self::infallible(Shorthands::border_left_width(None)));
    result.extend(Self::infallible(Shorthands::border_left_style(None)));
    result.extend(Self::infallible(Shorthands::border_left_color(None)));

    Ok(result)
  }
  fn border_inline_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineWidth".into(), value),
      OrderPair("borderInlineStartWidth".into(), None),
      OrderPair("borderLeftWidth".into(), None),
      OrderPair("borderInlineEndWidth".into(), None),
      OrderPair("borderRightWidth".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineStyle".into(), value),
      OrderPair("borderInlineStartStyle".into(), None),
      OrderPair("borderLeftStyle".into(), None),
      OrderPair("borderInlineEndStyle".into(), None),
      OrderPair("borderRightStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineColor".into(), value),
      OrderPair("borderInlineStartColor".into(), None),
      OrderPair("borderLeftColor".into(), None),
      OrderPair("borderInlineEndColor".into(), None),
      OrderPair("borderRightColor".into(), None),
    ];

    Ok(result)
  }
  fn border_block_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBlockWidth".into(), value),
      OrderPair("borderTopWidth".into(), None),
      OrderPair("borderBottomWidth".into(), None),
    ];

    Ok(result)
  }
  fn border_block_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBlockStyle".into(), value),
      OrderPair("borderTopStyle".into(), None),
      OrderPair("borderBottomStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_block_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBlockColor".into(), value),
      OrderPair("borderTopColor".into(), None),
      OrderPair("borderBottomColor".into(), None),
    ];

    Ok(result)
  }
  fn border_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderColor".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_color(None)));
    result.extend(Self::infallible(Shorthands::border_block_color(None)));

    Ok(result)
  }
  fn border_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderStyle".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_style(None)));
    result.extend(Self::infallible(Shorthands::border_block_style(None)));

    Ok(result)
  }
  fn border_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("borderWidth".into(), value)];

    result.extend(Self::infallible(Shorthands::border_inline_width(None)));
    result.extend(Self::infallible(Shorthands::border_block_width(None)));

    Ok(result)
  }
  fn border_inline_start_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineStartColor".into(), value),
      OrderPair("borderLeftColor".into(), None),
      OrderPair("borderRightColor".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_end_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineEndColor".into(), value),
      OrderPair("borderLeftColor".into(), None),
      OrderPair("borderRightColor".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_start_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineStartStyle".into(), value),
      OrderPair("borderLeftStyle".into(), None),
      OrderPair("borderRightStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_end_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineEndStyle".into(), value),
      OrderPair("borderLeftStyle".into(), None),
      OrderPair("borderRightStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_start_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineStartWidth".into(), value),
      OrderPair("borderLeftWidth".into(), None),
      OrderPair("borderRightWidth".into(), None),
    ];

    Ok(result)
  }
  fn border_inline_end_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderInlineEndWidth".into(), value),
      OrderPair("borderLeftWidth".into(), None),
      OrderPair("borderRightWidth".into(), None),
    ];

    Ok(result)
  }
  fn border_left_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderLeftColor".into(), value),
      OrderPair("borderInlineStartColor".into(), None),
      OrderPair("borderInlineEndColor".into(), None),
    ];

    Ok(result)
  }
  fn border_right_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderRightColor".into(), value),
      OrderPair("borderInlineStartColor".into(), None),
      OrderPair("borderInlineEndColor".into(), None),
    ];

    Ok(result)
  }
  fn border_left_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderLeftStyle".into(), value),
      OrderPair("borderInlineStartStyle".into(), None),
      OrderPair("borderInlineEndStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_right_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderRightStyle".into(), value),
      OrderPair("borderInlineStartStyle".into(), None),
      OrderPair("borderInlineEndStyle".into(), None),
    ];

    Ok(result)
  }
  fn border_left_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderLeftWidth".into(), value),
      OrderPair("borderInlineStartWidth".into(), None),
      OrderPair("borderInlineEndWidth".into(), None),
    ];

    Ok(result)
  }
  fn border_right_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderRightWidth".into(), value),
      OrderPair("borderInlineStartWidth".into(), None),
      OrderPair("borderInlineEndWidth".into(), None),
    ];

    Ok(result)
  }

  fn border_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderRadius".into(), value),
      // // logical constituents
      OrderPair("borderStartStartRadius".into(), None),
      OrderPair("borderStartEndRadius".into(), None),
      OrderPair("borderEndStartRadius".into(), None),
      OrderPair("borderEndEndRadius".into(), None),
      // physical constituents
      OrderPair("borderTopLeftRadius".into(), None),
      OrderPair("borderTopRightRadius".into(), None),
      OrderPair("borderBottomLeftRadius".into(), None),
      OrderPair("borderBottomRightRadius".into(), None),
    ];

    Ok(result)
  }

  fn border_start_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderStartStartRadius".into(), value),
      OrderPair("borderTopLeftRadius".into(), None),
      OrderPair("borderTopRightRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_start_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderStartEndRadius".into(), value),
      OrderPair("borderTopLeftRadius".into(), None),
      OrderPair("borderTopRightRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_end_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderEndStartRadius".into(), value),
      OrderPair("borderBottomLeftRadius".into(), None),
      OrderPair("borderBottomRightRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_end_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderEndEndRadius".into(), value),
      OrderPair("borderBottomLeftRadius".into(), None),
      OrderPair("borderBottomRightRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_top_left_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderTopLeftRadius".into(), value),
      OrderPair("borderStartStartRadius".into(), None),
      OrderPair("borderStartEndRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_top_right_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderTopRightRadius".into(), value),
      OrderPair("borderStartStartRadius".into(), None),
      OrderPair("borderStartEndRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_bottom_left_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBottomLeftRadius".into(), value),
      OrderPair("borderEndStartRadius".into(), None),
      OrderPair("borderEndEndRadius".into(), None),
    ];

    Ok(result)
  }
  fn border_bottom_right_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderBottomRightRadius".into(), value),
      OrderPair("borderEndStartRadius".into(), None),
      OrderPair("borderEndEndRadius".into(), None),
    ];

    Ok(result)
  }

  fn corner_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerShape".into(), value),
      OrderPair("cornerStartStartShape".into(), None),
      OrderPair("cornerStartEndShape".into(), None),
      OrderPair("cornerEndStartShape".into(), None),
      OrderPair("cornerEndEndShape".into(), None),
      OrderPair("cornerTopLeftShape".into(), None),
      OrderPair("cornerTopRightShape".into(), None),
      OrderPair("cornerBottomLeftShape".into(), None),
      OrderPair("cornerBottomRightShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_start_start_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerStartStartShape".into(), value),
      OrderPair("cornerTopLeftShape".into(), None),
      OrderPair("cornerTopRightShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_start_end_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerStartEndShape".into(), value),
      OrderPair("cornerTopLeftShape".into(), None),
      OrderPair("cornerTopRightShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_end_start_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerEndStartShape".into(), value),
      OrderPair("cornerBottomLeftShape".into(), None),
      OrderPair("cornerBottomRightShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_end_end_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerEndEndShape".into(), value),
      OrderPair("cornerBottomLeftShape".into(), None),
      OrderPair("cornerBottomRightShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_top_left_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerTopLeftShape".into(), value),
      OrderPair("cornerStartStartShape".into(), None),
      OrderPair("cornerStartEndShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_top_right_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerTopRightShape".into(), value),
      OrderPair("cornerStartStartShape".into(), None),
      OrderPair("cornerStartEndShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_bottom_left_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerBottomLeftShape".into(), value),
      OrderPair("cornerEndStartShape".into(), None),
      OrderPair("cornerEndEndShape".into(), None),
    ];

    Ok(result)
  }

  fn corner_bottom_right_shape(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("cornerBottomRightShape".into(), value),
      OrderPair("cornerEndStartShape".into(), None),
      OrderPair("cornerEndEndShape".into(), None),
    ];

    Ok(result)
  }

  fn border_image(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("borderImage".into(), value),
      OrderPair("borderImageOutset".into(), None),
      OrderPair("borderImageRepeat".into(), None),
      OrderPair("borderImageSlice".into(), None),
      OrderPair("borderImageSource".into(), None),
      OrderPair("borderImageWidth".into(), None),
    ];

    Ok(result)
  }

  fn column_rule(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("columnRule".into(), value),
      OrderPair("columnRuleColor".into(), None),
      OrderPair("columnRuleStyle".into(), None),
      OrderPair("columnRuleWidth".into(), None),
    ];

    Ok(result)
  }
  fn columns(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("columns".into(), value),
      OrderPair("columnCount".into(), None),
      OrderPair("columnWidth".into(), None),
    ];

    Ok(result)
  }

  fn container(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("container".into(), value),
      OrderPair("containerName".into(), None),
      OrderPair("containerType".into(), None),
    ];

    Ok(result)
  }

  fn contain_intrinsic_size(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("containIntrinsicSize".into(), value),
      OrderPair("containIntrinsicWidth".into(), None),
      OrderPair("containIntrinsicHeight".into(), None),
    ];

    Ok(result)
  }

  fn flex(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("flex".into(), value),
      OrderPair("flexGrow".into(), None),
      OrderPair("flexShrink".into(), None),
      OrderPair("flexBasis".into(), None),
    ];

    Ok(result)
  }
  fn flex_flow(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("flexFlow".into(), value),
      OrderPair("flexDirection".into(), None),
      OrderPair("flexWrap".into(), None),
    ];

    Ok(result)
  }
  // @Deprecated ?
  fn font(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![
      OrderPair("font".into(), value),
      OrderPair("fontFamily".into(), None),
      OrderPair("fontSize".into(), None),
      OrderPair("fontStretch".into(), None),
      OrderPair("fontStyle".into(), None),
      OrderPair("fontWeight".into(), None),
      OrderPair("lineHeight".into(), None),
    ];

    result.extend(Self::infallible(Shorthands::font_variant(None)));

    Ok(result)
  }
  fn font_variant(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("fontVariant".into(), value),
      OrderPair("fontVariantAlternates".into(), None),
      OrderPair("fontVariantCaps".into(), None),
      OrderPair("fontVariantEastAsian".into(), None),
      OrderPair("fontVariantEmoji".into(), None),
      OrderPair("fontVariantLigatures".into(), None),
      OrderPair("fontVariantNumeric".into(), None),
      OrderPair("fontVariantPosition".into(), None),
    ];

    Ok(result)
  }
  fn gap(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("gap".into(), value),
      OrderPair("rowGap".into(), None),
      OrderPair("columnGap".into(), None),
    ];

    Ok(result)
  }
  fn grid(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![
      OrderPair("grid".into(), value),
      OrderPair("gridAutoRows".into(), None),
      OrderPair("gridAutoColumns".into(), None),
      OrderPair("gridAutoFlow".into(), None),
    ];

    result.extend(Self::infallible(Shorthands::grid_template(None)));

    Ok(result)
  }
  fn grid_area(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("gridArea".into(), value),
      OrderPair("gridRow".into(), None),
      OrderPair("gridRowStart".into(), None),
      OrderPair("gridRowEnd".into(), None),
      OrderPair("gridColumn".into(), None),
      OrderPair("gridColumnStart".into(), None),
      OrderPair("gridColumnEnd".into(), None),
    ];

    Ok(result)
  }
  fn grid_row(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("gridRow".into(), value),
      OrderPair("gridRowStart".into(), None),
      OrderPair("gridRowEnd".into(), None),
    ];

    Ok(result)
  }
  fn grid_column(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("gridColumn".into(), value),
      OrderPair("gridColumnStart".into(), None),
      OrderPair("gridColumnEnd".into(), None),
    ];

    Ok(result)
  }
  fn grid_template(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("gridTemplate".into(), value),
      OrderPair("gridTemplateAreas".into(), None),
      OrderPair("gridTemplateColumns".into(), None),
      OrderPair("gridTemplateRows".into(), None),
    ];

    Ok(result)
  }
  fn inset(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("inset".into(), value)];

    result.extend(Self::infallible(Shorthands::inset_inline(None)));
    result.extend(Self::infallible(Shorthands::inset_block(None)));

    Ok(result)
  }
  fn inset_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("insetInline".into(), value),
      OrderPair("insetInlineStart".into(), None),
      OrderPair("insetInlineEnd".into(), None),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ];

    Ok(result)
  }
  fn inset_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("insetBlock".into(), value),
      OrderPair("top".into(), None),
      OrderPair("bottom".into(), None),
    ];

    Ok(result)
  }
  fn inset_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("insetInlineStart".into(), value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ];

    Ok(result)
  }
  fn inset_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("insetInlineEnd".into(), value),
      OrderPair("left".into(), None),
      OrderPair("right".into(), None),
    ];

    Ok(result)
  }
  fn left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("left".into(), value),
      OrderPair("insetInlineStart".into(), None),
      OrderPair("insetInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("right".into(), value),
      OrderPair("insetInlineStart".into(), None),
      OrderPair("insetInlineEnd".into(), None),
    ];

    Ok(result)
  }

  fn list_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("listStyle".into(), value),
      OrderPair("listStyleImage".into(), None),
      OrderPair("listStylePosition".into(), None),
      OrderPair("listStyleType".into(), None),
    ];

    Ok(result)
  }

  fn margin(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("margin".into(), value)];

    result.extend(Self::infallible(Shorthands::margin_inline(None)));
    result.extend(Self::infallible(Shorthands::margin_block(None)));

    Ok(result)
  }
  fn margin_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginInline".into(), value),
      OrderPair("marginInlineStart".into(), None),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginInlineEnd".into(), None),
      OrderPair("marginRight".into(), None),
    ];

    Ok(result)
  }
  fn margin_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginBlock".into(), value),
      OrderPair("marginTop".into(), None),
      OrderPair("marginBottom".into(), None),
    ];

    Ok(result)
  }
  fn margin_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginInlineStart".into(), value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ];

    Ok(result)
  }
  fn margin_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginInlineEnd".into(), value),
      OrderPair("marginLeft".into(), None),
      OrderPair("marginRight".into(), None),
    ];

    Ok(result)
  }
  fn margin_left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginLeft".into(), value),
      OrderPair("marginInlineStart".into(), None),
      OrderPair("marginInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn margin_right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("marginRight".into(), value),
      OrderPair("marginInlineStart".into(), None),
      OrderPair("marginInlineEnd".into(), None),
    ];

    Ok(result)
  }

  fn mask(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("mask".into(), value),
      OrderPair("maskClip".into(), None),
      OrderPair("maskComposite".into(), None),
      OrderPair("maskImage".into(), None),
      OrderPair("maskMode".into(), None),
      OrderPair("maskOrigin".into(), None),
      OrderPair("maskPosition".into(), None),
      OrderPair("maskRepeat".into(), None),
      OrderPair("maskSize".into(), None),
    ];

    Ok(result)
  }

  fn mask_border(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("maskBorder".into(), value),
      OrderPair("maskBorderMode".into(), None),
      OrderPair("maskBorderOutset".into(), None),
      OrderPair("maskBorderRepeat".into(), None),
      OrderPair("maskBorderSlice".into(), None),
      OrderPair("maskBorderSource".into(), None),
      OrderPair("maskBorderWidth".into(), None),
    ];

    Ok(result)
  }

  fn offset(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("offset".into(), value),
      OrderPair("offsetAnchor".into(), None),
      OrderPair("offsetDistance".into(), None),
      OrderPair("offsetPath".into(), None),
      OrderPair("offsetPosition".into(), None),
      OrderPair("offsetRotate".into(), None),
    ];

    Ok(result)
  }

  fn outline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("outline".into(), value),
      OrderPair("outlineColor".into(), None),
      OrderPair("outlineOffset".into(), None),
      OrderPair("outlineStyle".into(), None),
      OrderPair("outlineWidth".into(), None),
    ];

    Ok(result)
  }

  fn overflow(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("overflow".into(), value),
      OrderPair("overflowX".into(), None),
      OrderPair("overflowY".into(), None),
    ];

    Ok(result)
  }

  fn padding(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("padding".into(), value)];

    result.extend(Self::infallible(Shorthands::padding_inline(None)));
    result.extend(Self::infallible(Shorthands::padding_block(None)));

    Ok(result)
  }

  fn padding_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingInline".into(), value),
      OrderPair("paddingStart".into(), None),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingEnd".into(), None),
      OrderPair("paddingRight".into(), None),
    ];

    Ok(result)
  }
  fn padding_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingBlock".into(), value),
      OrderPair("paddingTop".into(), None),
      OrderPair("paddingBottom".into(), None),
    ];

    Ok(result)
  }
  fn padding_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingInlineStart".into(), value),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ];

    Ok(result)
  }
  fn padding_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingInlineEnd".into(), value),
      OrderPair("paddingLeft".into(), None),
      OrderPair("paddingRight".into(), None),
    ];

    Ok(result)
  }
  fn padding_left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingLeft".into(), value),
      OrderPair("paddingInlineStart".into(), None),
      OrderPair("paddingInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn padding_right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("paddingRight".into(), value),
      OrderPair("paddingInlineStart".into(), None),
      OrderPair("paddingInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn place_content(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("placeContent".into(), value),
      OrderPair("alignContent".into(), None),
      OrderPair("justifyContent".into(), None),
    ];

    Ok(result)
  }
  fn place_items(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("placeItems".into(), value),
      OrderPair("alignItems".into(), None),
      OrderPair("justifyItems".into(), None),
    ];

    Ok(result)
  }
  fn place_self(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("placeSelf".into(), value),
      OrderPair("alignSelf".into(), None),
      OrderPair("justifySelf".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("scrollMargin".into(), value)];

    result.extend(Self::infallible(Shorthands::scroll_margin_inline(None)));
    result.extend(Self::infallible(Shorthands::scroll_margin_block(None)));

    Ok(result)
  }
  fn scroll_margin_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginBlock".into(), value),
      OrderPair("scrollMarginTop".into(), None),
      OrderPair("scrollMarginBottom".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginInline".into(), value),
      OrderPair("scrollMarginInlineStart".into(), None),
      OrderPair("scrollMarginInlineEnd".into(), None),
      OrderPair("scrollMarginLeft".into(), None),
      OrderPair("scrollMarginRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginInlineStart".into(), value),
      OrderPair("scrollMarginLeft".into(), None),
      OrderPair("scrollMarginRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginInlineEnd".into(), value),
      OrderPair("scrollMarginLeft".into(), None),
      OrderPair("scrollMarginRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin_left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginLeft".into(), value),
      OrderPair("scrollMarginInlineStart".into(), None),
      OrderPair("scrollMarginInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn scroll_margin_right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollMarginRight".into(), value),
      OrderPair("scrollMarginInlineStart".into(), None),
      OrderPair("scrollMarginInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let mut result = vec![OrderPair("scrollPadding".into(), value)];

    result.extend(Self::infallible(Shorthands::scroll_padding_block(None)));
    result.extend(Self::infallible(Shorthands::scroll_padding_inline(None)));

    Ok(result)
  }
  fn scroll_padding_block(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingBlock".into(), value),
      OrderPair("scrollPaddingTop".into(), None),
      OrderPair("scrollPaddingBottom".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding_inline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingInline".into(), value),
      OrderPair("scrollPaddingInlineStart".into(), None),
      OrderPair("scrollPaddingInlineEnd".into(), None),
      OrderPair("scrollPaddingLeft".into(), None),
      OrderPair("scrollPaddingRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding_inline_start(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingInlineStart".into(), value),
      OrderPair("scrollPaddingLeft".into(), None),
      OrderPair("scrollPaddingRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding_inline_end(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingInlineEnd".into(), value),
      OrderPair("scrollPaddingLeft".into(), None),
      OrderPair("scrollPaddingRight".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding_left(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingLeft".into(), value),
      OrderPair("scrollPaddingInlineStart".into(), None),
      OrderPair("scrollPaddingInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn scroll_padding_right(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollPaddingRight".into(), value),
      OrderPair("scrollPaddingInlineStart".into(), None),
      OrderPair("scrollPaddingInlineEnd".into(), None),
    ];

    Ok(result)
  }
  fn scroll_snap_type(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollSnapType".into(), value),
      OrderPair("scrollSnapTypeX".into(), None),
      OrderPair("scrollSnapTypeY".into(), None),
    ];

    Ok(result)
  }
  fn scroll_timeline(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("scrollTimeline".into(), value),
      OrderPair("scrollTimelineName".into(), None),
      OrderPair("scrollTimelineAxis".into(), None),
    ];

    Ok(result)
  }
  fn text_decoration(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("textDecoration".into(), value),
      OrderPair("textDecorationColor".into(), None),
      OrderPair("textDecorationLine".into(), None),
      OrderPair("textDecorationStyle".into(), None),
      OrderPair("textDecorationThickness".into(), None),
    ];

    Ok(result)
  }
  fn text_emphasis(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("textEmphasis".into(), value),
      OrderPair("textEmphasisColor".into(), None),
      OrderPair("textEmphasisStyle".into(), None),
    ];

    Ok(result)
  }
  fn transition(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    let result = vec![
      OrderPair("transition".into(), value),
      OrderPair("transitionBehavior".into(), None),
      OrderPair("transitionDelay".into(), None),
      OrderPair("transitionDuration".into(), None),
      OrderPair("transitionProperty".into(), None),
      OrderPair("transitionTimingFunction".into(), None),
    ];

    Ok(result)
  }

  pub fn get(name: &str) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    match name {
      "all" => Some(Shorthands::all),
      "animation" => Some(Shorthands::animation),
      "animationRange" => Some(Shorthands::animation_range),
      "background" => Some(Shorthands::background),
      "backgroundPosition" => Some(Shorthands::background_position),
      "border" => Some(Shorthands::border),
      "borderInline" => Some(Shorthands::border_inline),
      "borderBlock" => Some(Shorthands::border_block),
      "borderTop" => Some(Shorthands::border_top),
      "borderInlineEnd" => Some(Shorthands::border_inline_end),
      "borderRight" => Some(Shorthands::border_right),
      "borderBottom" => Some(Shorthands::border_bottom),
      "borderInlineStart" => Some(Shorthands::border_inline_start),
      "borderLeft" => Some(Shorthands::border_left),
      "borderInlineWidth" => Some(Shorthands::border_inline_width),
      "borderInlineStyle" => Some(Shorthands::border_inline_style),
      "borderInlineColor" => Some(Shorthands::border_inline_color),
      "borderBlockWidth" => Some(Shorthands::border_block_width),
      "borderBlockStyle" => Some(Shorthands::border_block_style),
      "borderBlockColor" => Some(Shorthands::border_block_color),
      "borderColor" => Some(Shorthands::border_color),
      "borderStyle" => Some(Shorthands::border_style),
      "borderWidth" => Some(Shorthands::border_width),
      "borderInlineStartColor" => Some(Shorthands::border_inline_start_color),
      "borderInlineEndColor" => Some(Shorthands::border_inline_end_color),
      "borderInlineStartStyle" => Some(Shorthands::border_inline_start_style),
      "borderInlineEndStyle" => Some(Shorthands::border_inline_end_style),
      "borderInlineStartWidth" => Some(Shorthands::border_inline_start_width),
      "borderInlineEndWidth" => Some(Shorthands::border_inline_end_width),
      "borderLeftColor" => Some(Shorthands::border_left_color),
      "borderRightColor" => Some(Shorthands::border_right_color),
      "borderLeftStyle" => Some(Shorthands::border_left_style),
      "borderRightStyle" => Some(Shorthands::border_right_style),
      "borderLeftWidth" => Some(Shorthands::border_left_width),
      "borderRightWidth" => Some(Shorthands::border_right_width),
      "borderRadius" => Some(Shorthands::border_radius),
      "borderStartStartRadius" => Some(Shorthands::border_start_start_radius),
      "borderStartEndRadius" => Some(Shorthands::border_start_end_radius),
      "borderEndStartRadius" => Some(Shorthands::border_end_start_radius),
      "borderEndEndRadius" => Some(Shorthands::border_end_end_radius),
      "borderTopLeftRadius" => Some(Shorthands::border_top_left_radius),
      "borderTopRightRadius" => Some(Shorthands::border_top_right_radius),
      "borderBottomLeftRadius" => Some(Shorthands::border_bottom_left_radius),
      "borderBottomRightRadius" => Some(Shorthands::border_bottom_right_radius),
      "cornerShape" => Some(Shorthands::corner_shape),
      "cornerStartStartShape" => Some(Shorthands::corner_start_start_shape),
      "cornerStartEndShape" => Some(Shorthands::corner_start_end_shape),
      "cornerEndStartShape" => Some(Shorthands::corner_end_start_shape),
      "cornerEndEndShape" => Some(Shorthands::corner_end_end_shape),
      "cornerTopLeftShape" => Some(Shorthands::corner_top_left_shape),
      "cornerTopRightShape" => Some(Shorthands::corner_top_right_shape),
      "cornerBottomLeftShape" => Some(Shorthands::corner_bottom_left_shape),
      "cornerBottomRightShape" => Some(Shorthands::corner_bottom_right_shape),
      "borderImage" => Some(Shorthands::border_image),
      "columnRule" => Some(Shorthands::column_rule),
      "columns" => Some(Shorthands::columns),
      "container" => Some(Shorthands::container),
      "containIntrinsicSize" => Some(Shorthands::contain_intrinsic_size),
      "flex" => Some(Shorthands::flex),
      "flexFlow" => Some(Shorthands::flex_flow),
      "font" => Some(Shorthands::font),
      "fontVariant" => Some(Shorthands::font_variant),
      "gap" => Some(Shorthands::gap),
      "grid" => Some(Shorthands::grid),
      "gridArea" => Some(Shorthands::grid_area),
      "gridRow" => Some(Shorthands::grid_row),
      "gridColumn" => Some(Shorthands::grid_column),
      "gridTemplate" => Some(Shorthands::grid_template),
      "inset" => Some(Shorthands::inset),
      "insetInline" => Some(Shorthands::inset_inline),
      "insetBlock" => Some(Shorthands::inset_block),
      "insetInlineStart" => Some(Shorthands::inset_inline_start),
      "insetInlineEnd" => Some(Shorthands::inset_inline_end),
      "left" => Some(Shorthands::left),
      "right" => Some(Shorthands::right),
      "listStyle" => Some(Shorthands::list_style),
      "margin" => Some(Shorthands::margin),
      "marginInline" => Some(Shorthands::margin_inline),
      "marginBlock" => Some(Shorthands::margin_block),
      "marginInlineStart" => Some(Shorthands::margin_inline_start),
      "marginInlineEnd" => Some(Shorthands::margin_inline_end),
      "marginLeft" => Some(Shorthands::margin_left),
      "marginRight" => Some(Shorthands::margin_right),
      "mask" => Some(Shorthands::mask),
      "maskBorder" => Some(Shorthands::mask_border),
      "offset" => Some(Shorthands::offset),
      "outline" => Some(Shorthands::outline),
      "overflow" => Some(Shorthands::overflow),
      "padding" => Some(Shorthands::padding),
      "paddingInline" => Some(Shorthands::padding_inline),
      "paddingBlock" => Some(Shorthands::padding_block),
      "paddingInlineStart" => Some(Shorthands::padding_inline_start),
      "paddingInlineEnd" => Some(Shorthands::padding_inline_end),
      "paddingLeft" => Some(Shorthands::padding_left),
      "paddingRight" => Some(Shorthands::padding_right),
      "placeContent" => Some(Shorthands::place_content),
      "placeItems" => Some(Shorthands::place_items),
      "placeSelf" => Some(Shorthands::place_self),
      "scrollMargin" => Some(Shorthands::scroll_margin),
      "scrollMarginBlock" => Some(Shorthands::scroll_margin_block),
      "scrollMarginInline" => Some(Shorthands::scroll_margin_inline),
      "scrollMarginInlineStart" => Some(Shorthands::scroll_margin_inline_start),
      "scrollMarginInlineEnd" => Some(Shorthands::scroll_margin_inline_end),
      "scrollMarginLeft" => Some(Shorthands::scroll_margin_left),
      "scrollMarginRight" => Some(Shorthands::scroll_margin_right),
      "scrollPadding" => Some(Shorthands::scroll_padding),
      "scrollPaddingBlock" => Some(Shorthands::scroll_padding_block),
      "scrollPaddingInline" => Some(Shorthands::scroll_padding_inline),
      "scrollPaddingInlineStart" => Some(Shorthands::scroll_padding_inline_start),
      "scrollPaddingInlineEnd" => Some(Shorthands::scroll_padding_inline_end),
      "scrollPaddingLeft" => Some(Shorthands::scroll_padding_left),
      "scrollPaddingRight" => Some(Shorthands::scroll_padding_right),
      "scrollSnapType" => Some(Shorthands::scroll_snap_type),
      "scrollTimeline" => Some(Shorthands::scroll_timeline),
      "textDecoration" => Some(Shorthands::text_decoration),
      "textEmphasis" => Some(Shorthands::text_emphasis),
      "transition" => Some(Shorthands::transition),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Aliases;

impl Aliases {
  fn height(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("height".into(), value)])
  }

  fn width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("width".into(), value)])
  }

  fn min_height(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("minHeight".into(), value)])
  }

  fn min_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("minWidth".into(), value)])
  }

  fn max_height(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("maxHeight".into(), value)])
  }

  fn max_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("maxWidth".into(), value)])
  }

  fn border_top_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopColor".into(), value)])
  }

  fn border_bottom_color(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomColor".into(), value)])
  }

  fn border_top_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopStyle".into(), value)])
  }

  fn border_bottom_style(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomStyle".into(), value)])
  }

  fn border_top_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderTopWidth".into(), value)])
  }

  fn border_bottom_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderBottomWidth".into(), value)])
  }

  fn border_start_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderStartStartRadius".into(), value)])
  }

  fn border_start_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderStartEndRadius".into(), value)])
  }

  fn border_end_start_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderEndStartRadius".into(), value)])
  }

  fn border_end_end_radius(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("borderEndEndRadius".into(), value)])
  }

  fn contain_intrinsic_height(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("containIntrinsicHeight".into(), value)])
  }

  fn contain_intrinsic_width(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("containIntrinsicWidth".into(), value)])
  }

  fn row_gap(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("rowGap".into(), value)])
  }

  fn column_gap(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("columnGap".into(), value)])
  }

  fn margin_top(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginTop".into(), value)])
  }

  fn margin_bottom(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("marginBottom".into(), value)])
  }

  fn overflow_y(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("overflowY".into(), value)])
  }

  fn overflow_x(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("overflowX".into(), value)])
  }

  fn padding_top(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingTop".into(), value)])
  }

  fn padding_bottom(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("paddingBottom".into(), value)])
  }

  fn scroll_margin_top(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("scrollMarginTop".into(), value)])
  }

  fn scroll_margin_bottom(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("scrollMarginBottom".into(), value)])
  }

  fn top(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("top".into(), value)])
  }

  fn bottom(value: Option<String>) -> Result<Vec<OrderPair>, String> {
    Ok(vec![OrderPair("bottom".into(), value)])
  }

  pub fn get(name: &str) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    match name {
      // @Deprecated
      "borderHorizontal" => Shorthands::get("borderInline"),
      // @Deprecated
      "borderVertical" => Shorthands::get("borderBlock"),
      // @Deprecated
      "borderBlockStart" => Shorthands::get("borderTop"),
      // @Deprecated
      "borderEnd" => Shorthands::get("borderInlineEnd"),
      // @Deprecated
      "borderBlockEnd" => Shorthands::get("borderBottom"),
      // @Deprecated
      "borderStart" => Shorthands::get("borderInlineStart"),

      "blockSize" => Some(Aliases::height),
      "inlineSize" => Some(Aliases::width),
      "minBlockSize" => Some(Aliases::min_height),
      "minInlineSize" => Some(Aliases::min_width),
      "maxBlockSize" => Some(Aliases::max_height),
      "maxInlineSize" => Some(Aliases::max_width),

      "borderHorizontalWidth" => Shorthands::get("borderInlineWidth"),
      "borderHorizontalStyle" => Shorthands::get("borderInlineStyle"),
      "borderHorizontalColor" => Shorthands::get("borderInlineColor"),
      "borderVerticalWidth" => Shorthands::get("borderBlockWidth"),
      "borderVerticalStyle" => Shorthands::get("borderBlockStyle"),
      "borderVerticalColor" => Shorthands::get("borderBlockColor"),

      "borderBlockStartColor" => Some(Aliases::border_top_color),
      "borderBlockEndColor" => Some(Aliases::border_bottom_color),
      "borderBlockStartStyle" => Some(Aliases::border_top_style),
      "borderBlockEndStyle" => Some(Aliases::border_bottom_style),
      "borderBlockStartWidth" => Some(Aliases::border_top_width),
      "borderBlockEndWidth" => Some(Aliases::border_bottom_width),
      "borderStartColor" => Shorthands::get("borderInlineStartColor"),
      "borderEndColor" => Shorthands::get("borderInlineEndColor"),
      "borderStartStyle" => Shorthands::get("borderInlineStartStyle"),
      "borderEndStyle" => Shorthands::get("borderInlineEndStyle"),
      "borderStartWidth" => Shorthands::get("borderInlineStartWidth"),
      "borderEndWidth" => Shorthands::get("borderInlineEndWidth"),

      "borderTopStartRadius" => Some(Aliases::border_start_start_radius),
      "borderTopEndRadius" => Some(Aliases::border_start_end_radius),
      "borderBottomStartRadius" => Some(Aliases::border_end_start_radius),
      "borderBottomEndRadius" => Some(Aliases::border_end_end_radius),

      "containIntrinsicBlockSize" => Some(Aliases::contain_intrinsic_height),
      "containIntrinsicInlineSize" => Some(Aliases::contain_intrinsic_width),

      "gridGap" => Shorthands::get("gap"),
      "gridRowGap" => Some(Aliases::row_gap),
      "gridColumnGap" => Some(Aliases::column_gap),

      "marginBlockStart" => Some(Aliases::margin_top),
      "marginBlockEnd" => Some(Aliases::margin_bottom),
      "marginStart" => Shorthands::get("marginInlineStart"),
      "marginEnd" => Shorthands::get("marginInlineEnd"),
      "marginHorizontal" => Shorthands::get("marginInline"),
      "marginVertical" => Shorthands::get("marginBlock"),

      "overflowBlock" => Some(Aliases::overflow_y),
      "overflowInline" => Some(Aliases::overflow_x),

      "paddingBlockStart" => Some(Aliases::padding_top),
      "paddingBlockEnd" => Some(Aliases::padding_bottom),
      "paddingStart" => Shorthands::get("paddingInlineStart"),
      "paddingEnd" => Shorthands::get("paddingInlineEnd"),
      "paddingHorizontal" => Shorthands::get("paddingInline"),
      "paddingVertical" => Shorthands::get("paddingBlock"),

      "scrollMarginBlockStart" => Some(Aliases::scroll_margin_top),
      "scrollMarginBlockEnd" => Some(Aliases::scroll_margin_bottom),

      "insetBlockStart" => Some(Aliases::top),
      "insetBlockEnd" => Some(Aliases::bottom),
      "start" => Shorthands::get("insetInlineStart"),
      "end" => Shorthands::get("insetInlineEnd"),
      _ => None,
    }
  }
}

#[cfg(test)]
mod coverage_tests {
  use super::Shorthands;

  #[test]
  #[should_panic(expected = "infallible shorthand returned Err")]
  fn infallible_panics_on_unexpected_error() {
    Shorthands::infallible(Err("boom".to_string()));
  }
}
