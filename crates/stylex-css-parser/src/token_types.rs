/*!
Token types and tokenization utilities for CSS parsing.

This module provides CSS tokenization capabilities using the cssparser crate,
mirroring the functionality of the JavaScript TokenList and TokenIterator.
*/

use crate::CssResult;
use cssparser::{Parser, ParserInput, Token as CssToken};

/// Simple token representation (mirrors @csstools/css-tokenizer kinds)
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
    // Unsupported/less common tokens mapped to Unknown for now
    CssToken::BadUrl(_) | CssToken::BadString(_) => Some(T::Unknown(format!("{:?}", token))),
    _ => Some(T::Unknown(format!("{:?}", token))),
  }
}

fn tokenize_all(input: &str) -> Vec<SimpleToken> {
  let mut input_buf = ParserInput::new(input);
  let mut parser = Parser::new(&mut input_buf);

  let mut tokens = Vec::new();
  while let Ok(t) = parser.next_including_whitespace_and_comments() {
    if let Some(mapped) = map_css_token(&t) {
      tokens.push(mapped);
    }
  }
  tokens
}

/// A list of CSS tokens with parsing state, mirroring the JavaScript TokenList class
pub struct TokenList {
  tokens: Vec<SimpleToken>,
  pub current_index: usize,
}

impl TokenList {
  /// Create a new TokenList from a CSS string
  /// Mirrors: constructor(input: TokenIterator | string)
  pub fn new(input: &str) -> Self {
    Self {
      tokens: tokenize_all(input),
      current_index: 0,
    }
  }

  /// Consume the next token
  /// Mirrors: consumeNextToken(): CSSToken | null
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
  /// Mirrors: peek(): CSSToken | null
  pub fn peek(&mut self) -> CssResult<Option<SimpleToken>> {
    if self.current_index < self.tokens.len() {
      Ok(Some(self.tokens[self.current_index].clone()))
    } else {
      Ok(None)
    }
  }

  /// Get the first token (alias for peek)
  pub fn first(&mut self) -> CssResult<Option<SimpleToken>> {
    self.peek()
  }

  /// Set the current parsing index
  /// Mirrors: setCurrentIndex(newIndex: number): void
  pub fn set_current_index(&mut self, new_index: usize) {
    self.current_index = new_index.min(self.tokens.len());
  }

  /// Rewind the parser by a number of positions
  /// Mirrors: rewind(positions: number = 1): void
  pub fn rewind(&mut self, positions: usize) {
    self.current_index = self.current_index.saturating_sub(positions);
  }

  /// Check if the token list is empty
  /// Mirrors: get isEmpty(): boolean
  pub fn is_empty(&self) -> bool {
    self.current_index >= self.tokens.len()
  }

  /// Get all tokens
  pub fn get_all_tokens(&mut self) -> Vec<SimpleToken> {
    self.tokens.clone()
  }

  /// Get a slice of tokens from start to end (exclusive)
  /// Mirrors: slice(start: number, end: number = this.currentIndex): Array<CSSToken>
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
    assert_eq!(list.peek().unwrap().is_some(), true);
    assert!(list.get_all_tokens().len() > 0);
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
