#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Shorthands;

#[allow(dead_code)]
impl Shorthands {
    fn all(_: Option<&str>) -> Vec<(&str, Option<&str>)> {
        panic!("all is not supported")
    }
    fn animation(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![
            ("animation", value),
            ("animationComposition", None),
            ("animationName", None),
            ("animationDuration", None),
            ("animationTimingFunction", None),
            ("animationDelay", None),
            ("animationIterationCount", None),
            ("animationDirection", None),
            ("animationFillMode", None),
            ("animationPlayState", None),
            ("animationTimeline", None),
        ];

        result.extend(Shorthands::animation_range(None).clone());

        result
    }

    fn animation_range(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("animationRange", value),
            ("animationRangeEnd", None),
            ("animationRangeStart", None),
        ];

        result
    }

    fn background(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![
            ("background", value),
            ("backgroundAttachment", None),
            ("backgroundClip", None),
            ("backgroundColor", None),
            ("backgroundImage", None),
            ("backgroundOrigin", None),
            ("backgroundRepeat", None),
            ("backgroundSize", None),
        ];

        result.extend(Shorthands::background_position(None).clone());

        result
    }

    fn background_position(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("backgroundPosition", value),
            ("backgroundPositionX", None),
            ("backgroundPositionY", None),
        ];

        result
    }

    // These will be removed later, matching the properties with React Native.
    // For now, we're compiling them to the React Native properties.
    fn border(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("border", value)];

        result.extend(Shorthands::border_width(None).clone());
        result.extend(Shorthands::border_style(None).clone());
        result.extend(Shorthands::border_color(None).clone());

        result
    }
    fn border_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderInline", value)];

        result.extend(Shorthands::border_inline_width(None).clone());
        result.extend(Shorthands::border_inline_style(None).clone());
        result.extend(Shorthands::border_inline_color(None).clone());

        result
    }
    fn border_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderBlock", value)];

        result.extend(Shorthands::border_block_width(None).clone());
        result.extend(Shorthands::border_block_style(None).clone());
        result.extend(Shorthands::border_block_color(None).clone());

        result
    }

    fn border_top(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderTop", value),
            ("borderTopWidth", None),
            ("borderTopStyle", None),
            ("borderTopColor", None),
        ];

        result
    }
    // @Deprecated
    fn border_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderInlineEnd", value)];

        result.extend(Shorthands::border_inline_end_width(None).clone());
        result.extend(Shorthands::border_inline_end_style(None).clone());
        result.extend(Shorthands::border_inline_end_color(None).clone());

        result
    }
    // @Deprecated
    fn border_right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderRight", value)];

        result.extend(Shorthands::border_right_width(None).clone());
        result.extend(Shorthands::border_right_style(None).clone());
        result.extend(Shorthands::border_right_color(None).clone());

        result
    }
    // @Deprecated
    fn border_bottom(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBottom", value),
            ("borderBottomWidth", None),
            ("borderBottomStyle", None),
            ("borderBottomColor", None),
        ];

        result
    }
    // @Deprecated
    fn border_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderInlineStart", value)];

        result.extend(Shorthands::border_inline_start_width(None).clone());
        result.extend(Shorthands::border_inline_start_style(None).clone());
        result.extend(Shorthands::border_inline_start_color(None).clone());

        result
    }
    // @Deprecated
    fn border_left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderLeft", value)];

        result.extend(Shorthands::border_left_width(None).clone());
        result.extend(Shorthands::border_left_style(None).clone());
        result.extend(Shorthands::border_left_color(None).clone());

        result
    }
    fn border_inline_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineWidth", value),
            ("borderInlineStartWidth", None),
            ("borderLeftWidth", None),
            ("borderInlineEndWidth", None),
            ("borderRightWidth", None),
        ];

        result
    }
    fn border_inline_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineStyle", value),
            ("borderInlineStartStyle", None),
            ("borderLeftStyle", None),
            ("borderInlineEndStyle", None),
            ("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineColor", value),
            ("borderInlineStartColor", None),
            ("borderLeftColor", None),
            ("borderInlineEndColor", None),
            ("borderRightColor", None),
        ];

        result
    }
    fn border_block_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBlockWidth", value),
            ("borderTopWidth", None),
            ("borderBottomWidth", None),
        ];

        result
    }
    fn border_block_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBlockStyle", value),
            ("borderTopStyle", None),
            ("borderBottomStyle", None),
        ];

        result
    }
    fn border_block_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBlockColor", value),
            ("borderTopColor", None),
            ("borderBottomColor", None),
        ];

        result
    }
    fn border_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderColor", value)];

        result.extend(Shorthands::border_inline_color(None).clone());
        result.extend(Shorthands::border_block_color(None).clone());

        result
    }
    fn border_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderStyle", value)];

        result.extend(Shorthands::border_inline_style(None).clone());
        result.extend(Shorthands::border_block_style(None).clone());

        result
    }
    fn border_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("borderWidth", value)];

        result.extend(Shorthands::border_inline_width(None).clone());
        result.extend(Shorthands::border_block_width(None).clone());

        result
    }
    fn border_inline_start_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineStartColor", value),
            ("borderLeftColor", None),
            ("borderRightColor", None),
        ];

        result
    }
    fn border_inline_end_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineEndColor", value),
            ("borderLeftColor", None),
            ("borderRightColor", None),
        ];

        result
    }
    fn border_inline_start_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineStartStyle", value),
            ("borderLeftStyle", None),
            ("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_end_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineEndStyle", value),
            ("borderLeftStyle", None),
            ("borderRightStyle", None),
        ];

        result
    }
    fn border_inline_start_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineStartWidth", value),
            ("borderLeftWidth", None),
            ("borderRightWidth", None),
        ];

        result
    }
    fn border_inline_end_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderInlineEndWidth", value),
            ("borderLeftWidth", None),
            ("borderRightWidth", None),
        ];

        result
    }
    fn border_left_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderLeftColor", value),
            ("borderInlineStartColor", None),
            ("borderInlineEndColor", None),
        ];

        result
    }
    fn border_right_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderRightColor", value),
            ("borderInlineStartColor", None),
            ("borderInlineEndColor", None),
        ];

        result
    }
    fn border_left_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderLeftStyle", value),
            ("borderInlineStartStyle", None),
            ("borderInlineEndStyle", None),
        ];

        result
    }
    fn border_right_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderRightStyle", value),
            ("borderInlineStartStyle", None),
            ("borderInlineEndStyle", None),
        ];

        result
    }
    fn border_left_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderLeftWidth", value),
            ("borderInlineStartWidth", None),
            ("borderInlineEndWidth", None),
        ];

        result
    }
    fn border_right_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderRightWidth", value),
            ("borderInlineStartWidth", None),
            ("borderInlineEndWidth", None),
        ];

        result
    }

    fn border_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderRadius", value),
            // // logical constituents
            ("borderStartStartRadius", None),
            ("borderStartEndRadius", None),
            ("borderEndStartRadius", None),
            ("borderEndEndRadius", None),
            // physical constituents
            ("borderTopLeftRadius", None),
            ("borderTopRightRadius", None),
            ("borderBottomLeftRadius", None),
            ("borderBottomRightRadius", None),
        ];

        result
    }

    fn border_start_start_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderStartStartRadius", value),
            ("borderTopLeftRadius", None),
            ("borderTopRightRadius", None),
        ];

        result
    }
    fn border_start_end_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderStartEndRadius", value),
            ("borderTopLeftRadius", None),
            ("borderTopRightRadius", None),
        ];

        result
    }
    fn border_end_start_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderEndStartRadius", value),
            ("borderBottomLeftRadius", None),
            ("borderBottomRightRadius", None),
        ];

        result
    }
    fn border_end_end_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderEndEndRadius", value),
            ("borderBottomLeftRadius", None),
            ("borderBottomRightRadius", None),
        ];

        result
    }
    fn border_top_left_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderTopLeftRadius", value),
            ("borderStartStartRadius", None),
            ("borderStartEndRadius", None),
        ];

        result
    }
    fn border_top_right_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderTopRightRadius", value),
            ("borderStartStartRadius", None),
            ("borderStartEndRadius", None),
        ];

        result
    }
    fn border_bottom_left_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBottomLeftRadius", value),
            ("borderEndStartRadius", None),
            ("borderEndEndRadius", None),
        ];

        result
    }
    fn border_bottom_right_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderBottomRightRadius", value),
            ("borderEndStartRadius", None),
            ("borderEndEndRadius", None),
        ];

        result
    }

    fn border_image(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("borderImage", value),
            ("borderImageOutset", None),
            ("borderImageRepeat", None),
            ("borderImageSlice", None),
            ("borderImageSource", None),
            ("borderImageWidth", None),
        ];

        result
    }

    fn column_rule(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("columnRule", value),
            ("columnRuleColor", None),
            ("columnRuleStyle", None),
            ("columnRuleWidth", None),
        ];

        result
    }
    fn columns(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("columns", value),
            ("columnCount", None),
            ("columnWidth", None),
        ];

        result
    }

    fn container(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("container", value),
            ("containerName", None),
            ("containerType", None),
        ];

        result
    }

    fn contain_intrinsic_size(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("containIntrinsicSize", value),
            ("containIntrinsicWidth", None),
            ("containIntrinsicHeight", None),
        ];

        result
    }

    fn flex(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("flex", value),
            ("flexGrow", None),
            ("flexShrink", None),
            ("flexBasis", None),
        ];

        result
    }
    fn flex_flow(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("flexFlow", value),
            ("flexDirection", None),
            ("flexWrap", None),
        ];

        result
    }
    // @Deprecated ?
    fn font(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![
            ("font", value),
            ("fontFamily", None),
            ("fontSize", None),
            ("fontStretch", None),
            ("fontStyle", None),
            ("fontWeight", None),
            ("lineHeight", None),
        ];

        result.extend(Shorthands::font_variant(None).clone());

        result
    }
    fn font_variant(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("fontVariant", value),
            ("fontVariantAlternates", None),
            ("fontVariantCaps", None),
            ("fontVariantEastAsian", None),
            ("fontVariantEmoji", None),
            ("fontVariantLigatures", None),
            ("fontVariantNumeric", None),
            ("fontVariantPosition", None),
        ];

        result
    }
    fn gap(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("gap", value), ("rowGap", None), ("columnGap", None)];

        result
    }
    fn grid(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![
            ("grid", value),
            ("gridAutoRows", None),
            ("gridAutoColumns", None),
            ("gridAutoFlow", None),
        ];

        result.extend(Shorthands::grid_template(None).clone());

        result
    }
    fn grid_area(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("gridArea", value),
            ("gridRow", None),
            ("gridRowStart", None),
            ("gridRowEnd", None),
            ("gridColumn", None),
            ("gridColumnStart", None),
            ("gridColumnEnd", None),
        ];

        result
    }
    fn grid_row(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("gridRow", value),
            ("gridRowStart", None),
            ("gridRowEnd", None),
        ];

        result
    }
    fn grid_column(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("gridColumn", value),
            ("gridColumnStart", None),
            ("gridColumnEnd", None),
        ];

        result
    }
    fn grid_template(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("gridTemplate", value),
            ("gridTemplateAreas", None),
            ("gridTemplateColumns", None),
            ("gridTemplateRows", None),
        ];

        result
    }
    fn inset(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("inset", value)];

        result.extend(Shorthands::inset_inline(None).clone());
        result.extend(Shorthands::inset_block(None).clone());

        result
    }
    fn inset_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("insetInline", value),
            ("insetInlineStart", None),
            ("insetInlineEnd", None),
            ("left", None),
            ("right", None),
        ];

        result
    }
    fn inset_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("insetBlock", value), ("top", None), ("bottom", None)];

        result
    }
    fn inset_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("insetInlineStart", value), ("left", None), ("right", None)];

        result
    }
    fn inset_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("insetInlineEnd", value), ("left", None), ("right", None)];

        result
    }
    fn left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("left", value),
            ("insetInlineStart", None),
            ("insetInlineEnd", None),
        ];

        result
    }
    fn right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("right", value),
            ("insetInlineStart", None),
            ("insetInlineEnd", None),
        ];

        result
    }

    fn list_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("listStyle", value),
            ("listStyleImage", None),
            ("listStylePosition", None),
            ("listStyleType", None),
        ];

        result
    }

    fn margin(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("margin", value)];

        result.extend(Shorthands::margin_inline(None).clone());
        result.extend(Shorthands::margin_block(None).clone());

        result
    }
    fn margin_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginInline", value),
            ("marginInlineStart", None),
            ("marginLeft", None),
            ("marginInlineEnd", None),
            ("marginRight", None),
        ];

        result
    }
    fn margin_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginBlock", value),
            ("marginTop", None),
            ("marginBottom", None),
        ];

        result
    }
    fn margin_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginInlineStart", value),
            ("marginLeft", None),
            ("marginRight", None),
        ];

        result
    }
    fn margin_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginInlineEnd", value),
            ("marginLeft", None),
            ("marginRight", None),
        ];

        result
    }
    fn margin_left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginLeft", value),
            ("marginInlineStart", None),
            ("marginInlineEnd", None),
        ];

        result
    }
    fn margin_right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("marginRight", value),
            ("marginInlineStart", None),
            ("marginInlineEnd", None),
        ];

        result
    }

    fn mask(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("mask", value),
            ("maskClip", None),
            ("maskComposite", None),
            ("maskImage", None),
            ("maskMode", None),
            ("maskOrigin", None),
            ("maskPosition", None),
            ("maskRepeat", None),
            ("maskSize", None),
        ];

        result
    }

    fn mask_border(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("maskBorder", value),
            ("maskBorderMode", None),
            ("maskBorderOutset", None),
            ("maskBorderRepeat", None),
            ("maskBorderSlice", None),
            ("maskBorderSource", None),
            ("maskBorderWidth", None),
        ];

        result
    }

    fn offset(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("offset", value),
            ("offsetAnchor", None),
            ("offsetDistance", None),
            ("offsetPath", None),
            ("offsetPosition", None),
            ("offsetRotate", None),
        ];

        result
    }

    fn outline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("outline", value),
            ("outlineColor", None),
            ("outlineOffset", None),
            ("outlineStyle", None),
            ("outlineWidth", None),
        ];

        result
    }

    fn overflow(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("overflow", value),
            ("overflowX", None),
            ("overflowY", None),
        ];

        result
    }

    fn padding(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("padding", value),
            ("paddingStart", None),
            ("paddingLeft", None),
            ("paddingEnd", None),
            ("paddingRight", None),
            ("paddingTop", None),
            ("paddingBottom", None),
        ];

        result
    }

    fn padding_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingInline", value),
            ("paddingStart", None),
            ("paddingLeft", None),
            ("paddingEnd", None),
            ("paddingRight", None),
        ];

        result
    }
    fn padding_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingBlock", value),
            ("paddingTop", None),
            ("paddingBottom", None),
        ];

        result
    }
    fn padding_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingInlineStart", value),
            ("paddingLeft", None),
            ("paddingRight", None),
        ];

        result
    }
    fn padding_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingInlineEnd", value),
            ("paddingLeft", None),
            ("paddingRight", None),
        ];

        result
    }
    fn padding_left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingLeft", value),
            ("paddingInlineStart", None),
            ("paddingInlineEnd", None),
        ];

        result
    }
    fn padding_right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("paddingRight", value),
            ("paddingInlineStart", None),
            ("paddingInlineEnd", None),
        ];

        result
    }
    fn place_content(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("placeContent", value),
            ("alignContent", None),
            ("justifyContent", None),
        ];

        result
    }
    fn place_items(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("placeItems", value),
            ("alignItems", None),
            ("justifyItems", None),
        ];

        result
    }
    fn place_self(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("placeSelf", value),
            ("alignSelf", None),
            ("justifySelf", None),
        ];

        result
    }
    fn scroll_margin(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("scrollMargin", value)];

        result.extend(Shorthands::scroll_margin_inline(None).clone());
        result.extend(Shorthands::scroll_margin_block(None).clone());

        result
    }
    fn scroll_margin_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginBlock", value),
            ("scrollMarginTop", None),
            ("scrollMarginBottom", None),
        ];

        result
    }
    fn scroll_margin_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginInline", value),
            ("scrollMarginInlineStart", None),
            ("scrollMarginInlineEnd", None),
            ("scrollMarginLeft", None),
            ("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginInlineStart", value),
            ("scrollMarginLeft", None),
            ("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginInlineEnd", value),
            ("scrollMarginLeft", None),
            ("scrollMarginRight", None),
        ];

        result
    }
    fn scroll_margin_left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginLeft", value),
            ("scrollMarginInlineStart", None),
            ("scrollMarginInlineEnd", None),
        ];

        result
    }
    fn scroll_margin_right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollMarginRight", value),
            ("scrollMarginInlineStart", None),
            ("scrollMarginInlineEnd", None),
        ];

        result
    }
    fn scroll_padding(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let mut result = vec![("scrollPadding", value)];

        result.extend(Shorthands::scroll_padding_block(None).clone());
        result.extend(Shorthands::scroll_padding_inline(None).clone());

        result
    }
    fn scroll_padding_block(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingBlock", value),
            ("scrollPaddingTop", None),
            ("scrollPaddingBottom", None),
        ];

        result
    }
    fn scroll_padding_inline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingInline", value),
            ("scrollPaddingInlineStart", None),
            ("scrollPaddingInlineEnd", None),
            ("scrollPaddingLeft", None),
            ("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_inline_start(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingInlineStart", value),
            ("scrollPaddingLeft", None),
            ("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_inline_end(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingInlineEnd", value),
            ("scrollPaddingLeft", None),
            ("scrollPaddingRight", None),
        ];

        result
    }
    fn scroll_padding_left(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingLeft", value),
            ("scrollPaddingInlineStart", None),
            ("scrollPaddingInlineEnd", None),
        ];

        result
    }
    fn scroll_padding_right(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollPaddingRight", value),
            ("scrollPaddingInlineStart", None),
            ("scrollPaddingInlineEnd", None),
        ];

        result
    }
    fn scroll_snap_type(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollSnapType", value),
            ("scrollSnapTypeX", None),
            ("scrollSnapTypeY", None),
        ];

        result
    }
    fn scroll_timeline(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("scrollTimeline", value),
            ("scrollTimelineName", None),
            ("scrollTimelineAxis", None),
        ];

        result
    }
    fn text_decoration(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("textDecoration", value),
            ("textDecorationColor", None),
            ("textDecorationLine", None),
            ("textDecorationStyle", None),
            ("textDecorationThickness", None),
        ];

        result
    }
    fn text_emphasis(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("textEmphasis", value),
            ("textEmphasisColor", None),
            ("textEmphasisStyle", None),
        ];

        result
    }
    fn transition(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![
            ("transition", value),
            ("transitionBehavior", None),
            ("transitionDelay", None),
            ("transitionDuration", None),
            ("transitionProperty", None),
            ("transitionTimingFunction", None),
        ];

        result
    }

    pub(crate) fn get(name: &str) -> Option<fn(Option<&str>) -> Vec<(&str, Option<&str>)>> {
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
            _ => panic!("Unknown shorthand: {}", name),
        }
    }
}
