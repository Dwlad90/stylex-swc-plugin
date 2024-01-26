#[cfg(test)]
mod css_tests {
    use crate::shared::utils::css::transform_css_property_value_to_str;

    #[test]
    #[should_panic(expected = "Unexpected end of file, but expected ')'")]
    fn disallow_unclosed_style_value_functions() {
        assert_eq!(transform_css_property_value_to_str("color", "var(--foo"), "1px");
    }

    #[test]
    #[should_panic(expected = "Unprefixed custom properties")]
    fn disallow_unprefixed_custom_properties() {
        assert_eq!(transform_css_property_value_to_str("color", "var(foo)"), "1px");
    }

    #[test]
    fn allow_custom_properties() {
        assert_eq!(transform_css_property_value_to_str("color", "var(--foo)"), "var(--foo)");
        assert_eq!(
            transform_css_property_value_to_str("backgroundColor", "var(--bar)"),
            "var(--bar)"
        );
        assert_eq!(
            transform_css_property_value_to_str("transitionProperty", "opacity, margin-top"),
            "opacity,margin-top"
        );
    }
}
