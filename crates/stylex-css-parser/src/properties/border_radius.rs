/*!
CSS Border Radius property parsing.

Handles border-radius property syntax including individual values and shorthand notation.
Supports both horizontal and vertical radius values with proper fallback logic.
Mirrors: packages/style-value-parser/src/properties/border-radius.js
*/

use crate::{
    token_parser::TokenParser,
    css_types::{LengthPercentage, length_percentage_parser}
};
use std::fmt::{self, Display};

/// Individual border radius value (can have different horizontal and vertical values)
/// Mirrors: BorderRadiusIndividual class in border-radius.js
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadiusIndividual {
    pub horizontal: LengthPercentage,
    pub vertical: LengthPercentage,
}

impl BorderRadiusIndividual {
    /// Create a new BorderRadiusIndividual
    pub fn new(horizontal: LengthPercentage, vertical: Option<LengthPercentage>) -> Self {
        Self {
            horizontal: horizontal.clone(),
            vertical: vertical.unwrap_or(horizontal),
        }
    }

    /// Parser for individual border radius values
    /// Mirrors: BorderRadiusIndividual.parse in border-radius.js
    pub fn parser() -> TokenParser<BorderRadiusIndividual> {
        TokenParser::one_of(vec![
            // Two values: horizontal vertical
            TokenParser::<LengthPercentage>::sequence(vec![
                length_percentage_parser(),
                length_percentage_parser(),
            ])
            .map(|values| {
                let horizontal = values[0].clone();
                let vertical = values[1].clone();
                BorderRadiusIndividual::new(horizontal, Some(vertical))
            }, Some("two_values")),

            // Single value: applies to both horizontal and vertical
            length_percentage_parser()
                .map(|lp| {
                    BorderRadiusIndividual::new(lp, None)
                }, Some("single_value")),
        ])
    }
}

impl Display for BorderRadiusIndividual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let horizontal = self.horizontal.to_string();
        let vertical = self.vertical.to_string();

        if horizontal == vertical {
            write!(f, "{}", horizontal)
        } else {
            write!(f, "{} {}", horizontal, vertical)
        }
    }
}

/// Border radius shorthand property (all four corners)
/// Mirrors: BorderRadiusShorthand class in border-radius.js
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadiusShorthand {
    // Horizontal radii
    pub horizontal_top_left: LengthPercentage,
    pub horizontal_top_right: LengthPercentage,
    pub horizontal_bottom_right: LengthPercentage,
    pub horizontal_bottom_left: LengthPercentage,

    // Vertical radii
    pub vertical_top_left: LengthPercentage,
    pub vertical_top_right: LengthPercentage,
    pub vertical_bottom_right: LengthPercentage,
    pub vertical_bottom_left: LengthPercentage,
}

impl BorderRadiusShorthand {
    /// Create a new BorderRadiusShorthand with CSS shorthand expansion logic
    /// Mirrors the constructor logic in border-radius.js
    pub fn new(
        horizontal_top_left: LengthPercentage,
        horizontal_top_right: Option<LengthPercentage>,
        horizontal_bottom_right: Option<LengthPercentage>,
        horizontal_bottom_left: Option<LengthPercentage>,
        vertical_top_left: Option<LengthPercentage>,
        vertical_top_right: Option<LengthPercentage>,
        vertical_bottom_right: Option<LengthPercentage>,
        vertical_bottom_left: Option<LengthPercentage>,
    ) -> Self {
        // CSS shorthand expansion logic
        let h_top_right = horizontal_top_right.clone().unwrap_or(horizontal_top_left.clone());
        let h_bottom_right = horizontal_bottom_right.clone().unwrap_or(horizontal_top_left.clone());
        let h_bottom_left = horizontal_bottom_left.clone().unwrap_or(h_top_right.clone());

        let v_top_left = vertical_top_left.clone().unwrap_or(horizontal_top_left.clone());
        let v_top_right = vertical_top_right.clone().unwrap_or(v_top_left.clone());
        let v_bottom_right = vertical_bottom_right.clone().unwrap_or(v_top_left.clone());
        let v_bottom_left = vertical_bottom_left.clone().unwrap_or(v_top_right.clone());

        Self {
            horizontal_top_left,
            horizontal_top_right: h_top_right,
            horizontal_bottom_right: h_bottom_right,
            horizontal_bottom_left: h_bottom_left,
            vertical_top_left: v_top_left,
            vertical_top_right: v_top_right,
            vertical_bottom_right: v_bottom_right,
            vertical_bottom_left: v_bottom_left,
        }
    }

    /// Simplified parser for border radius shorthand
    /// Mirrors: BorderRadiusShorthand.parse in border-radius.js
    pub fn parser() -> TokenParser<BorderRadiusShorthand> {
        // Simplified implementation - full implementation would handle:
        // - Space-separated radii with proper fallback logic
        // - Asymmetric borders with "/" separator
        // - Complex parsing of 1-4 values with expansion

        // For now, just handle single value
        length_percentage_parser()
            .map(|lp| {
                BorderRadiusShorthand::new(lp, None, None, None, None, None, None, None)
            }, Some("simple_shorthand"))
    }

    /// Get the shortest possible string representation
    /// Mirrors: toString() method in border-radius.js
    fn to_shortest_string(&self) -> String {
        let h_top_left = self.horizontal_top_left.to_string();
        let h_top_right = self.horizontal_top_right.to_string();
        let h_bottom_right = self.horizontal_bottom_right.to_string();
        let h_bottom_left = self.horizontal_bottom_left.to_string();

        // Determine shortest horizontal representation
        let horizontal_str = if h_top_left == h_top_right &&
                               h_top_right == h_bottom_right &&
                               h_bottom_right == h_bottom_left {
            // All four are the same
            h_top_left.clone()
        } else if h_top_left == h_bottom_right && h_top_right == h_bottom_left {
            // TopLeft === BottomRight && TopRight === BottomLeft
            format!("{} {}", h_top_left, h_top_right)
        } else if h_top_right == h_bottom_left {
            // TopRight === BottomLeft
            format!("{} {} {}", h_top_left, h_top_right, h_bottom_right)
        } else {
            // All four values needed
            format!("{} {} {} {}", h_top_left, h_top_right, h_bottom_right, h_bottom_left)
        };

        let v_top_left = self.vertical_top_left.to_string();
        let v_top_right = self.vertical_top_right.to_string();
        let v_bottom_right = self.vertical_bottom_right.to_string();
        let v_bottom_left = self.vertical_bottom_left.to_string();

        // Determine shortest vertical representation
        let vertical_str = if v_top_left == v_top_right &&
                             v_top_right == v_bottom_right &&
                             v_bottom_right == v_bottom_left {
            // All four are the same
            v_top_left.clone()
        } else if v_top_left == v_bottom_right && v_top_right == v_bottom_left {
            // TopLeft === BottomRight && TopRight === BottomLeft
            format!("{} {}", v_top_left, v_top_right)
        } else if v_top_right == v_bottom_left {
            // TopRight === BottomLeft
            format!("{} {} {}", v_top_left, v_top_right, v_bottom_right)
        } else {
            // All four values needed
            format!("{} {} {} {}", v_top_left, v_top_right, v_bottom_right, v_bottom_left)
        };

        // If horizontal and vertical are the same, just return horizontal
        if horizontal_str == vertical_str {
            horizontal_str
        } else {
            format!("{} / {}", horizontal_str, vertical_str)
        }
    }
}

impl Display for BorderRadiusShorthand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_shortest_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_types::{Length, Percentage};

    #[test]
    fn test_border_radius_individual_creation() {
        let length = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let radius = BorderRadiusIndividual::new(length.clone(), None);

        assert_eq!(radius.horizontal, length);
        assert_eq!(radius.vertical, length);
    }

    #[test]
    fn test_border_radius_individual_different_values() {
        let horizontal = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let vertical = LengthPercentage::Percentage(Percentage::new(50.0));
        let radius = BorderRadiusIndividual::new(horizontal.clone(), Some(vertical.clone()));

        assert_eq!(radius.horizontal, horizontal);
        assert_eq!(radius.vertical, vertical);
    }

    #[test]
    fn test_border_radius_individual_display() {
        let length = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let radius = BorderRadiusIndividual::new(length, None);
        assert_eq!(radius.to_string(), "5px");

        let horizontal = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let vertical = LengthPercentage::Percentage(Percentage::new(20.0));
        let radius2 = BorderRadiusIndividual::new(horizontal, Some(vertical));
        assert_eq!(radius2.to_string(), "10px 20%");
    }

    #[test]
    fn test_border_radius_shorthand_creation() {
        let value = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let shorthand = BorderRadiusShorthand::new(value.clone(), None, None, None, None, None, None, None);

        // All corners should be the same
        assert_eq!(shorthand.horizontal_top_left, value);
        assert_eq!(shorthand.horizontal_top_right, value);
        assert_eq!(shorthand.horizontal_bottom_right, value);
        assert_eq!(shorthand.horizontal_bottom_left, value);
        assert_eq!(shorthand.vertical_top_left, value);
        assert_eq!(shorthand.vertical_top_right, value);
        assert_eq!(shorthand.vertical_bottom_right, value);
        assert_eq!(shorthand.vertical_bottom_left, value);
    }

    #[test]
    fn test_border_radius_shorthand_display_single_value() {
        let value = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let shorthand = BorderRadiusShorthand::new(value, None, None, None, None, None, None, None);

        assert_eq!(shorthand.to_string(), "5px");
    }

    #[test]
    fn test_border_radius_shorthand_css_expansion() {
        let top_left = LengthPercentage::Length(Length::new(1.0, "px".to_string()));
        let top_right = LengthPercentage::Length(Length::new(2.0, "px".to_string()));

        let shorthand = BorderRadiusShorthand::new(
            top_left.clone(),
            Some(top_right.clone()),
            None, // Should default to top_left
            None, // Should default to top_right
            None, None, None, None
        );

        assert_eq!(shorthand.horizontal_top_left, top_left);
        assert_eq!(shorthand.horizontal_top_right, top_right);
        assert_eq!(shorthand.horizontal_bottom_right, top_left); // Defaults to top_left
        assert_eq!(shorthand.horizontal_bottom_left, top_right); // Defaults to top_right
    }

    #[test]
    fn test_border_radius_individual_parser_creation() {
        // Basic test that parser can be created
        let _parser = BorderRadiusIndividual::parser();
    }

    #[test]
    fn test_border_radius_shorthand_parser_creation() {
        // Basic test that parser can be created
        let _parser = BorderRadiusShorthand::parser();
    }

    #[test]
    fn test_border_radius_equality() {
        let value1 = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let value2 = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let value3 = LengthPercentage::Length(Length::new(10.0, "px".to_string()));

        let radius1 = BorderRadiusIndividual::new(value1.clone(), None);
        let radius2 = BorderRadiusIndividual::new(value2, None);
        let radius3 = BorderRadiusIndividual::new(value3, None);

        assert_eq!(radius1, radius2);
        assert_ne!(radius1, radius3);
    }

    #[test]
    fn test_border_radius_common_css_values() {
        // Test common border-radius values
        let small = LengthPercentage::Length(Length::new(3.0, "px".to_string()));
        let medium = LengthPercentage::Length(Length::new(6.0, "px".to_string()));
        let large = LengthPercentage::Length(Length::new(12.0, "px".to_string()));
        let circle = LengthPercentage::Percentage(Percentage::new(50.0));

        let small_radius = BorderRadiusIndividual::new(small, None);
        assert_eq!(small_radius.to_string(), "3px");

        let medium_radius = BorderRadiusIndividual::new(medium, None);
        assert_eq!(medium_radius.to_string(), "6px");

        let large_radius = BorderRadiusIndividual::new(large, None);
        assert_eq!(large_radius.to_string(), "12px");

        let circle_radius = BorderRadiusIndividual::new(circle, None);
        assert_eq!(circle_radius.to_string(), "50%");
    }

    #[test]
    fn test_border_radius_elliptical() {
        // Test elliptical border radius (different horizontal and vertical)
        let horizontal = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
        let vertical = LengthPercentage::Length(Length::new(10.0, "px".to_string()));

        let elliptical = BorderRadiusIndividual::new(horizontal, Some(vertical));
        assert_eq!(elliptical.to_string(), "20px 10px");
    }

    #[test]
    fn test_border_radius_mixed_units() {
        // Test mixing different units
        let pixels = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let percentage = LengthPercentage::Percentage(Percentage::new(25.0));

        let mixed = BorderRadiusIndividual::new(pixels, Some(percentage));
        assert_eq!(mixed.to_string(), "5px 25%");
    }
}
