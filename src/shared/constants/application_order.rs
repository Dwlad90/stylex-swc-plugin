use crate::shared::structures::{order::Order, order_pair::OrderPair};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

#[allow(dead_code)]
impl Shorthands {
    fn all(_: Option<&str>) -> Vec<OrderPair> {
        panic!("all is not supported")
    }
    fn animation(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![
            OrderPair("animation", value),
            OrderPair("animationComposition", None),
            OrderPair("animationName", None),
            OrderPair("animationDuration", None),
            OrderPair("animationTimingFunction", None),
            OrderPair("animationDelay", None),
            OrderPair("animationIterationCount", None),
            OrderPair("animationDirection", None),
            OrderPair("animationFillMode", None),
            OrderPair("animationPlayState", None),
            OrderPair("animationTimeline", None),
        ];

        result.extend(Shorthands::animation_range(None));

        result
    }

    fn animation_range(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("animationRange", value),
            OrderPair("animationRangeEnd", None),
            OrderPair("animationRangeStart", None),
        ];

        result
    }

    fn background(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![
            OrderPair("background", value),
            OrderPair("backgroundAttachment", None),
            OrderPair("backgroundClip", None),
            OrderPair("backgroundColor", None),
            OrderPair("backgroundImage", None),
            OrderPair("backgroundOrigin", None),
            OrderPair("backgroundRepeat", None),
            OrderPair("backgroundSize", None),
        ];

        result.extend(Shorthands::background_position(None));

        result
    }

    fn background_position(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("backgroundPosition", value),
            OrderPair("backgroundPositionX", None),
            OrderPair("backgroundPositionY", None),
        ];

        result
    }

    // These will be removed later, matching the properties with React Native.
    // For now, we're compiling them to the React Native properties.
    fn border(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("border", value)];

        result.extend(Shorthands::border_width(None));
        result.extend(Shorthands::border_style(None));
        result.extend(Shorthands::border_color(None));

        result
    }
    fn border_inline(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderInline", value)];

        result.extend(Shorthands::border_inline_width(None));
        result.extend(Shorthands::border_inline_style(None));
        result.extend(Shorthands::border_inline_color(None));

        result
    }
    fn border_block(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderBlock", value)];

        result.extend(Shorthands::border_block_width(None));
        result.extend(Shorthands::border_block_style(None));
        result.extend(Shorthands::border_block_color(None));

        result
    }

    fn border_top(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderTop", value),
            OrderPair("borderTopWidth", None),
            OrderPair("borderTopStyle", None),
            OrderPair("borderTopColor", None),
        ];

        result
    }
    // @Deprecated
    fn border_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderInlineEnd", value)];

        result.extend(Shorthands::border_inline_end_width(None));
        result.extend(Shorthands::border_inline_end_style(None));
        result.extend(Shorthands::border_inline_end_color(None));

        result
    }
    // @Deprecated
    fn border_right(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderRight", value)];

        result.extend(Shorthands::border_right_width(None));
        result.extend(Shorthands::border_right_style(None));
        result.extend(Shorthands::border_right_color(None));

        result
    }
    // @Deprecated
    fn border_bottom(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBottom", value),
            OrderPair("borderBottomWidth", None),
            OrderPair("borderBottomStyle", None),
            OrderPair("borderBottomColor", None),
        ];

        result
    }
    // @Deprecated
    fn border_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderInlineStart", value)];

        result.extend(Shorthands::border_inline_start_width(None));
        result.extend(Shorthands::border_inline_start_style(None));
        result.extend(Shorthands::border_inline_start_color(None));

        result
    }
    // @Deprecated
    fn border_left(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderLeft", value)];

        result.extend(Shorthands::border_left_width(None));
        result.extend(Shorthands::border_left_style(None));
        result.extend(Shorthands::border_left_color(None));

        result
    }
    fn border_inline_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineWidth", value),
            OrderPair("borderInlineStartWidth", None),
            OrderPair("borderLeftWidth", None),
            OrderPair("borderInlineEndWidth", None),
            OrderPair("borderRightWidth", None),
        ];

        result
    }
    fn border_inline_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineStyle", value),
            OrderPair("borderInlineStartStyle", None),
            OrderPair("borderLeftStyle", None),
            OrderPair("borderInlineEndStyle", None),
            OrderPair("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineColor", value),
            OrderPair("borderInlineStartColor", None),
            OrderPair("borderLeftColor", None),
            OrderPair("borderInlineEndColor", None),
            OrderPair("borderRightColor", None),
        ];

        result
    }
    fn border_block_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBlockWidth", value),
            OrderPair("borderTopWidth", None),
            OrderPair("borderBottomWidth", None),
        ];

        result
    }
    fn border_block_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBlockStyle", value),
            OrderPair("borderTopStyle", None),
            OrderPair("borderBottomStyle", None),
        ];

        result
    }
    fn border_block_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBlockColor", value),
            OrderPair("borderTopColor", None),
            OrderPair("borderBottomColor", None),
        ];

        result
    }
    fn border_color(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderColor", value)];

        result.extend(Shorthands::border_inline_color(None));
        result.extend(Shorthands::border_block_color(None));

        result
    }
    fn border_style(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderStyle", value)];

        result.extend(Shorthands::border_inline_style(None));
        result.extend(Shorthands::border_block_style(None));

        result
    }
    fn border_width(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("borderWidth", value)];

        result.extend(Shorthands::border_inline_width(None));
        result.extend(Shorthands::border_block_width(None));

        result
    }
    fn border_inline_start_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineStartColor", value),
            OrderPair("borderLeftColor", None),
            OrderPair("borderRightColor", None),
        ];

        result
    }
    fn border_inline_end_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineEndColor", value),
            OrderPair("borderLeftColor", None),
            OrderPair("borderRightColor", None),
        ];

        result
    }
    fn border_inline_start_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineStartStyle", value),
            OrderPair("borderLeftStyle", None),
            OrderPair("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_end_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineEndStyle", value),
            OrderPair("borderLeftStyle", None),
            OrderPair("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_start_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineStartWidth", value),
            OrderPair("borderLeftWidth", None),
            OrderPair("borderRightWidth", None),
        ];

        result
    }
    fn border_inline_end_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderInlineEndWidth", value),
            OrderPair("borderLeftWidth", None),
            OrderPair("borderRightWidth", None),
        ];

        result
    }
    fn border_left_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderLeftColor", value),
            OrderPair("borderInlineStartColor", None),
            OrderPair("borderInlineEndColor", None),
        ];

        result
    }
    fn border_right_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderRightColor", value),
            OrderPair("borderInlineStartColor", None),
            OrderPair("borderInlineEndColor", None),
        ];

        result
    }
    fn border_left_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderLeftStyle", value),
            OrderPair("borderInlineStartStyle", None),
            OrderPair("borderInlineEndStyle", None),
        ];

        result
    }
    fn border_right_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderRightStyle", value),
            OrderPair("borderInlineStartStyle", None),
            OrderPair("borderInlineEndStyle", None),
        ];

        result
    }
    fn border_left_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderLeftWidth", value),
            OrderPair("borderInlineStartWidth", None),
            OrderPair("borderInlineEndWidth", None),
        ];

        result
    }
    fn border_right_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderRightWidth", value),
            OrderPair("borderInlineStartWidth", None),
            OrderPair("borderInlineEndWidth", None),
        ];

        result
    }

    fn border_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderRadius", value),
            // // logical constituents
            OrderPair("borderStartStartRadius", None),
            OrderPair("borderStartEndRadius", None),
            OrderPair("borderEndStartRadius", None),
            OrderPair("borderEndEndRadius", None),
            // physical constituents
            OrderPair("borderTopLeftRadius", None),
            OrderPair("borderTopRightRadius", None),
            OrderPair("borderBottomLeftRadius", None),
            OrderPair("borderBottomRightRadius", None),
        ];

        result
    }

    fn border_start_start_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderStartStartRadius", value),
            OrderPair("borderTopLeftRadius", None),
            OrderPair("borderTopRightRadius", None),
        ];

        result
    }
    fn border_start_end_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderStartEndRadius", value),
            OrderPair("borderTopLeftRadius", None),
            OrderPair("borderTopRightRadius", None),
        ];

        result
    }
    fn border_end_start_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderEndStartRadius", value),
            OrderPair("borderBottomLeftRadius", None),
            OrderPair("borderBottomRightRadius", None),
        ];

        result
    }
    fn border_end_end_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderEndEndRadius", value),
            OrderPair("borderBottomLeftRadius", None),
            OrderPair("borderBottomRightRadius", None),
        ];

        result
    }
    fn border_top_left_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderTopLeftRadius", value),
            OrderPair("borderStartStartRadius", None),
            OrderPair("borderStartEndRadius", None),
        ];

        result
    }
    fn border_top_right_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderTopRightRadius", value),
            OrderPair("borderStartStartRadius", None),
            OrderPair("borderStartEndRadius", None),
        ];

        result
    }
    fn border_bottom_left_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBottomLeftRadius", value),
            OrderPair("borderEndStartRadius", None),
            OrderPair("borderEndEndRadius", None),
        ];

        result
    }
    fn border_bottom_right_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderBottomRightRadius", value),
            OrderPair("borderEndStartRadius", None),
            OrderPair("borderEndEndRadius", None),
        ];

        result
    }

    fn border_image(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("borderImage", value),
            OrderPair("borderImageOutset", None),
            OrderPair("borderImageRepeat", None),
            OrderPair("borderImageSlice", None),
            OrderPair("borderImageSource", None),
            OrderPair("borderImageWidth", None),
        ];

        result
    }

    fn column_rule(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("columnRule", value),
            OrderPair("columnRuleColor", None),
            OrderPair("columnRuleStyle", None),
            OrderPair("columnRuleWidth", None),
        ];

        result
    }
    fn columns(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("columns", value),
            OrderPair("columnCount", None),
            OrderPair("columnWidth", None),
        ];

        result
    }

    fn container(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("container", value),
            OrderPair("containerName", None),
            OrderPair("containerType", None),
        ];

        result
    }

    fn contain_intrinsic_size(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("containIntrinsicSize", value),
            OrderPair("containIntrinsicWidth", None),
            OrderPair("containIntrinsicHeight", None),
        ];

        result
    }

    fn flex(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("flex", value),
            OrderPair("flexGrow", None),
            OrderPair("flexShrink", None),
            OrderPair("flexBasis", None),
        ];

        result
    }
    fn flex_flow(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("flexFlow", value),
            OrderPair("flexDirection", None),
            OrderPair("flexWrap", None),
        ];

        result
    }
    // @Deprecated ?
    fn font(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![
            OrderPair("font", value),
            OrderPair("fontFamily", None),
            OrderPair("fontSize", None),
            OrderPair("fontStretch", None),
            OrderPair("fontStyle", None),
            OrderPair("fontWeight", None),
            OrderPair("lineHeight", None),
        ];

        result.extend(Shorthands::font_variant(None));

        result
    }
    fn font_variant(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("fontVariant", value),
            OrderPair("fontVariantAlternates", None),
            OrderPair("fontVariantCaps", None),
            OrderPair("fontVariantEastAsian", None),
            OrderPair("fontVariantEmoji", None),
            OrderPair("fontVariantLigatures", None),
            OrderPair("fontVariantNumeric", None),
            OrderPair("fontVariantPosition", None),
        ];

        result
    }
    fn gap(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("gap", value),
            OrderPair("rowGap", None),
            OrderPair("columnGap", None),
        ];

        result
    }
    fn grid(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![
            OrderPair("grid", value),
            OrderPair("gridAutoRows", None),
            OrderPair("gridAutoColumns", None),
            OrderPair("gridAutoFlow", None),
        ];

        result.extend(Shorthands::grid_template(None));

        result
    }
    fn grid_area(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("gridArea", value),
            OrderPair("gridRow", None),
            OrderPair("gridRowStart", None),
            OrderPair("gridRowEnd", None),
            OrderPair("gridColumn", None),
            OrderPair("gridColumnStart", None),
            OrderPair("gridColumnEnd", None),
        ];

        result
    }
    fn grid_row(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("gridRow", value),
            OrderPair("gridRowStart", None),
            OrderPair("gridRowEnd", None),
        ];

        result
    }
    fn grid_column(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("gridColumn", value),
            OrderPair("gridColumnStart", None),
            OrderPair("gridColumnEnd", None),
        ];

        result
    }
    fn grid_template(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("gridTemplate", value),
            OrderPair("gridTemplateAreas", None),
            OrderPair("gridTemplateColumns", None),
            OrderPair("gridTemplateRows", None),
        ];

        result
    }
    fn inset(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("inset", value)];

        result.extend(Shorthands::inset_inline(None));
        result.extend(Shorthands::inset_block(None));

        result
    }
    fn inset_inline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("insetInline", value),
            OrderPair("insetInlineStart", None),
            OrderPair("insetInlineEnd", None),
            OrderPair("left", None),
            OrderPair("right", None),
        ];

        result
    }
    fn inset_block(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("insetBlock", value),
            OrderPair("top", None),
            OrderPair("bottom", None),
        ];

        result
    }
    fn inset_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("insetInlineStart", value),
            OrderPair("left", None),
            OrderPair("right", None),
        ];

        result
    }
    fn inset_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("insetInlineEnd", value),
            OrderPair("left", None),
            OrderPair("right", None),
        ];

        result
    }
    fn left(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("left", value),
            OrderPair("insetInlineStart", None),
            OrderPair("insetInlineEnd", None),
        ];

        result
    }
    fn right(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("right", value),
            OrderPair("insetInlineStart", None),
            OrderPair("insetInlineEnd", None),
        ];

        result
    }

    fn list_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("listStyle", value),
            OrderPair("listStyleImage", None),
            OrderPair("listStylePosition", None),
            OrderPair("listStyleType", None),
        ];

        result
    }

    fn margin(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("margin", value)];

        result.extend(Shorthands::margin_inline(None));
        result.extend(Shorthands::margin_block(None));

        result
    }
    fn margin_inline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginInline", value),
            OrderPair("marginInlineStart", None),
            OrderPair("marginLeft", None),
            OrderPair("marginInlineEnd", None),
            OrderPair("marginRight", None),
        ];

        result
    }
    fn margin_block(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginBlock", value),
            OrderPair("marginTop", None),
            OrderPair("marginBottom", None),
        ];

        result
    }
    fn margin_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginInlineStart", value),
            OrderPair("marginLeft", None),
            OrderPair("marginRight", None),
        ];

        result
    }
    fn margin_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginInlineEnd", value),
            OrderPair("marginLeft", None),
            OrderPair("marginRight", None),
        ];

        result
    }
    fn margin_left(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginLeft", value),
            OrderPair("marginInlineStart", None),
            OrderPair("marginInlineEnd", None),
        ];

        result
    }
    fn margin_right(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("marginRight", value),
            OrderPair("marginInlineStart", None),
            OrderPair("marginInlineEnd", None),
        ];

        result
    }

    fn mask(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("mask", value),
            OrderPair("maskClip", None),
            OrderPair("maskComposite", None),
            OrderPair("maskImage", None),
            OrderPair("maskMode", None),
            OrderPair("maskOrigin", None),
            OrderPair("maskPosition", None),
            OrderPair("maskRepeat", None),
            OrderPair("maskSize", None),
        ];

        result
    }

    fn mask_border(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("maskBorder", value),
            OrderPair("maskBorderMode", None),
            OrderPair("maskBorderOutset", None),
            OrderPair("maskBorderRepeat", None),
            OrderPair("maskBorderSlice", None),
            OrderPair("maskBorderSource", None),
            OrderPair("maskBorderWidth", None),
        ];

        result
    }

    fn offset(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("offset", value),
            OrderPair("offsetAnchor", None),
            OrderPair("offsetDistance", None),
            OrderPair("offsetPath", None),
            OrderPair("offsetPosition", None),
            OrderPair("offsetRotate", None),
        ];

        result
    }

    fn outline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("outline", value),
            OrderPair("outlineColor", None),
            OrderPair("outlineOffset", None),
            OrderPair("outlineStyle", None),
            OrderPair("outlineWidth", None),
        ];

        result
    }

    fn overflow(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("overflow", value),
            OrderPair("overflowX", None),
            OrderPair("overflowY", None),
        ];

        result
    }

    fn padding(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("padding", value)];

        result.extend(Shorthands::padding_inline(None));
        result.extend(Shorthands::padding_block(None));

        result
    }

    fn padding_inline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingInline", value),
            OrderPair("paddingStart", None),
            OrderPair("paddingLeft", None),
            OrderPair("paddingEnd", None),
            OrderPair("paddingRight", None),
        ];

        result
    }
    fn padding_block(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingBlock", value),
            OrderPair("paddingTop", None),
            OrderPair("paddingBottom", None),
        ];

        result
    }
    fn padding_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingInlineStart", value),
            OrderPair("paddingLeft", None),
            OrderPair("paddingRight", None),
        ];

        result
    }
    fn padding_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingInlineEnd", value),
            OrderPair("paddingLeft", None),
            OrderPair("paddingRight", None),
        ];

        result
    }
    fn padding_left(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingLeft", value),
            OrderPair("paddingInlineStart", None),
            OrderPair("paddingInlineEnd", None),
        ];

        result
    }
    fn padding_right(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("paddingRight", value),
            OrderPair("paddingInlineStart", None),
            OrderPair("paddingInlineEnd", None),
        ];

        result
    }
    fn place_content(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("placeContent", value),
            OrderPair("alignContent", None),
            OrderPair("justifyContent", None),
        ];

        result
    }
    fn place_items(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("placeItems", value),
            OrderPair("alignItems", None),
            OrderPair("justifyItems", None),
        ];

        result
    }
    fn place_self(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("placeSelf", value),
            OrderPair("alignSelf", None),
            OrderPair("justifySelf", None),
        ];

        result
    }
    fn scroll_margin(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("scrollMargin", value)];

        result.extend(Shorthands::scroll_margin_inline(None));
        result.extend(Shorthands::scroll_margin_block(None));

        result
    }
    fn scroll_margin_block(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginBlock", value),
            OrderPair("scrollMarginTop", None),
            OrderPair("scrollMarginBottom", None),
        ];

        result
    }
    fn scroll_margin_inline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginInline", value),
            OrderPair("scrollMarginInlineStart", None),
            OrderPair("scrollMarginInlineEnd", None),
            OrderPair("scrollMarginLeft", None),
            OrderPair("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginInlineStart", value),
            OrderPair("scrollMarginLeft", None),
            OrderPair("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginInlineEnd", value),
            OrderPair("scrollMarginLeft", None),
            OrderPair("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_left(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginLeft", value),
            OrderPair("scrollMarginInlineStart", None),
            OrderPair("scrollMarginInlineEnd", None),
        ];

        result
    }
    fn scroll_margin_right(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollMarginRight", value),
            OrderPair("scrollMarginInlineStart", None),
            OrderPair("scrollMarginInlineEnd", None),
        ];

        result
    }
    fn scroll_padding(value: Option<&str>) -> Vec<OrderPair> {
        let mut result = vec![OrderPair("scrollPadding", value)];

        result.extend(Shorthands::scroll_padding_block(None));
        result.extend(Shorthands::scroll_padding_inline(None));

        result
    }
    fn scroll_padding_block(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingBlock", value),
            OrderPair("scrollPaddingTop", None),
            OrderPair("scrollPaddingBottom", None),
        ];

        result
    }
    fn scroll_padding_inline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingInline", value),
            OrderPair("scrollPaddingInlineStart", None),
            OrderPair("scrollPaddingInlineEnd", None),
            OrderPair("scrollPaddingLeft", None),
            OrderPair("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_inline_start(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingInlineStart", value),
            OrderPair("scrollPaddingLeft", None),
            OrderPair("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_inline_end(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingInlineEnd", value),
            OrderPair("scrollPaddingLeft", None),
            OrderPair("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_left(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingLeft", value),
            OrderPair("scrollPaddingInlineStart", None),
            OrderPair("scrollPaddingInlineEnd", None),
        ];

        result
    }
    fn scroll_padding_right(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollPaddingRight", value),
            OrderPair("scrollPaddingInlineStart", None),
            OrderPair("scrollPaddingInlineEnd", None),
        ];

        result
    }
    fn scroll_snap_type(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollSnapType", value),
            OrderPair("scrollSnapTypeX", None),
            OrderPair("scrollSnapTypeY", None),
        ];

        result
    }
    fn scroll_timeline(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("scrollTimeline", value),
            OrderPair("scrollTimelineName", None),
            OrderPair("scrollTimelineAxis", None),
        ];

        result
    }
    fn text_decoration(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("textDecoration", value),
            OrderPair("textDecorationColor", None),
            OrderPair("textDecorationLine", None),
            OrderPair("textDecorationStyle", None),
            OrderPair("textDecorationThickness", None),
        ];

        result
    }
    fn text_emphasis(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("textEmphasis", value),
            OrderPair("textEmphasisColor", None),
            OrderPair("textEmphasisStyle", None),
        ];

        result
    }
    fn transition(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![
            OrderPair("transition", value),
            OrderPair("transitionBehavior", None),
            OrderPair("transitionDelay", None),
            OrderPair("transitionDuration", None),
            OrderPair("transitionProperty", None),
            OrderPair("transitionTimingFunction", None),
        ];

        result
    }

    pub(crate) fn get(name: &str) -> Option<fn(Option<&str>) -> Vec<OrderPair>> {
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
    fn block_size(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("height", value)];

        result
    }

    fn height(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("height", value)];

        result
    }

    fn width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("width", value)];

        result
    }

    fn min_height(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("minHeight", value)];

        result
    }

    fn min_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("minWidth", value)];

        result
    }

    fn max_height(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("maxHeight", value)];

        result
    }

    fn max_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("maxWidth", value)];

        result
    }

    fn border_top_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderTopColor", value)];

        result
    }

    fn border_bottom_color(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderBottomColor", value)];

        result
    }

    fn border_top_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderTopStyle", value)];

        result
    }

    fn border_bottom_style(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderBottomStyle", value)];

        result
    }

    fn border_top_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderTopWidth", value)];

        result
    }

    fn border_bottom_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderBottomWidth", value)];

        result
    }

    fn border_start_start_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderStartStartRadius", value)];

        result
    }

    fn border_start_end_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderStartEndRadius", value)];

        result
    }

    fn border_end_start_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderEndStartRadius", value)];

        result
    }

    fn border_end_end_radius(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("borderEndEndRadius", value)];

        result
    }

    fn contain_intrinsic_height(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("containIntrinsicHeight", value)];

        result
    }

    fn contain_intrinsic_width(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("containIntrinsicWidth", value)];

        result
    }

    fn row_gap(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("rowGap", value)];

        result
    }

    fn column_gap(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("columnGap", value)];

        result
    }

    fn margin_top(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("marginTop", value)];

        result
    }

    fn margin_bottom(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("marginBottom", value)];

        result
    }

    fn overflow_y(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("overflowY", value)];

        result
    }

    fn overflow_x(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("overflowX", value)];

        result
    }

    fn padding_top(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("paddingTop", value)];

        result
    }

    fn padding_bottom(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("paddingBottom", value)];

        result
    }

    fn scroll_margin_top(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("scrollMarginTop", value)];

        result
    }

    fn scroll_margin_bottom(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("scrollMarginBottom", value)];

        result
    }

    fn top(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("top", value)];

        result
    }

    fn bottom(value: Option<&str>) -> Vec<OrderPair> {
        let result = vec![OrderPair("bottom", value)];

        result
    }

    pub(crate) fn get(name: &str) -> Option<fn(Option<&str>) -> Vec<OrderPair>> {
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
            _ => Option::None,
        }
    }
}
