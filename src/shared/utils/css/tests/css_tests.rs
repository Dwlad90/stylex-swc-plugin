#[cfg(test)]
mod css_tests {
    use crate::shared::utils::css::{get_number_suffix, transform_css_property_value_to_str};

    #[test]
    fn should_transform_css_property_value() {
        assert_eq!(transform_css_property_value_to_str("padding", "1"), "1px");
    }

    #[test]
    fn should_return_correct_suffix() {
        assert_eq!(get_number_suffix("padding"), "px");
        assert_eq!(get_number_suffix("opacity"), "");
        assert_eq!(get_number_suffix("voiceDuration"), "ms");
    }
}
