use cssparser::{ParseError, Parser, ParserInput, Token};
use serde::de;
use std::fmt;
use std::rc::Rc;

use crate::token_list::TokenList;

#[derive(Debug, Clone)]
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
#[derive(Clone)]
pub struct TokenParser<'a, T: 'a> {
  parse_fn: Rc<dyn Fn(&mut TokenList<'a>) -> Result<T, TokenParseError> + 'a>,
  label: String,
}

impl<'a, T: 'a> TokenParser<'a, T> {
  pub fn new<F>(parse_fn: F, label: &str) -> Self
  where
    F: Fn(&mut TokenList<'a>) -> Result<T, TokenParseError> + 'a,
  {
    TokenParser {
      parse_fn: Rc::new(parse_fn),
      label: label.to_string(),
    }
  }

  /// Parses the given CSS string.
  pub fn parse(&self, css: &'a str) -> Result<T, TokenParseError> {
    let mut tokens = TokenList::new(css);
    (self.parse_fn)(&mut tokens)
  }

  /// Parses the given CSS string and ensures all input is consumed.
  /// Parses the given CSS string and ensures all input is consumed.
  /// Parses the given CSS string and ensures all input is consumed.
  pub fn parse_to_end(&self, css: &'a str) -> Result<T, TokenParseError> {
    let mut tokens = TokenList::new(css);
    let initial_index = tokens.current_index;

    // Run the parser (equivalent to this.run(tokens) in JS)
    let output = (self.parse_fn)(&mut tokens);

    // Check for parser errors
    if let Err(e) = &output {
      let consumed_tokens = tokens.slice(initial_index, None);
      tokens.set_current_index(initial_index);

      // Format error message similar to JS version
      return Err(TokenParseError::new(format!(
        "Expected {} but got {}\nConsumed tokens: {:?}",
        self.label, e, consumed_tokens
      )));
    }

    // Check if there are more tokens left (we should have consumed everything)
    match tokens.peek() {
      Ok(Some(token)) => {
        let consumed_tokens = tokens.slice(initial_index, None);

        // Format error message similar to JS version
        Err(TokenParseError::new(format!(
          "Expected end of input, got {:?} instead\nConsumed tokens: {:?}",
          token, consumed_tokens
        )))
      }
      Ok(None) => output,
      Err(err) => {
        let consumed_tokens = tokens.slice(initial_index, None);
        tokens.set_current_index(initial_index);

        // Extract token types for display, similar to token[0] in JS
        let token_types = consumed_tokens
          .iter()
          .map(|token_opt| token_opt.as_ref().map_or("None", token_type_name))
          .collect::<Vec<_>>()
          .join(", ");

        // Format error message to match JS version more closely
        Err(TokenParseError::new(format!(
          "Expected {} but got {}\nConsumed tokens: {}",
          self.label, err, token_types
        )))
      }
    }
  }

  /// Creates a parser that tries multiple parsers in sequence and returns the first success
  pub fn one_of(parsers: Vec<TokenParser<T>>) -> TokenParser<T> {
    TokenParser::new(
      move |tokens| {
        let mut errors = Vec::new();
        let index = tokens.current_index;

        for parser in &parsers {
          match (parser.parse_fn)(tokens) {
            Ok(output) => return Ok(output),
            Err(e) => {
              tokens.set_current_index(index);
              errors.push(e);
            }
          }
        }

        Err(TokenParseError::new(format!(
          "No parser matched\n{}",
          errors
            .iter()
            .map(|err| format!("- {}", err))
            .collect::<Vec<_>>()
            .join("\n")
        )))
      },
      "oneOf",
    )
  }

  /// Maps the output of this parser with the given function.
  pub fn map<U, F>(&'a self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    F: Fn(T) -> U + 'a,
    U: 'a,
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
            Err(e.clone())
          }
        }
      },
      &format!("{}.map({})", parser_label, label.unwrap_or("")),
    )
  }

  /// Returns a parser that tries this parser and then parser2 if this fails.
  pub fn or<'b, U>(&self, parser2: &'b TokenParser<'a, U>) -> TokenParser<'a, Result<T, U>>
  where
    U: 'a,
    T: 'a,
  {
    let parse_fn1 = self.parse_fn.clone();
    let parse_fn2 = parser2.parse_fn.clone();
    let label1 = self.label.clone();
    let label2 = parser2.label.clone();

    let label = if label2 == "optional" {
      format!("Optional<{}>", label1)
    } else {
      format!("OneOf<{}, {}>", label1, label2)
    };

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
      &label,
    )
  }

  /// Returns a parser that tries this parser and then applies the given function to create a new parser.
  pub fn flat_map<U, F>(&'a self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    F: Fn(T) -> TokenParser<'a, U> + 'a,
    U: 'a,
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
  pub fn always(value: T) -> TokenParser<'a, T>
  where
    T: Clone + 'a,
  {
    TokenParser::new(move |_| Ok(value.clone()), "Always")
  }

  /// Create a token parser that always fails.
  pub fn never() -> TokenParser<'a, T> {
    TokenParser::new(|_| Err(TokenParseError::new("Never")), "Never")
  }

  pub fn where_fn<F>(&self, predicate: F, description: &str) -> TokenParser<'a, T>
  where
    F: Fn(&T) -> bool + 'a,
    T: Clone,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();
    let description = description.to_string(); // Clone to own the string
    let description_for_closure = description.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        let result = (parse_fn)(tokens);

        match result {
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
  pub fn token(expected_type: &'a str) -> TokenParser<'a, Token<'a>> {
    // Clone expected_type to own it
    let expected_type_owned = expected_type.to_string();
    let expected_type_for_closure = expected_type_owned.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match tokens.consume_next_token() {
          Ok(token) => {
            dbg!(&token);
            // Convert the token to an owned version
            match token {
              Some(t) => {
                // Convert the token to a string representation that's fully owned
                // let owned_token = format!("{:?}", t);

                Ok(t)
              }
              None => Err(TokenParseError::new("No token available")),
            }
          }
          Err(_) => {
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
    // Create the expected string once and move it into the closure
    let expected_str = str.to_string();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match tokens.consume_next_token() {
          Ok(Some(Token::Ident(value))) => {
            // For debugging, similar to the console.log in JS
            println!("Found Ident token with value: {}", value);

            // Check if the value matches the expected string
            let value_str = value.to_string();
            if value_str == expected_str {
              Ok(value_str)
            } else {
              tokens.set_current_index(current_index);
              Err(TokenParseError::new(format!(
                "Expected Ident token with value '{}', got '{}'",
                expected_str, value
              )))
            }
          }
          Ok(other_token) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(format!(
              "Expected Ident token, got {:?}",
              other_token
            )))
          }
          Err(_) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(
              "Expected Ident token, got end of input",
            ))
          }
        }
      },
      &format!("String('{}')", str),
    )
  }
}

// Helper function to get the token type name (equivalent to token[0] in JS)
fn token_type_name(token: &Token) -> &'static str {
  match token {
    Token::Ident(_) => "Ident",
    Token::Function(_) => "Function",
    Token::AtKeyword(_) => "AtKeyword",
    Token::Hash(_) => "Hash",
    Token::IDHash(_) => "IDHash",
    Token::QuotedString(_) => "QuotedString",
    Token::UnquotedUrl(_) => "UnquotedUrl",
    Token::Delim(_) => "Delim",
    Token::Number { .. } => "Number",
    Token::Percentage { .. } => "Percentage",
    Token::Dimension { .. } => "Dimension",
    Token::WhiteSpace(_) => "WhiteSpace",
    Token::Comment(_) => "Comment",
    Token::Colon => "Colon",
    Token::Semicolon => "Semicolon",
    Token::Comma => "Comma",
    Token::IncludeMatch => "IncludeMatch",
    Token::DashMatch => "DashMatch",
    Token::PrefixMatch => "PrefixMatch",
    Token::SuffixMatch => "SuffixMatch",
    Token::SubstringMatch => "SubstringMatch",
    Token::CDO => "CDO",
    Token::CDC => "CDC",
    Token::ParenthesisBlock => "ParenthesisBlock",
    Token::SquareBracketBlock => "SquareBracketBlock",
    Token::CurlyBracketBlock => "CurlyBracketBlock",
    Token::BadUrl(cow_rc_str) => "BadUrl",
    Token::BadString(cow_rc_str) => "BadString",
    Token::CloseParenthesis => "CloseParenthesis",
    Token::CloseSquareBracket => "CloseSquareBracket",
    Token::CloseCurlyBracket => "CloseCurlyBracket",
  }
}
