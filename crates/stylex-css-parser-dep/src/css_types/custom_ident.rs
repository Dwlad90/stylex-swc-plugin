use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::{self, Display};

use crate::parser::Parser;

static HEX_DIGITS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\da-fA-F]{1,6} ").unwrap());

#[derive(Debug, Clone, PartialEq)]
pub struct CustomIdentifier {
  pub value: String,
}

impl CustomIdentifier {
  pub fn new(value: String) -> Self {
    Self { value }
  }

  pub fn parse<'a>() -> Parser<'a, CustomIdentifier> {
    let escape: Parser<'a, String> = Parser::one_of(vec![
      Parser::<'a, String>::string("\\\\"), // Backslash
      Parser::<'a, String>::string("\\\""), // Double quote
      Parser::<'a, String>::string("\\'"),  // Single quote
      Parser::<'a, String>::string("\\."),  // dot
      Parser::<'a, String>::string("\\#"),  // hash
      Parser::<'a, String>::string("\\:"),  // colon
      Parser::<'a, String>::string("\\;"),  // semi-colon
      Parser::<'a, String>::string("\\ "),  // space
      Parser::<'a, String>::string("\\+"),  // plus
      // Unicode character. Backslash followed by 1-6 hex digits
      {
        let backslash = Parser::<'a, String>::string("\\");
        let hex_digits = Parser::<'a, String>::regex(&HEX_DIGITS_REGEX);

        Parser::<'a, String>::sequence::<String, String, String, String>(
          Some(backslash),
          Some(hex_digits),
          None,
          None,
        )
        .to_parser()
        .map(|opt| {
          opt
            .map(|(start, rest, _, _)| format!("{}{}", start.unwrap(), rest.unwrap()))
            .unwrap_or_default()
        })
      },
    ]);

    let name_start: Parser<'a, String> = Parser::one_of(vec![
      Parser::<'a, String>::letter(),
      Parser::<'a, String>::string("_"),
      {
        let dash = Parser::<'a, String>::string("-");
        let letter = Parser::<'a, String>::letter();

        Parser::<'a, String>::sequence::<String, String, String, String>(
          Some(dash),
          Some(letter),
          None,
          None,
        )
        .to_parser()
        .map(|opt| {
          opt
            .map(|(dash, letter, _, _)| format!("{}{}", dash.unwrap(), letter.unwrap()))
            .unwrap_or_default()
        })
      },
      escape.clone(),
    ]);

    let rest_of_the_name: Parser<'a, String> = Parser::one_of(vec![
      name_start.clone(),
      Parser::<'a, String>::digit(),
      Parser::<'a, String>::string("-"),
      escape.clone(),
    ]);

    Parser::<'a, String>::sequence::<String, String, String, String>(
      Some(name_start),
      Some(Parser::zero_or_more(rest_of_the_name).map(|arr| arr.unwrap_or_default().join(""))),
      None,
      None,
    )
    .to_parser()
    .map(|opt| {
      opt
        .map(|(start, rest, _, _)| {
          CustomIdentifier::new(format!("{}{}", start.unwrap(), rest.unwrap()))
        })
        .unwrap_or_else(|| CustomIdentifier::new(String::new()))
    })
  }
}

impl Display for CustomIdentifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

// Add the necessary trait implementation for letter() and digit() to Parser<String>
impl<'a> Parser<'a, String> {
  pub fn letter() -> Parser<'a, String> {
    Parser::new(|input| {
      if let Some(c) = input.first() {
        if c.is_alphabetic() {
          input.start_index += c.len_utf8();
          Ok(Some(c.to_string()))
        } else {
          Err(crate::parser::ParseError {
            message: format!("Expected letter, got {}", c),
          })
        }
      } else {
        Err(crate::parser::ParseError {
          message: "End of input".to_string(),
        })
      }
    })
  }
}
