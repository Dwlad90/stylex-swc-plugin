/*!
Token types and tokenization utilities for CSS parsing.
*/

use crate::CssResult;
use cssparser::{Parser, ParserInput, Token as CssToken};
use log::error;
use stylex_macros::stylex_panic;

/// Simple token representation
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleToken {
  Ident(String),
  AtKeyword(String),
  Hash(String),
  String(String),
  Number(f64),
  Dimension { value: f64, unit: String },
  Percentage(f64),
  Url(String),
  Function(String),
  Delim(char),
  LeftParen,
  RightParen,
  LeftBracket,
  RightBracket,
  LeftBrace,
  RightBrace,
  Comma,
  Semicolon,
  Colon,
  Whitespace,
  Comment(String),
  Unknown(String),
}

impl SimpleToken {
  /// Extract token value
  pub fn extract_value(&self) -> Option<String> {
    match self {
      SimpleToken::Function(name) => Some(name.clone()),
      SimpleToken::Ident(value) => Some(value.clone()),
      SimpleToken::String(value) => Some(value.clone()),
      SimpleToken::Hash(value) => Some(value.clone()),
      SimpleToken::AtKeyword(value) => Some(value.clone()),
      SimpleToken::Comment(value) => Some(value.clone()),
      SimpleToken::Number(value) => Some(value.to_string()),
      SimpleToken::Percentage(value) => Some(value.to_string()),
      SimpleToken::Dimension { value, unit } => Some(format!("{}{}", value, unit)),
      SimpleToken::Delim(ch) => Some(ch.to_string()),
      SimpleToken::Unknown(value) => Some(value.clone()),
      _ => None, // No extractable value for structural tokens
    }
  }

  /// Extract numeric value for Number and Percentage tokens
  pub fn extract_number(&self) -> Option<f64> {
    match self {
      SimpleToken::Number(value) => Some(*value),
      SimpleToken::Percentage(value) => Some(*value),
      SimpleToken::Dimension { value, .. } => Some(*value),
      _ => None,
    }
  }
}

// `map_css_token` is total: every `CssToken` maps to a `SimpleToken` (the
// wildcard arm falls back to `SimpleToken::Unknown`), so it returns the token
// directly rather than an `Option`.
fn map_css_token(token: &CssToken) -> SimpleToken {
  use SimpleToken as T;
  match token {
    CssToken::Ident(v) => T::Ident(v.as_ref().to_string()),
    CssToken::AtKeyword(v) => T::AtKeyword(v.as_ref().to_string()),
    CssToken::IDHash(v) | CssToken::Hash(v) => T::Hash(v.as_ref().to_string()),
    CssToken::QuotedString(v) => T::String(v.as_ref().to_string()),
    CssToken::Number { value, .. } => T::Number(*value as f64),
    CssToken::Percentage { unit_value, .. } => T::Percentage(*unit_value as f64),
    CssToken::Dimension { value, unit, .. } => T::Dimension {
      value: *value as f64,
      unit: unit.as_ref().to_string(),
    },
    CssToken::Function(v) => T::Function(v.as_ref().to_string()),
    // Map parenthesis via Delim tokens if present
    CssToken::Delim('(') => T::LeftParen,
    CssToken::Delim(')') => T::RightParen,
    CssToken::Delim(c) => T::Delim(*c),
    CssToken::WhiteSpace(_) => T::Whitespace,
    CssToken::Comma => T::Comma,
    CssToken::Colon => T::Colon,
    CssToken::Semicolon => T::Semicolon,
    CssToken::BadUrl(_) | CssToken::BadString(_) => T::Unknown(format!("{:?}", token)),
    CssToken::UnquotedUrl(url) => T::String(url.as_ref().to_string()),
    CssToken::CloseParenthesis => T::RightParen,
    CssToken::SquareBracketBlock => T::Delim('['),
    CssToken::CloseSquareBracket => T::Delim(']'),
    CssToken::CurlyBracketBlock => T::Delim('{'),
    CssToken::CloseCurlyBracket => T::Delim('}'),
    CssToken::CDC => T::Delim('>'), // --> CSS comment close
    CssToken::CDO => T::Delim('<'), // <!-- CSS comment open

    // Remaining tokens mapped to Unknown (e.g., future cssparser additions)
    _ => T::Unknown(format!("{:?}", token)),
  }
}

/// Descend into a block/function with `parse_nested_block`, panicking if the
/// nested parse fails.
///
/// All call sites pass a closure that tokenizes the nested content and returns
/// `Ok(())`, so in normal operation this never panics. The panic is a defensive
/// guard against a malformed nested block surfacing a `cssparser` error.
fn parse_nested_or_panic<'i, 't, F>(parser: &mut Parser<'i, 't>, parse: F)
where
  F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<(), cssparser::ParseError<'i, ()>>,
{
  // The error-handling branch is deliberately kept in a non-generic helper. If
  // it lived here, every monomorphization of this function (one per closure
  // type at each call site) would report the unreached arm as an uncovered
  // region — a "phantom" gap that no single instantiation covers. Routing the
  // `Result` through one non-generic function collapses that coverage into a
  // single instantiation exercised by both the success and the error path.
  handle_nested_block_result(parser.parse_nested_block(parse));
}

/// Panic (with diagnostics) when a nested `cssparser` block failed to parse.
///
/// Non-generic on purpose — see `parse_nested_or_panic`.
fn handle_nested_block_result(result: Result<(), cssparser::ParseError<'_, ()>>) {
  if let Err(e) = result {
    error!("Error parsing nested content: {:?}", e);
    stylex_panic!("Error parsing nested content: {:?}", e); // Exit on error
  }
}

/// Recursively tokenize nested content, handling ParenthesisBlock and other
/// nested structures
fn tokenize_nested_content(parser: &mut Parser, tokens: &mut Vec<SimpleToken>) {
  while let Ok(inner_token) = parser.next_including_whitespace_and_comments() {
    match &inner_token {
      // Handle nested ParenthesisBlock recursively
      CssToken::ParenthesisBlock => {
        // Add opening parenthesis
        tokens.push(SimpleToken::LeftParen);

        // Parse the nested parenthesis content recursively
        parse_nested_or_panic(parser, |nested_parser| {
          tokenize_nested_content(nested_parser, tokens);
          Ok(())
        });

        // Add closing parenthesis
        tokens.push(SimpleToken::RightParen);
      },
      // Handle nested Function tokens
      CssToken::Function(func_name) => {
        // Add the function name token
        tokens.push(SimpleToken::Function(func_name.as_ref().to_string()));

        // Parse the function content recursively
        parse_nested_or_panic(parser, |nested_parser| {
          tokenize_nested_content(nested_parser, tokens);
          Ok(())
        });

        // Add closing paren token
        tokens.push(SimpleToken::RightParen);
      },
      // Handle all other tokens normally
      _ => {
        tokens.push(map_css_token(inner_token));
      },
    }
  }
}

fn tokenize_all(input: &str) -> Vec<SimpleToken> {
  let mut input_buf = ParserInput::new(input);
  let mut parser = Parser::new(&mut input_buf);

  let mut tokens = Vec::new();
  while let Ok(t) = parser.next_including_whitespace_and_comments() {
    match &t {
      // ENHANCED: Handle Function tokens by expanding their content
      CssToken::Function(func_name) => {
        // Add the function name token first
        tokens.push(SimpleToken::Function(func_name.as_ref().to_string()));

        // Parse the function content to get individual argument tokens
        parse_nested_or_panic(&mut parser, |nested_parser| {
          // Recursively tokenize everything inside the function parentheses
          tokenize_nested_content(nested_parser, &mut tokens);
          Ok(())
        });

        // Add closing paren token (cssparser consumes it automatically)
        tokens.push(SimpleToken::RightParen);
      },
      // ENHANCED: Handle ParenthesisBlock tokens by expanding their content
      CssToken::ParenthesisBlock => {
        // Add opening parenthesis
        tokens.push(SimpleToken::LeftParen);

        // Parse the parenthesis content to get individual tokens
        parse_nested_or_panic(&mut parser, |nested_parser| {
          // Recursively tokenize everything inside the parentheses, handling nested
          // structures
          tokenize_nested_content(nested_parser, &mut tokens);
          Ok(())
        });

        // Add closing parenthesis (cssparser consumes it automatically)
        tokens.push(SimpleToken::RightParen);
      },
      // Handle all other tokens normally
      _ => {
        tokens.push(map_css_token(t));
      },
    }
  }
  tokens
}

/// A list of CSS tokens with parsing state
pub struct TokenList {
  pub tokens: Vec<SimpleToken>, // Made public for debugging
  pub current_index: usize,
}

impl TokenList {
  /// Create a new TokenList from a CSS string
  pub fn new(input: &str) -> Self {
    Self {
      tokens: tokenize_all(input),
      current_index: 0,
    }
  }

  /// Consume the next token
  pub fn consume_next_token(&mut self) -> CssResult<Option<SimpleToken>> {
    if self.current_index < self.tokens.len() {
      let token = self.tokens[self.current_index].clone();
      self.current_index += 1;
      Ok(Some(token))
    } else {
      Ok(None)
    }
  }

  /// Consume the next token from an in-memory TokenList.
  ///
  /// TokenList::consume_next_token is backed by a Vec<SimpleToken> and always
  /// returns Ok(…); the Result wrapper exists only for trait compatibility.
  /// This wrapper makes the infallibility explicit so call sites don't need a
  /// `?` operator whose Err branch can never be reached.
  pub fn consume_next_token_infallible(&mut self) -> Option<SimpleToken> {
    self.consume_next_token().ok().flatten()
  }

  /// Peek at the next token without consuming it, returning `None` at
  /// end-of-input.
  ///
  /// `TokenList::peek()` always returns `Ok(Some(...))` or `Ok(None)`.
  /// `.ok().flatten()` converts the infallible `Result` to a plain `Option`,
  /// avoiding an uncovered Err-propagation region.
  pub fn peek_infallible(&mut self) -> Option<SimpleToken> {
    self.peek().ok().flatten()
  }

  /// Peek at the next token without consuming it
  pub fn peek(&mut self) -> CssResult<Option<SimpleToken>> {
    if self.current_index < self.tokens.len() {
      Ok(Some(self.tokens[self.current_index].clone()))
    } else {
      Ok(None)
    }
  }

  /// Save the current position for potential rollback
  pub fn save_position(&self) -> usize {
    self.current_index
  }

  /// Restore to a previously saved position
  pub fn restore_position(&mut self, position: usize) -> CssResult<()> {
    if position <= self.tokens.len() {
      self.current_index = position;
      Ok(())
    } else {
      Err(crate::CssParseError::ParseError {
        message: "Invalid position for restore".to_string(),
      })
    }
  }

  /// Get the first token (alias for peek)
  pub fn first(&mut self) -> CssResult<Option<SimpleToken>> {
    self.peek()
  }

  /// Set the current parsing index
  pub fn set_current_index(&mut self, new_index: usize) {
    self.current_index = new_index.min(self.tokens.len());
  }

  /// Rewind the parser by a number of positions
  pub fn rewind(&mut self, positions: usize) {
    self.current_index = self.current_index.saturating_sub(positions);
  }

  /// Check if the token list is empty
  pub fn is_empty(&self) -> bool {
    self.current_index >= self.tokens.len()
  }

  /// Get all tokens
  pub fn get_all_tokens(&mut self) -> Vec<SimpleToken> {
    self.tokens.clone()
  }

  /// Get a slice of tokens from start to end index
  pub fn slice(&mut self, start: usize, end: Option<usize>) -> Vec<SimpleToken> {
    let end = end.unwrap_or(self.current_index);
    if start >= end || start >= self.tokens.len() {
      return Vec::new();
    }
    self.tokens[start..end.min(self.tokens.len())].to_vec()
  }
}

#[cfg(test)]
#[path = "tests/token_types_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/token_types_test.rs"]
mod token_types_test;

#[cfg(test)]
#[path = "tests/token_types_coverage_test.rs"]
mod token_types_coverage_test;
