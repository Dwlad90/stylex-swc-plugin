/*!
CSS Box Shadow property parsing.

Handles box-shadow property syntax including offset, blur, spread, color, and inset values.
Supports multiple shadow values separated by commas.
*/

use crate::{
  css_types::{Color, Length},
  token_parser::TokenParser,
  token_types::SimpleToken,
};
use std::fmt::{self, Display};

/// Individual box shadow value
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
  pub fn parser() -> TokenParser<BoxShadow> {
    let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

    // Parse outer shadow: offsetX offsetY [blurRadius] [spreadRadius] color
    let outer_shadow = {
      let offset_x = Length::parser();
      let offset_y = whitespace
        .clone()
        .flat_map(|_| Length::parser(), Some("offset_y"));
      let blur_radius = whitespace
        .clone()
        .flat_map(|_| Length::parser(), Some("blur_radius"))
        .optional();
      let spread_radius = whitespace
        .clone()
        .flat_map(|_| Length::parser(), Some("spread_radius"))
        .optional();
      let color = whitespace
        .clone()
        .flat_map(|_| Color::parse(), Some("color"));

      offset_x
        .flat_map(
          move |x| {
            let x_clone = x.clone();
            offset_y
              .clone()
              .map(move |y| (x_clone.clone(), y), Some("with_y"))
          },
          Some("x_step"),
        )
        .flat_map(
          move |(x, y)| {
            let x_clone = x.clone();
            let y_clone = y.clone();
            blur_radius.clone().map(
              move |blur| (x_clone.clone(), y_clone.clone(), blur),
              Some("with_blur"),
            )
          },
          Some("blur_step"),
        )
        .flat_map(
          move |(x, y, blur)| {
            let x_clone = x.clone();
            let y_clone = y.clone();
            let blur_clone = blur.clone();
            spread_radius.clone().map(
              move |spread| (x_clone.clone(), y_clone.clone(), blur_clone.clone(), spread),
              Some("with_spread"),
            )
          },
          Some("spread_step"),
        )
        .flat_map(
          move |(x, y, blur, spread)| {
            let x_clone = x.clone();
            let y_clone = y.clone();
            let blur_clone = blur.clone();
            let spread_clone = spread.clone();
            color.clone().map(
              move |color| {
                BoxShadow::new(
                  x_clone.clone(),
                  y_clone.clone(),
                  blur_clone
                    .clone()
                    .unwrap_or_else(|| Length::new(0.0, "px".to_string())),
                  spread_clone
                    .clone()
                    .unwrap_or_else(|| Length::new(0.0, "px".to_string())),
                  color,
                  false,
                )
              },
              Some("create_shadow"),
            )
          },
          Some("color_step"),
        )
    };

    let inset_shadow = {
      let inset_keyword =
        TokenParser::<SimpleToken>::token(SimpleToken::Ident("inset".to_string()), Some("Ident"))
          .where_fn(
            |token| {
              if let SimpleToken::Ident(value) = token {
                value == "inset"
              } else {
                false
              }
            },
            Some("inset_check"),
          );

      let whitespace_for_inset = whitespace.clone();
      let inset_keyword_for_inset = inset_keyword.clone();

      let shadow_then_inset = outer_shadow.clone().flat_map(
        move |shadow| {
          let shadow_clone = shadow.clone();
          let whitespace_clone = whitespace_for_inset.clone();
          let inset_clone = inset_keyword_for_inset.clone();

          whitespace_clone.flat_map(
            move |_| {
              let shadow_for_map = shadow_clone.clone();
              inset_clone.map(
                move |_| {
                  BoxShadow::new(
                    shadow_for_map.offset_x.clone(),
                    shadow_for_map.offset_y.clone(),
                    shadow_for_map.blur_radius.clone(),
                    shadow_for_map.spread_radius.clone(),
                    shadow_for_map.color.clone(),
                    true,
                  )
                },
                Some("to_inset"),
              )
            },
            Some("add_inset"),
          )
        },
        Some("shadow_then_inset"),
      );

      let whitespace_for_prefix = whitespace.clone();
      let outer_shadow_for_prefix = outer_shadow.clone();

      let inset_then_shadow = inset_keyword.flat_map(
        move |_| {
          let whitespace_clone = whitespace_for_prefix.clone();
          let shadow_parser = outer_shadow_for_prefix.clone();

          whitespace_clone.flat_map(
            move |_| {
              shadow_parser.map(
                move |shadow| {
                  BoxShadow::new(
                    shadow.offset_x.clone(),
                    shadow.offset_y.clone(),
                    shadow.blur_radius.clone(),
                    shadow.spread_radius.clone(),
                    shadow.color.clone(),
                    true,
                  )
                },
                Some("to_inset_prefix"),
              )
            },
            Some("add_inset_prefix"),
          )
        },
        Some("inset_then_shadow"),
      );

      TokenParser::one_of(vec![shadow_then_inset, inset_then_shadow])
    };

    TokenParser::one_of(vec![inset_shadow, outer_shadow])
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

    write!(
      f,
      "{}{} {}{}{} {}",
      inset_str, self.offset_x, self.offset_y, blur_str, spread_str, self.color
    )
  }
}

/// List of box shadows (comma-separated)
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
  pub fn parser() -> TokenParser<BoxShadowList> {
    let comma = TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"));
    let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

    // Parse "none" keyword
    let none_parser =
      TokenParser::<SimpleToken>::token(SimpleToken::Ident("none".to_string()), Some("none_ident"))
        .where_fn(
          |token| {
            if let SimpleToken::Ident(value) = token {
              value == "none"
            } else {
              false
            }
          },
          Some("none_check"),
        )
        .map(
          |_| BoxShadowList::new(Vec::new()),
          Some("empty_shadow_list"),
        );

    // Parse comma with optional surrounding whitespace
    let comma_separator =
      comma.surrounded_by(whitespace.clone().optional(), Some(whitespace.optional()));

    // Parse one or more shadows separated by commas
    let shadow_list_parser =
      TokenParser::one_or_more_separated_by(BoxShadow::parser(), comma_separator)
        .map(BoxShadowList::new, Some("shadow_list"));

    // Try "none" first, then shadow list
    TokenParser::one_of(vec![none_parser, shadow_list_parser])
  }
}

impl Display for BoxShadowList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.shadows.is_empty() {
      write!(f, "none")
    } else {
      let shadow_strings: Vec<String> = self
        .shadows
        .iter()
        .map(|shadow| shadow.to_string())
        .collect();
      write!(f, "{}", shadow_strings.join(", "))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::css_types::{HashColor, NamedColor};

  #[test]
  fn test_box_shadow_creation() {
    let offset_x = Length::new(2.0, "px".to_string());
    let offset_y = Length::new(4.0, "px".to_string());
    let blur = Length::new(6.0, "px".to_string());
    let spread = Length::new(0.0, "px".to_string());
    let color = Color::Named(NamedColor::new("red".to_string()));

    let shadow = BoxShadow::new(
      offset_x.clone(),
      offset_y.clone(),
      blur.clone(),
      spread.clone(),
      color.clone(),
      false,
    );

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

    let shadow = BoxShadow::simple(
      offset_x.clone(),
      offset_y.clone(),
      None,
      None,
      color.clone(),
      false,
    );

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
      true,
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
      false,
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
      false,
    );

    let shadow2 = BoxShadow::simple(
      Length::new(2.0, "px".to_string()),
      Length::new(2.0, "px".to_string()),
      Some(Length::new(4.0, "px".to_string())),
      None,
      Color::Named(NamedColor::new("blue".to_string())),
      true,
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
      false,
    );

    let shadow2 = BoxShadow::new(
      Length::new(2.0, "px".to_string()),
      Length::new(2.0, "px".to_string()),
      Length::new(4.0, "px".to_string()),
      Length::new(0.0, "px".to_string()),
      Color::Named(NamedColor::new("blue".to_string())),
      true,
    );

    let shadow_list = BoxShadowList::new(vec![shadow1, shadow2]);
    assert_eq!(
      shadow_list.to_string(),
      "1px 1px red, inset 2px 2px 4px blue"
    );
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
      false,
    );

    let shadow2 = BoxShadow::simple(
      Length::new(1.0, "px".to_string()),
      Length::new(1.0, "px".to_string()),
      None,
      None,
      Color::Named(NamedColor::new("red".to_string())),
      false,
    );

    let shadow3 = BoxShadow::simple(
      Length::new(2.0, "px".to_string()),
      Length::new(2.0, "px".to_string()),
      None,
      None,
      Color::Named(NamedColor::new("red".to_string())),
      false,
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
      false,
    );
    assert!(!drop_shadow.inset);

    // Inner shadow
    let inner_shadow = BoxShadow::simple(
      Length::new(0.0, "px".to_string()),
      Length::new(1.0, "px".to_string()),
      Some(Length::new(2.0, "px".to_string())),
      None,
      Color::Hash(HashColor::new("#0000001a".to_string())), // 10% opacity black
      true,
    );
    assert!(inner_shadow.inset);

    // No shadow (all zero)
    let no_shadow = BoxShadow::simple(
      Length::new(0.0, "px".to_string()),
      Length::new(0.0, "px".to_string()),
      None,
      None,
      Color::Named(NamedColor::new("transparent".to_string())),
      false,
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
      false,
    );

    assert_eq!(shadow_with_spread.spread_radius.value, 5.0);
    assert_eq!(shadow_with_spread.to_string(), "0px 0px 10px 5px black");
  }
}
