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

pub struct TokenList {
  token_iterator: Box<dyn TokenIterator>,
  consumed_tokens: Vec<Token<'static>>,
  current_index: usize,
  is_at_end: bool,
}

impl TokenList {
  /// Create a new TokenList from a CSS string or an existing TokenIterator
  pub fn new<T: TokenIterator + 'static>(input: T) -> Self {
    Self {
      token_iterator: Box::new(input),
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
  pub fn consume_next_token(&mut self) -> Option<Token<'static>> {
    if self.current_index < self.consumed_tokens.len() {
      // Return already consumed token
      let token = self.consumed_tokens[self.current_index].clone();
      self.current_index += 1;
      return Some(token);
    }

    if self.is_at_end {
      return None;
    }

    if self.token_iterator.end_of_file() {
      self.is_at_end = true;
      return None;
    }

    let token = self.token_iterator.next_token();
    if let Some(ref token) = token {
      self.consumed_tokens.push(token.clone());
      self.current_index += 1;

      if self.token_iterator.end_of_file() {
        self.is_at_end = true;
      }
    } else {
      self.is_at_end = true;
    }

    token
  }

  /// Look at the next token without consuming it
  pub fn peek(&mut self) -> Option<Token<'static>> {
    if self.current_index < self.consumed_tokens.len() {
      return Some(self.consumed_tokens[self.current_index].clone());
    }

    if self.is_at_end || self.token_iterator.end_of_file() {
      return None;
    }

    let token = self.token_iterator.next_token();
    if let Some(ref token) = token {
      self.consumed_tokens.push(token.clone());
      return Some(token.clone());
    }

    None
  }

  /// Get the first token (same as peek)
  pub fn first(&mut self) -> Option<Token<'static>> {
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
      && !self.token_iterator.end_of_file()
      && self.consumed_tokens.len() <= new_index
    {
      if let Some(token) = self.token_iterator.next_token() {
        self.consumed_tokens.push(token);
      }

      if self.token_iterator.end_of_file() {
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
  pub fn is_empty(&self) -> bool {
    self.is_at_end
      || (self.current_index >= self.consumed_tokens.len() && self.token_iterator.end_of_file())
  }

  /// Get all tokens, consuming the entire stream
  pub fn get_all_tokens(&mut self) -> &[Token<'static>] {
    // Consume all remaining tokens
    while !self.is_empty() {
      self.consume_next_token();
    }
    &self.consumed_tokens
  }

  /// Get a slice of tokens within the specified range
  pub fn slice(&mut self, start: usize, end: Option<usize>) -> Vec<Token<'static>> {
    let end = end.unwrap_or(self.current_index);
    let initial_index = self.current_index;

    if start >= end {
      return Vec::new();
    }

    self.set_current_index(start);
    let mut result = Vec::new();

    // Consume tokens until we have enough to satisfy the slice request
    while self.current_index < end {
      if let Some(token) = self.consume_next_token() {
        result.push(token);
      } else {
        break;
      }
    }

    self.set_current_index(initial_index);
    result
  }
}
