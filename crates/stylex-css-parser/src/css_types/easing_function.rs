/*!
CSS easing function parser.
*/

use crate::{CssParseError, token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
  Linear(LinearEasingFunction),
  CubicBezier(CubicBezierEasingFunction),
  CubicBezierKeyword(CubicBezierKeyword),
  Steps(StepsEasingFunction),
  StepsKeyword(StepsKeyword),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinearEasingFunction {
  pub points: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezierEasingFunction {
  pub points: [f64; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezierKeyword {
  pub keyword: CubicBezierKeywordType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepsEasingFunction {
  pub steps: u32,
  pub start: StepsStartType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepsKeyword {
  pub keyword: StepsKeywordType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CubicBezierKeywordType {
  Ease,
  EaseIn,
  EaseOut,
  EaseInOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsStartType {
  Start,
  End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsKeywordType {
  StepStart,
  StepEnd,
}

impl EasingFunction {
  pub fn parse() -> TokenParser<EasingFunction> {
    TokenParser::one_of(vec![
      LinearEasingFunction::parse().map(EasingFunction::Linear, Some("linear")),
      CubicBezierEasingFunction::parse().map(EasingFunction::CubicBezier, Some("cubic_bezier")),
      StepsEasingFunction::parse().map(EasingFunction::Steps, Some("steps")),
      CubicBezierKeyword::parse().map(
        EasingFunction::CubicBezierKeyword,
        Some("cubic_bezier_keyword"),
      ),
      StepsKeyword::parse().map(EasingFunction::StepsKeyword, Some("steps_keyword")),
    ])
  }
}

impl LinearEasingFunction {
  pub fn new(points: Vec<f64>) -> Self {
    Self { points }
  }

  pub fn parse() -> TokenParser<LinearEasingFunction> {
    TokenParser::new(
      |tokens| {
        // Parse 'linear(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "linear" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected linear() function, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected linear() function but reached end of input".to_string(),
            });
          }
        }

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse one or more numbers separated by commas
        let mut points = Vec::new();

        loop {
          // Parse number
          let number = match tokens.consume_next_token()? {
            Some(SimpleToken::Number(value)) => value,
            Some(token) => {
              return Err(CssParseError::ParseError {
                message: format!("Expected number in linear function, got {:?}", token),
              });
            }
            None => {
              return Err(CssParseError::ParseError {
                message: "Expected number but reached end of input".to_string(),
              });
            }
          };

          points.push(number);

          // Skip optional whitespace
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Check for comma (more numbers) or closing paren (end)
          if let Ok(Some(token)) = tokens.peek() {
            match token {
              SimpleToken::Comma => {
                tokens.consume_next_token()?; // consume the comma
                // Skip optional whitespace after comma
                while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
                  tokens.consume_next_token()?;
                }
                continue; // parse next number
              }
              SimpleToken::RightParen => {
                break; // done parsing numbers
              }
              _ => {
                return Err(CssParseError::ParseError {
                  message: format!(
                    "Expected comma or closing paren in linear function, got {:?}",
                    token
                  ),
                });
              }
            }
          } else {
            return Err(CssParseError::ParseError {
              message: "Unexpected end of input in linear function".to_string(),
            });
          }
        }

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            });
          }
        }

        if points.is_empty() {
          return Err(CssParseError::ParseError {
            message: "Linear function must have at least one point".to_string(),
          });
        }

        Ok(LinearEasingFunction::new(points))
      },
      "linear_parser",
    )
  }
}

impl CubicBezierEasingFunction {
  pub fn new(points: [f64; 4]) -> Self {
    Self { points }
  }

  pub fn parse() -> TokenParser<CubicBezierEasingFunction> {
    TokenParser::new(
      |tokens| {
        // Parse 'cubic-bezier(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "cubic-bezier" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected cubic-bezier() function, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected cubic-bezier() function but reached end of input".to_string(),
            });
          }
        }

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse exactly 4 numbers separated by commas: x1, y1, x2, y2
        let mut numbers = Vec::new();

        for i in 0..4 {
          if i > 0 {
            // Skip optional whitespace before comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Expect comma
            match tokens.consume_next_token()? {
              Some(SimpleToken::Comma) => {}
              Some(token) => {
                return Err(CssParseError::ParseError {
                  message: format!("Expected comma in cubic-bezier function, got {:?}", token),
                });
              }
              None => {
                return Err(CssParseError::ParseError {
                  message: "Expected comma but reached end of input".to_string(),
                });
              }
            }

            // Skip optional whitespace after comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }
          }

          // Parse number
          let number = match tokens.consume_next_token()? {
            Some(SimpleToken::Number(value)) => value,
            Some(token) => {
              return Err(CssParseError::ParseError {
                message: format!("Expected number in cubic-bezier function, got {:?}", token),
              });
            }
            None => {
              return Err(CssParseError::ParseError {
                message: "Expected number but reached end of input".to_string(),
              });
            }
          };

          numbers.push(number);
        }

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            });
          }
        }

        Ok(CubicBezierEasingFunction::new([
          numbers[0], numbers[1], numbers[2], numbers[3],
        ]))
      },
      "cubic_bezier_parser",
    )
  }
}

impl CubicBezierKeyword {
  pub fn new(keyword: CubicBezierKeywordType) -> Self {
    Self { keyword }
  }

  /// Create from string representation
  pub fn from_string(value: &str) -> Result<Self, CssParseError> {
    let keyword = match value {
      "ease" => CubicBezierKeywordType::Ease,
      "ease-in" => CubicBezierKeywordType::EaseIn,
      "ease-out" => CubicBezierKeywordType::EaseOut,
      "ease-in-out" => CubicBezierKeywordType::EaseInOut,
      _ => {
        return Err(CssParseError::ParseError {
          message: format!("Unknown cubic-bezier keyword: {}", value),
        });
      }
    };
    Ok(Self::new(keyword))
  }

  pub fn parse() -> TokenParser<CubicBezierKeyword> {
    TokenParser::<SimpleToken>::ident()
      .where_fn(
        |token| {
          if let SimpleToken::Ident(value) = token {
            matches!(
              value.as_str(),
              "ease" | "ease-in" | "ease-out" | "ease-in-out"
            )
          } else {
            false
          }
        },
        Some("easing_keyword"),
      )
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            let keyword = match value.as_str() {
              "ease" => CubicBezierKeywordType::Ease,
              "ease-in" => CubicBezierKeywordType::EaseIn,
              "ease-out" => CubicBezierKeywordType::EaseOut,
              "ease-in-out" => CubicBezierKeywordType::EaseInOut,
              _ => unreachable!(),
            };
            CubicBezierKeyword::new(keyword)
          } else {
            unreachable!()
          }
        },
        Some("to_keyword"),
      )
  }
}

impl StepsEasingFunction {
  pub fn new(steps: u32, start: StepsStartType) -> Self {
    Self { steps, start }
  }

  pub fn parse() -> TokenParser<StepsEasingFunction> {
    TokenParser::new(
      |tokens| {
        // Parse 'steps(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "steps" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected steps() function, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected steps() function but reached end of input".to_string(),
            });
          }
        }

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse step count (integer number)
        let steps = match tokens.consume_next_token()? {
          Some(SimpleToken::Number(value)) => {
            let int_value = value as u32;
            if int_value as f64 == value && value >= 0.0 {
              int_value
            } else {
              return Err(CssParseError::ParseError {
                message: "Steps count must be a positive integer".to_string(),
              });
            }
          }
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected number for steps count, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected steps count but reached end of input".to_string(),
            });
          }
        };

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected comma in steps function, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected comma but reached end of input".to_string(),
            });
          }
        }

        // Skip optional whitespace after comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse start/end type
        let start_type = match tokens.consume_next_token()? {
          Some(SimpleToken::Ident(value)) => match value.as_str() {
            "start" => StepsStartType::Start,
            "end" => StepsStartType::End,
            _ => {
              return Err(CssParseError::ParseError {
                message: format!(
                  "Expected 'start' or 'end' in steps function, got '{}'",
                  value
                ),
              });
            }
          },
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!(
                "Expected 'start' or 'end' in steps function, got {:?}",
                token
              ),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected 'start' or 'end' but reached end of input".to_string(),
            });
          }
        };

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            });
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            });
          }
        }

        Ok(StepsEasingFunction::new(steps, start_type))
      },
      "steps_parser",
    )
  }
}

impl StepsKeyword {
  pub fn new(keyword: StepsKeywordType) -> Self {
    Self { keyword }
  }

  /// Create from string representation
  pub fn from_string(value: &str) -> Result<Self, CssParseError> {
    let keyword = match value {
      "step-start" => StepsKeywordType::StepStart,
      "step-end" => StepsKeywordType::StepEnd,
      _ => {
        return Err(CssParseError::ParseError {
          message: format!("Unknown steps keyword: {}", value),
        });
      }
    };
    Ok(Self::new(keyword))
  }

  pub fn parse() -> TokenParser<StepsKeyword> {
    TokenParser::<SimpleToken>::ident()
      .where_fn(
        |token| {
          if let SimpleToken::Ident(value) = token {
            matches!(value.as_str(), "step-start" | "step-end")
          } else {
            false
          }
        },
        Some("steps_keyword"),
      )
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            let keyword = match value.as_str() {
              "step-start" => StepsKeywordType::StepStart,
              "step-end" => StepsKeywordType::StepEnd,
              _ => unreachable!(),
            };
            StepsKeyword::new(keyword)
          } else {
            unreachable!()
          }
        },
        Some("to_steps_keyword"),
      )
  }
}

impl Display for EasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      EasingFunction::Linear(linear) => linear.fmt(f),
      EasingFunction::CubicBezier(cubic_bezier) => cubic_bezier.fmt(f),
      EasingFunction::CubicBezierKeyword(keyword) => keyword.fmt(f),
      EasingFunction::Steps(steps) => steps.fmt(f),
      EasingFunction::StepsKeyword(keyword) => keyword.fmt(f),
    }
  }
}

impl Display for LinearEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Format numbers to avoid floating point precision issues
    let format_number = |n: f64| -> String {
      // Round to 6 decimal places to avoid floating point precision issues
      let rounded = (n * 1_000_000.0).round() / 1_000_000.0;
      if rounded.fract() == 0.0 {
        format!("{}", rounded as i64)
      } else {
        // Remove trailing zeros and format cleanly
        let s = format!("{:.6}", rounded);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
      }
    };

    let points_str = self
      .points
      .iter()
      .map(|p| format_number(*p))
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "linear({})", points_str)
  }
}

impl Display for CubicBezierEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Format numbers to avoid floating point precision issues
    let format_number = |n: f64| -> String {
      // Round to 6 decimal places to avoid floating point precision issues
      let rounded = (n * 1_000_000.0).round() / 1_000_000.0;
      if rounded.fract() == 0.0 {
        format!("{}", rounded as i64)
      } else {
        // Remove trailing zeros and format cleanly
        let s = format!("{:.6}", rounded);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
      }
    };

    write!(
      f,
      "cubic-bezier({}, {}, {}, {})",
      format_number(self.points[0]),
      format_number(self.points[1]),
      format_number(self.points[2]),
      format_number(self.points[3])
    )
  }
}

impl Display for CubicBezierKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let keyword_str = match self.keyword {
      CubicBezierKeywordType::Ease => "ease",
      CubicBezierKeywordType::EaseIn => "ease-in",
      CubicBezierKeywordType::EaseOut => "ease-out",
      CubicBezierKeywordType::EaseInOut => "ease-in-out",
    };
    write!(f, "{}", keyword_str)
  }
}

impl Display for StepsEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let start_str = match self.start {
      StepsStartType::Start => "start",
      StepsStartType::End => "end",
    };
    write!(f, "steps({}, {})", self.steps, start_str)
  }
}

impl Display for StepsKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let keyword_str = match self.keyword {
      StepsKeywordType::StepStart => "step-start",
      StepsKeywordType::StepEnd => "step-end",
    };
    write!(f, "{}", keyword_str)
  }
}
