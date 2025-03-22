use anyhow::{bail, Error};
use cssparser::{ParseError, Parser, ParserInput, Token};

use crate::resolvers::parse_css;

/// TokenIterator trait defines the interface for a token stream
pub trait TokenIterator {
  /// Get the next token from the stream
  fn next_token(&mut self) -> Option<Token<'static>>;

  /// Check if we've reached the end of the token stream
  fn end_of_file(&self) -> bool;
}

/// A CSS token iterator implementation using cssparser
pub struct CssTokenIterator<'a> {
  parser: Parser<'a, 'a>,
  exhausted: bool,
}

impl<'i> CssTokenIterator<'i> {
  /// Create a new token iterator from a CSS string
  pub fn new(css: &'i str) -> Result<Vec<String>, ParseError<'_, ()>> {
    parse_css(css)
  }
}

// impl<'i> TokenIterator for CssTokenIterator<'i> {
//   fn next_token(&mut self) -> Option<Token<'static>> {
//     if self.exhausted {
//       return None;
//     }

//     // Try to get the next token
//     let token = match self.parser.next() {
//       Ok(token) => token,
//       Err(_) => {
//         self.exhausted = true;
//         return None;
//       }
//     };

//     // Convert to 'static lifetime - this requires cloning the token data
//     // which may not be the most efficient but ensures safe handling
//     Some(owned_token(token))
//   }

//   fn end_of_file(&self) -> bool {
//     self.exhausted || self.parser.is_exhausted()
//   }
// }

/// Convert a cssparser Token to a 'static lifetime
// fn owned_token(token: Token) -> Token<'static> {
//   match token {
//     Token::Ident(s) => Token::Ident(s.to_owned().into()),
//     Token::Function(s) => Token::Function(s.to_owned().into()),
//     Token::AtKeyword(s) => Token::AtKeyword(s.to_owned().into()),
//     Token::Hash(s) => Token::Hash(s.to_owned().into()),
//     Token::IDHash(s) => Token::IDHash(s.to_owned().into()),
//     Token::QuotedString(s) => Token::QuotedString(s.to_owned().into()),
//     Token::UnquotedUrl(s) => Token::UnquotedUrl(s.to_owned().into()),
//     // Simple tokens that don't contain string data can be copied directly
//     Token::Delim(c) => Token::Delim(c),
//     Token::Number { value, int_value } => Token::Number { value, int_value },
//     Token::Percentage {
//       unit_value,
//       int_value,
//     } => Token::Percentage {
//       unit_value,
//       int_value,
//     },
//     Token::Dimension {
//       value,
//       int_value,
//       unit,
//     } => Token::Dimension {
//       value,
//       int_value,
//       unit: unit.to_owned().into(),
//     },
//     // Other token types can be copied as is
//     other => other,
//   }
// }

pub struct TokenList<'a> {
  token_iterator: &'a mut Parser<'a, 'a>,
  consumed_tokens: Vec<Token<'a>>,
  pub(crate) current_index: usize,
  is_at_end: bool,
}

impl<'a> TokenList<'a> {
  /// Create a new TokenList from a CSS string or an existing TokenIterator
  pub fn new(parser: &'a mut Parser<'a, 'a>) -> Self {
    Self {
      token_iterator: parser,
      consumed_tokens: Vec::new(),
      current_index: 0,
      is_at_end: false,
    }
  }

  /// Create a TokenList from a CSS string
  // pub fn from_css(css: &str) -> Self {
  //   Self::new(CssTokenIterator::new(css))
  // }

  /// Consume the next token in the stream
  pub fn consume_next_token(&mut self) -> Result<Option<Token<'a>>, Error> {
    if self.current_index < self.consumed_tokens.len() {
      // Return already consumed token
      let token = self.consumed_tokens[self.current_index].clone();
      self.current_index += 1;
      return Ok(Some(token));
    }

    if self.is_at_end {
      return Ok(None);
    }

    if self.token_iterator.is_exhausted() {
      self.is_at_end = true;
      return Ok(None);
    }

    match self.token_iterator.next() {
      Ok(token) => {
        let token_cloned = token.clone();
        self.consumed_tokens.push(token_cloned.clone());
        self.current_index += 1;

        if self.token_iterator.is_exhausted() {
          self.is_at_end = true;
        }

        Ok(Some(token_cloned))
      }
      Err(error) => {
        self.is_at_end = true;
        bail!(
          "Parser error. Kind: {}, location column: {}, location line: {}",
          error.kind,
          error.location.column,
          error.location.line
        )
      }
    }
  }

  /// Look at the next token without consuming it
  pub fn peek(&mut self) -> Result<Option<Token<'a>>, Error> {
    if self.current_index < self.consumed_tokens.len() {
      return Ok(Some(self.consumed_tokens[self.current_index].clone()));
    }

    if self.is_at_end || self.token_iterator.is_exhausted() {
      return Ok(None);
    }

    let token = self.token_iterator.next();
    if let Ok(ref token) = token {
      let token_cloned = token.clone();
      self.consumed_tokens.push(token_cloned.clone());

      return Ok(Some(token_cloned.clone()));
    }

    Ok(None)
  }

  /// Get the first token (same as peek)
  pub fn first(&mut self) -> Result<Option<Token<'a>>, Error> {
    self.peek()
  }

  /// Set the current index to a new position
  pub fn set_current_index(&mut self, new_index: usize) {
    if new_index < self.consumed_tokens.len() {
      // If we already have these tokens consumed, just update the index
      self.current_index = new_index;
      return;
    }

    // Try to consume tokens until we reach the target index
    while !self.is_at_end
      && !self.token_iterator.is_exhausted()
      && self.consumed_tokens.len() <= new_index
    {
      if let Ok(token) = self.token_iterator.next() {
        self.consumed_tokens.push(token.clone());
      }

      if self.token_iterator.is_exhausted() {
        self.is_at_end = true;
      }
    }

    // Clamp to the end if we couldn't reach the target
    self.current_index = std::cmp::min(new_index, self.consumed_tokens.len());
  }

  /// Rewind the current position by the specified number of tokens
  pub fn rewind(&mut self, positions: usize) {
    self.current_index = self.current_index.saturating_sub(positions);
  }

  /// Check if the token list is empty
  pub fn is_empty(&mut self) -> bool {
    self.is_at_end
      || (self.current_index >= self.consumed_tokens.len() && self.token_iterator.is_exhausted())
  }

  /// Get all tokens, consuming the entire stream
  pub fn get_all_tokens(&mut self) -> &[Token<'a>] {
    // Consume all remaining tokens
    while !self.is_empty() {
      self.consume_next_token();
    }
    &self.consumed_tokens
  }

  /// Get a slice of tokens within the specified range
  pub fn slice(&mut self, start: usize, end: Option<usize>) -> Vec<Option<Token<'a>>> {
    let end = end.unwrap_or(self.current_index);
    let initial_index = self.current_index;

    if start >= end {
      return Vec::new();
    }

    self.set_current_index(start);
    let mut result = Vec::new();

    // Consume tokens until we have enough to satisfy the slice request
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
