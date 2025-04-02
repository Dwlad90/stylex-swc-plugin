use cssparser::Token;
use std::rc::Rc;
use std::{
  cmp::Ordering,
  fmt::{self, Debug, Display},
};

use crate::{
  token_list::TokenList,
  tokens::{TokenType, TOKEN_PARSERS},
};

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
#[derive(Clone)]
pub struct TokenParser<'a, T: 'a + Clone> {
  parse_fn: Rc<dyn Fn(&mut TokenList<'a>) -> Result<Option<T>, TokenParseError> + 'a>,
  label: String,
}

impl<'a, T: 'a + Debug + std::clone::Clone> TokenParser<'a, T> {
  pub fn new<F>(parse_fn: F, label: &str) -> Self
  where
    F: Fn(&mut TokenList<'a>) -> Result<Option<T>, TokenParseError> + 'a,
  {
    TokenParser {
      parse_fn: Rc::new(parse_fn),
      label: label.to_string(),
    }
  }

  pub fn parse(&self, css: &'a str) -> Result<Option<T>, TokenParseError> {
    let mut tokens = TokenList::new(css);
    (self.parse_fn)(&mut tokens)
  }

  pub fn parse_to_end(&self, css: &'a str) -> Result<T, TokenParseError> {
    let mut tokens = TokenList::new(css);
    let initial_index = tokens.current_index;

    let output = (self.parse_fn)(&mut tokens);
    dbg!(&output);

    if let Err(e) = &output {
      let consumed_tokens = tokens.slice(initial_index, None);
      tokens.set_current_index(initial_index);

      return Err(TokenParseError::new(format!(
        "Expected {} but got {}\nConsumed tokens: {:?}",
        self.label, e, consumed_tokens
      )));
    }

    let peeked_token = tokens.peek();

    match peeked_token {
      Ok(Some(token)) => {
        let consumed_tokens = tokens.slice(initial_index, None);

        Err(TokenParseError::new(format!(
          "Expected end of input, got {:?} instead\nConsumed tokens: {:?}",
          token, consumed_tokens
        )))
      }
      Ok(None) => match output {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(TokenParseError::new(format!(
          "Expected {} to return a value, but got None",
          self.label
        ))),
        Err(e) => Err(e),
      },
      Err(err) => {
        let consumed_tokens = tokens.slice(initial_index, None);
        tokens.set_current_index(initial_index);

        let token_types = consumed_tokens
          .iter()
          .map(|token_opt| {
            token_opt
              .as_ref()
              .map_or("None".to_string(), |token| token_type_name(token))
          })
          .collect::<Vec<_>>()
          .join(", ");

        Err(TokenParseError::new(format!(
          "Expected {} but got {}\nConsumed tokens: {}",
          self.label, err, token_types
        )))
      }
    }
  }

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

  pub fn set_of(parsers: Vec<TokenParser<'a, T>>) -> TokenParserSet<'a, T> {
    TokenParserSet::new(parsers)
  }

  pub fn sequence(parsers: Vec<TokenParser<'a, T>>) -> TokenParserSequence<'a, T> {
    TokenParserSequence::new(parsers, None)
  }

  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(T) -> U + 'a,
    U: 'a + Debug + Clone,
  {
    let parse_fn = self.parse_fn.clone();

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;
        let result = (parse_fn)(tokens);

        match result {
          Ok(Some(value)) => Ok(Some(f(value))),
          Ok(None) => Ok(None),
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(e.clone())
          }
        }
      },
      &format!("{}.map({})", self.label, label.unwrap_or("")),
    )
  }

  pub fn or<'b, U>(&self, parser2: &'b TokenParser<'a, U>) -> TokenParser<'a, Result<T, U>>
  where
    U: 'a + Debug + std::clone::Clone,
    T: 'a,
  {
    let parse_fn1 = self.parse_fn.clone();
    let parse_fn2 = parser2.parse_fn.clone();

    let label2 = parser2.label.clone();

    let label = if label2 == "optional" {
      format!("Optional<{}>", self.label)
    } else {
      format!("OneOf<{}, {}>", self.label, label2)
    };

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match (parse_fn1)(tokens) {
          Ok(Some(value)) => Ok(Some(Ok(value))),
          Ok(None) => Ok(None),
          Err(_) => {
            tokens.set_current_index(current_index);

            match (parse_fn2)(tokens) {
              Ok(Some(value)) => Ok(Some(Err(value))),
              Ok(None) => Ok(None),
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

  pub fn flat_map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(T) -> TokenParser<'a, U> + 'a,
    U: 'a + Debug + std::clone::Clone,
  {
    let parse_fn = self.parse_fn.clone();
    let label_suffix = label.unwrap_or("");

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        let output1 = match (parse_fn)(tokens) {
          Ok(value) => value,
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        };

        let second_parser = f(output1.clone().unwrap());
        let output2 = (second_parser.parse_fn)(tokens);

        let output2 = match output2 {
          Ok(value) => value,
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        };

        Ok(output2)
      },
      &format!("{}.flatMap({})", self.label, label_suffix),
    )
  }

  pub fn always(value: T) -> TokenParser<'a, T>
  where
    T: Clone + 'a,
  {
    TokenParser::new(move |_| Ok(Some(value.clone())), "Always")
  }

  pub fn never() -> TokenParser<'a, T> {
    TokenParser::new(|_| Err(TokenParseError::new("Never")), "Never")
  }

  pub fn where_fn<F>(&self, predicate: F, description: Option<&str>) -> TokenParser<'a, T>
  where
    F: Fn(&T) -> bool + 'a,
    T: Clone,
  {
    let description_str = description.unwrap_or("");

    self.flat_map(
      move |output| {
        let is_valid = predicate(&output);

        if is_valid {
          TokenParser::always(output)
        } else {
          TokenParser::never()
        }
      },
      Some(description_str),
    )
  }

  pub fn surrounded_by<P, S>(
    &self,
    prefix: TokenParser<'a, P>,
    suffix: Option<TokenParser<'a, S>>,
  ) -> TokenParser<'a, T>
  where
    P: 'a + Debug + std::clone::Clone,
    S: 'a + Debug + std::clone::Clone,
  {
    let suffix_parser = match suffix {
      Some(s) => s.map(|_| (), None),
      None => prefix.map(|_| (), None),
    };

    let parse_fn = self.parse_fn.clone();
    let prefix_void = prefix.map(|_| (), None);

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        match (prefix_void.parse_fn)(tokens) {
          Ok(_) => {}
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        }

        let value = match (parse_fn)(tokens) {
          Ok(v) => v,
          Err(e) => {
            tokens.set_current_index(current_index);
            return Err(e);
          }
        };

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

  pub fn zero_or_more(parser: TokenParser<'a, T>) -> TokenZeroOrMoreParsers<'a, T> {
    TokenZeroOrMoreParsers::new(parser, None)
  }

  pub fn one_or_more(parser: TokenParser<'a, T>) -> TokenOneOrMoreParsers<'a, T> {
    TokenOneOrMoreParsers::new(parser, None)
  }

  pub fn optional(self) -> TokenParser<'a, T> {
    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        let result = (self.parse_fn)(tokens);

        match result {
          Ok(value) => Ok(value),
          Err(_) => {
            tokens.set_current_index(current_index);
            Ok(None)
          }
        }
      },
      &format!("Optional<{}>", self.label),
    )
  }

  pub fn token(token_type: &'a TokenType, label: Option<&'a str>) -> TokenParser<'a, Token<'a>> {
    let binding = token_type.to_string();
    let label_str = label.unwrap_or(&binding);

    TokenParser::new(
      move |tokens| {
        let current_index = tokens.current_index;

        let token_result = tokens.consume_next_token();
        dbg!(&token_result);

        match token_result {
          Ok(Some(token)) => {
            if token_type_name(&token) == token_type.to_string() {
              Ok(Some(token))
            } else {
              tokens.set_current_index(current_index);
              Err(TokenParseError::new(format!(
                "Expected token type {}, got {}",
                token_type,
                token_type_name(&token)
              )))
            }
          }
          Ok(None) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new("Expected token, got end of input"))
          }
          Err(e) => {
            tokens.set_current_index(current_index);
            Err(TokenParseError::new(format!("Error: {}", e)))
          }
        }
      },
      label_str,
    )
  }
}

#[derive(Clone)]
pub struct TokenParserSequence<'a, T: 'a + Clone> {
  parsers: Vec<TokenParser<'a, T>>,
  separator: Option<TokenParser<'a, T>>,
}

impl<'a, T: 'a + Debug + std::clone::Clone> TokenParserSequence<'a, T> {
  pub fn new(parsers: Vec<TokenParser<'a, T>>, separator: Option<TokenParser<'a, T>>) -> Self {
    Self { parsers, separator }
  }

  pub fn to_parser(&self) -> TokenParser<'a, Vec<Option<T>>> {
    let parsers = self.parsers.clone();
    let separator = self.separator.clone();

    TokenParser::new(
      move |input| {
        let current_index = input.current_index;
        let mut results: Vec<Option<T>> = Vec::new();
        let mut failed: Option<TokenParseError> = None;

        for parser in parsers.iter() {
          if failed.is_some() {
            break;
          }

          let parser_to_use = parser.clone();

          let current_parser = if separator.is_some() && input.current_index > current_index {
            if parser.label.starts_with("Optional<") {
              let sep = separator.as_ref().unwrap().clone();
              let inner_parser = parser_to_use.clone();

              TokenParser::sequence(vec![sep.clone(), inner_parser])
                .map(|values| values[1].clone(), None)
                .optional()
            } else {
              let sep = separator.as_ref().unwrap().clone();

              TokenParser::sequence(vec![sep.clone(), parser_to_use])
                .map(|values| values[1].clone(), None)
            }
          } else {
            parser_to_use.map(Some, None)
          };

          let result = (current_parser.parse_fn)(input);

          if let Err(e) = result {
            failed = Some(e);
          } else {
            results.push(result.unwrap_or(None).unwrap_or(None));
          }
        }

        if let Some(error_to_return) = failed {
          input.set_current_index(current_index);
          return Err(error_to_return);
        }

        Ok(Some(results))
      },
      &format!(
        "Sequence<{}>",
        &self
          .parsers
          .iter()
          .map(|parser| parser.label.clone())
          .collect::<Vec<_>>()
          .join(", ")
      ),
    )
  }

  pub fn separated_by(&self, separator: TokenParser<'a, T>) -> Self {
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }
    let separator_void = separator;

    let new_separator = if let Some(existing_sep) = &self.separator {
      existing_sep.surrounded_by(separator_void.clone(), Some(separator_void.clone()))
    } else {
      separator_void
    };

    Self {
      parsers: self.parsers.clone(),
      separator: Some(new_separator),
    }
  }

  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(Vec<Option<T>>) -> U + 'a,
    U: 'a + Debug + Clone,
  {
    self.to_parser().map(f, label)
  }
}

#[derive(Clone)]
pub struct TokenParserSet<'a, T: 'a + Clone> {
  parsers: Vec<TokenParser<'a, T>>,
  separator: Option<TokenParser<'a, T>>,
}

impl<'a, T: 'a + Debug + Clone> TokenParserSet<'a, T> {
  pub fn new(_parsers: Vec<TokenParser<'a, T>>) -> Self {
    Self {
      parsers: _parsers,
      separator: None,
    }
  }

  pub fn to_parser(&self) -> TokenParser<'a, Vec<Option<T>>> {
    let mut sorted_parsers = self
      .parsers
      .iter()
      .enumerate()
      .map(|(i, tp)| (tp.clone(), i))
      .collect::<Vec<_>>();

    sorted_parsers.sort_by(|(a, _), (b, _)| {
      if a.label.starts_with("Optional<") {
        Ordering::Greater
      } else if b.label.starts_with("Optional<") {
        Ordering::Less
      } else {
        Ordering::Equal
      }
    });

    let separator = self.separator.clone();

    TokenParser::new(
      move |input| {
        let parsers: Vec<(TokenParser<'a, T>, usize)> = sorted_parsers.clone();
        let current_index = input.current_index;
        let mut failed: Option<TokenParseError> = None;

        let mut output: Vec<Option<T>> = vec![None; parsers.len()];
        let mut indices = std::collections::HashSet::new();

        for i in 0..parsers.len() {
          let mut found = false;
          let mut errors = Vec::new();

          for (j, (parser, index)) in parsers.iter().enumerate() {
            dbg!(&parser.label, &i, &j);
            if indices.contains(&j) {
              continue;
            }
            let mut parser_to_use = parser.clone().map(Some, None);

            if i > 0 && separator.is_some() {
              let sep = separator.as_ref().unwrap().clone();

              if parser.label.starts_with("Optional<") {
                parser_to_use = TokenParser::sequence(vec![sep.clone(), parser.clone()])
                  .map(|values| values[1].clone().map(Some), None)
                  .optional()
                  .map(|f| f.unwrap_or(None), None);
              } else {
                parser_to_use = TokenParser::sequence(vec![sep.clone(), parser.clone()])
                  .map(|values| values[1].clone(), None);
              }
            }

            let current_pos = input.current_index;

            let result = (parser_to_use.parse_fn)(input);
            dbg!(&parser.label, &result);
            match result {
              Ok(value) => {
                found = true;
                output[*index] = value.flatten();
                indices.insert(j);
                break;
              }
              Err(e) => {
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

        Ok(Some(output))
      },
      &format!(
        "Set<{}>",
        &self
          .parsers
          .iter()
          .map(|p| p.label.clone())
          .collect::<Vec<_>>()
          .join(", ")
      ),
    )
  }

  pub fn separated_by(&self, separator: TokenParser<'a, T>) -> Self {
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }

    let voided_separator = separator.map(|p| p, None);

    let sep = if let Some(existing_sep) = &self.separator {
      existing_sep
        .surrounded_by(voided_separator.clone(), Some(voided_separator.clone()))
        .map(|p| Some(p), None)
    } else {
      voided_separator.map(Some, None)
    };

    Self {
      parsers: self.parsers.clone(),
      separator: Some(sep.map(|p| p.unwrap(), None)),
    }
  }

  pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<'a, U>
  where
    F: Fn(Vec<Option<T>>) -> U + 'a,
    U: 'a + Debug + Clone,
  {
    self.to_parser().map(f, label)
  }
}

#[derive(Clone)]
pub struct TokenZeroOrMoreParsers<'a, T: 'a + Clone> {
  parser: TokenParser<'a, T>,
  separator: Option<TokenParser<'a, ()>>,
}

impl<'a, T: 'a + Debug + Clone> TokenZeroOrMoreParsers<'a, T> {
  pub fn new(parser: TokenParser<'a, T>, separator: Option<TokenParser<'a, ()>>) -> Self {
    Self { parser, separator }
  }

  pub fn to_parser(&self) -> TokenParser<'a, Vec<T>> {
    let parser = self.parser.clone();
    let separator = self.separator.clone();

    TokenParser::new(
      move |input| {
        let mut output: Vec<T> = Vec::new();

        for i in 0.. {
          if i > 0 && separator.is_some() {
            let current_index = input.current_index;
            let result = (separator.as_ref().unwrap().parse_fn)(input);

            if result.is_err() {
              input.set_current_index(current_index);
              return Ok(Some(output));
            }
          }

          let current_index = input.current_index;
          let result = (parser.parse_fn)(input);

          match result {
            Ok(Some(value)) => {
              output.push(value);
            }
            Ok(None) => {
              continue;
            }
            Err(_) => {
              input.set_current_index(current_index);
              return Ok(Some(output));
            }
          }
        }

        Ok(Some(output))
      },
      &format!("ZeroOrMore<{}>", self.parser.label),
    )
  }

  pub fn separated_by<S>(&self, separator: TokenParser<'a, S>) -> Self
  where
    S: 'a + Debug + Clone,
  {
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }

    let voided_separator = separator.map(|_| (), None);

    let new_separator = if let Some(existing_sep) = &self.separator {
      existing_sep.surrounded_by(voided_separator.clone(), Some(voided_separator.clone()))
    } else {
      voided_separator
    };

    Self {
      parser: self.parser.clone(),
      separator: Some(new_separator),
    }
  }
}

#[derive(Clone)]
pub struct TokenOneOrMoreParsers<'a, T: 'a + Clone> {
  parser: TokenParser<'a, T>,
  separator: Option<TokenParser<'a, ()>>,
}

impl<'a, T: 'a + Debug + Clone> TokenOneOrMoreParsers<'a, T> {
  pub fn new(parser: TokenParser<'a, T>, separator: Option<TokenParser<'a, ()>>) -> Self {
    Self { parser, separator }
  }

  pub fn to_parser(&self) -> TokenParser<'a, Vec<T>> {
    let parser = self.parser.clone();
    let separator = self.separator.clone();

    TokenParser::new(
      move |input| {
        let mut output: Vec<T> = Vec::new();

        for i in 0.. {
          if i > 0 && separator.is_some() {
            let current_index = input.current_index;
            let result = (separator.as_ref().unwrap().parse_fn)(input);

            if result.is_err() {
              input.set_current_index(current_index);
              return Ok(Some(output));
            }
          }

          let current_index = input.current_index;
          let result = (parser.parse_fn)(input);

          match result {
            Ok(Some(value)) => {
              output.push(value);
            }
            Ok(None) => {
              continue;
            }
            Err(e) => {
              input.set_current_index(current_index);
              if i == 0 {
                return Err(e);
              }
              return Ok(Some(output));
            }
          }
        }

        Ok(Some(output))
      },
      &format!("OneOrMore<{}>", self.parser.label),
    )
  }

  pub fn separated_by<S>(&self, separator: TokenParser<'a, S>) -> Self
  where
    S: 'a + Debug + Clone,
  {
    if separator.label == TokenType::Whitespace.to_string() {
      return self.clone();
    }

    let voided_separator = separator.map(|_| (), None);

    let new_separator = if let Some(existing_sep) = &self.separator {
      existing_sep.surrounded_by(voided_separator.clone(), Some(voided_separator.clone()))
    } else {
      voided_separator
    };

    Self {
      parser: self.parser.clone(),
      separator: Some(new_separator),
    }
  }
}

impl<T: 'static + std::clone::Clone + PartialEq> TokenParser<'_, T> {
  pub fn get_token_parser(token_type: TokenType) -> TokenParser<'static, Token<'static>> {
    TOKEN_PARSERS.with(|parsers| {
      (*parsers
        .borrow()
        .get(&token_type)
        .expect("Token parser not found"))
      .clone()
    })
  }

  pub fn comment() -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::Comment)
  }

  pub fn at_keyword() -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::AtKeyword)
  }

  pub fn ident() -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident)
  }

  pub fn delim() -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::Delim)
  }

  pub fn number() -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::Number)
  }

  pub fn string(name: &'static str) -> TokenParser<'static, Token<'static>> {
    TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident)
      .map(|t| t, None)
      .where_fn(
        |t| *t == Token::Ident(name.into()),
        Some(format!("=== {}", name).as_str()),
      )
  }
}

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
    _ => "Unknown".to_string(),
  }
}
