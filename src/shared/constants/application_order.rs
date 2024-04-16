use crate::shared::structures::order_pair::OrderPair;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

#[allow(dead_code)]
impl Shorthands {
  fn all(_: Option<String>) -> Vec<OrderPair> {
    panic!("all is not supported")
  }
  fn animation(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![
      OrderPair("animation".to_string(), value),
      OrderPair("animationComposition".to_string(), None),
      OrderPair("animationName".to_string(), None),
      OrderPair("animationDuration".to_string(), None),
      OrderPair("animationTimingFunction".to_string(), None),
      OrderPair("animationDelay".to_string(), None),
      OrderPair("animationIterationCount".to_string(), None),
      OrderPair("animationDirection".to_string(), None),
      OrderPair("animationFillMode".to_string(), None),
      OrderPair("animationPlayState".to_string(), None),
      OrderPair("animationTimeline".to_string(), None),
    ];

    result.extend(Shorthands::animation_range(None));

    result
  }

  fn animation_range(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("animationRange".to_string(), value),
      OrderPair("animationRangeEnd".to_string(), None),
      OrderPair("animationRangeStart".to_string(), None),
    ];

    result
  }

  fn background(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![
      OrderPair("background".to_string(), value),
      OrderPair("backgroundAttachment".to_string(), None),
      OrderPair("backgroundClip".to_string(), None),
      OrderPair("backgroundColor".to_string(), None),
      OrderPair("backgroundImage".to_string(), None),
      OrderPair("backgroundOrigin".to_string(), None),
      OrderPair("backgroundRepeat".to_string(), None),
      OrderPair("backgroundSize".to_string(), None),
    ];

    result.extend(Shorthands::background_position(None));

    result
  }

  fn background_position(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("backgroundPosition".to_string(), value),
      OrderPair("backgroundPositionX".to_string(), None),
      OrderPair("backgroundPositionY".to_string(), None),
    ];

    result
  }

  // These will be removed later, matching the properties with React Native.
  // For now, we're compiling them to the React Native properties.
  fn border(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("border".to_string(), value)];

    result.extend(Shorthands::border_width(None));
    result.extend(Shorthands::border_style(None));
    result.extend(Shorthands::border_color(None));

    result
  }
  fn border_inline(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderInline".to_string(), value)];

    result.extend(Shorthands::border_inline_width(None));
    result.extend(Shorthands::border_inline_style(None));
    result.extend(Shorthands::border_inline_color(None));

    result
  }
  fn border_block(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderBlock".to_string(), value)];

    result.extend(Shorthands::border_block_width(None));
    result.extend(Shorthands::border_block_style(None));
    result.extend(Shorthands::border_block_color(None));

    result
  }

  fn border_top(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTop".to_string(), value),
      OrderPair("borderTopWidth".to_string(), None),
      OrderPair("borderTopStyle".to_string(), None),
      OrderPair("borderTopColor".to_string(), None),
    ];

    result
  }
  // @Deprecated
  fn border_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderInlineEnd".to_string(), value)];

    result.extend(Shorthands::border_inline_end_width(None));
    result.extend(Shorthands::border_inline_end_style(None));
    result.extend(Shorthands::border_inline_end_color(None));

    result
  }
  // @Deprecated
  fn border_right(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderRight".to_string(), value)];

    result.extend(Shorthands::border_right_width(None));
    result.extend(Shorthands::border_right_style(None));
    result.extend(Shorthands::border_right_color(None));

    result
  }
  // @Deprecated
  fn border_bottom(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBottom".to_string(), value),
      OrderPair("borderBottomWidth".to_string(), None),
      OrderPair("borderBottomStyle".to_string(), None),
      OrderPair("borderBottomColor".to_string(), None),
    ];

    result
  }
  // @Deprecated
  fn border_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderInlineStart".to_string(), value)];

    result.extend(Shorthands::border_inline_start_width(None));
    result.extend(Shorthands::border_inline_start_style(None));
    result.extend(Shorthands::border_inline_start_color(None));

    result
  }
  // @Deprecated
  fn border_left(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderLeft".to_string(), value)];

    result.extend(Shorthands::border_left_width(None));
    result.extend(Shorthands::border_left_style(None));
    result.extend(Shorthands::border_left_color(None));

    result
  }
  fn border_inline_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineWidth".to_string(), value),
      OrderPair("borderInlineStartWidth".to_string(), None),
      OrderPair("borderLeftWidth".to_string(), None),
      OrderPair("borderInlineEndWidth".to_string(), None),
      OrderPair("borderRightWidth".to_string(), None),
    ];

    result
  }
  fn border_inline_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineStyle".to_string(), value),
      OrderPair("borderInlineStartStyle".to_string(), None),
      OrderPair("borderLeftStyle".to_string(), None),
      OrderPair("borderInlineEndStyle".to_string(), None),
      OrderPair("borderRightStyle".to_string(), None),
    ];

    result
  }
  fn border_inline_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineColor".to_string(), value),
      OrderPair("borderInlineStartColor".to_string(), None),
      OrderPair("borderLeftColor".to_string(), None),
      OrderPair("borderInlineEndColor".to_string(), None),
      OrderPair("borderRightColor".to_string(), None),
    ];

    result
  }
  fn border_block_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBlockWidth".to_string(), value),
      OrderPair("borderTopWidth".to_string(), None),
      OrderPair("borderBottomWidth".to_string(), None),
    ];

    result
  }
  fn border_block_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBlockStyle".to_string(), value),
      OrderPair("borderTopStyle".to_string(), None),
      OrderPair("borderBottomStyle".to_string(), None),
    ];

    result
  }
  fn border_block_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBlockColor".to_string(), value),
      OrderPair("borderTopColor".to_string(), None),
      OrderPair("borderBottomColor".to_string(), None),
    ];

    result
  }
  fn border_color(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderColor".to_string(), value)];

    result.extend(Shorthands::border_inline_color(None));
    result.extend(Shorthands::border_block_color(None));

    result
  }
  fn border_style(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderStyle".to_string(), value)];

    result.extend(Shorthands::border_inline_style(None));
    result.extend(Shorthands::border_block_style(None));

    result
  }
  fn border_width(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("borderWidth".to_string(), value)];

    result.extend(Shorthands::border_inline_width(None));
    result.extend(Shorthands::border_block_width(None));

    result
  }
  fn border_inline_start_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineStartColor".to_string(), value),
      OrderPair("borderLeftColor".to_string(), None),
      OrderPair("borderRightColor".to_string(), None),
    ];

    result
  }
  fn border_inline_end_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineEndColor".to_string(), value),
      OrderPair("borderLeftColor".to_string(), None),
      OrderPair("borderRightColor".to_string(), None),
    ];

    result
  }
  fn border_inline_start_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineStartStyle".to_string(), value),
      OrderPair("borderLeftStyle".to_string(), None),
      OrderPair("borderRightStyle".to_string(), None),
    ];

    result
  }
  fn border_inline_end_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineEndStyle".to_string(), value),
      OrderPair("borderLeftStyle".to_string(), None),
      OrderPair("borderRightStyle".to_string(), None),
    ];

    result
  }
  fn border_inline_start_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineStartWidth".to_string(), value),
      OrderPair("borderLeftWidth".to_string(), None),
      OrderPair("borderRightWidth".to_string(), None),
    ];

    result
  }
  fn border_inline_end_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderInlineEndWidth".to_string(), value),
      OrderPair("borderLeftWidth".to_string(), None),
      OrderPair("borderRightWidth".to_string(), None),
    ];

    result
  }
  fn border_left_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderLeftColor".to_string(), value),
      OrderPair("borderInlineStartColor".to_string(), None),
      OrderPair("borderInlineEndColor".to_string(), None),
    ];

    result
  }
  fn border_right_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderRightColor".to_string(), value),
      OrderPair("borderInlineStartColor".to_string(), None),
      OrderPair("borderInlineEndColor".to_string(), None),
    ];

    result
  }
  fn border_left_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderLeftStyle".to_string(), value),
      OrderPair("borderInlineStartStyle".to_string(), None),
      OrderPair("borderInlineEndStyle".to_string(), None),
    ];

    result
  }
  fn border_right_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderRightStyle".to_string(), value),
      OrderPair("borderInlineStartStyle".to_string(), None),
      OrderPair("borderInlineEndStyle".to_string(), None),
    ];

    result
  }
  fn border_left_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderLeftWidth".to_string(), value),
      OrderPair("borderInlineStartWidth".to_string(), None),
      OrderPair("borderInlineEndWidth".to_string(), None),
    ];

    result
  }
  fn border_right_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderRightWidth".to_string(), value),
      OrderPair("borderInlineStartWidth".to_string(), None),
      OrderPair("borderInlineEndWidth".to_string(), None),
    ];

    result
  }

  fn border_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderRadius".to_string(), value),
      // // logical constituents
      OrderPair("borderStartStartRadius".to_string(), None),
      OrderPair("borderStartEndRadius".to_string(), None),
      OrderPair("borderEndStartRadius".to_string(), None),
      OrderPair("borderEndEndRadius".to_string(), None),
      // physical constituents
      OrderPair("borderTopLeftRadius".to_string(), None),
      OrderPair("borderTopRightRadius".to_string(), None),
      OrderPair("borderBottomLeftRadius".to_string(), None),
      OrderPair("borderBottomRightRadius".to_string(), None),
    ];

    result
  }

  fn border_start_start_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderStartStartRadius".to_string(), value),
      OrderPair("borderTopLeftRadius".to_string(), None),
      OrderPair("borderTopRightRadius".to_string(), None),
    ];

    result
  }
  fn border_start_end_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderStartEndRadius".to_string(), value),
      OrderPair("borderTopLeftRadius".to_string(), None),
      OrderPair("borderTopRightRadius".to_string(), None),
    ];

    result
  }
  fn border_end_start_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderEndStartRadius".to_string(), value),
      OrderPair("borderBottomLeftRadius".to_string(), None),
      OrderPair("borderBottomRightRadius".to_string(), None),
    ];

    result
  }
  fn border_end_end_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderEndEndRadius".to_string(), value),
      OrderPair("borderBottomLeftRadius".to_string(), None),
      OrderPair("borderBottomRightRadius".to_string(), None),
    ];

    result
  }
  fn border_top_left_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTopLeftRadius".to_string(), value),
      OrderPair("borderStartStartRadius".to_string(), None),
      OrderPair("borderStartEndRadius".to_string(), None),
    ];

    result
  }
  fn border_top_right_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderTopRightRadius".to_string(), value),
      OrderPair("borderStartStartRadius".to_string(), None),
      OrderPair("borderStartEndRadius".to_string(), None),
    ];

    result
  }
  fn border_bottom_left_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBottomLeftRadius".to_string(), value),
      OrderPair("borderEndStartRadius".to_string(), None),
      OrderPair("borderEndEndRadius".to_string(), None),
    ];

    result
  }
  fn border_bottom_right_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderBottomRightRadius".to_string(), value),
      OrderPair("borderEndStartRadius".to_string(), None),
      OrderPair("borderEndEndRadius".to_string(), None),
    ];

    result
  }

  fn border_image(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("borderImage".to_string(), value),
      OrderPair("borderImageOutset".to_string(), None),
      OrderPair("borderImageRepeat".to_string(), None),
      OrderPair("borderImageSlice".to_string(), None),
      OrderPair("borderImageSource".to_string(), None),
      OrderPair("borderImageWidth".to_string(), None),
    ];

    result
  }

  fn column_rule(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("columnRule".to_string(), value),
      OrderPair("columnRuleColor".to_string(), None),
      OrderPair("columnRuleStyle".to_string(), None),
      OrderPair("columnRuleWidth".to_string(), None),
    ];

    result
  }
  fn columns(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("columns".to_string(), value),
      OrderPair("columnCount".to_string(), None),
      OrderPair("columnWidth".to_string(), None),
    ];

    result
  }

  fn container(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("container".to_string(), value),
      OrderPair("containerName".to_string(), None),
      OrderPair("containerType".to_string(), None),
    ];

    result
  }

  fn contain_intrinsic_size(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("containIntrinsicSize".to_string(), value),
      OrderPair("containIntrinsicWidth".to_string(), None),
      OrderPair("containIntrinsicHeight".to_string(), None),
    ];

    result
  }

  fn flex(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("flex".to_string(), value),
      OrderPair("flexGrow".to_string(), None),
      OrderPair("flexShrink".to_string(), None),
      OrderPair("flexBasis".to_string(), None),
    ];

    result
  }
  fn flex_flow(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("flexFlow".to_string(), value),
      OrderPair("flexDirection".to_string(), None),
      OrderPair("flexWrap".to_string(), None),
    ];

    result
  }
  // @Deprecated ?
  fn font(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![
      OrderPair("font".to_string(), value),
      OrderPair("fontFamily".to_string(), None),
      OrderPair("fontSize".to_string(), None),
      OrderPair("fontStretch".to_string(), None),
      OrderPair("fontStyle".to_string(), None),
      OrderPair("fontWeight".to_string(), None),
      OrderPair("lineHeight".to_string(), None),
    ];

    result.extend(Shorthands::font_variant(None));

    result
  }
  fn font_variant(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("fontVariant".to_string(), value),
      OrderPair("fontVariantAlternates".to_string(), None),
      OrderPair("fontVariantCaps".to_string(), None),
      OrderPair("fontVariantEastAsian".to_string(), None),
      OrderPair("fontVariantEmoji".to_string(), None),
      OrderPair("fontVariantLigatures".to_string(), None),
      OrderPair("fontVariantNumeric".to_string(), None),
      OrderPair("fontVariantPosition".to_string(), None),
    ];

    result
  }
  fn gap(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("gap".to_string(), value),
      OrderPair("rowGap".to_string(), None),
      OrderPair("columnGap".to_string(), None),
    ];

    result
  }
  fn grid(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![
      OrderPair("grid".to_string(), value),
      OrderPair("gridAutoRows".to_string(), None),
      OrderPair("gridAutoColumns".to_string(), None),
      OrderPair("gridAutoFlow".to_string(), None),
    ];

    result.extend(Shorthands::grid_template(None));

    result
  }
  fn grid_area(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("gridArea".to_string(), value),
      OrderPair("gridRow".to_string(), None),
      OrderPair("gridRowStart".to_string(), None),
      OrderPair("gridRowEnd".to_string(), None),
      OrderPair("gridColumn".to_string(), None),
      OrderPair("gridColumnStart".to_string(), None),
      OrderPair("gridColumnEnd".to_string(), None),
    ];

    result
  }
  fn grid_row(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("gridRow".to_string(), value),
      OrderPair("gridRowStart".to_string(), None),
      OrderPair("gridRowEnd".to_string(), None),
    ];

    result
  }
  fn grid_column(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("gridColumn".to_string(), value),
      OrderPair("gridColumnStart".to_string(), None),
      OrderPair("gridColumnEnd".to_string(), None),
    ];

    result
  }
  fn grid_template(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("gridTemplate".to_string(), value),
      OrderPair("gridTemplateAreas".to_string(), None),
      OrderPair("gridTemplateColumns".to_string(), None),
      OrderPair("gridTemplateRows".to_string(), None),
    ];

    result
  }
  fn inset(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("inset".to_string(), value)];

    result.extend(Shorthands::inset_inline(None));
    result.extend(Shorthands::inset_block(None));

    result
  }
  fn inset_inline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("insetInline".to_string(), value),
      OrderPair("insetInlineStart".to_string(), None),
      OrderPair("insetInlineEnd".to_string(), None),
      OrderPair("left".to_string(), None),
      OrderPair("right".to_string(), None),
    ];

    result
  }
  fn inset_block(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("insetBlock".to_string(), value),
      OrderPair("top".to_string(), None),
      OrderPair("bottom".to_string(), None),
    ];

    result
  }
  fn inset_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("insetInlineStart".to_string(), value),
      OrderPair("left".to_string(), None),
      OrderPair("right".to_string(), None),
    ];

    result
  }
  fn inset_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("insetInlineEnd".to_string(), value),
      OrderPair("left".to_string(), None),
      OrderPair("right".to_string(), None),
    ];

    result
  }
  fn left(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("left".to_string(), value),
      OrderPair("insetInlineStart".to_string(), None),
      OrderPair("insetInlineEnd".to_string(), None),
    ];

    result
  }
  fn right(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("right".to_string(), value),
      OrderPair("insetInlineStart".to_string(), None),
      OrderPair("insetInlineEnd".to_string(), None),
    ];

    result
  }

  fn list_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("listStyle".to_string(), value),
      OrderPair("listStyleImage".to_string(), None),
      OrderPair("listStylePosition".to_string(), None),
      OrderPair("listStyleType".to_string(), None),
    ];

    result
  }

  fn margin(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("margin".to_string(), value)];

    result.extend(Shorthands::margin_inline(None));
    result.extend(Shorthands::margin_block(None));

    result
  }
  fn margin_inline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginInline".to_string(), value),
      OrderPair("marginInlineStart".to_string(), None),
      OrderPair("marginLeft".to_string(), None),
      OrderPair("marginInlineEnd".to_string(), None),
      OrderPair("marginRight".to_string(), None),
    ];

    result
  }
  fn margin_block(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginBlock".to_string(), value),
      OrderPair("marginTop".to_string(), None),
      OrderPair("marginBottom".to_string(), None),
    ];

    result
  }
  fn margin_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginInlineStart".to_string(), value),
      OrderPair("marginLeft".to_string(), None),
      OrderPair("marginRight".to_string(), None),
    ];

    result
  }
  fn margin_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginInlineEnd".to_string(), value),
      OrderPair("marginLeft".to_string(), None),
      OrderPair("marginRight".to_string(), None),
    ];

    result
  }
  fn margin_left(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginLeft".to_string(), value),
      OrderPair("marginInlineStart".to_string(), None),
      OrderPair("marginInlineEnd".to_string(), None),
    ];

    result
  }
  fn margin_right(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("marginRight".to_string(), value),
      OrderPair("marginInlineStart".to_string(), None),
      OrderPair("marginInlineEnd".to_string(), None),
    ];

    result
  }

  fn mask(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("mask".to_string(), value),
      OrderPair("maskClip".to_string(), None),
      OrderPair("maskComposite".to_string(), None),
      OrderPair("maskImage".to_string(), None),
      OrderPair("maskMode".to_string(), None),
      OrderPair("maskOrigin".to_string(), None),
      OrderPair("maskPosition".to_string(), None),
      OrderPair("maskRepeat".to_string(), None),
      OrderPair("maskSize".to_string(), None),
    ];

    result
  }

  fn mask_border(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("maskBorder".to_string(), value),
      OrderPair("maskBorderMode".to_string(), None),
      OrderPair("maskBorderOutset".to_string(), None),
      OrderPair("maskBorderRepeat".to_string(), None),
      OrderPair("maskBorderSlice".to_string(), None),
      OrderPair("maskBorderSource".to_string(), None),
      OrderPair("maskBorderWidth".to_string(), None),
    ];

    result
  }

  fn offset(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("offset".to_string(), value),
      OrderPair("offsetAnchor".to_string(), None),
      OrderPair("offsetDistance".to_string(), None),
      OrderPair("offsetPath".to_string(), None),
      OrderPair("offsetPosition".to_string(), None),
      OrderPair("offsetRotate".to_string(), None),
    ];

    result
  }

  fn outline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("outline".to_string(), value),
      OrderPair("outlineColor".to_string(), None),
      OrderPair("outlineOffset".to_string(), None),
      OrderPair("outlineStyle".to_string(), None),
      OrderPair("outlineWidth".to_string(), None),
    ];

    result
  }

  fn overflow(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("overflow".to_string(), value),
      OrderPair("overflowX".to_string(), None),
      OrderPair("overflowY".to_string(), None),
    ];

    result
  }

  fn padding(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("padding".to_string(), value)];

    result.extend(Shorthands::padding_inline(None));
    result.extend(Shorthands::padding_block(None));

    result
  }

  fn padding_inline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingInline".to_string(), value),
      OrderPair("paddingStart".to_string(), None),
      OrderPair("paddingLeft".to_string(), None),
      OrderPair("paddingEnd".to_string(), None),
      OrderPair("paddingRight".to_string(), None),
    ];

    result
  }
  fn padding_block(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingBlock".to_string(), value),
      OrderPair("paddingTop".to_string(), None),
      OrderPair("paddingBottom".to_string(), None),
    ];

    result
  }
  fn padding_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingInlineStart".to_string(), value),
      OrderPair("paddingLeft".to_string(), None),
      OrderPair("paddingRight".to_string(), None),
    ];

    result
  }
  fn padding_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingInlineEnd".to_string(), value),
      OrderPair("paddingLeft".to_string(), None),
      OrderPair("paddingRight".to_string(), None),
    ];

    result
  }
  fn padding_left(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingLeft".to_string(), value),
      OrderPair("paddingInlineStart".to_string(), None),
      OrderPair("paddingInlineEnd".to_string(), None),
    ];

    result
  }
  fn padding_right(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("paddingRight".to_string(), value),
      OrderPair("paddingInlineStart".to_string(), None),
      OrderPair("paddingInlineEnd".to_string(), None),
    ];

    result
  }
  fn place_content(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("placeContent".to_string(), value),
      OrderPair("alignContent".to_string(), None),
      OrderPair("justifyContent".to_string(), None),
    ];

    result
  }
  fn place_items(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("placeItems".to_string(), value),
      OrderPair("alignItems".to_string(), None),
      OrderPair("justifyItems".to_string(), None),
    ];

    result
  }
  fn place_self(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("placeSelf".to_string(), value),
      OrderPair("alignSelf".to_string(), None),
      OrderPair("justifySelf".to_string(), None),
    ];

    result
  }
  fn scroll_margin(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("scrollMargin".to_string(), value)];

    result.extend(Shorthands::scroll_margin_inline(None));
    result.extend(Shorthands::scroll_margin_block(None));

    result
  }
  fn scroll_margin_block(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginBlock".to_string(), value),
      OrderPair("scrollMarginTop".to_string(), None),
      OrderPair("scrollMarginBottom".to_string(), None),
    ];

    result
  }
  fn scroll_margin_inline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginInline".to_string(), value),
      OrderPair("scrollMarginInlineStart".to_string(), None),
      OrderPair("scrollMarginInlineEnd".to_string(), None),
      OrderPair("scrollMarginLeft".to_string(), None),
      OrderPair("scrollMarginRight".to_string(), None),
    ];

    result
  }
  fn scroll_margin_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginInlineStart".to_string(), value),
      OrderPair("scrollMarginLeft".to_string(), None),
      OrderPair("scrollMarginRight".to_string(), None),
    ];

    result
  }
  fn scroll_margin_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginInlineEnd".to_string(), value),
      OrderPair("scrollMarginLeft".to_string(), None),
      OrderPair("scrollMarginRight".to_string(), None),
    ];

    result
  }
  fn scroll_margin_left(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginLeft".to_string(), value),
      OrderPair("scrollMarginInlineStart".to_string(), None),
      OrderPair("scrollMarginInlineEnd".to_string(), None),
    ];

    result
  }
  fn scroll_margin_right(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollMarginRight".to_string(), value),
      OrderPair("scrollMarginInlineStart".to_string(), None),
      OrderPair("scrollMarginInlineEnd".to_string(), None),
    ];

    result
  }
  fn scroll_padding(value: Option<String>) -> Vec<OrderPair> {
    let mut result = vec![OrderPair("scrollPadding".to_string(), value)];

    result.extend(Shorthands::scroll_padding_block(None));
    result.extend(Shorthands::scroll_padding_inline(None));

    result
  }
  fn scroll_padding_block(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingBlock".to_string(), value),
      OrderPair("scrollPaddingTop".to_string(), None),
      OrderPair("scrollPaddingBottom".to_string(), None),
    ];

    result
  }
  fn scroll_padding_inline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingInline".to_string(), value),
      OrderPair("scrollPaddingInlineStart".to_string(), None),
      OrderPair("scrollPaddingInlineEnd".to_string(), None),
      OrderPair("scrollPaddingLeft".to_string(), None),
      OrderPair("scrollPaddingRight".to_string(), None),
    ];

    result
  }
  fn scroll_padding_inline_start(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingInlineStart".to_string(), value),
      OrderPair("scrollPaddingLeft".to_string(), None),
      OrderPair("scrollPaddingRight".to_string(), None),
    ];

    result
  }
  fn scroll_padding_inline_end(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingInlineEnd".to_string(), value),
      OrderPair("scrollPaddingLeft".to_string(), None),
      OrderPair("scrollPaddingRight".to_string(), None),
    ];

    result
  }
  fn scroll_padding_left(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingLeft".to_string(), value),
      OrderPair("scrollPaddingInlineStart".to_string(), None),
      OrderPair("scrollPaddingInlineEnd".to_string(), None),
    ];

    result
  }
  fn scroll_padding_right(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollPaddingRight".to_string(), value),
      OrderPair("scrollPaddingInlineStart".to_string(), None),
      OrderPair("scrollPaddingInlineEnd".to_string(), None),
    ];

    result
  }
  fn scroll_snap_type(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollSnapType".to_string(), value),
      OrderPair("scrollSnapTypeX".to_string(), None),
      OrderPair("scrollSnapTypeY".to_string(), None),
    ];

    result
  }
  fn scroll_timeline(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("scrollTimeline".to_string(), value),
      OrderPair("scrollTimelineName".to_string(), None),
      OrderPair("scrollTimelineAxis".to_string(), None),
    ];

    result
  }
  fn text_decoration(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("textDecoration".to_string(), value),
      OrderPair("textDecorationColor".to_string(), None),
      OrderPair("textDecorationLine".to_string(), None),
      OrderPair("textDecorationStyle".to_string(), None),
      OrderPair("textDecorationThickness".to_string(), None),
    ];

    result
  }
  fn text_emphasis(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("textEmphasis".to_string(), value),
      OrderPair("textEmphasisColor".to_string(), None),
      OrderPair("textEmphasisStyle".to_string(), None),
    ];

    result
  }
  fn transition(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![
      OrderPair("transition".to_string(), value),
      OrderPair("transitionBehavior".to_string(), None),
      OrderPair("transitionDelay".to_string(), None),
      OrderPair("transitionDuration".to_string(), None),
      OrderPair("transitionProperty".to_string(), None),
      OrderPair("transitionTimingFunction".to_string(), None),
    ];

    result
  }

  pub(crate) fn get(name: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name.as_str() {
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
      _ => Option::None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Aliases;

#[allow(dead_code)]
impl Aliases {
  fn block_size(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("height".to_string(), value)];

    result
  }

  fn height(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("height".to_string(), value)];

    result
  }

  fn width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("width".to_string(), value)];

    result
  }

  fn min_height(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("minHeight".to_string(), value)];

    result
  }

  fn min_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("minWidth".to_string(), value)];

    result
  }

  fn max_height(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("maxHeight".to_string(), value)];

    result
  }

  fn max_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("maxWidth".to_string(), value)];

    result
  }

  fn border_top_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderTopColor".to_string(), value)];

    result
  }

  fn border_bottom_color(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderBottomColor".to_string(), value)];

    result
  }

  fn border_top_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderTopStyle".to_string(), value)];

    result
  }

  fn border_bottom_style(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderBottomStyle".to_string(), value)];

    result
  }

  fn border_top_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderTopWidth".to_string(), value)];

    result
  }

  fn border_bottom_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderBottomWidth".to_string(), value)];

    result
  }

  fn border_start_start_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderStartStartRadius".to_string(), value)];

    result
  }

  fn border_start_end_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderStartEndRadius".to_string(), value)];

    result
  }

  fn border_end_start_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderEndStartRadius".to_string(), value)];

    result
  }

  fn border_end_end_radius(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("borderEndEndRadius".to_string(), value)];

    result
  }

  fn contain_intrinsic_height(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("containIntrinsicHeight".to_string(), value)];

    result
  }

  fn contain_intrinsic_width(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("containIntrinsicWidth".to_string(), value)];

    result
  }

  fn row_gap(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("rowGap".to_string(), value)];

    result
  }

  fn column_gap(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("columnGap".to_string(), value)];

    result
  }

  fn margin_top(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("marginTop".to_string(), value)];

    result
  }

  fn margin_bottom(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("marginBottom".to_string(), value)];

    result
  }

  fn overflow_y(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("overflowY".to_string(), value)];

    result
  }

  fn overflow_x(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("overflowX".to_string(), value)];

    result
  }

  fn padding_top(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("paddingTop".to_string(), value)];

    result
  }

  fn padding_bottom(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("paddingBottom".to_string(), value)];

    result
  }

  fn scroll_margin_top(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("scrollMarginTop".to_string(), value)];

    result
  }

  fn scroll_margin_bottom(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("scrollMarginBottom".to_string(), value)];

    result
  }

  fn top(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("top".to_string(), value)];

    result
  }

  fn bottom(value: Option<String>) -> Vec<OrderPair> {
    let result = vec![OrderPair("bottom".to_string(), value)];

    result
  }

  pub(crate) fn get(name: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    match name.as_str() {
      // @Deprecated
      "borderHorizontal" => Shorthands::get("borderInline".to_string()),
      // @Deprecated
      "borderVertical" => Shorthands::get("borderBlock".to_string()),
      // @Deprecated
      "borderBlockStart" => Shorthands::get("borderTop".to_string()),
      // @Deprecated
      "borderEnd" => Shorthands::get("borderInlineEnd".to_string()),
      // @Deprecated
      "borderBlockEnd" => Shorthands::get("borderBottom".to_string()),
      // @Deprecated
      "borderStart" => Shorthands::get("borderInlineStart".to_string()),

      "blockSize" => Some(Aliases::height),
      "inlineSize" => Some(Aliases::width),
      "minBlockSize" => Some(Aliases::min_height),
      "minInlineSize" => Some(Aliases::min_width),
      "maxBlockSize" => Some(Aliases::max_height),
      "maxInlineSize" => Some(Aliases::max_width),

      "borderHorizontalWidth" => Shorthands::get("borderInlineWidth".to_string()),
      "borderHorizontalStyle" => Shorthands::get("borderInlineStyle".to_string()),
      "borderHorizontalColor" => Shorthands::get("borderInlineColor".to_string()),
      "borderVerticalWidth" => Shorthands::get("borderBlockWidth".to_string()),
      "borderVerticalStyle" => Shorthands::get("borderBlockStyle".to_string()),
      "borderVerticalColor" => Shorthands::get("borderBlockColor".to_string()),

      "borderBlockStartColor" => Some(Aliases::border_top_color),
      "borderBlockEndColor" => Some(Aliases::border_bottom_color),
      "borderBlockStartStyle" => Some(Aliases::border_top_style),
      "borderBlockEndStyle" => Some(Aliases::border_bottom_style),
      "borderBlockStartWidth" => Some(Aliases::border_top_width),
      "borderBlockEndWidth" => Some(Aliases::border_bottom_width),
      "borderStartColor" => Shorthands::get("borderInlineStartColor".to_string()),
      "borderEndColor" => Shorthands::get("borderInlineEndColor".to_string()),
      "borderStartStyle" => Shorthands::get("borderInlineStartStyle".to_string()),
      "borderEndStyle" => Shorthands::get("borderInlineEndStyle".to_string()),
      "borderStartWidth" => Shorthands::get("borderInlineStartWidth".to_string()),
      "borderEndWidth" => Shorthands::get("borderInlineEndWidth".to_string()),

      "borderTopStartRadius" => Some(Aliases::border_start_start_radius),
      "borderTopEndRadius" => Some(Aliases::border_start_end_radius),
      "borderBottomStartRadius" => Some(Aliases::border_end_start_radius),
      "borderBottomEndRadius" => Some(Aliases::border_end_end_radius),

      "containIntrinsicBlockSize" => Some(Aliases::contain_intrinsic_height),
      "containIntrinsicInlineSize" => Some(Aliases::contain_intrinsic_width),

      "gridGap" => Shorthands::get("gap".to_string()),
      "gridRowGap" => Some(Aliases::row_gap),
      "gridColumnGap" => Some(Aliases::column_gap),

      "marginBlockStart" => Some(Aliases::margin_top),
      "marginBlockEnd" => Some(Aliases::margin_bottom),
      "marginStart" => Shorthands::get("marginInlineStart".to_string()),
      "marginEnd" => Shorthands::get("marginInlineEnd".to_string()),
      "marginHorizontal" => Shorthands::get("marginInline".to_string()),
      "marginVertical" => Shorthands::get("marginBlock".to_string()),

      "overflowBlock" => Some(Aliases::overflow_y),
      "overflowInline" => Some(Aliases::overflow_x),

      "paddingBlockStart" => Some(Aliases::padding_top),
      "paddingBlockEnd" => Some(Aliases::padding_bottom),
      "paddingStart" => Shorthands::get("paddingInlineStart".to_string()),
      "paddingEnd" => Shorthands::get("paddingInlineEnd".to_string()),
      "paddingHorizontal" => Shorthands::get("paddingInline".to_string()),
      "paddingVertical" => Shorthands::get("paddingBlock".to_string()),

      "scrollMarginBlockStart" => Some(Aliases::scroll_margin_top),
      "scrollMarginBlockEnd" => Some(Aliases::scroll_margin_bottom),

      "insetBlockStart" => Some(Aliases::top),
      "insetBlockEnd" => Some(Aliases::bottom),
      "start" => Shorthands::get("insetInlineStart".to_string()),
      "end" => Shorthands::get("insetInlineEnd".to_string()),
      _ => Option::None,
    }
  }
}
