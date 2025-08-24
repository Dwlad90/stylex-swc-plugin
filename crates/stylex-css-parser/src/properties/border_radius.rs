/*!
CSS Border Radius property parsing.

Handles border-radius property syntax including individual values and shorthand notation.
Supports both horizontal and vertical radius values with proper fallback logic.
*/

use crate::{
  css_types::{length_percentage_parser, LengthPercentage},
  token_parser::TokenParser,
  token_types::SimpleToken,
};
use std::fmt::{self, Display};

/// Individual border radius value (can have different horizontal and vertical values)
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
  pub fn parser() -> TokenParser<BorderRadiusIndividual> {
    let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

    // Use the WORKING pattern from BorderRadiusShorthand
    let first_value = length_percentage_parser();
    let second_value_optional = whitespace
      .clone()
      .flat_map(|_| length_percentage_parser(), Some("second_value"))
      .optional();

    first_value.flat_map(
      move |first| {
        second_value_optional.clone().map(
          move |second_opt| match second_opt {
            Some(second) => BorderRadiusIndividual::new(first.clone(), Some(second)),
            None => BorderRadiusIndividual::new(first.clone(), None),
          },
          Some("individual_radius"),
        )
      },
      Some("individual_parser"),
    )
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
  #[allow(clippy::too_many_arguments)]
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
    let h_top_right = horizontal_top_right
      .clone()
      .unwrap_or(horizontal_top_left.clone());
    let h_bottom_right = horizontal_bottom_right
      .clone()
      .unwrap_or(horizontal_top_left.clone());
    let h_bottom_left = horizontal_bottom_left
      .clone()
      .unwrap_or(h_top_right.clone());

    let v_top_left = vertical_top_left
      .clone()
      .unwrap_or(horizontal_top_left.clone());
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

  pub fn parser() -> TokenParser<BorderRadiusShorthand> {
    // Syntax: horizontal-radii [ / vertical-radii ]?

    let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));
    let slash = TokenParser::<SimpleToken>::token(SimpleToken::Delim('/'), Some("Slash"));

    // Helper to parse 1-4 space-separated length/percentage values
    let space_separated_radii = {
      let first_value = length_percentage_parser();
      let remaining_values = TokenParser::<LengthPercentage>::zero_or_more(
        whitespace
          .clone()
          .flat_map(|_| length_percentage_parser(), Some("next_value")),
      );

      first_value.flat_map(
        move |first| {
          let first_clone = first.clone();
          remaining_values.clone().map(
            move |rest| {
              let mut all_values = vec![first_clone.clone()];
              all_values.extend(rest);

              // Limit to 4 values and apply CSS shorthand expansion
              let values = if all_values.len() > 4 {
                all_values[..4].to_vec()
              } else {
                all_values
              };

              // Apply CSS shorthand rules
              match values.len() {
                1 => [
                  values[0].clone(),
                  values[0].clone(),
                  values[0].clone(),
                  values[0].clone(),
                ],
                2 => [
                  values[0].clone(),
                  values[1].clone(),
                  values[0].clone(),
                  values[1].clone(),
                ],
                3 => [
                  values[0].clone(),
                  values[1].clone(),
                  values[2].clone(),
                  values[1].clone(),
                ],
                4 => [
                  values[0].clone(),
                  values[1].clone(),
                  values[2].clone(),
                  values[3].clone(),
                ],
                _ => [
                  values[0].clone(),
                  values[0].clone(),
                  values[0].clone(),
                  values[0].clone(),
                ],
              }
            },
            Some("expand_radii"),
          )
        },
        Some("space_separated"),
      )
    };

    // Parse optional " / vertical-radii" part
    let slash_vertical = {
      let whitespace_before_slash = whitespace.clone().optional();
      let whitespace_after_slash = whitespace.clone().optional();
      let slash_clone = slash.clone();
      let radii_clone = space_separated_radii.clone();

      whitespace_before_slash
        .flat_map(move |_| slash_clone.clone(), Some("slash"))
        .flat_map(
          move |_| whitespace_after_slash.clone(),
          Some("ws_after_slash"),
        )
        .flat_map(move |_| radii_clone.clone(), Some("vertical_radii"))
        .optional()
    };

    // Main parser: horizontal-radii [/ vertical-radii]?
    space_separated_radii.clone().flat_map(
      move |horizontal_radii| {
        let h_radii = horizontal_radii.clone();
        slash_vertical.clone().map(
          move |vertical_opt| {
            let [h_tl, h_tr, h_br, h_bl] = h_radii.clone();

            match vertical_opt {
              Some(vertical_radii) => {
                let [v_tl, v_tr, v_br, v_bl] = vertical_radii;
                BorderRadiusShorthand::new(
                  h_tl,
                  Some(h_tr),
                  Some(h_br),
                  Some(h_bl),
                  Some(v_tl),
                  Some(v_tr),
                  Some(v_br),
                  Some(v_bl),
                )
              }
              None => {
                // Only horizontal radii provided, vertical defaults to horizontal
                BorderRadiusShorthand::new(
                  h_tl,
                  Some(h_tr),
                  Some(h_br),
                  Some(h_bl),
                  None,
                  None,
                  None,
                  None,
                )
              }
            }
          },
          Some("with_vertical"),
        )
      },
      Some("main_parser"),
    )
  }

  /// Get the shortest possible string representation
  fn to_shortest_string(&self) -> String {
    let h_top_left = self.horizontal_top_left.to_string();
    let h_top_right = self.horizontal_top_right.to_string();
    let h_bottom_right = self.horizontal_bottom_right.to_string();
    let h_bottom_left = self.horizontal_bottom_left.to_string();

    // Determine shortest horizontal representation
    let horizontal_str = if h_top_left == h_top_right
      && h_top_right == h_bottom_right
      && h_bottom_right == h_bottom_left
    {
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
      format!(
        "{} {} {} {}",
        h_top_left, h_top_right, h_bottom_right, h_bottom_left
      )
    };

    let v_top_left = self.vertical_top_left.to_string();
    let v_top_right = self.vertical_top_right.to_string();
    let v_bottom_right = self.vertical_bottom_right.to_string();
    let v_bottom_left = self.vertical_bottom_left.to_string();

    // Determine shortest vertical representation
    let vertical_str = if v_top_left == v_top_right
      && v_top_right == v_bottom_right
      && v_bottom_right == v_bottom_left
    {
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
      format!(
        "{} {} {} {}",
        v_top_left, v_top_right, v_bottom_right, v_bottom_left
      )
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
    let shorthand =
      BorderRadiusShorthand::new(value.clone(), None, None, None, None, None, None, None);

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
      None,
      None,
      None,
      None,
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

  #[test]
  fn test_border_radius_shorthand_slash_separated() {
    // Test asymmetric border radius: 10px 20px / 5px 15px
    let h_tl = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
    let h_tr = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
    let v_tl = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
    let v_tr = LengthPercentage::Length(Length::new(15.0, "px".to_string()));

    let radius = BorderRadiusShorthand::new(
      h_tl,
      Some(h_tr),
      None,
      None,
      Some(v_tl),
      Some(v_tr),
      None,
      None,
    );

    // Should output the slash-separated format when horizontal and vertical differ
    let result = radius.to_string();
    assert!(result.contains("/"));
    assert!(result.contains("10px"));
    assert!(result.contains("20px"));
    assert!(result.contains("5px"));
    assert!(result.contains("15px"));
  }

  #[test]
  fn test_border_radius_shorthand_no_slash_when_same() {
    // Test when horizontal and vertical radii are the same, no slash should appear
    let value = LengthPercentage::Length(Length::new(10.0, "px".to_string()));

    let radius = BorderRadiusShorthand::new(
      value.clone(),
      None,
      None,
      None,
      Some(value.clone()),
      None,
      None,
      None,
    );

    let result = radius.to_string();
    assert!(!result.contains("/"));
    assert_eq!(result, "10px");
  }

  #[test]
  fn test_border_radius_parser_creation() {
    // Test that both parsers can be created without issues
    let _individual = BorderRadiusIndividual::parser();
    let _shorthand = BorderRadiusShorthand::parser();
  }
}
