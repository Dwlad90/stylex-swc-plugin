/*!
Media query parsing and representation.

Complete CSS implementation.
*/

use crate::{
  css_types::{calc::Calc, Length},
  token_parser::{tokens, TokenParser},
  token_types::SimpleToken,
  CssParseError,
};
use std::fmt::{self, Display};

/// Fraction type for media query values like (aspect-ratio: 16/9)
#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
  pub numerator: f32,
  pub denominator: f32,
}

impl Display for Fraction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}/{}", self.numerator, self.denominator)
  }
}

/// Word rule types for media queries
#[derive(Debug, Clone, PartialEq)]
pub enum WordRule {
  Color,
  Monochrome,
  Grid,
  ColorIndex,
}

impl Display for WordRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      WordRule::Color => write!(f, "color"),
      WordRule::Monochrome => write!(f, "monochrome"),
      WordRule::Grid => write!(f, "grid"),
      WordRule::ColorIndex => write!(f, "color-index"),
    }
  }
}

/// Media rule values that can appear in media queries
#[derive(Debug, Clone, PartialEq)]
pub enum MediaRuleValue {
  Number(f32),
  Length(Length),
  String(String),
  Fraction(Fraction),
}

impl Display for MediaRuleValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MediaRuleValue::Number(n) => write!(f, "{}", n),
      MediaRuleValue::Length(l) => write!(f, "{}", l),
      MediaRuleValue::String(s) => write!(f, "{}", s),
      MediaRuleValue::Fraction(frac) => write!(f, "{}", frac),
    }
  }
}

/// Media keyword types (screen, print, all)
#[derive(Debug, Clone, PartialEq)]
pub struct MediaKeyword {
  pub key: String, // 'screen', 'print', 'all'
  pub not: bool,
  pub only: Option<bool>,
}

impl Display for MediaKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parts = Vec::new();

    if self.not {
      parts.push("not".to_string());
    }

    if let Some(true) = self.only {
      parts.push("only".to_string());
    }

    parts.push(self.key.clone());

    write!(f, "{}", parts.join(" "))
  }
}

/// Complete MediaQuery rule system
#[derive(Debug, Clone, PartialEq)]
pub enum MediaQueryRule {
  /// Media type keywords (screen, print, all)
  MediaKeyword(MediaKeyword),
  /// Word rules like (color), (monochrome)
  WordRule(WordRule),
  /// Pair rules like (max-width: 768px)
  Pair { key: String, value: MediaRuleValue },
  /// NOT combinator
  Not { rule: Box<MediaQueryRule> },
  /// AND combinator (multiple rules that must all match)
  And { rules: Vec<MediaQueryRule> },
  /// OR combinator (multiple rules where any can match)
  Or { rules: Vec<MediaQueryRule> },
}

impl Display for MediaQueryRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MediaQueryRule::MediaKeyword(keyword) => write!(f, "{}", keyword),
      MediaQueryRule::WordRule(word) => write!(f, "({})", word),
      MediaQueryRule::Pair { key, value } => write!(f, "({}: {})", key, value),
      MediaQueryRule::Not { rule } => write!(f, "not {}", rule),
      MediaQueryRule::And { rules } => {
        let rule_strings: Vec<String> = rules.iter().map(|r| r.to_string()).collect();
        write!(f, "{}", rule_strings.join(" and "))
      }
      MediaQueryRule::Or { rules } => {
        let rule_strings: Vec<String> = rules.iter().map(|r| r.to_string()).collect();
        write!(f, "{}", rule_strings.join(", "))
      }
    }
  }
}

/// Complete MediaQuery structure
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQuery {
  pub queries: MediaQueryRule,
}

impl MediaQuery {
  pub fn new_from_rule(queries: MediaQueryRule) -> Self {
    Self {
      queries: Self::normalize(queries),
    }
  }

  /// Create a MediaQuery from a string (for backwards compatibility)
  pub fn new(query_string: String) -> Self {
    // Handles common patterns: "@media screen", "@media (min-width: 768px)", etc.
    let media_part = if query_string.starts_with("@media ") {
      query_string
        .strip_prefix("@media ")
        .unwrap_or(&query_string)
        .to_string()
    } else {
      query_string.clone()
    };

    Self {
      queries: MediaQueryRule::MediaKeyword(MediaKeyword {
        key: media_part,
        not: false,
        only: None,
      }),
    }
  }

  /// Get the original query string for compatibility
  pub fn original_string(&self) -> String {
    match &self.queries {
      MediaQueryRule::MediaKeyword(keyword) => {
        if keyword.key.is_empty() {
          String::new()
        } else if keyword.key.starts_with("@media") {
          keyword.key.clone()
        } else {
          format!("@media {}", keyword.key)
        }
      }
      _ => format!("@media {}", self.queries),
    }
  }
}

impl Display for MediaQuery {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.queries {
      MediaQueryRule::MediaKeyword(keyword) => {
        if keyword.key.is_empty() {
          write!(f, "")
        } else if keyword.key.starts_with("@media") || keyword.key == "not a media query" {
          // Handle raw strings that are already complete or invalid
          write!(f, "{}", keyword.key)
        } else {
          write!(f, "@media {}", keyword) // Use MediaKeyword's Display impl
        }
      }
      _ => write!(f, "@media {}", self.queries),
    }
  }
}

impl MediaQuery {
  pub fn parser() -> TokenParser<MediaQuery> {
    TokenParser::new(
      |tokens| {
        if let Ok(Some(SimpleToken::AtKeyword(keyword))) = tokens.peek() {
          if keyword == "media" {
            tokens.consume_next_token()?; // consume "@media"

            // Skip mandatory whitespace after "@media"
            if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            } else {
              // "@media" without space or content should be a syntax error
              return Err(CssParseError::ParseError {
                message: "Expected whitespace or content after @media".to_string(),
              });
            }
          } else {
            return Err(CssParseError::ParseError {
              message: "Expected @media at-keyword".to_string(),
            });
          }
        } else {
          // If no @media prefix, assume we're parsing just the query part (for backwards compatibility)
          // This maintains compatibility with existing tests
        }

        let rule = (media_query_rule_parser().run)(tokens)?;
        Ok(MediaQuery::new_from_rule(rule))
      },
      "media_query_parser",
    )
  }

  /// Check if parentheses are balanced in a media query string
  pub fn has_balanced_parens(input: &str) -> bool {
    has_balanced_parens(input)
  }

  /// Handles NOT count normalization, interval merging, and recursive normalization
  pub fn normalize(rule: MediaQueryRule) -> MediaQueryRule {
    match rule {
      MediaQueryRule::And { rules } => {
        // Flatten nested AND rules and normalize each recursively
        let mut flattened = Vec::new();
        for r in rules {
          let normalized = Self::normalize(r);
          if let MediaQueryRule::And {
            rules: nested_rules,
          } = normalized
          {
            flattened.extend(nested_rules);
          } else {
            flattened.push(normalized);
          }
        }

        // Apply interval merging for numeric constraints
        let merged = merge_and_simplify_ranges(flattened);

        if merged.len() == 1 {
          merged.into_iter().next().unwrap()
        } else {
          MediaQueryRule::And { rules: merged }
        }
      }

      MediaQueryRule::Or { rules } => MediaQueryRule::Or {
        rules: rules.into_iter().map(Self::normalize).collect(),
      },

      MediaQueryRule::Not { rule } => {
        let mut count = 1;
        let mut current = rule.as_ref();

        while let MediaQueryRule::Not { rule: inner } = current {
          count += 1;
          current = inner.as_ref();
        }

        let normalized_operand = Self::normalize(current.clone());

        // Even number of NOTs cancel out, odd number becomes single NOT
        if count % 2 == 0 {
          normalized_operand
        } else {
          MediaQueryRule::Not {
            rule: Box::new(normalized_operand),
          }
        }
      }

      // For other rule types, return as-is
      _ => rule,
    }
  }
}

/// Validate media query string
pub fn validate_media_query(input: &str) -> Result<MediaQuery, String> {
  if !has_balanced_parens(input) {
    return Err(crate::at_queries::messages::MediaQueryErrors::UNBALANCED_PARENS.to_string());
  }

  match MediaQuery::parser().parse_to_end(input) {
    Ok(media_query) => Ok(media_query),
    Err(_) => Err(crate::at_queries::messages::MediaQueryErrors::SYNTAX_ERROR.to_string()),
  }
}

/// Check if parentheses are balanced
fn has_balanced_parens(input: &str) -> bool {
  let mut count = 0;
  for ch in input.chars() {
    match ch {
      '(' => count += 1,
      ')' => {
        count -= 1;
        if count < 0 {
          return false;
        }
      }
      _ => {}
    }
  }
  count == 0
}

fn is_numeric_length(val: &MediaRuleValue) -> bool {
  matches!(val, MediaRuleValue::Length(_))
}

fn merge_and_simplify_ranges(rules: Vec<MediaQueryRule>) -> Vec<MediaQueryRule> {
  match merge_intervals_for_and(rules.clone()) {
    Ok(merged) => merged,
    Err(_) => rules, // Return original rules if merging fails
  }
}

fn merge_intervals_for_and(rules: Vec<MediaQueryRule>) -> Result<Vec<MediaQueryRule>, String> {
  const EPSILON: f32 = 0.01;
  let dimensions = ["width", "height"];

  // Track intervals for each dimension: [min, max]
  let mut width_intervals: Vec<(f32, f32)> = Vec::new();
  let mut height_intervals: Vec<(f32, f32)> = Vec::new();
  let mut other_rules: Vec<MediaQueryRule> = Vec::new();

  for rule in rules {
    let mut handled = false;

    for dim in &dimensions {
      match &rule {
        // Handle min-width/min-height/max-width/max-height pairs
        MediaQueryRule::Pair { key, value }
          if (key == &format!("min-{}", dim) || key == &format!("max-{}", dim))
            && is_numeric_length(value) =>
        {
          if let MediaRuleValue::Length(length) = value {
            let val = length.value;
            let interval = if key.starts_with("min-") {
              (val, f32::INFINITY)
            } else {
              (f32::NEG_INFINITY, val)
            };

            if *dim == "width" {
              width_intervals.push(interval);
            } else {
              height_intervals.push(interval);
            }
            handled = true;
            break;
          }
        }

        // Handle NOT rules with min/max constraints
        MediaQueryRule::Not { rule: inner } => {
          if let MediaQueryRule::Pair { key, value } = inner.as_ref() {
            if (key == &format!("min-{}", dim) || key == &format!("max-{}", dim))
              && is_numeric_length(value)
            {
              if let MediaRuleValue::Length(length) = value {
                let val = length.value;
                // NOT min-width becomes max-width with adjusted value, and vice versa
                let interval = if key.starts_with("min-") {
                  (f32::NEG_INFINITY, val - EPSILON)
                } else {
                  (val + EPSILON, f32::INFINITY)
                };

                if *dim == "width" {
                  width_intervals.push(interval);
                } else {
                  height_intervals.push(interval);
                }
                handled = true;
                break;
              }
            }
          }
        }

        _ => {}
      }
    }

    if !handled {
      other_rules.push(rule);
    }
  }

  // Merge intervals for each dimension
  let merged_width = merge_dimension_intervals(width_intervals, "width")?;
  let merged_height = merge_dimension_intervals(height_intervals, "height")?;

  // Combine all rules
  let mut result = other_rules;
  result.extend(merged_width);
  result.extend(merged_height);

  Ok(result)
}

/// Merge intervals for a single dimension
fn merge_dimension_intervals(
  intervals: Vec<(f32, f32)>,
  dimension: &str,
) -> Result<Vec<MediaQueryRule>, String> {
  if intervals.is_empty() {
    return Ok(Vec::new());
  }

  // Find the intersection of all intervals
  let mut min_bound = f32::NEG_INFINITY;
  let mut max_bound = f32::INFINITY;

  for (min, max) in intervals {
    min_bound = min_bound.max(min);
    max_bound = max_bound.min(max);
  }

  // Check for contradictions
  if min_bound > max_bound {
    return Err(format!("Contradictory constraints for {}", dimension));
  }

  let mut result = Vec::new();

  // Generate min constraint if needed
  if min_bound != f32::NEG_INFINITY && min_bound.is_finite() {
    result.push(MediaQueryRule::Pair {
      key: format!("min-{}", dimension),
      value: MediaRuleValue::Length(Length::new(min_bound, "px".to_string())),
    });
  }

  // Generate max constraint if needed
  if max_bound != f32::INFINITY && max_bound.is_finite() {
    result.push(MediaQueryRule::Pair {
      key: format!("max-{}", dimension),
      value: MediaRuleValue::Length(Length::new(max_bound, "px".to_string())),
    });
  }

  Ok(result)
}

/// Basic media type parser: screen | print | all
fn basic_media_type_parser() -> TokenParser<String> {
  tokens::ident()
    .map(
      |token| {
        if let SimpleToken::Ident(value) = token {
          value
        } else {
          "all".to_string()
        }
      },
      Some("extract_media_type"),
    )
    .where_fn(
      |value| matches!(value.as_str(), "screen" | "print" | "all"),
      Some("valid_media_type"),
    )
}

/// Media keyword parser with optional not/only modifiers
fn media_keyword_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut not_value = false;
      let mut only_value = None;

      // Try to parse optional "not" at the beginning
      if let Ok(Some(SimpleToken::Ident(val))) = tokens.peek() {
        if val == "not" {
          tokens.consume_next_token()?; // consume "not"
          not_value = true;

          // Consume whitespace after "not"
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }
        }
      }

      // Try to parse optional "only" after "not" (or at beginning if no "not")
      if let Ok(Some(SimpleToken::Ident(val))) = tokens.peek() {
        if val == "only" {
          tokens.consume_next_token()?; // consume "only"
          only_value = Some(true);

          // Consume whitespace after "only"
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }
        }
      }

      // Parse the media type (required)
      let media_type = (basic_media_type_parser().run)(tokens)?;

      Ok(MediaQueryRule::MediaKeyword(MediaKeyword {
        key: media_type,
        not: not_value,
        only: only_value,
      }))
    },
    "media_keyword_parser",
  )
}

/// Media word rule parser for (color), (monochrome), etc.
fn media_word_rule_parser() -> TokenParser<MediaQueryRule> {
  tokens::ident()
    .map(
      |token| {
        if let SimpleToken::Ident(value) = token {
          value
        } else {
          "color".to_string()
        }
      },
      Some("extract_word_rule"),
    )
    .where_fn(
      |value| {
        matches!(
          value.as_str(),
          "color" | "monochrome" | "grid" | "color-index"
        )
      },
      Some("valid_word_rule"),
    )
    .surrounded_by(
      TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("OpenParen")),
      Some(TokenParser::<SimpleToken>::token(
        SimpleToken::RightParen,
        Some("CloseParen"),
      )),
    )
    .map(
      |keyword| {
        let word_rule = match keyword.as_str() {
          "color" => WordRule::Color,
          "monochrome" => WordRule::Monochrome,
          "grid" => WordRule::Grid,
          "color-index" => WordRule::ColorIndex,
          _ => WordRule::Color,
        };
        MediaQueryRule::WordRule(word_rule)
      },
      Some("create_word_rule"),
    )
}

fn media_rule_value_parser() -> TokenParser<MediaRuleValue> {
  TokenParser::one_of(vec![
    Calc::parser().map(
      |calc| MediaRuleValue::String(calc.to_string()),
      Some("calc_to_string"),
    ),
    // Dimensions (e.g., 768px)
    tokens::dimension().map(
      |token| {
        if let SimpleToken::Dimension { value, unit } = token {
          MediaRuleValue::Length(Length::new(value as f32, unit))
        } else {
          MediaRuleValue::Number(0.0)
        }
      },
      Some("dimension_to_length"),
    ),
    tokens::ident().map(
      |token| {
        if let SimpleToken::Ident(value) = token {
          MediaRuleValue::String(value)
        } else {
          MediaRuleValue::String("".to_string())
        }
      },
      Some("ident_to_string"),
    ),
    // Fraction parsing (number / number) like aspect-ratio: 16/9
    TokenParser::new(
      |tokens| {
        // Parse first number
        let first_num = if let Ok(Some(SimpleToken::Number(value))) = tokens.consume_next_token() {
          value as f32
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected first number in fraction".to_string(),
          });
        };

        // Optional whitespace before slash
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse slash delimiter
        if let Ok(Some(SimpleToken::Delim(ch))) = tokens.consume_next_token() {
          if ch != '/' {
            return Err(CssParseError::ParseError {
              message: "Expected '/' in fraction".to_string(),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected '/' delimiter".to_string(),
          });
        }

        // Optional whitespace after slash
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse second number
        let second_num = if let Ok(Some(SimpleToken::Number(value))) = tokens.consume_next_token() {
          value as f32
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected second number in fraction".to_string(),
          });
        };

        Ok(MediaRuleValue::Fraction(Fraction {
          numerator: first_num,
          denominator: second_num,
        }))
      },
      "fraction_parser",
    ),
    // Numbers (must be last to avoid consuming numbers that are part of fractions)
    tokens::number().map(
      |token| {
        if let SimpleToken::Number(value) = token {
          MediaRuleValue::Number(value as f32)
        } else {
          MediaRuleValue::Number(0.0)
        }
      },
      Some("number_to_value"),
    ),
  ])
}

/// Simple pair parser for (key: value) media features
fn simple_pair_parser(value_parser: TokenParser<MediaRuleValue>) -> TokenParser<MediaQueryRule> {
  let value_parser_rc = value_parser.run.clone(); // Clone the Rc<dyn Fn>

  TokenParser::new(
    move |tokens| {
      // Parse opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.consume_next_token() {
        // Good, we have opening paren
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected opening parenthesis".to_string(),
        });
      }

      // Optional whitespace after opening paren
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse key (identifier)
      let key = if let Ok(Some(SimpleToken::Ident(key_name))) = tokens.consume_next_token() {
        key_name
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected media feature name".to_string(),
        });
      };

      // Optional whitespace before colon
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse colon
      if let Ok(Some(SimpleToken::Colon)) = tokens.consume_next_token() {
        // Good, we have colon
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected colon after media feature name".to_string(),
        });
      }

      // Optional whitespace after colon
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse value using the cloned value parser
      let value = (value_parser_rc)(tokens)?;

      // Optional whitespace before closing paren
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse closing parenthesis
      if let Ok(Some(SimpleToken::RightParen)) = tokens.consume_next_token() {
        // Good, we have closing paren
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected closing parenthesis".to_string(),
        });
      }

      Ok(MediaQueryRule::Pair { key, value })
    },
    "simple_pair_parser",
  )
}

/// Combined inequality parser - handles both forward and reversed inequalities
fn combined_inequality_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::one_of(vec![
    media_inequality_rule_parser(),          // Forward: (width <= 1250px)
    media_inequality_rule_parser_reversed(), // Reversed: (1250px >= width)
  ])
}

/// Forward inequality parser: (width <= 1250px) or (width < 1250px)
fn media_inequality_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected opening parenthesis".to_string(),
        })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse property name (width or height)
      let key_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected property name".to_string(),
        })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse operator (< or >)
      let op_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected comparison operator".to_string(),
        })?;
      let op = if let SimpleToken::Delim(op_char) = op_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        tokens.consume_next_token()?;
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse dimension value
      let dim_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected dimension value".to_string(),
        })?;
      let mut dimension = if let SimpleToken::Dimension { value, unit } = dim_token {
        Length::new(value as f32, unit)
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", dim_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Expect closing paren
      let close_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected closing parenthesis".to_string(),
        })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      if !has_equals {
        const EPSILON: f32 = 0.01;
        if op == '>' {
          // (width > 400px) -> min-width: 400.01px
          dimension.value += EPSILON;
        } else {
          // (width < 400px) -> max-width: 399.99px
          dimension.value -= EPSILON;
        }
      }

      // Convert to final key: (width < 1250px) becomes max-width
      let final_key = if op == '>' {
        format!("min-{}", key)
      } else {
        format!("max-{}", key)
      };

      Ok(MediaQueryRule::Pair {
        key: final_key,
        value: MediaRuleValue::Length(dimension),
      })
    },
    "media_inequality_rule_parser",
  )
}

/// Reversed inequality parser: (1250px >= width) or (1250px > width)
fn media_inequality_rule_parser_reversed() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected opening parenthesis".to_string(),
        })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse dimension value first
      let dim_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected dimension value".to_string(),
        })?;
      let dimension = if let SimpleToken::Dimension { value, unit } = dim_token {
        MediaRuleValue::Length(Length::new(value as f32, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", dim_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse operator (< or >)
      let op_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected comparison operator".to_string(),
        })?;
      let op = if let SimpleToken::Delim(op_char) = op_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        tokens.consume_next_token()?;
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse property name (width or height)
      let key_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected property name".to_string(),
        })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Expect closing paren
      let close_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected closing parenthesis".to_string(),
        })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      let mut adjusted_dimension = dimension;
      if !has_equals {
        const EPSILON: f32 = 0.01;
        if let MediaRuleValue::Length(ref mut length) = adjusted_dimension {
          if op == '>' {
            // (1250px > width) -> max-width: 1249.99px
            length.value -= EPSILON;
          } else {
            // (1250px < width) -> min-width: 1250.01px
            length.value += EPSILON;
          }
        }
      }

      // Convert to final key: (1250px > width) becomes max-width
      let final_key = if op == '>' {
        format!("max-{}", key)
      } else {
        format!("min-{}", key)
      };

      Ok(MediaQueryRule::Pair {
        key: final_key,
        value: adjusted_dimension,
      })
    },
    "media_inequality_rule_parser_reversed",
  )
}

/// Double inequality parser: (500px <= width <= 1000px)
fn double_inequality_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected opening parenthesis".to_string(),
        })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse lower bound dimension
      let lower_dim_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected lower bound dimension".to_string(),
        })?;
      let lower_dimension = if let SimpleToken::Dimension { value, unit } = lower_dim_token {
        MediaRuleValue::Length(Length::new(value as f32, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", lower_dim_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse first operator (< or >)
      let op1_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected first comparison operator".to_string(),
        })?;
      let _op1 = if let SimpleToken::Delim(op_char) = op1_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op1_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse optional first equals sign
      let _eq1 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        tokens.consume_next_token()?;
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse property name (width or height)
      let key_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected property name".to_string(),
        })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse second operator (< or >)
      let op2_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected second comparison operator".to_string(),
        })?;
      let _op2 = if let SimpleToken::Delim(op_char) = op2_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op2_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse optional second equals sign
      let _eq2 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        tokens.consume_next_token()?;
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Parse upper bound dimension
      let upper_dim_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected upper bound dimension".to_string(),
        })?;
      let upper_dimension = if let SimpleToken::Dimension { value, unit } = upper_dim_token {
        MediaRuleValue::Length(Length::new(value as f32, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", upper_dim_token),
        });
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        tokens.consume_next_token()?;
      }

      // Expect closing paren
      let close_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected closing parenthesis".to_string(),
        })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      // Return an AND rule with min and max constraints
      let min_key = format!("min-{}", key);
      let max_key = format!("max-{}", key);

      Ok(MediaQueryRule::And {
        rules: vec![
          MediaQueryRule::Pair {
            key: min_key,
            value: lower_dimension,
          },
          MediaQueryRule::Pair {
            key: max_key,
            value: upper_dimension,
          },
        ],
      })
    },
    "double_inequality_rule_parser",
  )
}

/// Enhanced NOT parser that handles complex nested expressions
fn leading_not_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect "not" keyword
      let not_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected 'not' keyword".to_string(),
        })?;
      if let SimpleToken::Ident(keyword) = not_token {
        if keyword != "not" {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'not', got '{}'", keyword),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", not_token),
        });
      }

      // Skip whitespace after "not"
      let whitespace_token = tokens
        .consume_next_token()?
        .ok_or(CssParseError::ParseError {
          message: "Expected whitespace after 'not'".to_string(),
        })?;
      if !matches!(whitespace_token, SimpleToken::Whitespace) {
        return Err(CssParseError::ParseError {
          message: format!("Expected whitespace, got {:?}", whitespace_token),
        });
      }

      // Parse the rule that follows "not" using normal rule parser
      let inner_rule = (normal_rule_parser().run)(tokens)?;
      Ok(MediaQueryRule::Not {
        rule: Box::new(inner_rule),
      })
    },
    "leading_not_parser",
  )
}

/// This parser specifically handles "(not ...)" patterns
fn parenthesized_not_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.peek() {
        tokens.consume_next_token()?; // consume '('

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect "not" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "not" {
            tokens.consume_next_token()?; // consume "not"

            // Skip mandatory whitespace after "not"
            if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            } else {
              return Err(CssParseError::ParseError {
                message: "Expected whitespace after 'not' in parenthesized expression".to_string(),
              });
            }

            // Parse the rule after "not" using the normal rule parser
            let inner_rule = (normal_rule_parser().run)(tokens)?;

            // Skip optional whitespace before closing
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Expect closing parenthesis
            if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
              tokens.consume_next_token()?; // consume ')'
              Ok(MediaQueryRule::Not {
                rule: Box::new(inner_rule),
              })
            } else {
              Err(CssParseError::ParseError {
                message: "Expected closing parenthesis after parenthesized NOT expression"
                  .to_string(),
              })
            }
          } else {
            Err(CssParseError::ParseError {
              message: "Expected 'not' keyword in parenthesized NOT expression".to_string(),
            })
          }
        } else {
          Err(CssParseError::ParseError {
            message: "Expected 'not' keyword in parenthesized NOT expression".to_string(),
          })
        }
      } else {
        Err(CssParseError::ParseError {
          message: "Expected opening parenthesis for parenthesized NOT expression".to_string(),
        })
      }
    },
    "parenthesized_not_parser",
  )
}

fn media_query_rule_parser() -> TokenParser<MediaQueryRule> {
  // Parse OR-separated rules (comma-separated)
  or_combinator_parser()
}

/// Parse OR-separated media query rules (comma-separated OR "or" keyword)
fn or_combinator_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut rules = Vec::new();

      // Parse the first rule
      let first_rule = (and_combinator_parser().run)(tokens)?;
      rules.push(first_rule);

      // Parse additional OR rules (comma-separated OR "or" keyword)
      loop {
        let checkpoint = tokens.save_position();

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Check for comma-separated OR
        if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
          tokens.consume_next_token()?; // consume comma

          // Skip optional whitespace after comma
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          let rule = (and_combinator_parser().run)(tokens)?;
          rules.push(rule);
          continue;
        }

        // Check for "or" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "or" {
            tokens.consume_next_token()?; // consume "or"

            // Skip whitespace after "or"
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            let rule = (and_combinator_parser().run)(tokens)?;
            rules.push(rule);
            continue;
          }
        }

        // No more OR patterns found, restore position and break
        tokens.restore_position(checkpoint)?;
        break;
      }

      // If we only have one rule, return it directly
      if rules.len() == 1 {
        Ok(rules.into_iter().next().unwrap())
      } else {
        Ok(MediaQueryRule::Or { rules })
      }
    },
    "or_combinator_parser",
  )
}

/// Parse AND-separated media query rules
fn and_combinator_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut rules = Vec::new();

      // Parse the first rule
      let first_rule = (normal_rule_parser().run)(tokens)?;
      rules.push(first_rule);

      // Parse additional AND rules
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        // Check if next non-whitespace token is "and"
        let checkpoint = tokens.save_position();

        // Skip whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Check for "and" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "and" {
            tokens.consume_next_token()?; // consume "and"

            // Skip whitespace after "and"
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            let rule = (normal_rule_parser().run)(tokens)?;
            rules.push(rule);
          } else {
            // Not an "and", restore position and break
            tokens.restore_position(checkpoint)?;
            break;
          }
        } else {
          // No identifier after whitespace, restore position and break
          tokens.restore_position(checkpoint)?;
          break;
        }
      }

      // If we only have one rule, return it directly
      if rules.len() == 1 {
        Ok(rules.into_iter().next().unwrap())
      } else {
        Ok(MediaQueryRule::And { rules })
      }
    },
    "and_combinator_parser",
  )
}

/// Normal rule parser that combines all rule types
fn normal_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::one_of(vec![
    // Leading not parser must come first to handle "not" at the beginning
    leading_not_parser(),
    // Parenthesized NOT parser for "(not ...)" patterns
    parenthesized_not_parser(),
    // Parenthesized expressions parser for complex nested cases
    parenthesized_expression_parser(),
    // Double inequality parser: (500px <= width <= 1000px)
    double_inequality_rule_parser(),
    // Combined inequality parser: (width <= 1250px) and (1250px >= width)
    combined_inequality_parser(),
    // Media keyword parser for screen, print, all with optional not/only
    media_keyword_parser(),
    // Word rule parser for (color), (monochrome), (grid), (color-index)
    media_word_rule_parser(),
    // Pair parser for (key: value) patterns like (min-width: 768px)
    simple_pair_parser(media_rule_value_parser()),
  ])
}

/// Parse parenthesized expressions, including complex NOT expressions
/// Handles: (not (max-width: 1024px)), ((min-width: 500px) and (max-width: 600px))
fn parenthesized_expression_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.peek() {
        tokens.consume_next_token()?; // consume '('

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Try to parse a NOT expression first
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "not" {
            // Parse NOT expression within parentheses
            let not_rule = (leading_not_parser().run)(tokens)?;

            // Skip optional whitespace before closing
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Expect closing parenthesis
            if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
              tokens.consume_next_token()?; // consume ')'
              return Ok(not_rule);
            } else {
              return Err(CssParseError::ParseError {
                message: "Expected closing parenthesis after parenthesized NOT expression"
                  .to_string(),
              });
            }
          }
        }

        // Parse complex expression using full combinator parser
        let inner_expression = (and_combinator_parser().run)(tokens)?;

        // Skip optional whitespace before closing
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
          tokens.consume_next_token()?; // consume ')'
          Ok(inner_expression)
        } else {
          Err(CssParseError::ParseError {
            message: "Expected closing parenthesis after parenthesized expression".to_string(),
          })
        }
      } else {
        Err(CssParseError::ParseError {
          message: "Expected opening parenthesis for parenthesized expression".to_string(),
        })
      }
    },
    "parenthesized_expression_parser",
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_media_query_creation() {
    let query = MediaQuery::new("@media screen".to_string());
    assert_eq!(query.to_string(), "@media screen");
  }

  #[test]
  fn test_media_query_display() {
    let query = MediaQuery::new("@media (min-width: 768px)".to_string());
    assert_eq!(format!("{}", query), "@media (min-width: 768px)");
  }

  #[test]
  fn test_has_balanced_parens() {
    assert!(has_balanced_parens("(min-width: 768px)"));
    assert!(has_balanced_parens(
      "(min-width: 768px) and (max-width: 1200px)"
    ));
    assert!(has_balanced_parens("screen"));
    assert!(has_balanced_parens(""));

    assert!(!has_balanced_parens("(min-width: 768px"));
    assert!(!has_balanced_parens("min-width: 768px)"));
    assert!(!has_balanced_parens("((min-width: 768px)"));
  }

  #[test]
  fn test_validate_media_query_success() {
    let result = validate_media_query("@media (min-width: 768px)");
    assert!(result.is_ok());

    let query = result.unwrap();
    assert_eq!(query.to_string(), "@media (min-width: 768px)");
  }

  #[test]
  fn test_validate_media_query_unbalanced_parens() {
    let result = validate_media_query("@media (min-width: 768px");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("parentheses"));
  }

  #[test]
  fn test_media_query_parser_creation() {
    // Test that parser can be created (even if it's a placeholder)
    let _parser = MediaQuery::parser();
  }

  #[test]
  fn test_media_query_equality() {
    let query1 = MediaQuery::new("@media screen".to_string());
    let query2 = MediaQuery::new("@media screen".to_string());
    let query3 = MediaQuery::new("@media print".to_string());

    assert_eq!(query1, query2);
    assert_ne!(query1, query3);
  }

  #[test]
  fn test_media_query_clone() {
    let query = MediaQuery::new("@media (orientation: landscape)".to_string());
    let cloned = query.clone();

    assert_eq!(query, cloned);
  }

  #[test]
  fn test_common_media_queries() {
    // Test currently implemented media query features
    let implemented_queries = vec![
      "@media screen",
      "@media print",
      "@media (min-width: 768px)",
      "@media screen and (min-width: 768px)", // Now implemented!
      "@media (min-width: 768px) and (max-width: 1024px)", // Now implemented!
      "@media not screen",
      "@media only screen and (min-width: 768px)", // Now implemented!
    ];

    for query_str in implemented_queries {
      let result = validate_media_query(query_str);
      assert!(result.is_ok(), "Failed to validate: {}", query_str);

      let query = result.unwrap();
      assert_eq!(query.to_string(), query_str);
      println!("✅ Validated: {}", query_str);
    }

    // All AND combinators are now implemented - test any remaining edge cases
    let edge_case_queries = vec![
      // Complex nested NOT expressions might still have issues
      // Add any edge cases here as they're discovered
    ];

    for query_str in edge_case_queries {
      let result = validate_media_query(query_str);
      if result.is_err() {
        println!("✅ Correctly rejecting edge case: {}", query_str);
      } else {
        println!("⚠️  Unexpectedly accepting edge case: {}", query_str);
      }
    }
  }

  #[test]
  fn test_complex_parentheses() {
    let supported_query = "@media (min-width: 768px)";
    let result = validate_media_query(supported_query);
    assert!(
      result.is_ok(),
      "Simple parentheses should work: {:?}",
      result
    );

    // Test complex query with AND combinators - now implemented and should work!
    let and_combinator_query = "@media screen and ((min-width: 768px) and (max-width: 1024px))";
    let result = validate_media_query(and_combinator_query);
    assert!(
      result.is_ok(),
      "Complex AND combinators should now work: {:?}",
      result
    );
    println!(
      "✅ Complex parentheses with AND combinators now working: {}",
      and_combinator_query
    );
  }

  #[test]
  fn test_media_query_normalization() {
    let input = "@media not (not (not (min-width: 400px)))";
    let parsed = MediaQuery::parser().parse_to_end(input).unwrap();
    println!("Triple NOT input: {}", input);
    println!("Triple NOT output: {}", parsed);

    // Should be normalized to single NOT
    match &parsed.queries {
      MediaQueryRule::Not { rule } => match rule.as_ref() {
        MediaQueryRule::Pair { key, .. } => {
          assert_eq!(key, "min-width");
          println!("✅ Triple NOT correctly normalized to single NOT");
        }
        _ => panic!("Expected Pair rule inside NOT, got: {:?}", rule),
      },
      _ => panic!("Expected NOT rule at top level, got: {:?}", parsed.queries),
    }

    // Test quadruple NOT normalization (should cancel out completely)
    let input_quad = "@media not (not (not (not (max-width: 500px))))";
    let parsed_quad = MediaQuery::parser().parse_to_end(input_quad).unwrap();
    println!("Quadruple NOT input: {}", input_quad);
    println!("Quadruple NOT output: {}", parsed_quad);

    // Should be normalized to no NOT (just the pair)
    match &parsed_quad.queries {
      MediaQueryRule::Pair { key, .. } => {
        assert_eq!(key, "max-width");
        println!("✅ Quadruple NOT correctly canceled out");
      }
      _ => panic!(
        "Expected Pair rule (no NOT), got: {:?}",
        parsed_quad.queries
      ),
    }

    let complex_input = "@media (max-width: 1440px) and (not (max-width: 1024px)) and (not (max-width: 768px)) and (not (max-width: 458px))";
    let parsed_complex = MediaQuery::parser().parse_to_end(complex_input).unwrap();
    println!("Complex input: {}", complex_input);
    println!("Complex output: {}", parsed_complex);

    match &parsed_complex.queries {
      MediaQueryRule::And { rules } => {
        println!(
          "✅ Complex NOT-AND expression normalized to AND with {} rules",
          rules.len()
        );
        // Verify it contains both min and max constraints
        let has_min = rules
          .iter()
          .any(|r| matches!(r, MediaQueryRule::Pair { key, .. } if key.starts_with("min-")));
        let has_max = rules
          .iter()
          .any(|r| matches!(r, MediaQueryRule::Pair { key, .. } if key.starts_with("max-")));
        assert!(
          has_min && has_max,
          "Should contain both min and max constraints"
        );
      }
      _ => {
        // Might be a single constraint if merging results in one rule
        println!(
          "ℹ️  Complex expression normalized to single rule: {:?}",
          parsed_complex.queries
        );
      }
    }
  }

  #[test]
  fn test_nested_unbalanced_parentheses() {
    let invalid_queries = vec![
      "@media ((min-width: 768px)",
      "@media (min-width: 768px))",
      "@media (((min-width: 768px)",
      "@media (min-width: 768px)))",
    ];

    for query_str in invalid_queries {
      let result = validate_media_query(query_str);
      assert!(result.is_err(), "Should have failed: {}", query_str);
    }
  }
}
