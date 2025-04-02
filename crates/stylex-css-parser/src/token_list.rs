use anyhow::Error;
use cssparser::{Parser, ParserInput, Token};

use crate::resolvers::parse_tokens_from_parser;

#[derive(Debug)]
pub struct TokenList<'a> {
  pub(crate) tokens: Vec<Token<'a>>,
  pub(crate) consumed_tokens: Vec<Token<'a>>,
  pub(crate) current_index: usize,
  pub(crate) is_at_end: bool,
}

impl<'a> TokenList<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut input_buffer = ParserInput::<'a>::new(input);
    let mut css_parser = Parser::new(&mut input_buffer);

    let tokens = match parse_tokens_from_parser(&mut css_parser) {
      Ok(tokens) => tokens,
      Err(_) => panic!("Failed to parse tokens"),
    };

    Self {
      tokens,
      consumed_tokens: Vec::new(),
      current_index: 0,
      is_at_end: false,
    }
  }
  pub fn consume_next_token(&mut self) -> Result<Option<Token<'a>>, Error> {
    dbg!(&self.current_index);
    if self.current_index < self.consumed_tokens.len() {
      let token = self.consumed_tokens[self.current_index].clone();

      self.current_index += 1;

      return Ok(Some(token));
    }

    if self.is_at_end || self.tokens.is_empty() {
      self.is_at_end = true;
      return Ok(None);
    }

    let token = self.tokens.remove(0);
    dbg!(&token);

    self.consumed_tokens.push(token.clone());
    self.current_index += 1;

    if self.tokens.is_empty() {
      self.is_at_end = true;
    }

    Ok(Some(token))
  }

  pub fn peek(&mut self) -> Result<Option<Token<'a>>, Error> {
    if self.current_index < self.consumed_tokens.len() {
      return Ok(Some(self.consumed_tokens[self.current_index].clone()));
    }

    if self.is_at_end || self.current_index >= self.tokens.len() {
      return Ok(None);
    }

    let token = self.tokens[self.current_index].clone();

    self.consumed_tokens.push(token.clone());

    Ok(Some(token))
  }

  pub fn first(&mut self) -> Result<Option<Token<'a>>, Error> {
    self.peek()
  }

  pub fn set_current_index(&mut self, new_index: usize) {
    if new_index < self.consumed_tokens.len() {
      self.current_index = new_index;
      return;
    }

    if new_index >= self.tokens.len() {
      while self.consumed_tokens.len() < self.tokens.len() {
        let idx = self.consumed_tokens.len();
        self.consumed_tokens.push(self.tokens[idx].clone());
      }

      self.current_index = self.consumed_tokens.len();
      self.is_at_end = true;
      return;
    }

    while self.consumed_tokens.len() <= new_index {
      let idx = self.consumed_tokens.len();
      if idx < self.tokens.len() {
        self.consumed_tokens.push(self.tokens[idx].clone());
      } else {
        self.is_at_end = true;
        break;
      }
    }

    self.current_index = std::cmp::min(new_index, self.consumed_tokens.len());
  }

  pub fn rewind(&mut self, positions: usize) {
    self.current_index = self.current_index.saturating_sub(positions);
  }

  pub fn is_empty(&self) -> bool {
    self.is_at_end || (self.current_index >= self.consumed_tokens.len() && self.tokens.is_empty())
  }

  pub fn get_all_tokens(&mut self) -> &[Token<'a>] {
    while !self.is_empty() {
      let _ = self.consume_next_token();
    }
    &self.consumed_tokens
  }

  pub fn slice(&mut self, start: usize, end: Option<usize>) -> Vec<Option<Token<'a>>> {
    let end = end.unwrap_or(self.current_index);
    let initial_index = self.current_index;

    if start >= end {
      return Vec::new();
    }

    self.set_current_index(start);
    let mut result = Vec::new();

    while self.current_index < end {
      if let Ok(token) = self.consume_next_token() {
        result.push(token);
      } else {
        break;
      }
    }

    self.set_current_index(initial_index);
    result
  }
}
