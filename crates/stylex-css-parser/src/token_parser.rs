/*!
Token parser combinators for CSS parsing.

This module provides a monadic parser combinator library for building CSS parsers,
with a comprehensive set of parsing tools and combinators.

**COMPREHENSIVE PARSING API**

This implementation provides a complete parsing API with:
- Return types: `T | Error` (using Result<T, CssParseError> in Rust)
- Consistent method signatures
- Helper classes: TokenZeroOrMoreParsers, TokenOneOrMoreParsers, etc.
- Fluent APIs with .separated_by() methods
- Static methods return specialized parser types
*/

use crate::{
  token_types::{SimpleToken, TokenList},
  CssParseError,
};
use std::fmt::Debug;
use std::rc::Rc;

/// Provides: TokenParser.tokens.Ident, TokenParser.tokens.Whitespace.optional, etc.
pub mod tokens {
  use super::*;

  pub fn ident() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
  }

  pub fn whitespace() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"))
  }

  pub fn comma() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"))
  }

  pub fn colon() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Colon, Some("Colon"))
  }

  pub fn semicolon() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Semicolon, Some("Semicolon"))
  }

  pub fn number() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
  }

  pub fn percentage() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
  }

  pub fn dimension() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
  }

  pub fn function() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Function(String::new()), Some("Function"))
  }

  pub fn string() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::String(String::new()), Some("String"))
  }

  pub fn hash() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Hash(String::new()), Some("Hash"))
  }

  pub fn url() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Url(String::new()), Some("URL"))
  }

  pub fn open_paren() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("OpenParen"))
  }

  pub fn close_paren() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("CloseParen"))
  }

  pub fn open_square() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftBracket, Some("OpenSquare"))
  }

  pub fn close_square() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightBracket, Some("CloseSquare"))
  }

  pub fn open_curly() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftBrace, Some("OpenCurly"))
  }

  pub fn close_curly() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightBrace, Some("CloseCurly"))
  }

  pub fn delim(ch: char) -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Delim(ch), Some("Delim"))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Either<T, U> {
  Left(T),
  Right(U),
}

/// A parser function that takes a TokenList and returns T or Error
pub type RunFn<T> = Rc<dyn Fn(&mut TokenList) -> Result<T, CssParseError>>;

#[derive(Clone)]
pub struct TokenParser<T: Clone + Debug> {
  pub run: RunFn<T>,
  pub label: String,
}

impl<T: Clone + Debug + 'static> TokenParser<T> {
  /// Create a new TokenParser
  pub fn new<F>(parser_fn: F, label: &str) -> Self
  where
    F: Fn(&mut TokenList) -> Result<T, CssParseError> + 'static,
  {
    Self {
      run: Rc::new(parser_fn),
      label: label.to_string(),
    }
  }

  /// Parse a CSS string using this parser
  pub fn parse(&self, css: &str) -> Result<T, CssParseError> {
    let mut tokens = TokenList::new(css);
    (self.run)(&mut tokens)
  }

  /// Parse a CSS string and ensure all input is consumed
  pub fn parse_to_end(&self, css: &str) -> Result<T, CssParseError> {
    let mut tokens = TokenList::new(css);
    let initial_index = tokens.current_index;

    let output = (self.run)(&mut tokens);

    match output {
      Ok(value) => {
        // Check if we've consumed all input
        if let Some(token) = tokens.peek()? {
          let consumed_tokens = tokens.slice(initial_index, Some(tokens.current_index));
          return Err(CssParseError::ParseError {
            message: format!(
              "Expected end of input, got {:?} instead\nConsumed tokens: {:?}",
              token, consumed_tokens
            ),
          });
        }
        Ok(value)
      }
      Err(error) => {
        let consumed_tokens = tokens.slice(initial_index, Some(tokens.current_index));
        tokens.set_current_index(initial_index);
        Err(CssParseError::ParseError {
          message: format!(
            "Expected {} but got {}\nConsumed tokens: {:?}",
            self.to_string(),
            error,
            consumed_tokens
          ),
        })
      }
    }
  }

  /// Map the output of this parser using a function
  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    U: Clone + Debug + 'static,
    F: Fn(T) -> U + 'static,
  {
    let run_fn = self.run.clone();
    let new_label = format!("{}.map({})", self.label, label.unwrap_or(""));

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match (run_fn)(tokens) {
          Ok(value) => Ok(f(value)),
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &new_label,
    )
  }

  /// Flat map operation for chaining parsers
  pub fn flat_map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
  where
    U: Clone + Debug + 'static,
    F: Fn(T) -> TokenParser<U> + 'static,
  {
    let run_fn = self.run.clone();
    let new_label = format!("{}.flatMap({})", self.label, label.unwrap_or(""));

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        let output1 = match (run_fn)(tokens) {
          Ok(value) => value,
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        };

        let second_parser = f(output1);
        match (second_parser.run)(tokens) {
          Ok(output2) => Ok(output2),
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &new_label,
    )
  }

  /// Try this parser, or fall back to another parser
  pub fn or<U>(&self, other: TokenParser<U>) -> TokenParser<Either<T, U>>
  where
    U: Clone + Debug + 'static,
  {
    let run_fn1 = self.run.clone();
    let run_fn2 = other.run.clone();
    let new_label = if other.label == "optional" {
      format!("Optional<{}>", self.label)
    } else {
      format!("OneOf<{}, {}>", self.label, other.label)
    };

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match (run_fn1)(tokens) {
          Ok(value) => Ok(Either::Left(value)),
          Err(_) => {
            tokens.set_current_index(current_index);
            match (run_fn2)(tokens) {
              Ok(value) => Ok(Either::Right(value)),
              Err(e) => {
                tokens.set_current_index(current_index);
                Err(e)
              }
            }
          }
        }
      },
      &new_label,
    )
  }

  /// Apply a predicate to filter the results
  pub fn where_predicate<F>(&self, predicate: F, label: Option<&str>) -> TokenParser<T>
  where
    F: Fn(&T) -> bool + 'static,
  {
    let description = label.unwrap_or("");
    self.flat_map(
      move |output| {
        if predicate(&output) {
          TokenParser::always(output)
        } else {
          TokenParser::never()
        }
      },
      Some(description),
    )
  }

  pub fn where_fn<F>(&self, predicate: F, label: Option<&str>) -> TokenParser<T>
  where
    F: Fn(&T) -> bool + 'static,
  {
    self.where_predicate(predicate, label)
  }

  /// Parse with prefix and suffix parsers
  pub fn surrounded_by<P, S>(
    &self,
    prefix: TokenParser<P>,
    suffix: Option<TokenParser<S>>,
  ) -> TokenParser<T>
  where
    P: Clone + Debug + 'static,
    S: Clone + Debug + 'static,
  {
    let main_parser = self.clone();
    match suffix {
      Some(suffix_parser) => {
        // Use flat_map to sequence the parsers
        prefix.flat_map(
          move |_| {
            let main = main_parser.clone();
            let suffix = suffix_parser.clone();
            main.flat_map(
              move |value| {
                let result_value = value.clone();
                suffix.map(move |_| result_value.clone(), None)
              },
              Some("surrounded_middle"),
            )
          },
          Some("surrounded_prefix"),
        )
      }
      None => {
        // Use the prefix as both prefix and suffix
        let prefix_clone = prefix.clone();
        prefix.flat_map(
          move |_| {
            let main = main_parser.clone();
            let prefix_clone2 = prefix_clone.clone();
            main.flat_map(
              move |value| {
                let result_value = value.clone();
                prefix_clone2.map(move |_| result_value.clone(), None)
              },
              Some("surrounded_middle_same"),
            )
          },
          Some("surrounded_prefix_same"),
        )
      }
    }
  }

  /// Skip a parser after this one
  pub fn skip<U>(&self, skip_parser: TokenParser<U>) -> TokenParser<T>
  where
    U: Clone + Debug + 'static,
  {
    self.flat_map(
      move |output| {
        let output_clone = output.clone();
        skip_parser.map(move |_| output_clone.clone(), None)
      },
      Some("skip"),
    )
  }

  /// Add a prefix parser
  pub fn prefix<P>(&self, prefix_parser: TokenParser<P>) -> TokenParser<T>
  where
    P: Clone + Debug + 'static,
  {
    prefix_parser.flat_map(
      {
        let self_clone = self.clone();
        move |_| self_clone.clone()
      },
      Some("prefix"),
    )
  }

  /// Add a suffix parser
  pub fn suffix<S>(&self, suffix_parser: TokenParser<S>) -> TokenParser<T>
  where
    S: Clone + Debug + 'static,
  {
    self.flat_map(
      move |output| {
        let output_clone = output.clone();
        suffix_parser.map(move |_| output_clone.clone(), None)
      },
      Some("suffix"),
    )
  }

  /// Add a separator parser - creates a SeparatedParser for fluent API
  pub fn separated_by<S: Clone + Debug + 'static>(
    &self,
    separator: TokenParser<S>,
  ) -> SeparatedParser<T, S> {
    SeparatedParser {
      parser: self.clone(),
      separator,
    }
  }

  /// Get string representation of this parser
  pub fn to_string(&self) -> String {
    self.label.clone()
  }

  /// Get the label of this parser
  pub fn label(&self) -> &str {
    &self.label
  }

  /// Debug method that provides detailed parsing information
  /// Enhanced Rust-specific method for development and troubleshooting
  pub fn debug(&self, css: &str) -> Result<T, CssParseError> {
    println!("🔍 DEBUG: Parsing '{}' with parser '{}'", css, self.label);
    let mut tokens = TokenList::new(css);
    let result = (self.run)(&mut tokens);
    match &result {
      Ok(_value) => println!(
        "✅ SUCCESS: Parser '{}' matched. Consumed {} tokens.",
        self.label, tokens.current_index
      ),
      Err(error) => println!(
        "❌ FAILED: Parser '{}' failed at token {}. Error: {}",
        self.label, tokens.current_index, error
      ),
    }
    result
  }

  pub fn parse_with_context(&self, css: &str) -> Result<T, CssParseError> {
    let mut tokens = TokenList::new(css);
    let _initial_index = tokens.current_index;
    let result = (self.run)(&mut tokens);

    match result {
      Err(error) => {
        let context_tokens = peek_tokens(css, 5);
        let remaining_css = &css[tokens.current_index.min(css.len())..];
        Err(CssParseError::ParseError {
          message: format!(
            "{}\n📍 Context: Failed at position {} in '{}'\n🔍 Next tokens: {:?}\n📋 Remaining: '{}'",
            error, tokens.current_index, css, context_tokens, remaining_css.chars().take(20).collect::<String>()
          )
        })
      }
      Ok(value) => Ok(value),
    }
  }

  /// Enhanced labeling method for better debugging
  pub fn with_label(mut self, new_label: &str) -> Self {
    self.label = new_label.to_string();
    self
  }

  /// Parser that always succeeds with the given value
  pub fn always(value: T) -> TokenParser<T> {
    let label = if std::any::type_name::<T>() == "()" {
      "optional".to_string()
    } else {
      format!("Always<{:?}>", value)
    };
    TokenParser::new(move |_| Ok(value.clone()), &label)
  }

  /// Parser that always fails
  pub fn never() -> TokenParser<T> {
    TokenParser::new(
      |_| {
        Err(CssParseError::ParseError {
          message: "Never".to_string(),
        })
      },
      "Never",
    )
  }

  pub fn separated_by_optional_whitespace(self) -> TokenParser<Vec<T>>
  where
    T: Clone + Debug + 'static,
  {
    self
      .separated_by(tokens::whitespace().optional())
      .one_or_more()
  }

  /// Try multiple parsers in order
  pub fn one_of(parsers: Vec<TokenParser<T>>) -> TokenParser<T> {
    TokenParser::new(
      move |tokens| {
        let mut errors = Vec::new();
        let index = tokens.current_index;

        for parser in &parsers {
          match (parser.run)(tokens) {
            Ok(output) => return Ok(output),
            Err(e) => {
              tokens.set_current_index(index);
              errors.push(e);
            }
          }
        }

        Err(CssParseError::ParseError {
          message: format!(
            "No parser matched\n{}",
            errors
              .iter()
              .map(|err| format!("- {}", err))
              .collect::<Vec<_>>()
              .join("\n")
          ),
        })
      },
      "oneOf",
    )
  }

  /// Parse a sequence of parsers
  pub fn sequence<U: Clone + Debug + 'static>(parsers: Vec<TokenParser<U>>) -> TokenParser<Vec<U>> {
    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        let mut results = Vec::new();

        for parser in &parsers {
          match (parser.run)(tokens) {
            Ok(value) => results.push(value),
            Err(e) => {
              tokens.set_current_index(current_index);
              return Err(e);
            }
          }
        }

        Ok(results)
      },
      "sequence",
    )
  }

  /// Parse a set of parsers in any order (order-insensitive)
  pub fn set_of<U: Clone + Debug + 'static>(parsers: Vec<TokenParser<U>>) -> TokenParser<Vec<U>> {
    TokenParser::new(
      move |tokens| {
        let start_index = tokens.current_index;
        let mut results = vec![None; parsers.len()];
        let mut used_indices = std::collections::HashSet::new();
        let mut errors = Vec::new();

        // Try to match each position in order, but parsers can match in any order
        for position in 0..parsers.len() {
          let mut found = false;
          let mut position_errors = Vec::new();

          // Try each unused parser
          for (parser_index, parser) in parsers.iter().enumerate() {
            if used_indices.contains(&parser_index) {
              continue;
            }

            let before_attempt = tokens.current_index;
            match (parser.run)(tokens) {
              Ok(value) => {
                results[parser_index] = Some(value);
                used_indices.insert(parser_index);
                found = true;
                break;
              }
              Err(e) => {
                tokens.set_current_index(before_attempt);
                position_errors.push(format!("Parser {}: {}", parser_index, e));
              }
            }
          }

          if !found {
            errors.extend(position_errors);
            tokens.set_current_index(start_index);
            return Err(CssParseError::ParseError {
              message: format!(
                "SetOf failed at position {}: {}",
                position,
                errors.join("; ")
              ),
            });
          }
        }

        // Convert Option<T> to T, ensuring all parsers matched
        let final_results: Result<Vec<U>, String> = results
          .into_iter()
          .enumerate()
          .map(|(i, opt)| opt.ok_or_else(|| format!("Parser {} did not match", i)))
          .collect();

        match final_results {
          Ok(values) => Ok(values),
          Err(err) => {
            tokens.set_current_index(start_index);
            Err(CssParseError::ParseError {
              message: format!("SetOf incomplete: {}", err),
            })
          }
        }
      },
      "setOf",
    )
  }

  /// Parse zero or more occurrences
  pub fn zero_or_more(parser: TokenParser<T>) -> TokenParser<Vec<T>> {
    let label = format!("ZeroOrMore<{}>", parser.label);
    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();
        loop {
          let current_index = tokens.current_index;
          match (parser.run)(tokens) {
            Ok(value) => results.push(value),
            Err(_) => {
              tokens.set_current_index(current_index);
              break;
            }
          }
        }
        Ok(results)
      },
      &label,
    )
  }

  /// Parse one or more occurrences
  pub fn one_or_more(parser: TokenParser<T>) -> TokenParser<Vec<T>> {
    let label = format!("OneOrMore<{}>", parser.label);
    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();
        let start_index = tokens.current_index;

        // Must match at least once
        match (parser.run)(tokens) {
          Ok(value) => results.push(value),
          Err(e) => {
            tokens.set_current_index(start_index);
            return Err(e);
          }
        }

        // Then try to match more
        loop {
          let current_index = tokens.current_index;
          match (parser.run)(tokens) {
            Ok(value) => results.push(value),
            Err(_) => {
              tokens.set_current_index(current_index);
              break;
            }
          }
        }

        Ok(results)
      },
      &label,
    )
  }

  /// Parse a specific token type
  pub fn token(expected_token: SimpleToken, label: Option<&str>) -> TokenParser<SimpleToken> {
    let label_str = label
      .unwrap_or(&format!("{:?}", expected_token))
      .to_string();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match tokens.consume_next_token() {
          Ok(Some(token)) => {
            if std::mem::discriminant(&token) == std::mem::discriminant(&expected_token) {
              Ok(token)
            } else {
              tokens.set_current_index(current_index);
              Err(CssParseError::ParseError {
                message: format!("Expected token type {:?}, got {:?}", expected_token, token),
              })
            }
          }
          Ok(None) => {
            tokens.set_current_index(current_index);
            Err(CssParseError::ParseError {
              message: "Expected token, got end of input".to_string(),
            })
          }
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e)
          }
        }
      },
      &label_str,
    )
  }

  /// Parse a specific string as an identifier
  pub fn string(expected: &str) -> TokenParser<String> {
    let expected_clone = expected.to_string();
    Self::token(SimpleToken::Ident(String::new()), Some("Ident"))
      .map(
        |token| {
          if let SimpleToken::Ident(value) = token {
            value
          } else {
            unreachable!()
          }
        },
        Some(".value"),
      )
      .where_predicate(
        move |value| value == &expected_clone,
        Some(&format!("=== {}", expected)),
      )
  }

  pub fn fn_name(name: &str) -> TokenParser<String> {
    let name_owned = name.to_string();
    Self::token(SimpleToken::Function(String::new()), Some("Function"))
      .map(
        |token| {
          if let SimpleToken::Function(value) = token {
            value
          } else {
            unreachable!()
          }
        },
        Some(".value"),
      )
      .where_predicate(
        move |value| value == &name_owned,
        Some(&format!("=== {}", name)),
      )
  }

  /// Parse an identifier token
  pub fn ident() -> TokenParser<SimpleToken> {
    Self::token(SimpleToken::Ident(String::new()), Some("Ident"))
  }

  /// One or more separated by separator
  pub fn one_or_more_separated_by<S>(
    parser: TokenParser<T>,
    separator: TokenParser<S>,
  ) -> TokenParser<Vec<T>>
  where
    S: Clone + Debug + 'static,
  {
    let label = format!(
      "OneOrMoreSeparatedBy<{}, {}>",
      parser.label, separator.label
    );
    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();
        let start_index = tokens.current_index;

        // Must match at least once
        match (parser.run)(tokens) {
          Ok(value) => results.push(value),
          Err(e) => {
            tokens.set_current_index(start_index);
            return Err(e);
          }
        }

        // Try to match additional occurrences with separators
        loop {
          let separator_index = tokens.current_index;

          // Try to parse separator
          match (separator.run)(tokens) {
            Ok(_) => {
              // Separator found, try to parse next value
              match (parser.run)(tokens) {
                Ok(value) => results.push(value),
                Err(_) => {
                  // Failed to parse value after separator, rewind to before separator
                  tokens.set_current_index(separator_index);
                  break;
                }
              }
            }
            Err(_) => {
              // No separator found, we're done
              tokens.set_current_index(separator_index);
              break;
            }
          }
        }

        Ok(results)
      },
      &label,
    )
  }

  /// Zero or more separated by separator
  pub fn zero_or_more_separated_by<S>(
    parser: TokenParser<T>,
    separator: TokenParser<S>,
  ) -> TokenParser<Vec<T>>
  where
    S: Clone + Debug + 'static,
  {
    let label = format!(
      "ZeroOrMoreSeparatedBy<{}, {}>",
      parser.label, separator.label
    );
    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();
        let current_index = tokens.current_index;

        // Try to match first occurrence
        match (parser.run)(tokens) {
          Ok(value) => results.push(value),
          Err(_) => {
            tokens.set_current_index(current_index);
            return Ok(results); // Empty list is valid for zero or more
          }
        }

        // Try to match additional occurrences with separators
        loop {
          let separator_index = tokens.current_index;

          // Try to parse separator
          match (separator.run)(tokens) {
            Ok(_) => {
              // Separator found, try to parse next value
              match (parser.run)(tokens) {
                Ok(value) => results.push(value),
                Err(_) => {
                  // Failed to parse value after separator, rewind to before separator
                  tokens.set_current_index(separator_index);
                  break;
                }
              }
            }
            Err(_) => {
              // No separator found, we're done
              tokens.set_current_index(separator_index);
              break;
            }
          }
        }

        Ok(results)
      },
      &label,
    )
  }
}

#[derive(Clone)]
pub struct TokenOptionalParser<T: Clone + Debug> {
  pub parser: TokenParser<T>,
}

impl<T: Clone + Debug + 'static> TokenOptionalParser<T> {
  pub fn new(parser: TokenParser<T>) -> Self {
    Self { parser }
  }

  /// Get the underlying parser as TokenParser<Option<T>>
  pub fn as_token_parser(self) -> TokenParser<Option<T>> {
    let parser_run = self.parser.run;
    let label = format!("Optional<{}>", self.parser.label);

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        match (parser_run)(tokens) {
          Ok(value) => Ok(Some(value)),
          Err(_) => {
            tokens.set_current_index(current_index);
            Ok(None)
          }
        }
      },
      &label,
    )
  }
}

impl<T: Clone + Debug + 'static> TokenParser<T> {
  /// Create a TokenOptionalParser from this parser
  pub fn optional(self) -> TokenParser<Option<T>> {
    TokenOptionalParser::new(self).as_token_parser()
  }
}

/// A parser that represents a main parser separated by a separator parser
/// This allows for fluent API like: parser.separatedBy(comma).oneOrMore()
#[derive(Clone)]
pub struct SeparatedParser<T: Clone + Debug + 'static, S: Clone + Debug + 'static> {
  parser: TokenParser<T>,
  separator: TokenParser<S>,
}

impl<T: Clone + Debug + 'static, S: Clone + Debug + 'static> SeparatedParser<T, S> {
  /// Parse one or more occurrences with separator
  pub fn one_or_more(self) -> TokenParser<Vec<T>> {
    TokenParser::one_or_more_separated_by(self.parser, self.separator)
  }

  /// Parse zero or more occurrences with separator
  pub fn zero_or_more(self) -> TokenParser<Vec<T>> {
    TokenParser::zero_or_more_separated_by(self.parser, self.separator)
  }

  /// Convert to a regular TokenParser (defaults to one or more)
  pub fn as_token_parser(self) -> TokenParser<Vec<T>> {
    self.one_or_more()
  }
}

#[derive(Clone)]
pub struct TokenZeroOrMoreParsers<T: Clone + Debug> {
  parser: TokenParser<T>,
  separator: Option<TokenParser<()>>,
}

impl<T: Clone + Debug + 'static> TokenZeroOrMoreParsers<T> {
  pub fn new(parser: TokenParser<T>, separator: Option<TokenParser<()>>) -> Self {
    Self { parser, separator }
  }

  /// Add a separator parser
  pub fn separated_by<S: Clone + Debug + 'static>(
    self,
    separator: TokenParser<S>,
  ) -> SeparatedParser<T, S> {
    SeparatedParser {
      parser: self.parser,
      separator,
    }
  }

  /// Convert to a regular TokenParser
  pub fn as_token_parser(self) -> TokenParser<Vec<T>> {
    let parser = self.parser;
    let separator = self.separator;
    let label = format!("ZeroOrMore<{}>", parser.label);

    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();

        for i in 0.. {
          if i > 0 && separator.is_some() {
            let current_index = tokens.current_index;
            if let Some(ref sep) = separator {
              match (sep.run)(tokens) {
                Ok(_) => {}
                Err(_) => {
                  tokens.set_current_index(current_index);
                  return Ok(results);
                }
              }
            }
          }

          let current_index = tokens.current_index;
          match (parser.run)(tokens) {
            Ok(value) => results.push(value),
            Err(_) => {
              tokens.set_current_index(current_index);
              return Ok(results);
            }
          }
        }

        Ok(results)
      },
      &label,
    )
  }
}

#[derive(Clone)]
pub struct TokenOneOrMoreParsers<T: Clone + Debug> {
  parser: TokenParser<T>,
  separator: Option<TokenParser<()>>,
}

impl<T: Clone + Debug + 'static> TokenOneOrMoreParsers<T> {
  pub fn new(parser: TokenParser<T>, separator: Option<TokenParser<()>>) -> Self {
    Self { parser, separator }
  }

  /// Add a separator parser
  pub fn separated_by<S: Clone + Debug + 'static>(
    self,
    separator: TokenParser<S>,
  ) -> SeparatedParser<T, S> {
    SeparatedParser {
      parser: self.parser,
      separator,
    }
  }

  /// Convert to a regular TokenParser
  pub fn as_token_parser(self) -> TokenParser<Vec<T>> {
    let parser = self.parser;
    let separator = self.separator;
    let label = format!("OneOrMore<{}>", parser.label);

    TokenParser::new(
      move |tokens| {
        let mut results = Vec::new();

        for i in 0.. {
          if i > 0 && separator.is_some() {
            let current_index = tokens.current_index;
            if let Some(ref sep) = separator {
              match (sep.run)(tokens) {
                Ok(_) => {}
                Err(_) => {
                  tokens.set_current_index(current_index);
                  return Ok(results);
                }
              }
            }
          }

          let current_index = tokens.current_index;
          match (parser.run)(tokens) {
            Ok(value) => results.push(value),
            Err(e) => {
              if i == 0 {
                tokens.set_current_index(current_index);
                return Err(e);
              }
              return Ok(results);
            }
          }
        }

        Ok(results)
      },
      &label,
    )
  }
}

/// Tokens API providing access to all basic token parsers
pub struct Tokens;

impl Tokens {
  /// Parse an identifier token
  pub fn ident() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
  }

  /// Parse a comma token
  pub fn comma() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"))
  }

  /// Parse a colon token
  pub fn colon() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Colon, Some("Colon"))
  }

  /// Parse a semicolon token
  pub fn semicolon() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Semicolon, Some("Semicolon"))
  }

  /// Parse a left parenthesis token
  pub fn open_paren() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("OpenParen"))
  }

  /// Parse a right parenthesis token
  pub fn close_paren() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("CloseParen"))
  }

  /// Parse a left bracket token
  pub fn open_square() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftBracket, Some("OpenSquare"))
  }

  /// Parse a right bracket token
  pub fn close_square() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightBracket, Some("CloseSquare"))
  }

  /// Parse a left brace token
  pub fn open_curly() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::LeftBrace, Some("OpenCurly"))
  }

  /// Parse a right brace token
  pub fn close_curly() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::RightBrace, Some("CloseCurly"))
  }

  /// Parse a number token
  pub fn number() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
  }

  /// Parse a percentage token
  pub fn percentage() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Percentage(0.0), Some("Percentage"))
  }

  /// Parse a dimension token
  pub fn dimension() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(
      SimpleToken::Dimension {
        value: 0.0,
        unit: String::new(),
      },
      Some("Dimension"),
    )
  }

  /// Parse a string token
  pub fn string() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::String(String::new()), Some("String"))
  }

  /// Parse a function token
  pub fn function() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Function(String::new()), Some("Function"))
  }

  /// Parse a hash token
  pub fn hash() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Hash(String::new()), Some("Hash"))
  }

  /// Parse a delimiter token
  pub fn delim() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Delim(' '), Some("Delim"))
  }

  /// Parse a whitespace token
  pub fn whitespace() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"))
  }

  /// Parse an at-keyword token
  pub fn at_keyword() -> TokenParser<SimpleToken> {
    TokenParser::<SimpleToken>::token(SimpleToken::AtKeyword(String::new()), Some("AtKeyword"))
  }
}

impl<T: Clone + Debug + 'static> TokenParser<T> {
  pub fn tokens() -> Tokens {
    Tokens
  }
}

/// Peek at what the next few tokens would be without consuming them
/// Enhanced debugging utility function
pub fn peek_tokens(css: &str, count: usize) -> Vec<SimpleToken> {
  let mut tokens = TokenList::new(css);
  let mut result = Vec::new();
  for _ in 0..count {
    if let Ok(Some(token)) = tokens.peek() {
      result.push(token);
      // Move forward to see next token
      let _ = tokens.consume_next_token();
    } else {
      break;
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_always_parser() {
    let parser = TokenParser::always(42);
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, 42);
  }

  #[test]
  fn test_never_parser() {
    let parser: TokenParser<i32> = TokenParser::never();
    assert!(parser.parse("anything").is_err());
  }

  #[test]
  fn test_map_parser() {
    let parser = TokenParser::always(10).map(|x| x * 2, Some("double"));
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, 20);
  }

  #[test]
  fn test_flat_map_parser() {
    let parser = TokenParser::always(5).flat_map(|x| TokenParser::always(x + 1), Some("add_one"));
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, 6);
  }

  #[test]
  fn test_optional_parser() {
    let success_parser = TokenParser::always(42).optional();
    let result = success_parser.parse("anything").unwrap();
    assert_eq!(result, Some(42));

    let fail_parser: TokenParser<Option<i32>> = TokenParser::<i32>::never().optional();
    let result = fail_parser.parse("anything").unwrap();
    assert_eq!(result, None);
  }

  #[test]
  fn test_where_predicate_parser() {
    let parser = TokenParser::always(10).where_predicate(|&x| x > 5, Some("greater_than_5"));
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, 10);

    let parser = TokenParser::always(3).where_predicate(|&x| x > 5, Some("greater_than_5"));
    assert!(parser.parse("anything").is_err());
  }

  #[test]
  fn test_one_of_parser() {
    let parser = TokenParser::one_of(vec![
      TokenParser::<i32>::never(),
      TokenParser::always(42),
      TokenParser::always(24),
    ]);
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, 42); // Should return first successful result
  }

  #[test]
  fn test_sequence_parser() {
    let parser = TokenParser::<i32>::sequence(vec![
      TokenParser::always(1),
      TokenParser::always(2),
      TokenParser::always(3),
    ]);
    let result = parser.parse("anything").unwrap();
    assert_eq!(result, vec![1, 2, 3]);
  }

  #[test]
  fn test_or_parser() {
    let parser1 = TokenParser::always(1);
    let parser2 = TokenParser::always(2);
    let combined = parser1.or(parser2);

    let result = combined.parse("anything").unwrap();
    assert!(matches!(result, Either::Left(1)));
  }

  #[test]
  fn test_parse_to_end() {
    let parser = TokenParser::always(42);
    // This should work since always parser doesn't consume tokens
    let result = parser.parse_to_end("").unwrap();
    assert_eq!(result, 42);
  }

  #[test]
  fn test_label_preservation() {
    let parser = TokenParser::always(42);
    assert!(parser.label.contains("Always"));

    let mapped = parser.map(|x| x * 2, Some("double"));
    assert!(mapped.label.contains("map(double)"));
  }
}
