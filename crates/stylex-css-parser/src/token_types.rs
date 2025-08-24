/*!
Token types and tokenization utilities for CSS parsing.
*/

use crate::CssResult;
use cssparser::{Parser, ParserInput, Token as CssToken};
use log::error;

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

fn map_css_token(token: &CssToken) -> Option<SimpleToken> {
  use SimpleToken as T;
  match token {
    CssToken::Ident(v) => Some(T::Ident(v.as_ref().to_string())),
    CssToken::AtKeyword(v) => Some(T::AtKeyword(v.as_ref().to_string())),
    CssToken::IDHash(v) | CssToken::Hash(v) => Some(T::Hash(v.as_ref().to_string())),
    CssToken::QuotedString(v) => Some(T::String(v.as_ref().to_string())),
    CssToken::Number { value, .. } => Some(T::Number(*value as f64)),
    CssToken::Percentage { unit_value, .. } => Some(T::Percentage(*unit_value as f64)),
    CssToken::Dimension { value, unit, .. } => Some(T::Dimension {
      value: *value as f64,
      unit: unit.as_ref().to_string(),
    }),
    CssToken::Function(v) => Some(T::Function(v.as_ref().to_string())),
    // Map parenthesis via Delim tokens if present
    CssToken::Delim('(') => Some(T::LeftParen),
    CssToken::Delim(')') => Some(T::RightParen),
    CssToken::Delim(c) => Some(T::Delim(*c)),
    CssToken::WhiteSpace(_) => Some(T::Whitespace),
    CssToken::Comma => Some(T::Comma),
    CssToken::Colon => Some(T::Colon),
    CssToken::Semicolon => Some(T::Semicolon),
    CssToken::BadUrl(_) | CssToken::BadString(_) => Some(T::Unknown(format!("{:?}", token))),
    CssToken::UnquotedUrl(url) => Some(T::String(url.as_ref().to_string())),
    CssToken::CloseParenthesis => Some(T::RightParen),
    CssToken::SquareBracketBlock => Some(T::Delim('[')),
    CssToken::CloseSquareBracket => Some(T::Delim(']')),
    CssToken::CurlyBracketBlock => Some(T::Delim('{')),
    CssToken::CloseCurlyBracket => Some(T::Delim('}')),
    CssToken::CDC => Some(T::Delim('>')), // --> CSS comment close
    CssToken::CDO => Some(T::Delim('<')), // <!-- CSS comment open

    // Remaining tokens mapped to Unknown (e.g., future cssparser additions)
    _ => Some(T::Unknown(format!("{:?}", token))),
  }
}

/// Recursively tokenize nested content, handling ParenthesisBlock and other nested structures
fn tokenize_nested_content(parser: &mut Parser, tokens: &mut Vec<SimpleToken>) {
  while let Ok(inner_token) = parser.next_including_whitespace_and_comments() {
    match &inner_token {
      // Handle nested ParenthesisBlock recursively
      CssToken::ParenthesisBlock => {
        // Add opening parenthesis
        tokens.push(SimpleToken::LeftParen);

        // Parse the nested parenthesis content recursively
        if let Err(e) = parser.parse_nested_block(|nested_parser| {
          tokenize_nested_content(nested_parser, tokens);
          Ok::<(), cssparser::ParseError<()>>(())
        }) {
          error!("Error parsing nested content: {:?}", e);
          panic!("Error parsing nested content: {:?}", e); // Exit on error
        }

        // Add closing parenthesis
        tokens.push(SimpleToken::RightParen);
      }
      // Handle nested Function tokens
      CssToken::Function(func_name) => {
        // Add the function name token
        tokens.push(SimpleToken::Function(func_name.as_ref().to_string()));

        // Parse the function content recursively
        if let Err(e) = parser.parse_nested_block(|nested_parser| {
          tokenize_nested_content(nested_parser, tokens);
          Ok::<(), cssparser::ParseError<()>>(())
        }) {
          error!("Error parsing nested content: {:?}", e);
          panic!("Error parsing nested content: {:?}", e); // Exit on error
        }

        // Add closing paren token
        tokens.push(SimpleToken::RightParen);
      }
      // Handle all other tokens normally
      _ => {
        if let Some(mapped_inner) = map_css_token(inner_token) {
          tokens.push(mapped_inner);
        }
      }
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
        if let Err(e) = parser.parse_nested_block(|nested_parser| {
          // Recursively tokenize everything inside the function parentheses
          tokenize_nested_content(nested_parser, &mut tokens);
          Ok::<(), cssparser::ParseError<()>>(())
        }) {
          error!("Error parsing nested content: {:?}", e);
          panic!("Error parsing nested content: {:?}", e); // Exit on error
        }

        // Add closing paren token (cssparser consumes it automatically)
        tokens.push(SimpleToken::RightParen);
      }
      // ENHANCED: Handle ParenthesisBlock tokens by expanding their content
      CssToken::ParenthesisBlock => {
        // Add opening parenthesis
        tokens.push(SimpleToken::LeftParen);

        // Parse the parenthesis content to get individual tokens
        if let Err(e) = parser.parse_nested_block(|nested_parser| {
          // Recursively tokenize everything inside the parentheses, handling nested structures
          tokenize_nested_content(nested_parser, &mut tokens);
          Ok::<(), cssparser::ParseError<()>>(())
        }) {
          error!("Error parsing nested content: {:?}", e);
          panic!("Error parsing nested content: {:?}", e); // Exit on error
        }

        // Add closing parenthesis (cssparser consumes it automatically)
        tokens.push(SimpleToken::RightParen);
      }
      // Handle all other tokens normally
      _ => {
        if let Some(mapped) = map_css_token(t) {
          tokens.push(mapped);
        }
      }
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
mod tests {
  use super::*;

  #[test]
  fn test_basic_tokenization() {
    let mut list = TokenList::new("color: red;\nbackground: rgb(1, 2, 3)");
    assert!(list.peek().unwrap().is_some());
    assert!(!list.get_all_tokens().is_empty());
  }

  #[test]
  fn test_token_list_basic_peek_consume() {
    let mut list = TokenList::new("color: red;");
    let first = list.peek().unwrap();
    assert_eq!(first, Some(SimpleToken::Ident("color".to_string())));
    let consumed = list.consume_next_token().unwrap();
    assert_eq!(consumed, Some(SimpleToken::Ident("color".to_string())));
    let second = list.peek().unwrap();
    assert_eq!(second, Some(SimpleToken::Colon));
  }

  #[test]
  fn test_rewind_and_slice() {
    let mut list = TokenList::new("a : b ; c");
    list.consume_next_token().unwrap(); // a
    list.consume_next_token().unwrap(); // :
    list.rewind(1);
    // With cssparser-backed tokenizer, whitespace tokens are preserved
    assert_eq!(list.peek().unwrap(), Some(SimpleToken::Whitespace));

    let slice = list.slice(1, Some(4));
    assert_eq!(slice.len(), 3);
    // slice should include whitespace, then colon, then whitespace
    assert_eq!(slice[0], SimpleToken::Whitespace);
    assert_eq!(slice[1], SimpleToken::Colon);
  }
}
