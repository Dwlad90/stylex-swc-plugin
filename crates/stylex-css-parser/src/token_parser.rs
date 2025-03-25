use cssparser::{ParseError, Parser, ParserInput, Token};
use std::fmt;
use std::rc::Rc;

use crate::{token_list::TokenList, tokens::TokenType};

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

impl<'a, T: 'a + std::fmt::Debug> TokenParser<'a, T> {
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
    dbg!(&tokens);
    let initial_index = tokens.current_index;

    // Run the parser (equivalent to this.run(tokens) in JS)
    let output = (self.parse_fn)(&mut tokens);

    dbg!(&output);

    // Check for parser errors
    if let Err(e) = &output {
      dbg!(&e);
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
          .map(|token_opt| {
            token_opt
              .as_ref()
              .map_or("None".to_string(), |token| token_type_name(token))
          })
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
  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(T) -> U + 'a,
    U: 'a + std::fmt::Debug,
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
    U: 'a + std::fmt::Debug,
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
    U: 'a + std::fmt::Debug,
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
  /// Create a token parser that parses a specific token type.
  pub fn token(token_type: &'a TokenType, label: Option<&'a str>) -> TokenParser<'a, Token<'a>> {
    // Use the provided label or the token_type as default
    let binding = token_type.to_string();
    let label_str = label.unwrap_or(&binding);

    // Clone token_type to own it
    let token_type_owned = token_type.to_string();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        let token_result = tokens.consume_next_token();

        match token_result {
          Ok(Some(token)) => {
            // Check if the token matches the expected type
            dbg!(&token_type_name(&token), token_type.to_string());
            if token_type_name(&token) == token_type.to_string() {
              Ok(token)
            } else {
              // Reset position and return error
              tokens.set_current_index(current_index);
              Err(TokenParseError::new(format!(
                "Expected token type {}, got {}",
                token_type,
                token_type_name(&token)
              )))
            }
          }
          Ok(None) => {
            // No token available (end of input)
            tokens.set_current_index(current_index);
            Err(TokenParseError::new("Expected token, got end of input"))
          }
          Err(e) => {
            // Error consuming token
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(format!("Error: {}", e)))
          }
        }
      },
      label_str,
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
fn token_type_name(token: &Token) -> String {
  match token {
    Token::Ident(_) => TokenType::Ident.to_string(),
    Token::Function(_) => TokenType::Function.to_string(),
    Token::AtKeyword(_) => TokenType::AtKeyword.to_string(),
    Token::Hash(_) => TokenType::Hash.to_string(),
    Token::QuotedString(_) => TokenType::String.to_string(),
    Token::UnquotedUrl(_) => TokenType::URL.to_string(),
    Token::Delim(_) => TokenType::Delim.to_string(),
    Token::Number { .. } => TokenType::Number.to_string(),
    Token::Percentage { .. } => TokenType::Percentage.to_string(),
    Token::Dimension { .. } => TokenType::Dimension.to_string(),
    Token::WhiteSpace(_) => TokenType::Whitespace.to_string(),
    Token::Comment(_) => TokenType::Comment.to_string(),
    Token::Colon => TokenType::Colon.to_string(),
    Token::Semicolon => TokenType::Semicolon.to_string(),
    Token::Comma => TokenType::Comma.to_string(),
    Token::CDO => TokenType::CDO.to_string(),
    Token::CDC => TokenType::CDC.to_string(),
    Token::ParenthesisBlock => TokenType::OpenParen.to_string(),
    Token::SquareBracketBlock => TokenType::OpenSquare.to_string(),
    Token::CurlyBracketBlock => TokenType::OpenCurly.to_string(),
    Token::BadUrl(_) => TokenType::BadURL.to_string(),
    Token::BadString(_) => TokenType::BadString.to_string(),
    Token::CloseParenthesis => TokenType::CloseParen.to_string(),
    Token::CloseSquareBracket => TokenType::CloseSquare.to_string(),
    Token::CloseCurlyBracket => TokenType::CloseCurly.to_string(),
    // Handle other tokens that don't have direct TokenType mappings
    _ => "Unknown".to_string(),
  }
}
