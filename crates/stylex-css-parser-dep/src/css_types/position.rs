use crate::css_types::length_percentage::length_percentage;
use crate::parser::Parser;
use std::fmt::{self, Display};

/// Horizontal position keywords: 'left', 'center', 'right'
pub type HorizontalKeyword = String;

/// Vertical position keywords: 'top', 'center', 'bottom'
pub type VerticalKeyword = String;

/// Horizontal position component, which can be a keyword, length/percentage, or keyword with length/percentage
#[derive(Debug, Clone, PartialEq)]
pub enum Horizontal {
  LengthPercentage(String),
  Keyword(HorizontalKeyword),
  KeywordWithLength(HorizontalKeyword, String),
}

impl Display for Horizontal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::LengthPercentage(val) => write!(f, "{}", val),
      Self::Keyword(keyword) => write!(f, "{}", keyword),
      Self::KeywordWithLength(keyword, length) => write!(f, "{} {}", keyword, length),
    }
  }
}

/// Vertical position component, which can be a keyword, length/percentage, or keyword with length/percentage
#[derive(Debug, Clone, PartialEq)]
pub enum Vertical {
  LengthPercentage(String),
  Keyword(VerticalKeyword),
  KeywordWithLength(VerticalKeyword, String),
}

impl Display for Vertical {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::LengthPercentage(val) => write!(f, "{}", val),
      Self::Keyword(keyword) => write!(f, "{}", keyword),
      Self::KeywordWithLength(keyword, length) => write!(f, "{} {}", keyword, length),
    }
  }
}

/// Position class representing CSS position values
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub horizontal: Option<Horizontal>,
  pub vertical: Option<Vertical>,
}

impl Position {
  pub fn new(horizontal: Option<Horizontal>, vertical: Option<Vertical>) -> Self {
    Self {
      horizontal,
      vertical,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Position> {
    // Define keyword parsers
    let horizontal_keyword = Parser::one_of(vec![
      Parser::<'a, String>::string("left"),
      Parser::<'a, String>::string("center"),
      Parser::<'a, String>::string("right"),
    ]);

    let vertical_keyword = Parser::one_of(vec![
      Parser::<'a, String>::string("top"),
      Parser::<'a, String>::string("center"),
      Parser::<'a, String>::string("bottom"),
    ]);

    // Parse keyword with optional length percentage
    let horizontal_with_length: Parser<'a, Horizontal> =
      Parser::<'a, String>::sequence::<String, String, (), ()>(
        Some(horizontal_keyword.clone()),
        Some(
          length_percentage()
            .prefix(Parser::<'a, String>::whitespace())
            .optional(),
        ),
        None,
        None,
      )
      .to_parser()
      .map(|values| {
        let (keyword, length, _, _) = values.unwrap();
        let keyword = keyword.unwrap();

        match length {
          Some(len) => Horizontal::KeywordWithLength(keyword, len),
          None => Horizontal::Keyword(keyword),
        }
      });

    // Parse keyword with optional length percentage
    let vertical_with_length: Parser<'a, Vertical> =
      Parser::<'a, String>::sequence::<String, String, (), ()>(
        Some(vertical_keyword.clone()),
        Some(
          length_percentage()
            .prefix(Parser::<'a, String>::whitespace())
            .optional(),
        ),
        None,
        None,
      )
      .to_parser()
      .map(|values| {
        let (keyword, length, _, _) = values.unwrap();
        let keyword = keyword.unwrap();

        match length {
          Some(len) => Vertical::KeywordWithLength(keyword, len),
          None => Vertical::Keyword(keyword),
        }
      });

    // Parse length percentage for position components
    let length_percentage_component = length_percentage().map(|lp| lp.unwrap());

    // Create horizontal and vertical value parsers
    let horizontal_value = Parser::one_of(vec![
      horizontal_with_length.clone(),
      // length_percentage_component
      //   .clone()
      //   .map(|lp| Horizontal::LengthPercentage(lp.unwrap())),
    ]);

    let vertical_value = Parser::one_of(vec![
      vertical_with_length.clone(),
      // length_percentage_component
      //   .clone()
      //   .map(|lp| Vertical::LengthPercentage(lp.unwrap())),
    ]);

    // Main position parsers
    Parser::one_of(vec![
      // Single keyword
      // horizontal_with_length
      //   .clone()
      //   .map(|h| Position::new(h, None)),
      // vertical_with_length.clone().map(|v| Position::new(None, v)),
      // Keyword combinations using ParserSet
      Parser::<'a, String>::set_of(Some(horizontal_value), Some(vertical_value))
        .separated_by(Parser::<'a, String>::whitespace())
        .to_parser()
        .map(|p| {
          let (h, v, _, _) = p.unwrap();

          Position::new(h, v)
        }),
      // // Two length percentages
      // Parser::<'a, String>::sequence::<String, String, (), ()>(
      //   Some(length_percentage()),
      //   Some(length_percentage().prefix(Parser::<'a, String>::whitespace())),
      //   None,
      //   None,
      // )
      // .to_parser()
      // .map(|values| {
      //   let (h, v, _, _) = values.unwrap();
      //   Position::new(
      //     Some(Horizontal::LengthPercentage(h.unwrap())),
      //     Some(Vertical::LengthPercentage(v.unwrap())),
      //   )
      // }),
    ])
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match (&self.horizontal, &self.vertical) {
      (Some(h), Some(v)) => write!(f, "{} {}", h, v),
      (Some(h), None) => write!(f, "{}", h),
      (None, Some(v)) => write!(f, "{}", v),
      (None, None) => write!(f, ""),
    }
  }
}
