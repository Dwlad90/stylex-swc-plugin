/*!
CSS Position type parsing.

Handles position values for properties like background-position and object-position.
Supports keywords, length-percentage values, and keyword-offset combinations.
Mirrors: packages/style-value-parser/src/css-types/position.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::LengthPercentage
};
use std::fmt::{self, Display};

/// Horizontal position keywords
/// Mirrors: HorizontalKeyword type in position.js
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorizontalKeyword {
    Left,
    Center,
    Right,
}

impl HorizontalKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            HorizontalKeyword::Left => "left",
            HorizontalKeyword::Center => "center",
            HorizontalKeyword::Right => "right",
        }
    }

    pub fn from_str(s: &str) -> Option<HorizontalKeyword> {
        match s {
            "left" => Some(HorizontalKeyword::Left),
            "center" => Some(HorizontalKeyword::Center),
            "right" => Some(HorizontalKeyword::Right),
            _ => None,
        }
    }
}

impl Display for HorizontalKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Vertical position keywords
/// Mirrors: VerticalKeyword type in position.js
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerticalKeyword {
    Top,
    Center,
    Bottom,
}

impl VerticalKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerticalKeyword::Top => "top",
            VerticalKeyword::Center => "center",
            VerticalKeyword::Bottom => "bottom",
        }
    }

    pub fn from_str(s: &str) -> Option<VerticalKeyword> {
        match s {
            "top" => Some(VerticalKeyword::Top),
            "center" => Some(VerticalKeyword::Center),
            "bottom" => Some(VerticalKeyword::Bottom),
            _ => None,
        }
    }
}

impl Display for VerticalKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Horizontal position component
/// Mirrors: Horizontal type in position.js
#[derive(Debug, Clone, PartialEq)]
pub enum Horizontal {
    LengthPercentage(LengthPercentage),
    Keyword(HorizontalKeyword),
    KeywordWithOffset(HorizontalKeyword, LengthPercentage),
}

impl Display for Horizontal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Horizontal::LengthPercentage(lp) => write!(f, "{}", lp),
            Horizontal::Keyword(keyword) => write!(f, "{}", keyword),
            Horizontal::KeywordWithOffset(keyword, offset) => write!(f, "{} {}", keyword, offset),
        }
    }
}

/// Vertical position component
/// Mirrors: Vertical type in position.js
#[derive(Debug, Clone, PartialEq)]
pub enum Vertical {
    LengthPercentage(LengthPercentage),
    Keyword(VerticalKeyword),
    KeywordWithOffset(VerticalKeyword, LengthPercentage),
}

impl Display for Vertical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vertical::LengthPercentage(lp) => write!(f, "{}", lp),
            Vertical::Keyword(keyword) => write!(f, "{}", keyword),
            Vertical::KeywordWithOffset(keyword, offset) => write!(f, "{} {}", keyword, offset),
        }
    }
}

/// CSS position value
/// Mirrors: Position class in position.js
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub horizontal: Option<Horizontal>,
    pub vertical: Option<Vertical>,
}

impl Position {
    /// Create a new Position
    pub fn new(horizontal: Option<Horizontal>, vertical: Option<Vertical>) -> Self {
        Self { horizontal, vertical }
    }

    /// Parser for horizontal keywords
    pub fn horizontal_keyword_parser() -> TokenParser<HorizontalKeyword> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .where_fn(|token| {
                if let SimpleToken::Ident(value) = token {
                    HorizontalKeyword::from_str(value).is_some()
                } else {
                    false
                }
            }, Some("horizontal_keyword"))
            .map(|token| {
                if let SimpleToken::Ident(value) = token {
                    HorizontalKeyword::from_str(&value).unwrap()
                } else {
                    unreachable!()
                }
            }, Some("to_horizontal_keyword"))
    }

    /// Parser for vertical keywords
    pub fn vertical_keyword_parser() -> TokenParser<VerticalKeyword> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .where_fn(|token| {
                if let SimpleToken::Ident(value) = token {
                    VerticalKeyword::from_str(value).is_some()
                } else {
                    false
                }
            }, Some("vertical_keyword"))
            .map(|token| {
                if let SimpleToken::Ident(value) = token {
                    VerticalKeyword::from_str(&value).unwrap()
                } else {
                    unreachable!()
                }
            }, Some("to_vertical_keyword"))
    }

    /// Parser for horizontal components
    pub fn horizontal_parser() -> TokenParser<Horizontal> {
        // For now, simplified - just keywords and length-percentages
        TokenParser::one_of(vec![
            // Keyword only
            Self::horizontal_keyword_parser().map(Horizontal::Keyword, Some("keyword_only")),

            // Length-percentage only
            crate::css_types::length_percentage_parser().map(Horizontal::LengthPercentage, Some("length_percentage")),
        ])
    }

    /// Parser for vertical components
    pub fn vertical_parser() -> TokenParser<Vertical> {
        // For now, simplified - just keywords and length-percentages
        TokenParser::one_of(vec![
            // Keyword only
            Self::vertical_keyword_parser().map(Vertical::Keyword, Some("keyword_only")),

            // Length-percentage only
            crate::css_types::length_percentage_parser().map(Vertical::LengthPercentage, Some("length_percentage")),
        ])
    }

    /// Parser for position values
    /// Mirrors: Position.parser in position.js
    pub fn parser() -> TokenParser<Position> {
        // Simplified implementation - full implementation would handle:
        // - Keyword combinations
        // - Keyword with offset
        // - Multiple length-percentage values
        // - Order-independent parsing

        TokenParser::one_of(vec![
            // Horizontal only
            Self::horizontal_parser().map(|h| Position::new(Some(h), None), Some("horizontal_only")),

            // Vertical only
            Self::vertical_parser().map(|v| Position::new(None, Some(v)), Some("vertical_only")),

            // Both horizontal and vertical
            Self::horizontal_parser()
                .flat_map(|h| {
                    Self::vertical_parser().map(move |v| Position::new(Some(h.clone()), Some(v)), Some("both"))
                }, Some("horizontal_then_vertical")),
        ])
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let horizontal_str = self.horizontal.as_ref().map(|h| h.to_string());
        let vertical_str = self.vertical.as_ref().map(|v| v.to_string());

        let parts: Vec<String> = [horizontal_str, vertical_str]
            .into_iter()
            .filter_map(|s| s)
            .collect();

        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_types::{Length, Percentage};

    #[test]
    fn test_horizontal_keyword_from_str() {
        assert_eq!(HorizontalKeyword::from_str("left"), Some(HorizontalKeyword::Left));
        assert_eq!(HorizontalKeyword::from_str("center"), Some(HorizontalKeyword::Center));
        assert_eq!(HorizontalKeyword::from_str("right"), Some(HorizontalKeyword::Right));
        assert_eq!(HorizontalKeyword::from_str("invalid"), None);
    }

    #[test]
    fn test_vertical_keyword_from_str() {
        assert_eq!(VerticalKeyword::from_str("top"), Some(VerticalKeyword::Top));
        assert_eq!(VerticalKeyword::from_str("center"), Some(VerticalKeyword::Center));
        assert_eq!(VerticalKeyword::from_str("bottom"), Some(VerticalKeyword::Bottom));
        assert_eq!(VerticalKeyword::from_str("invalid"), None);
    }

    #[test]
    fn test_horizontal_keyword_display() {
        assert_eq!(HorizontalKeyword::Left.to_string(), "left");
        assert_eq!(HorizontalKeyword::Center.to_string(), "center");
        assert_eq!(HorizontalKeyword::Right.to_string(), "right");
    }

    #[test]
    fn test_vertical_keyword_display() {
        assert_eq!(VerticalKeyword::Top.to_string(), "top");
        assert_eq!(VerticalKeyword::Center.to_string(), "center");
        assert_eq!(VerticalKeyword::Bottom.to_string(), "bottom");
    }

    #[test]
    fn test_horizontal_display() {
        let keyword = Horizontal::Keyword(HorizontalKeyword::Left);
        assert_eq!(keyword.to_string(), "left");

        let length_percent = Horizontal::LengthPercentage(LengthPercentage::Percentage(Percentage::new(50.0)));
        assert_eq!(length_percent.to_string(), "50%");

        let keyword_with_offset = Horizontal::KeywordWithOffset(
            HorizontalKeyword::Left,
            LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(keyword_with_offset.to_string(), "left 10px");
    }

    #[test]
    fn test_vertical_display() {
        let keyword = Vertical::Keyword(VerticalKeyword::Top);
        assert_eq!(keyword.to_string(), "top");

        let length_percent = Vertical::LengthPercentage(LengthPercentage::Percentage(Percentage::new(25.0)));
        assert_eq!(length_percent.to_string(), "25%");

        let keyword_with_offset = Vertical::KeywordWithOffset(
            VerticalKeyword::Bottom,
            LengthPercentage::Length(Length::new(5.0, "em".to_string()))
        );
        assert_eq!(keyword_with_offset.to_string(), "bottom 5em");
    }

    #[test]
    fn test_position_creation() {
        let pos = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Center)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );

        assert!(pos.horizontal.is_some());
        assert!(pos.vertical.is_some());
    }

    #[test]
    fn test_position_display() {
        let pos1 = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Left)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );
        assert_eq!(pos1.to_string(), "left top");

        let pos2 = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Center)),
            None
        );
        assert_eq!(pos2.to_string(), "center");

        let pos3 = Position::new(
            None,
            Some(Vertical::Keyword(VerticalKeyword::Bottom))
        );
        assert_eq!(pos3.to_string(), "bottom");
    }

    #[test]
    fn test_position_parser_creation() {
        // Basic test that parsers can be created
        let _horizontal_parser = Position::horizontal_keyword_parser();
        let _vertical_parser = Position::vertical_keyword_parser();
        let _position_parser = Position::parser();
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Left)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );

        let pos2 = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Left)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );

        let pos3 = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Right)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_position_with_length_percentage() {
        let pos = Position::new(
            Some(Horizontal::LengthPercentage(LengthPercentage::Percentage(Percentage::new(50.0)))),
            Some(Vertical::LengthPercentage(LengthPercentage::Length(Length::new(100.0, "px".to_string()))))
        );

        assert_eq!(pos.to_string(), "50% 100px");
    }

    #[test]
    fn test_position_center_keyword() {
        // Test that center can be both horizontal and vertical
        let h_center = HorizontalKeyword::Center;
        let v_center = VerticalKeyword::Center;

        assert_eq!(h_center.as_str(), "center");
        assert_eq!(v_center.as_str(), "center");

        // Different enum types but same string representation
        assert_eq!(h_center.to_string(), v_center.to_string());
    }

    #[test]
    fn test_position_keyword_coverage() {
        // Test all horizontal keywords
        assert_eq!(HorizontalKeyword::Left.as_str(), "left");
        assert_eq!(HorizontalKeyword::Center.as_str(), "center");
        assert_eq!(HorizontalKeyword::Right.as_str(), "right");

        // Test all vertical keywords
        assert_eq!(VerticalKeyword::Top.as_str(), "top");
        assert_eq!(VerticalKeyword::Center.as_str(), "center");
        assert_eq!(VerticalKeyword::Bottom.as_str(), "bottom");
    }

    #[test]
    fn test_position_common_use_cases() {
        // Common background-position values
        let center_center = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Center)),
            Some(Vertical::Keyword(VerticalKeyword::Center))
        );
        assert_eq!(center_center.to_string(), "center center");

        let top_left = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Left)),
            Some(Vertical::Keyword(VerticalKeyword::Top))
        );
        assert_eq!(top_left.to_string(), "left top");

        let bottom_right = Position::new(
            Some(Horizontal::Keyword(HorizontalKeyword::Right)),
            Some(Vertical::Keyword(VerticalKeyword::Bottom))
        );
        assert_eq!(bottom_right.to_string(), "right bottom");
    }
}
