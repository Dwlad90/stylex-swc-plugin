use super::shorthands::Shorthands;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Aliases;

#[allow(dead_code)]
impl Aliases {
    fn block_size(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("height", value)];

        result
    }

    fn height(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("height", value)];

        result
    }

    fn width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("width", value)];

        result
    }

    fn min_height(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("minHeight", value)];

        result
    }

    fn min_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("minWidth", value)];

        result
    }

    fn max_height(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("maxHeight", value)];

        result
    }

    fn max_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("maxWidth", value)];

        result
    }

    fn border_top_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderTopColor", value)];

        result
    }

    fn border_bottom_color(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderBottomColor", value)];

        result
    }

    fn border_top_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderTopStyle", value)];

        result
    }

    fn border_bottom_style(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderBottomStyle", value)];

        result
    }

    fn border_top_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderTopWidth", value)];

        result
    }

    fn border_bottom_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderBottomWidth", value)];

        result
    }

    fn border_start_start_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderStartStartRadius", value)];

        result
    }

    fn border_start_end_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderStartEndRadius", value)];

        result
    }

    fn border_end_start_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderEndStartRadius", value)];

        result
    }

    fn border_end_end_radius(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("borderEndEndRadius", value)];

        result
    }

    fn contain_intrinsic_height(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("containIntrinsicHeight", value)];

        result
    }

    fn contain_intrinsic_width(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("containIntrinsicWidth", value)];

        result
    }

    fn row_gap(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("rowGap", value)];

        result
    }

    fn column_gap(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("columnGap", value)];

        result
    }

    fn margin_top(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("marginTop", value)];

        result
    }

    fn margin_bottom(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("marginBottom", value)];

        result
    }

    fn overflow_y(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("overflowY", value)];

        result
    }

    fn overflow_x(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("overflowX", value)];

        result
    }

    fn padding_top(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("paddingTop", value)];

        result
    }

    fn padding_bottom(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("paddingBottom", value)];

        result
    }

    fn scroll_margin_top(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("scrollMarginTop", value)];

        result
    }

    fn scroll_margin_bottom(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("scrollMarginBottom", value)];

        result
    }

    fn top(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("top", value)];

        result
    }

    fn bottom(value: Option<&str>) -> Vec<(&str, Option<&str>)> {
        let result = vec![("bottom", value)];

        result
    }


    pub(crate) fn get(name: &str) -> Option<fn(Option<&str>) -> Vec<(&str, Option<&str>)>> {
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
            _ => panic!("Unknown shorthand: {}", name),
        }
    }
}
