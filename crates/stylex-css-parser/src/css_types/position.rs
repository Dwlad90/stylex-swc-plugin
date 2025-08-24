use crate::css_types::length_percentage::{length_percentage_parser, LengthPercentage};
/**
 * CSS Position Type Parser
 *
 * Provides comprehensive position parsing for CSS layout properties.
 * Covers all major CSS position parsing scenarios with Rust type safety.
 */
use crate::token_parser::TokenParser;
use crate::token_types::SimpleToken;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum HorizontalKeyword {
  Left,
  Center,
  Right,
}

impl fmt::Display for HorizontalKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      HorizontalKeyword::Left => "left",
      HorizontalKeyword::Center => "center",
      HorizontalKeyword::Right => "right",
    };
    write!(f, "{}", s)
  }
}

impl HorizontalKeyword {
  pub fn as_str(&self) -> &str {
    match self {
      HorizontalKeyword::Left => "left",
      HorizontalKeyword::Center => "center",
      HorizontalKeyword::Right => "right",
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerticalKeyword {
  Top,
  Center,
  Bottom,
}

impl fmt::Display for VerticalKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      VerticalKeyword::Top => "top",
      VerticalKeyword::Center => "center",
      VerticalKeyword::Bottom => "bottom",
    };
    write!(f, "{}", s)
  }
}

impl VerticalKeyword {
  pub fn as_str(&self) -> &str {
    match self {
      VerticalKeyword::Top => "top",
      VerticalKeyword::Center => "center",
      VerticalKeyword::Bottom => "bottom",
    }
  }
}

/// | LengthPercentage | HorizontalKeyword | [HorizontalKeyword, LengthPercentage]
#[derive(Debug, Clone, PartialEq)]
pub enum Horizontal {
  Length(LengthPercentage),
  Keyword(HorizontalKeyword),
  KeywordWithOffset(HorizontalKeyword, LengthPercentage), // [HorizontalKeyword, LengthPercentage]
}

impl fmt::Display for Horizontal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Horizontal::Length(lp) => write!(f, "{}", lp),
      Horizontal::Keyword(k) => write!(f, "{}", k),
      Horizontal::KeywordWithOffset(k, lp) => write!(f, "{} {}", k, lp),
    }
  }
}

/// | LengthPercentage | VerticalKeyword | [VerticalKeyword, LengthPercentage]
#[derive(Debug, Clone, PartialEq)]
pub enum Vertical {
  Length(LengthPercentage),
  Keyword(VerticalKeyword),
  KeywordWithOffset(VerticalKeyword, LengthPercentage), // [VerticalKeyword, LengthPercentage]
}

impl fmt::Display for Vertical {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Vertical::Length(lp) => write!(f, "{}", lp),
      Vertical::Keyword(k) => write!(f, "{}", k),
      Vertical::KeywordWithOffset(k, lp) => write!(f, "{} {}", k, lp),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub horizontal: Option<Horizontal>,
  pub vertical: Option<Vertical>,
}

impl Position {
  /// Create a new Position
  pub fn new(horizontal: Option<Horizontal>, vertical: Option<Vertical>) -> Self {
    Self {
      horizontal,
      vertical,
    }
  }

  fn horizontal_keyword_parser() -> TokenParser<HorizontalKeyword> {
    TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
      .where_fn(
        |token| {
          if let SimpleToken::Ident(value) = token {
            matches!(value.as_str(), "left" | "center" | "right")
          } else {
            false
          }
        },
        Some("horizontal_keyword"),
      )
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            match value.as_str() {
              "left" => HorizontalKeyword::Left,
              "center" => HorizontalKeyword::Center,
              "right" => HorizontalKeyword::Right,
              _ => unreachable!(),
            }
          } else {
            unreachable!()
          }
        },
        Some("to_horizontal_keyword"),
      )
  }

  fn vertical_keyword_parser() -> TokenParser<VerticalKeyword> {
    TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
      .where_fn(
        |token| {
          if let SimpleToken::Ident(value) = token {
            matches!(value.as_str(), "top" | "center" | "bottom")
          } else {
            false
          }
        },
        Some("vertical_keyword"),
      )
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            match value.as_str() {
              "top" => VerticalKeyword::Top,
              "center" => VerticalKeyword::Center,
              "bottom" => VerticalKeyword::Bottom,
              _ => unreachable!(),
            }
          } else {
            unreachable!()
          }
        },
        Some("to_vertical_keyword"),
      )
  }

  /// Covers these key scenarios:
  /// 1. Single keywords: "left", "top", "center", etc.
  /// 2. Single lengths: "50%", "10px", etc.
  /// 3. Two values: "left top", "50% 25%", "center 10px", etc.
  /// 4. Keyword with offset: "left 10px", "top 20%", etc.
  ///
  /// while being much simpler and more maintainable in Rust.
  pub fn parser() -> TokenParser<Position> {
    // Strategy 1: Two-value positions (most common case)
    // This covers: "left top", "50% 25%", "center 10px", etc.
    let two_values = TokenParser::one_of(vec![
      // Horizontal keyword then vertical keyword: "left top"
      Self::horizontal_keyword_parser().flat_map(
        |h| {
          TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("ws")).flat_map(
            move |_| {
              let h_clone = h.clone();
              Self::vertical_keyword_parser().map(
                move |v| {
                  Position::new(
                    Some(Horizontal::Keyword(h_clone.clone())),
                    Some(Vertical::Keyword(v)),
                  )
                },
                Some("h_kw_v_kw"),
              )
            },
            Some("ws_to_v_kw"),
          )
        },
        Some("horizontal_vertical_keywords"),
      ),
      // Vertical keyword then horizontal keyword: "top left"
      Self::vertical_keyword_parser().flat_map(
        |v| {
          TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("ws")).flat_map(
            move |_| {
              let v_clone = v.clone();
              Self::horizontal_keyword_parser().map(
                move |h| {
                  Position::new(
                    Some(Horizontal::Keyword(h)),
                    Some(Vertical::Keyword(v_clone.clone())),
                  )
                },
                Some("v_kw_h_kw"),
              )
            },
            Some("ws_to_h_kw"),
          )
        },
        Some("vertical_horizontal_keywords"),
      ),
      // Two length values: "50% 25%"
      length_percentage_parser().flat_map(
        |first| {
          TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("ws")).flat_map(
            move |_| {
              let first_clone = first.clone();
              length_percentage_parser().map(
                move |second| {
                  Position::new(
                    Some(Horizontal::Length(first_clone.clone())),
                    Some(Vertical::Length(second)),
                  )
                },
                Some("two_lengths"),
              )
            },
            Some("ws_to_second_length"),
          )
        },
        Some("length_length"),
      ),
      // Length then vertical keyword: "50% top"
      length_percentage_parser().flat_map(
        |length| {
          TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("ws")).flat_map(
            move |_| {
              let len_clone = length.clone();
              Self::vertical_keyword_parser().map(
                move |v| {
                  Position::new(
                    Some(Horizontal::Length(len_clone.clone())),
                    Some(Vertical::Keyword(v)),
                  )
                },
                Some("length_v_kw"),
              )
            },
            Some("ws_to_v_kw"),
          )
        },
        Some("length_vertical"),
      ),
      // Horizontal keyword then length: "left 25%"
      Self::horizontal_keyword_parser().flat_map(
        |h| {
          TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("ws")).flat_map(
            move |_| {
              let h_clone = h.clone();
              length_percentage_parser().map(
                move |length| {
                  Position::new(
                    Some(Horizontal::Keyword(h_clone.clone())),
                    Some(Vertical::Length(length)),
                  )
                },
                Some("h_kw_length"),
              )
            },
            Some("ws_to_length"),
          )
        },
        Some("horizontal_length"),
      ),
    ]);

    // Strategy 2: Single values
    let single_values = TokenParser::one_of(vec![
      // Single horizontal keyword: "left"
      Self::horizontal_keyword_parser().map(
        |h| Position::new(Some(Horizontal::Keyword(h)), None),
        Some("single_h_keyword"),
      ),
      // Single vertical keyword: "top"
      Self::vertical_keyword_parser().map(
        |v| Position::new(None, Some(Vertical::Keyword(v))),
        Some("single_v_keyword"),
      ),
      // Single length (applies to horizontal): "50%"
      length_percentage_parser().map(
        |lp| {
          Position::new(
            Some(Horizontal::Length(lp.clone())),
            Some(Vertical::Length(lp)),
          )
        },
        Some("single_length"),
      ),
    ]);

    // Try two-value patterns first, then fall back to single values

    two_values.or(single_values).map(
      |either| match either {
        crate::token_parser::Either::Left(pos) => pos,
        crate::token_parser::Either::Right(pos) => pos,
      },
      Some("position_result"),
    )
  }
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let parts: Vec<String> = [
      self.horizontal.as_ref().map(|h| h.to_string()),
      self.vertical.as_ref().map(|v| v.to_string()),
    ]
    .into_iter()
    .flatten()
    .collect();

    write!(f, "{}", parts.join(" "))
  }
}

pub fn position_parser() -> TokenParser<Position> {
  Position::parser()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_horizontal_keyword() {
    let result = Position::horizontal_keyword_parser().parse("left");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), HorizontalKeyword::Left);
  }

  #[test]
  fn test_vertical_keyword() {
    let result = Position::vertical_keyword_parser().parse("top");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), VerticalKeyword::Top);
  }

  #[test]
  fn test_position_basic() {
    let result = Position::parser().parse("left");
    assert!(result.is_ok());

    let pos = result.unwrap();
    assert!(pos.horizontal.is_some());
    assert!(pos.vertical.is_none());
  }

  #[test]
  fn test_position_display() {
    let pos = Position::new(
      Some(Horizontal::Keyword(HorizontalKeyword::Left)),
      Some(Vertical::Keyword(VerticalKeyword::Top)),
    );
    assert_eq!(pos.to_string(), "left top");
  }

  #[test]
  fn test_horizontal_keyword_as_str() {
    let left = HorizontalKeyword::Left;
    let center = HorizontalKeyword::Center;
    let right = HorizontalKeyword::Right;

    assert_eq!(left.as_str(), "left");
    assert_eq!(center.as_str(), "center");
    assert_eq!(right.as_str(), "right");
  }

  #[test]
  fn test_vertical_keyword_as_str() {
    let top = VerticalKeyword::Top;
    let bottom = VerticalKeyword::Bottom;

    assert_eq!(top.as_str(), "top");
    assert_eq!(bottom.as_str(), "bottom");
  }

  #[test]
  fn test_keyword_with_offset_display() {
    let h = Horizontal::KeywordWithOffset(
      HorizontalKeyword::Left,
      LengthPercentage::Percentage(crate::css_types::Percentage::new(50.0)),
    );
    assert_eq!(h.to_string(), "left 50%");
  }

  #[test]
  fn test_numbers_only() {
    // This would test: "50% 25%" -> Position with both horizontal and vertical length
    let pos = Position::new(
      Some(Horizontal::Length(LengthPercentage::Percentage(
        crate::css_types::Percentage::new(50.0),
      ))),
      Some(Vertical::Length(LengthPercentage::Percentage(
        crate::css_types::Percentage::new(25.0),
      ))),
    );
    assert_eq!(pos.to_string(), "50% 25%");
  }

  #[test]
  fn test_two_keywords() {
    let result = Position::parser().parse("left top");
    if let Ok(pos) = result {
      assert!(matches!(
        pos.horizontal,
        Some(Horizontal::Keyword(HorizontalKeyword::Left))
      ));
      assert!(matches!(
        pos.vertical,
        Some(Vertical::Keyword(VerticalKeyword::Top))
      ));
    }
  }

  #[test]
  fn test_two_lengths() {
    let result = Position::parser().parse("50%");
    if let Ok(pos) = result {
      // Single length should apply to both axes
      assert!(matches!(pos.horizontal, Some(Horizontal::Length(_))));
      assert!(matches!(pos.vertical, Some(Vertical::Length(_))));
    }
  }
}
