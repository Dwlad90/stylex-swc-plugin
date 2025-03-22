use cssparser::{ParseError, Parser, ParserInput, Token};
use serde::de;
use std::fmt;
use std::rc::Rc;

use crate::token_list::TokenList;

#[derive(Debug)]
pub struct TokenParseError {
  message: String,
}

impl TokenParseError {
  pub fn new<S: Into<String>>(message: S) -> Self {
    TokenParseError {
      message: message.into(),
    }
  }
}

impl fmt::Display for TokenParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for TokenParseError {}
/// A parser for CSS tokens that can be combined with other parsers.
pub struct TokenParser<T: 'static> {
  parse_fn: Rc<dyn Fn(&mut TokenList) -> Result<T, TokenParseError>>,
  label: String,
}

impl<T: 'static> TokenParser<T> {
  pub fn new<F>(parse_fn: F, label: &str) -> Self
  where
    F: Fn(&mut TokenList) -> Result<T, TokenParseError> + 'static,
  {
    TokenParser {
      parse_fn: Rc::new(parse_fn),
      label: label.to_string(),
    }
  }

  /// Parses the given CSS string.
  pub fn parse<'a>(&self, input: &'a str) -> Result<T, TokenParseError> {
    let mut tokens = TokenList::new(input);
    (self.parse_fn)(&mut tokens)
  }

  /// Parses the given CSS string and ensures all input is consumed.
  pub fn parse_to_end<'a>(&self, input: &'a str) -> Result<T, TokenParseError> {
    // Create a new parser instance using the one we just created
    let mut tokens = TokenList::new(&input);
    let initial_index = tokens.current_index;

    let result = (self.parse_fn)(&mut tokens);

    if let Err(e) = &result {
      let consumed_tokens = tokens.slice(initial_index, None);
      tokens.set_current_index(initial_index);
      return Err(TokenParseError::new(format!(
        "Expected {} but got {}\nConsumed tokens: {:?}",
        self.label, e, consumed_tokens
      )));
    }

    if let Ok(token) = tokens.peek() {
      let consumed_tokens = tokens.slice(initial_index, None);
      return Err(TokenParseError::new(format!(
        "Expected end of input, got {:?} instead\nConsumed tokens: {:?}",
        token, consumed_tokens
      )));
    }

    result
  }

  /// Maps the output of this parser with the given function.
  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    F: Fn(T) -> U + 'static,
    U: 'static,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match (parse_fn)(tokens) {
          Ok(value) => Ok(f(value)),
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &format!("{}.map({})", parser_label, label.unwrap_or("")),
    )
  }

  /// Returns a parser that tries this parser and then parser2 if this fails.
  pub fn or<U>(&self, parser2: &TokenParser<U>) -> TokenParser<Result<T, U>>
  where
    U: 'static,
    T: 'static,
  {
    let parse_fn1 = self.parse_fn.clone();
    let parse_fn2 = parser2.parse_fn.clone();
    let label1 = self.label.clone();
    let label2 = parser2.label.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match (parse_fn1)(tokens) {
          Ok(value) => Ok(Ok(value)),
          Err(_) => {
            tokens.set_current_index(current_index);

            match (parse_fn2)(tokens) {
              Ok(value) => Ok(Err(value)),
              Err(e) => {
                tokens.set_current_index(current_index);
                Err(e)
              }
            }
          }
        }
      },
      &if label2 == "optional" {
        format!("Optional<{}>", label1)
      } else {
        format!("OneOf<{}, {}>", label1, label2)
      },
    )
  }

  /// Returns a parser that tries this parser and then applies the given function to create a new parser.
  pub fn flat_map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    F: Fn(T) -> TokenParser<U> + 'static,
    U: 'static,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match (parse_fn)(tokens) {
          Ok(value) => {
            let next_parser = f(value);
            let result = (next_parser.parse_fn)(tokens);
            if result.is_err() {
              tokens.set_current_index(current_index);
            }
            result
          }
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &format!("{}.flatMap({})", parser_label, label.unwrap_or("")),
    )
  }

  /// Create a token parser that always succeeds with the given value.
  pub fn always(value: T) -> TokenParser<T>
  where
    T: Clone + 'static,
  {
    TokenParser::new(move |_| Ok(value.clone()), "Always")
  }

  /// Create a token parser that always fails.
  pub fn never() -> TokenParser<T> {
    TokenParser::new(|_| Err(TokenParseError::new("Never")), "Never")
  }

  pub fn where_fn<F>(&self, predicate: F, description: &str) -> TokenParser<T>
  where
    F: Fn(&T) -> bool + 'static,
    T: Clone,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();
    let description = description.to_string(); // Clone to own the string
    let description_for_closure = description.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match (parse_fn)(tokens) {
          Ok(value) => {
            if predicate(&value) {
              Ok(value)
            } else {
              tokens.set_current_index(current_index);
              Err(TokenParseError::new(format!(
                "Predicate '{}' failed for value",
                description_for_closure
              )))
            }
          }
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &format!("{}.where({})", parser_label, description),
    )
  }

  /// Create a token parser that parses a specific token type.
  pub fn token(expected_type: &str) -> TokenParser<Token<'static>> {
    // Clone expected_type to own it
    let expected_type_owned = expected_type.to_string();
    let expected_type_for_closure = expected_type_owned.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match tokens.consume_next_token() {
          Ok(token) => {
            // To address the lifetime issue, we'd need to convert token to a 'static lifetime
            // Since we can't easily do that with cssparser::Token, we should consider a different approach
            // For now, this will still fail at runtime with an error message:
            Err(TokenParseError::new(
              "Token lifetime cannot be converted to 'static",
            ))
          }
          Err(error) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(format!(
              "Expected token type {}",
              expected_type_for_closure.clone()
            )))
          }
        }
      },
      &expected_type_owned,
    )
  }

  pub fn string(str: &str) -> TokenParser<String> {
    // First, create a parser that extracts the value from Ident tokens
    let ident_parser = TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match tokens.consume_next_token() {
          Ok(Some(Token::Ident(value))) => {
            // For debugging, similar to the console.log in JS
            println!("Found Ident token with value: {}", value);
            Ok(value.to_string())
          }
          Ok(other_token) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(format!(
              "Expected Ident token, got {:?}",
              other_token
            )))
          }
          Err(error) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(
              "Expected Ident token, got end of input",
            ))
          }
        }
      },
      "Ident",
    );

    // Then, check if the value matches the expected string
    let expected_str = str.to_string(); // Clone to own the string
    let expected_for_where = expected_str.clone();

    ident_parser
      .map(move |value| value, Some(".value"))
      .where_fn(
        move |value| *value == expected_for_where,
        &format!("=== {}", expected_str),
      )
  }
}
