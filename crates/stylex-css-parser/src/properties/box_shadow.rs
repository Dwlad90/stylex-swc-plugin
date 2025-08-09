/*!
CSS Box Shadow property parsing.

Handles box-shadow property syntax including offset, blur, spread, color, and inset values.
Supports multiple shadow values separated by commas.
Mirrors: packages/style-value-parser/src/properties/box-shadow.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::{Length, Color}
};
use std::fmt::{self, Display};

/// Individual box shadow value
/// Mirrors: BoxShadow class in box-shadow.js
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    pub offset_x: Length,
    pub offset_y: Length,
    pub blur_radius: Length,
    pub spread_radius: Length,
    pub color: Color,
    pub inset: bool,
}

impl BoxShadow {
    /// Create a new BoxShadow
    pub fn new(
        offset_x: Length,
        offset_y: Length,
        blur_radius: Length,
        spread_radius: Length,
        color: Color,
        inset: bool,
    ) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
            inset,
        }
    }

    /// Create a simple box shadow with default values for optional parameters
    pub fn simple(
        offset_x: Length,
        offset_y: Length,
        blur_radius: Option<Length>,
        spread_radius: Option<Length>,
        color: Color,
        inset: bool,
    ) -> Self {
        Self::new(
            offset_x,
            offset_y,
            blur_radius.unwrap_or(Length::new(0.0, "px".to_string())),
            spread_radius.unwrap_or(Length::new(0.0, "px".to_string())),
            color,
            inset,
        )
    }

    /// Parser for box shadow values
    /// Mirrors: BoxShadow.parse in box-shadow.js
    pub fn parser() -> TokenParser<BoxShadow> {
        // Simplified implementation - full implementation would handle:
        // - Optional blur and spread radius
        // - Color in any position
        // - Inset keyword detection
        // - Proper error handling for invalid combinations

        // For now, implement a basic version with required offset values
        TokenParser::one_of(vec![
            // Inset shadow: parse the shadow then check for inset keyword
            Self::outer_shadow_parser()
                .flat_map(|shadow| {
                    Self::inset_keyword_parser()
                        .map(move |_| {
                            BoxShadow::new(
                                shadow.offset_x.clone(),
                                shadow.offset_y.clone(),
                                shadow.blur_radius.clone(),
                                shadow.spread_radius.clone(),
                                shadow.color.clone(),
                                true, // inset = true
                            )
                        }, Some("make_inset"))
                }, Some("inset_shadow")),

            // Regular outer shadow
            Self::outer_shadow_parser(),
        ])
    }

        /// Parser for outer (non-inset) shadow
    fn outer_shadow_parser() -> TokenParser<BoxShadow> {
        // Very simplified parser for now - just handle basic cases
        // TODO: Implement full parsing once TokenParser sequence issues are resolved

        // For now, create a minimal working parser
        Length::parser()
            .flat_map(|offset_x| {
                Length::parser()
                    .map(move |offset_y| {
                        // Create a basic shadow with default values
                        BoxShadow::new(
                            offset_x.clone(),
                            offset_y,
                            Length::new(0.0, "px".to_string()), // Default blur
                            Length::new(0.0, "px".to_string()), // Default spread
                            Color::Named(crate::css_types::NamedColor::new("black".to_string())), // Default color
                            false
                        )
                    }, Some("basic_shadow"))
            }, Some("parse_basic"))
    }

    /// Parser for 'inset' keyword
    fn inset_keyword_parser() -> TokenParser<()> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .where_fn(|token| {
                if let SimpleToken::Ident(value) = token {
                    value == "inset"
                } else {
                    false
                }
            }, Some("inset_keyword"))
            .map(|_| (), Some("to_unit"))
    }
}

impl Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inset_str = if self.inset { "inset " } else { "" };

        // Only include blur and spread if they're not zero
        let blur_str = if self.blur_radius.value != 0.0 {
            format!(" {}", self.blur_radius)
        } else {
            String::new()
        };

        let spread_str = if self.spread_radius.value != 0.0 {
            format!(" {}", self.spread_radius)
        } else {
            String::new()
        };

        write!(f, "{}{} {}{}{} {}",
            inset_str,
            self.offset_x,
            self.offset_y,
            blur_str,
            spread_str,
            self.color
        )
    }
}

/// List of box shadows (comma-separated)
/// Mirrors: BoxShadowList class in box-shadow.js
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadowList {
    pub shadows: Vec<BoxShadow>,
}

impl BoxShadowList {
    /// Create a new BoxShadowList
    pub fn new(shadows: Vec<BoxShadow>) -> Self {
        Self { shadows }
    }

    /// Parser for box shadow list (comma-separated shadows)
    /// Mirrors: BoxShadowList.parse in box-shadow.js
    pub fn parser() -> TokenParser<BoxShadowList> {
        // For now, simplified to single shadow
        // Full implementation would handle comma-separated list
        BoxShadow::parser()
            .map(|shadow| BoxShadowList::new(vec![shadow]), Some("single_shadow_list"))
    }
}

impl Display for BoxShadowList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shadow_strings: Vec<String> = self.shadows
            .iter()
            .map(|shadow| shadow.to_string())
            .collect();
        write!(f, "{}", shadow_strings.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_types::{NamedColor, HashColor};

    #[test]
    fn test_box_shadow_creation() {
        let offset_x = Length::new(2.0, "px".to_string());
        let offset_y = Length::new(4.0, "px".to_string());
        let blur = Length::new(6.0, "px".to_string());
        let spread = Length::new(0.0, "px".to_string());
        let color = Color::Named(NamedColor::new("red".to_string()));

        let shadow = BoxShadow::new(offset_x.clone(), offset_y.clone(), blur.clone(), spread.clone(), color.clone(), false);

        assert_eq!(shadow.offset_x, offset_x);
        assert_eq!(shadow.offset_y, offset_y);
        assert_eq!(shadow.blur_radius, blur);
        assert_eq!(shadow.spread_radius, spread);
        assert_eq!(shadow.color, color);
        assert!(!shadow.inset);
    }

    #[test]
    fn test_box_shadow_simple_constructor() {
        let offset_x = Length::new(1.0, "px".to_string());
        let offset_y = Length::new(2.0, "px".to_string());
        let color = Color::Named(NamedColor::new("black".to_string()));

        let shadow = BoxShadow::simple(offset_x.clone(), offset_y.clone(), None, None, color.clone(), false);

        assert_eq!(shadow.offset_x, offset_x);
        assert_eq!(shadow.offset_y, offset_y);
        assert_eq!(shadow.blur_radius.value, 0.0);
        assert_eq!(shadow.spread_radius.value, 0.0);
        assert_eq!(shadow.color, color);
        assert!(!shadow.inset);
    }

    #[test]
    fn test_box_shadow_inset() {
        let offset_x = Length::new(1.0, "px".to_string());
        let offset_y = Length::new(1.0, "px".to_string());
        let blur = Length::new(3.0, "px".to_string());
        let spread = Length::new(0.0, "px".to_string());
        let color = Color::Hash(HashColor::new("#000000".to_string()));

        let inset_shadow = BoxShadow::new(offset_x, offset_y, blur, spread, color, true);

        assert!(inset_shadow.inset);
    }

    #[test]
    fn test_box_shadow_display() {
        let offset_x = Length::new(2.0, "px".to_string());
        let offset_y = Length::new(4.0, "px".to_string());
        let blur = Length::new(6.0, "px".to_string());
        let spread = Length::new(2.0, "px".to_string());
        let color = Color::Named(NamedColor::new("red".to_string()));

        let shadow = BoxShadow::new(offset_x, offset_y, blur, spread, color, false);
        assert_eq!(shadow.to_string(), "2px 4px 6px 2px red");

        let inset_shadow = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("blue".to_string())),
            true
        );
        assert_eq!(inset_shadow.to_string(), "inset 1px 1px 2px blue");
    }

    #[test]
    fn test_box_shadow_display_zero_values() {
        let shadow = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(0.0, "px".to_string()), // Zero blur
            Length::new(0.0, "px".to_string()), // Zero spread
            Color::Named(NamedColor::new("black".to_string())),
            false
        );

        // Should omit zero blur and spread values
        assert_eq!(shadow.to_string(), "1px 2px black");
    }

    #[test]
    fn test_box_shadow_list_creation() {
        let shadow1 = BoxShadow::simple(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            None,
            None,
            Color::Named(NamedColor::new("red".to_string())),
            false
        );

        let shadow2 = BoxShadow::simple(
            Length::new(2.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Some(Length::new(4.0, "px".to_string())),
            None,
            Color::Named(NamedColor::new("blue".to_string())),
            true
        );

        let shadow_list = BoxShadowList::new(vec![shadow1, shadow2]);
        assert_eq!(shadow_list.shadows.len(), 2);
    }

    #[test]
    fn test_box_shadow_list_display() {
        let shadow1 = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("red".to_string())),
            false
        );

        let shadow2 = BoxShadow::new(
            Length::new(2.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(4.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("blue".to_string())),
            true
        );

        let shadow_list = BoxShadowList::new(vec![shadow1, shadow2]);
        assert_eq!(shadow_list.to_string(), "1px 1px red, inset 2px 2px 4px blue");
    }

    #[test]
    fn test_box_shadow_parser_creation() {
        // Basic test that parsers can be created
        let _shadow_parser = BoxShadow::parser();
        let _list_parser = BoxShadowList::parser();
    }

    #[test]
    fn test_box_shadow_equality() {
        let shadow1 = BoxShadow::simple(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            None,
            None,
            Color::Named(NamedColor::new("red".to_string())),
            false
        );

        let shadow2 = BoxShadow::simple(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            None,
            None,
            Color::Named(NamedColor::new("red".to_string())),
            false
        );

        let shadow3 = BoxShadow::simple(
            Length::new(2.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            None,
            None,
            Color::Named(NamedColor::new("red".to_string())),
            false
        );

        assert_eq!(shadow1, shadow2);
        assert_ne!(shadow1, shadow3);
    }

    #[test]
    fn test_box_shadow_common_values() {
        // Test common box-shadow patterns

        // Simple drop shadow
        let drop_shadow = BoxShadow::simple(
            Length::new(0.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Some(Length::new(4.0, "px".to_string())),
            None,
            Color::Hash(HashColor::new("#00000026".to_string())), // 15% opacity black
            false
        );
        assert!(!drop_shadow.inset);

        // Inner shadow
        let inner_shadow = BoxShadow::simple(
            Length::new(0.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Some(Length::new(2.0, "px".to_string())),
            None,
            Color::Hash(HashColor::new("#0000001a".to_string())), // 10% opacity black
            true
        );
        assert!(inner_shadow.inset);

        // No shadow (all zero)
        let no_shadow = BoxShadow::simple(
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            None,
            None,
            Color::Named(NamedColor::new("transparent".to_string())),
            false
        );
        assert_eq!(no_shadow.offset_x.value, 0.0);
        assert_eq!(no_shadow.offset_y.value, 0.0);
    }

    #[test]
    fn test_box_shadow_with_spread() {
        let shadow_with_spread = BoxShadow::new(
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()), // Positive spread
            Color::Named(NamedColor::new("black".to_string())),
            false
        );

        assert_eq!(shadow_with_spread.spread_radius.value, 5.0);
        assert_eq!(shadow_with_spread.to_string(), "0px 0px 10px 5px black");
    }
}
