use crate::css_types::length_percentage::{length_percentage_parser, LengthPercentage};
use crate::css_types::position::Position;
/**
 * CSS Basic Shape Type Parsers
 *
 * Provides comprehensive basic shape parsing for CSS clip-path and shape-outside properties.
 * Covers all shape types with essential functionality and Rust type safety.
 */
use crate::token_parser::TokenParser;
use crate::token_types::{SimpleToken, TokenList};
use crate::CssParseError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BasicShape {
  Inset {
    top: LengthPercentage,
    right: LengthPercentage,
    bottom: LengthPercentage,
    left: LengthPercentage,
    round: Option<LengthPercentage>,
  },
  Circle {
    radius: CircleRadius,
    position: Option<Position>,
  },
  Ellipse {
    radius_x: CircleRadius,
    radius_y: CircleRadius,
    position: Option<Position>,
  },
  Polygon {
    fill_rule: Option<String>,
    points: Vec<(LengthPercentage, LengthPercentage)>,
  },
  Path {
    fill_rule: Option<String>,
    path: String,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircleRadius {
  Length(LengthPercentage),
  ClosestSide,
  FarthestSide,
}

impl fmt::Display for CircleRadius {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CircleRadius::Length(lp) => write!(f, "{}", lp),
      CircleRadius::ClosestSide => write!(f, "closest-side"),
      CircleRadius::FarthestSide => write!(f, "farthest-side"),
    }
  }
}

impl fmt::Display for BasicShape {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        round,
      } => {
        let round_str = round
          .as_ref()
          .map(|r| format!(" round {}", r))
          .unwrap_or_default();

        if top == right
          && right == bottom
          && bottom == left
          && left == round.as_ref().unwrap_or(left)
        {
          write!(f, "inset({}{})", top, round_str)
        } else if top == bottom && left == right {
          write!(f, "inset({} {}{})", top, right, round_str)
        } else if top == bottom {
          write!(f, "inset({} {} {}{})", top, right, bottom, round_str)
        } else {
          write!(
            f,
            "inset({} {} {} {}{})",
            top, right, bottom, left, round_str
          )
        }
      }
      BasicShape::Circle { radius, position } => {
        let pos_str = position
          .as_ref()
          .map(|p| format!(" at {}", p))
          .unwrap_or_default();
        write!(f, "circle({}{})", radius, pos_str)
      }
      BasicShape::Ellipse {
        radius_x,
        radius_y,
        position,
      } => {
        let pos_str = position
          .as_ref()
          .map(|p| format!(" at {}", p))
          .unwrap_or_default();
        write!(f, "ellipse({} {}{})", radius_x, radius_y, pos_str)
      }
      BasicShape::Polygon { fill_rule, points } => {
        let fill_rule_str = fill_rule
          .as_ref()
          .map(|fr| format!("{}, ", fr))
          .unwrap_or_default();
        let points_str = points
          .iter()
          .map(|(x, y)| format!("{} {}", x, y))
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "polygon({}{})", fill_rule_str, points_str)
      }
      BasicShape::Path { fill_rule, path } => {
        let fill_rule_str = fill_rule
          .as_ref()
          .map(|fr| format!("{}, ", fr))
          .unwrap_or_default();
        write!(f, "path({}\"{}\")", fill_rule_str, path)
      }
    }
  }
}

impl BasicShape {
  fn inset_parser() -> TokenParser<BasicShape> {
    TokenParser::new(
      |tokens| {
        // Parse 'inset(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "inset" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected inset() function, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected inset() function but reached end of input".to_string(),
            })
          }
        }

        // Parse first length-percentage (required)
        let top = (length_percentage_parser().run)(tokens)?;

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse optional right value, or default to top
        let right = match (length_percentage_parser().run)(tokens) { Ok(right_val) => {
          // Skip optional whitespace after right
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }
          right_val
        } _ => {
          top.clone()
        }};

        // Parse optional bottom value, or default to top
        let bottom = match (length_percentage_parser().run)(tokens) { Ok(bottom_val) => {
          // Skip optional whitespace after bottom
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }
          bottom_val
        } _ => {
          top.clone()
        }};

        // Parse optional left value, or default to right
        let left = match (length_percentage_parser().run)(tokens) { Ok(left_val) => {
          // Skip optional whitespace after left
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }
          left_val
        } _ => {
          right.clone()
        }};

        // Handle optional "round" parameter
        // Skip whitespace before checking for round keyword
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        let round = if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "round" {
            // Consume the "round" keyword
            tokens.consume_next_token()?;

            // Skip whitespace after round
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Parse a single border-radius value (length or percentage)
            let radius_value = (length_percentage_parser().run)(tokens)?;
            Some(radius_value)
          } else {
            None
          }
        } else {
          None
        };

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            })
          }
        }

        Ok(BasicShape::Inset {
          top,
          right,
          bottom,
          left,
          round,
        })
      },
      "inset_parser",
    )
  }

  fn circle_parser() -> TokenParser<BasicShape> {
    TokenParser::new(
      |tokens| {
        // Parse 'circle(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "circle" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected circle() function, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected circle() function but reached end of input".to_string(),
            })
          }
        }

        // Parse radius: length-percentage | closest-side | farthest-side
        let radius = if let Some(token) = tokens.peek()? {
          match token {
            SimpleToken::Ident(value) if value == "closest-side" => {
              tokens.consume_next_token()?; // consume the ident
              CircleRadius::ClosestSide
            }
            SimpleToken::Ident(value) if value == "farthest-side" => {
              tokens.consume_next_token()?; // consume the ident
              CircleRadius::FarthestSide
            }
            _ => {
              // Try to parse as length-percentage
              let lp = (length_percentage_parser().run)(tokens)?;
              CircleRadius::Length(lp)
            }
          }
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected radius for circle".to_string(),
          });
        };

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse optional "at Position"
        let position = Self::parse_optional_position(tokens)?;

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            })
          }
        }

        Ok(BasicShape::Circle { radius, position })
      },
      "circle_parser",
    )
  }

  fn ellipse_parser() -> TokenParser<BasicShape> {
    TokenParser::new(
      |tokens| {
        // Parse 'ellipse(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "ellipse" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected ellipse() function, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected ellipse() function but reached end of input".to_string(),
            })
          }
        }

        // Helper function to parse radius
        let parse_radius = |tokens: &mut TokenList| -> Result<CircleRadius, CssParseError> {
          if let Some(token) = tokens.peek()? {
            match token {
              SimpleToken::Ident(value) if value == "closest-side" => {
                tokens.consume_next_token()?;
                Ok(CircleRadius::ClosestSide)
              }
              SimpleToken::Ident(value) if value == "farthest-side" => {
                tokens.consume_next_token()?;
                Ok(CircleRadius::FarthestSide)
              }
              _ => {
                let lp = (length_percentage_parser().run)(tokens)?;
                Ok(CircleRadius::Length(lp))
              }
            }
          } else {
            Err(CssParseError::ParseError {
              message: "Expected radius".to_string(),
            })
          }
        };

        // Parse radius_x
        let radius_x = parse_radius(tokens)?;

        // Skip whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse radius_y
        let radius_y = parse_radius(tokens)?;

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse optional "at Position"
        let position = Self::parse_optional_position(tokens)?;

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            })
          }
        }

        Ok(BasicShape::Ellipse {
          radius_x,
          radius_y,
          position,
        })
      },
      "ellipse_parser",
    )
  }

  fn polygon_parser() -> TokenParser<BasicShape> {
    TokenParser::new(
      |tokens| {
        // Parse 'polygon(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "polygon" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected polygon() function, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected polygon() function but reached end of input".to_string(),
            })
          }
        }

        // Parse optional fillRule (nonzero | evenodd)
        // Skip whitespace after polygon(
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        let fill_rule = if let Ok(Some(SimpleToken::Ident(rule))) = tokens.peek() {
          if rule == "nonzero" || rule == "evenodd" {
            // Consume the fill-rule
            let rule_value = tokens.consume_next_token()?.unwrap();
            if let SimpleToken::Ident(fill_rule_str) = rule_value {
              // Skip optional comma and whitespace after fill-rule
              while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
                tokens.consume_next_token()?;
              }
              if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
                tokens.consume_next_token()?;
                while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
                  tokens.consume_next_token()?;
                }
              }
              Some(fill_rule_str)
            } else {
              Some("nonzero".to_string()) // fallback
            }
          } else {
            Some("nonzero".to_string()) // default
          }
        } else {
          Some("nonzero".to_string()) // default
        };

        // Parse at least one point (x y, x y, ...)
        let mut points = Vec::new();

        loop {
          // Skip optional whitespace
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Check if we hit the end paren
          if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
            break;
          }

          // Parse point: x y
          let x = (length_percentage_parser().run)(tokens)?;

          // Skip whitespace between x and y
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          let y = (length_percentage_parser().run)(tokens)?;

          points.push((x, y));

          // Skip optional whitespace
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Check for comma (more points) or closing paren (end)
          if let Ok(Some(token)) = tokens.peek() {
            match token {
              SimpleToken::Comma => {
                tokens.consume_next_token()?; // consume the comma
                continue; // parse next point
              }
              SimpleToken::RightParen => {
                break; // done parsing points
              }
              _ => {
                return Err(CssParseError::ParseError {
                  message: format!(
                    "Expected comma or closing paren in polygon, got {:?}",
                    token
                  ),
                });
              }
            }
          } else {
            return Err(CssParseError::ParseError {
              message: "Unexpected end of input in polygon".to_string(),
            });
          }
        }

        // Parse closing paren
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected closing paren, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            })
          }
        }

        if points.is_empty() {
          return Err(CssParseError::ParseError {
            message: "Polygon must have at least one point".to_string(),
          });
        }

        Ok(BasicShape::Polygon { fill_rule, points })
      },
      "polygon_parser",
    )
  }

  fn path_parser() -> TokenParser<BasicShape> {
    TokenParser::new(
      |tokens| {
        // Parse 'path(' function start
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(fn_name)) if fn_name == "path" => {}
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected path() function, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected path() function but reached end of input".to_string(),
            })
          }
        }

        // Parse optional fillRule (nonzero | evenodd)
        // Skip whitespace after path(
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        let fill_rule = if let Ok(Some(SimpleToken::Ident(rule))) = tokens.peek() {
          if rule == "nonzero" || rule == "evenodd" {
            // Consume the fill-rule
            let rule_value = tokens.consume_next_token()?.unwrap();
            if let SimpleToken::Ident(fill_rule_str) = rule_value {
              // Skip optional comma and whitespace after fill-rule
              while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
                tokens.consume_next_token()?;
              }
              if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
                tokens.consume_next_token()?;
                while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
                  tokens.consume_next_token()?;
                }
              }
              Some(fill_rule_str)
            } else {
              Some("nonzero".to_string()) // fallback
            }
          } else {
            Some("nonzero".to_string()) // default
          }
        } else {
          Some("nonzero".to_string()) // default
        };

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse path string literal
        let path = match tokens.consume_next_token()? {
          Some(SimpleToken::String(s)) => s,
          Some(token) => {
            return Err(CssParseError::ParseError {
              message: format!("Expected string literal for path, got {:?}", token),
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected path string but reached end of input".to_string(),
            })
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
            })
          }
          None => {
            return Err(CssParseError::ParseError {
              message: "Expected closing paren but reached end of input".to_string(),
            })
          }
        }

        Ok(BasicShape::Path { fill_rule, path })
      },
      "path_parser",
    )
  }

  /// Parse optional "at Position" for circle and ellipse
  fn parse_optional_position(tokens: &mut TokenList) -> Result<Option<Position>, CssParseError> {
    // Check if there's an "at" keyword for position
    let checkpoint = tokens.current_index;

    // Skip optional whitespace
    if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
      tokens.consume_next_token()?;
    }

    // Check for "at" keyword
    match tokens.peek()? {
      Some(SimpleToken::Ident(keyword)) if keyword == "at" => {
        tokens.consume_next_token()?; // consume "at"

        // Skip optional whitespace after "at"
        if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        let position_parser = Position::parser();
        match position_parser.run.as_ref()(tokens) {
          Ok(position) => Ok(Some(position)),
          Err(e) => Err(e),
        }
      }
      _ => {
        // No "at" keyword, rewind to checkpoint
        tokens.set_current_index(checkpoint);
        Ok(None)
      }
    }
  }

  pub fn parse() -> TokenParser<BasicShape> {
    TokenParser::one_of(vec![
      Self::inset_parser(),
      Self::circle_parser(),
      Self::ellipse_parser(),
      Self::polygon_parser(),
      Self::path_parser(),
    ])
  }
}

pub fn basic_shape_parser() -> TokenParser<BasicShape> {
  BasicShape::parse()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_circle_radius() {
    let result = BasicShape::parse().parse_to_end("circle(closest-side)");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      BasicShape::Circle {
        radius: CircleRadius::ClosestSide,
        position: None,
      }
    );
  }

  #[test]
  fn test_basic_shape_display() {
    let circle = BasicShape::Circle {
      radius: CircleRadius::ClosestSide,
      position: None,
    };
    assert_eq!(circle.to_string(), "circle(closest-side)");
  }

  #[test]
  fn test_inset_display_optimization() {
    let inset = BasicShape::Inset {
      top: LengthPercentage::Length(crate::css_types::Length::new(10.0, "px".to_string())),
      right: LengthPercentage::Length(crate::css_types::Length::new(10.0, "px".to_string())),
      bottom: LengthPercentage::Length(crate::css_types::Length::new(10.0, "px".to_string())),
      left: LengthPercentage::Length(crate::css_types::Length::new(10.0, "px".to_string())),
      round: None,
    };
    assert_eq!(inset.to_string(), "inset(10px)");
  }

  #[test]
  fn test_polygon_display() {
    let polygon = BasicShape::Polygon {
      fill_rule: Some("nonzero".to_string()),
      points: vec![(
        LengthPercentage::Length(crate::css_types::Length::new(0.0, "px".to_string())),
        LengthPercentage::Length(crate::css_types::Length::new(0.0, "px".to_string())),
      )],
    };
    assert!(polygon.to_string().contains("polygon(nonzero"));
  }

  #[test]
  fn test_path_display() {
    let path = BasicShape::Path {
      fill_rule: Some("evenodd".to_string()),
      path: "M 0 0 L 100 100".to_string(),
    };
    assert_eq!(path.to_string(), "path(evenodd, \"M 0 0 L 100 100\")");
  }

  #[test]
  fn test_ellipse_two_radii() {
    let ellipse = BasicShape::Ellipse {
      radius_x: CircleRadius::Length(LengthPercentage::Length(crate::css_types::Length::new(
        50.0,
        "px".to_string(),
      ))),
      radius_y: CircleRadius::Length(LengthPercentage::Length(crate::css_types::Length::new(
        25.0,
        "px".to_string(),
      ))),
      position: None,
    };
    assert_eq!(ellipse.to_string(), "ellipse(50px 25px)");
  }
}
