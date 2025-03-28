use cssparser::{ParseError, Parser, ParserInput, Token};
use std::fmt::{self, Display};
use std::rc::Rc;

use crate::{token_list::TokenList, tokens::TokenType};

#[derive(Debug, Clone, PartialEq)]
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

impl Display for TokenParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for TokenParseError {}
/// A parser for CSS tokens that can be combined with other parsers.
#[derive(Clone)]
pub struct TokenParser<'a, T: 'a + Clone + PartialEq> {
  parse_fn: Rc<dyn Fn(&mut TokenList<'a>) -> Result<T, TokenParseError> + 'a>,
  label: String,
}

impl<'a, T: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq> TokenParser<'a, T> {
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

  /// Creates a set of parsers that will be run in any order
  pub fn set_of(parsers: Vec<TokenParser<'a, T>>) -> TokenParserSet<'a, T> {
    TokenParserSet::new(parsers)
  }
  /// Creates a sequence of parsers that will be run in order
  // pub fn sequence<'b, P>(parsers: Vec<TokenParser<'b, P>>) -> TokenParserSequence<'b, P>
  // where
  //   P: 'b + std::fmt::Debug + std::clone::Clone,
  // {
  //   TokenParserSequence::new(parsers)
  // }

  pub fn sequence(parsers: Vec<TokenParser<'a, T>>) -> TokenParserSequence<'a, T> {
    TokenParserSequence::new(parsers)
  }

  /// Maps the output of this parser with the given function.
  pub fn map<U, F>(&self, f: F, label: Option<&'a str>) -> TokenParser<'a, U>
  where
    F: Fn(T) -> U + 'a,
    U: 'a + std::fmt::Debug + Clone + std::cmp::PartialEq,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        dbg!(&tokens);
        match (parse_fn)(tokens) {
          Ok(value) => Ok(f(value)),
          Err(e) => {
            dbg!(&e);
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
    U: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
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
  pub fn flat_map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(T) -> TokenParser<'a, U> + 'a,
    U: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
  {
    let parse_fn = self.parse_fn.clone();
    let parser_label = self.label.clone();
    let label_suffix = label.unwrap_or("").to_string();

    TokenParser::new(
      move |input| {
        let current_index = input.current_index;

        // Run the first parser
        let output1 = match (parse_fn)(input) {
          Ok(value) => value,
          Err(e) => {
            input.set_current_index(current_index);
            return Err(e);
          }
        };
        dbg!(&output1);

        // Create and run the second parser
        let second_parser = f(output1);
        let output2 = match (second_parser.parse_fn)(input) {
          Ok(value) => value,
          Err(e) => {
            dbg!(&e);
            input.set_current_index(current_index);
            return Err(e);
          }
        };

        dbg!(&output2);
        Ok(output2)
      },
      &format!("{}.flatMap({})", parser_label, label_suffix),
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

  pub fn where_fn<F>(&self, predicate: F, description: Option<&str>) -> TokenParser<'a, T>
  where
    F: Fn(&T) -> bool + 'a,
    T: Clone,
  {
    let description_str = description.unwrap_or("").to_string();
    let description_str2 = description.unwrap_or("").to_string();

    self.flat_map(
      move |output| {
        let is_suited = predicate(&output);
        dbg!(&output, &is_suited, &description_str2.to_owned());

        if is_suited {
          TokenParser::always(output)
        } else {
          TokenParser::never()
        }
      },
      Some(&description_str),
    )
  }

  /// Creates a parser that is surrounded by prefix and optional suffix parsers
  pub fn surrounded_by<P, S>(
    &self,
    prefix: TokenParser<'a, P>,
    suffix: Option<TokenParser<'a, S>>,
  ) -> TokenParser<'a, T>
  where
    P: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
    S: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq,
  {
    // Use prefix as suffix if no suffix is provided
    let suffix_parser = match suffix {
      Some(s) => s.map(|_| Some(()), None),
      None => prefix.map(|_| Some(()), None),
    };

    let this_parser = self.clone();
    let prefix_void = prefix.map(|_| Some(()), None);

    // Create a new parser directly to avoid complex type recursion
    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        // Parse prefix
        match (prefix_void.parse_fn)(tokens) {
          Ok(_) => {}
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        }

        // Parse the main content
        let value = match (this_parser.parse_fn)(tokens) {
          Ok(v) => v,
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        };

        // Parse suffix
        match (suffix_parser.parse_fn)(tokens) {
          Ok(_) => {}
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        }

        Ok(value)
      },
      &format!("{} surrounded by prefix and suffix", self.label),
    )
  }

  pub fn optional(self) -> TokenParser<'a, Option<T>> {
    let parser_label = self.label.clone();

    // Create a new parser that either returns the value or None
    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        // Try the original parser
        match (self.parse_fn)(tokens) {
          Ok(value) => Ok(Some(value)),
          Err(err) => {
            dbg!(&err);
            // Reset position and return None on failure
            tokens.set_current_index(current_index);
            Ok(None)
          }
        }
      },
      &format!("Optional<{}>", parser_label),
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
      move |input| {
        let current_index = input.current_index;
        let consumed_token = input.consume_next_token();
        dbg!(&input, &current_index, &consumed_token, &token_type);

        match consumed_token {
          Ok(Some(token)) => {
            let expected_token_type = token_type.to_string();
            let actual_token_type = token_type_name(&token);

            // Check if the token matches the expected type
            dbg!(&actual_token_type, &expected_token_type, &token);
            if actual_token_type == expected_token_type {
              Ok(token)
            } else {
              // Reset position and return error
              input.set_current_index(current_index);
              panic!(
                "Expected token type {}, got {}",
                expected_token_type, actual_token_type
              );
              Err(TokenParseError::new(format!(
                "Expected token type {}, got {}",
                expected_token_type, actual_token_type
              )))
            }
          }
          Ok(None) => {
            // No token available (end of input)
            input.set_current_index(current_index);
            Err(TokenParseError::new("Expected token, got end of input"))
          }
          Err(e) => {
            // Error consuming token
            input.set_current_index(current_index);
            Err(TokenParseError::new(format!("Error: {}", e)))
          }
        }
      },
      label_str,
    )
  }

  // pub fn string(str: &str) -> TokenParser<String> {
  //   // Create the expected string once and move it into the closure
  //   let expected_str = str.to_string();

  //   TokenParser::new(
  //     move |tokens| {
  //       let current_index = tokens.current_index;

  //       match tokens.consume_next_token() {
  //         Ok(Some(Token::Ident(value))) => {
  //           // For debugging, similar to the console.log in JS
  //           println!("Found Ident token with value: {}", value);

  //           // Check if the value matches the expected string
  //           let value_str = value.to_string();
  //           if value_str == expected_str {
  //             Ok(value_str)
  //           } else {
  //             tokens.set_current_index(current_index);
  //             Err(TokenParseError::new(format!(
  //               "Expected Ident token with value '{}', got '{}'",
  //               expected_str, value
  //             )))
  //           }
  //         }
  //         Ok(other_token) => {
  //           tokens.set_current_index(current_index);
  //           Err(TokenParseError::new(format!(
  //             "Expected Ident token, got {:?}",
  //             other_token
  //           )))
  //         }
  //         Err(_) => {
  //           tokens.set_current_index(current_index);
  //           Err(TokenParseError::new(
  //             "Expected Ident token, got end of input",
  //           ))
  //         }
  //       }
  //     },
  //     &format!("String('{}')", str),
  //   )
  // }
}

// Add this struct at the module level
#[derive(Clone)]
pub struct TokenParserSequence<'a, T: 'a + Clone + std::cmp::PartialEq> {
  parsers: Vec<TokenParser<'a, T>>,
  separator: Option<TokenParser<'a, ()>>,
}

impl<'a, T: 'a + std::fmt::Debug + std::clone::Clone + std::cmp::PartialEq>
  TokenParserSequence<'a, T>
{
  /// Create a new sequence of parsers
  pub fn new(parsers: Vec<TokenParser<'a, T>>) -> Self {
    Self {
      parsers,
      separator: None,
    }
  }

  /// Convert the sequence to a regular parser
  pub fn to_parser(&self) -> TokenParser<'a, Vec<T>> {
    let parsers = self.parsers.clone();
    let separator = self.separator.clone();

    let parsers_for_label = self.parsers.clone();

    TokenParser::new(
      move |input| {
        let current_index = input.current_index;
        let mut results: Vec<Option<T>> = Vec::new();
        let mut failed: Option<TokenParseError> = None;

        // Run each parser in sequence
        dbg!(&parsers.len());
        for (i, parser) in parsers.iter().enumerate() {
          if failed.is_some() {
            break;
          }

          // Add separator before parsers (except the first)
          if i > 0 && separator.is_some() && input.current_index > current_index {
            let sep = separator.as_ref().unwrap();
            match (sep.parse_fn)(input) {
              Ok(_) => {} // Separator parsed successfully
              Err(e) => {
                dbg!(&e);
                // failed = Some(e);
                // break;
              }
            }
          }

          // Apply the parser
          let mut parser_to_use = parser.clone();

          dbg!(&input, parser_to_use.label.clone());
          match (parser_to_use.parse_fn)(input) {
            Ok(result) => {
              dbg!(&result);
              results.push(Some(result));
            }
            Err(e) => {
              dbg!(&e);
              results.push(None);
              // panic!("SAAAAwrfmer;kv");
              // failed = Some(e);
              // break;
            }
          }
        }

        if let Some(error) = failed {
          input.set_current_index(current_index);
          return Err(error);
        }

        dbg!(&results);
        Ok(Some(results))
      },
      &format!(
        "Sequence<{}>",
        parsers_for_label
          .iter()
          .map(|p| p.label.clone())
          .collect::<Vec<_>>()
          .join(", ")
      ),
    )
  }

  /// Add a separator between parsers in this sequence
  pub fn separated_by<S>(&self, separator: TokenParser<'a, S>) -> Self
  where
    S: 'a + std::fmt::Debug + Clone + std::cmp::PartialEq,
  {
    // Check if the separator is a whitespace token parser by comparing its label
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }
    // Convert separator to one that discards the result, similar to .map(() => undefined)
    let separator_void = separator.map(|_| (), None);

    // Create new separator by handling the cases like the JS version
    let new_separator = if let Some(ref existing_sep) = &self.separator {
      // Surround the existing separator with the new one
      existing_sep.surrounded_by(separator_void.clone(), Some(separator_void))
    } else {
      // No existing separator, just use the new one
      separator_void
    };

    Self {
      parsers: self.parsers.clone(),
      separator: Some(new_separator),
    }
  }

  pub fn map<U, F>(&self, f: F, label: Option<&'a str>) -> TokenParser<'a, U>
  where
    F: Fn(Vec<T>) -> U + 'a,
    U: 'a + std::fmt::Debug + Clone + std::cmp::PartialEq,
  {
    // Convert the sequence to a regular parser and then map the result
    self.to_parser().map(f, label)
  }
}

// Add this struct at the module level
// Add this struct at the module level
#[derive(Clone)]
pub struct TokenParserSet<'a, T: 'a + Clone + std::cmp::PartialEq> {
  parsers: Vec<TokenParser<'a, T>>,
  separator: Option<TokenParser<'a, T>>,
}

impl<'a, T> TokenParserSet<'a, T>
where
  T: 'a + std::fmt::Debug + Clone + std::cmp::PartialEq,
{
  /// Create a new set of parsers
  pub fn new(_parsers: Vec<TokenParser<'a, T>>) -> Self
  where
    T: 'a + std::fmt::Debug + Clone,
  {
    Self {
      parsers: _parsers,
      separator: None,
    }
  }

  /// Convert the set to a regular parser
  pub fn to_parser(&self) -> TokenParser<'a, Vec<Option<T>>> {
    let _parsers = self.parsers.clone();
    let separator = self.separator.clone();
    let parsers_for_label = self.parsers.clone();

    TokenParser::new(
      move |input| {
        // Sort parsers so optional ones come last
        let mut parsers: Vec<(TokenParser<'a, Option<T>>, usize)> = _parsers
          .iter()
          .enumerate()
          .map(|(i, p)| (p.clone(), i))
          .collect();

        // parsers.sort_by(|(a, _), (b, _)| {
        //   if a.label.starts_with("Optional<") && !b.label.starts_with("Optional<") {
        //     std::cmp::Ordering::Greater
        //   } else if !a.label.starts_with("Optional<") && b.label.starts_with("Optional<") {
        //     std::cmp::Ordering::Less
        //   } else {
        //     std::cmp::Ordering::Equal
        //   }
        // });

        let current_index = input.current_index;
        let mut failed: Option<TokenParseError> = None;

        let mut output = vec![None; _parsers.len()];
        let mut indices = std::collections::HashSet::new();

        for i in 0..parsers.len() {
          let mut found = false;
          let mut errors = Vec::new();

          for (j, (parser, index)) in parsers.iter().enumerate() {
            if indices.contains(&j) {
              continue;
            }
            let mut parser_to_use = parser.clone();

            // Add separator if needed (except for first parser)
            if i > 0 && separator.is_some() {
              let sep = separator.as_ref().unwrap().clone();

              dbg!(&parser.label.starts_with("Optional<"));
              if parser.label.starts_with("Optional<") {
                // For optional parsers, make the separator optional
                let parser_to_use =
                  TokenParser::sequence(vec![sep.clone(), parser.clone()]).map(
                    |values| -> Option<T> {
                      dbg!(&values);

                      values[1].clone()
                    },
                    None,
                  );
                // .optional();
              } else {
                // For regular parsers, require the separator
                parser_to_use = TokenParser::sequence(vec![sep.clone(), parser.clone()]).map(
                  |values| {
                    dbg!(&values);

                    values[1].clone()
                  },
                  None,
                );
              }
            }

            // Try to parse
            let current_pos = input.current_index;
            dbg!(&input);
            match (parser_to_use.parse_fn)(input) {
              Ok(value) => {
                dbg!(&input, &value);
                // If the value is None, skip this item
                // Check if value itself is an Option and is None

                // Otherwise process the value
                // Unwrap the nested options to get the correct type
                output[*index] = match value {
                  Some(inner_value) => inner_value,
                  _ => None,
                };
                indices.insert(j);
                found = true;
                break;
              }
              Err(e) => {
                dbg!(&e);
                input.set_current_index(current_pos);
                errors.push(e);
              }
            }
          }

          if found {
            continue;
          } else {
            failed = Some(TokenParseError::new(format!(
              "Expected one of {} but got {}",
              parsers
                .iter()
                .map(|(p, _)| p.label.clone())
                .collect::<Vec<_>>()
                .join(", "),
              errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
            )));
            break;
          }
        }

        if let Some(error) = failed {
          input.set_current_index(current_index);
          return Err(error);
        }

        dbg!(&output);
        Ok(output)
      },
      &format!(
        "Set<{}>",
        parsers_for_label
          .iter()
          .map(|p| p.label.clone())
          .collect::<Vec<_>>()
          .join(", ")
      ),
    )
  }

  /// Add a separator between parsers in this set
  pub fn separated_by(&self, separator: TokenParser<'a, T>) -> Self
  where
    T: 'a + std::fmt::Debug + Clone + PartialEq,
  {
    // Check if the separator is a whitespace token parser by comparing its label
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }
    // Convert separator to one that discards the result
    let voided_separator = separator.map(
      |p| {
        dbg!(&p);

        Some(p)
      },
      None,
    );

    // Create new separator by handling the cases like the JS version
    let sep = if let Some(existing_sep) = &self.separator {
      // Surround the existing separator with the new one
      existing_sep
        .surrounded_by(voided_separator.clone(), Some(voided_separator.clone()))
        .map(
          |p| {
            dbg!(&p);
            p
          },
          None,
        )
    } else {
      // No existing separator, just use the new one
      // Wrap in another Some to match the type from the if branch
      voided_separator.map(
        |p| {
          dbg!(&p);

          p
        },
        None,
      )
    };
    // &sep.map(
    //   |s| {
    //     dbg!(&s);

    //     s
    //   },
    //   None,
    // );

    // panic!("separated_by not implemented");
    Self {
      parsers: self.parsers.clone(),
      separator: Some(sep),
    }
  }

  // Map the set results with the given function
  pub fn map<U, F>(&self, f: F, label: Option<&'a str>) -> TokenParser<'a, Option<U>>
  where
    F: Fn(Vec<Option<T>>) -> Option<U> + 'a,
    U: 'a + std::fmt::Debug + Clone + std::cmp::PartialEq,
  {
    // Convert the set to a regular parser and then map the result
    // Need to use a wrapper function to adapt the type signatures
    self.to_parser().map(move |values| f(values), label)
  }
  // pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  // where
  //   F: Fn(Vec<Option<T>>) -> U + 'a,
  //   U: 'a + std::fmt::Debug + Clone,
  // {
  //   // Convert the set to a regular parser and then map the result
  //   self.to_parser().map(
  //     move |nested_options| {
  //       // Flatten the nested options before passing to the provided function
  //       let flattened = nested_options
  //         .into_iter()
  //         .map(|opt| opt.unwrap_or(None))
  //         .collect();
  //       f(flattened)
  //     },
  //     label,
  //   )
  // }
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
