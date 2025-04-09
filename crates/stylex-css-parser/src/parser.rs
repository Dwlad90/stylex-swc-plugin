use regex::Regex;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::base_types::SubString;

#[derive(Debug, PartialEq)]
pub struct ParseError {
  pub message: String,
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for ParseError {}
#[derive(Clone)]
pub struct Parser<'a, T: 'a + Clone> {
  pub(crate) run_fn: Rc<dyn Fn(&mut SubString) -> Result<Option<T>, ParseError> + 'a>,
}

impl<'a, T: 'a + Debug + std::clone::Clone> Parser<'a, T> {
  pub fn new<F>(parser: F) -> Self
  where
    F: Fn(&mut SubString) -> Result<Option<T>, ParseError> + 'a,
  {
    Self {
      run_fn: Rc::new(parser),
    }
  }

  pub fn run(&self, input: &mut SubString) -> Result<T, ParseError> {
    match (self.run_fn)(input) {
      Ok(Some(value)) => Ok(value),
      Ok(None) => Err(ParseError {
        message: "Parser returned None".to_string(),
      }),
      Err(e) => Err(e),
    }
  }

  pub fn parse(&self, input: &str) -> Result<T, ParseError> {
    let mut substr = SubString::new(input);
    self.run(&mut substr)
  }

  pub fn parse_to_end(&self, input: &str) -> Result<T, ParseError> {
    let mut substr = SubString::new(input);
    let output = self.run(&mut substr)?;

    if !substr.is_empty() {
      return Err(ParseError {
        message: format!(
          "Expected end of input, got {} instead",
          &substr.string[substr.start_index..]
        ),
      });
    }

    Ok(output)
  }

  pub fn map<U, F>(&self, f: F) -> Parser<'a, U>
  where
    F: Fn(Option<T>) -> U + 'a,
    U: 'a + Debug + Clone,
  {
    let run_fn = self.run_fn.clone();
    Parser::new(move |input| {
      let old_output = (run_fn)(input)?;
      Ok(Some(f(old_output)))
    })
  }

  pub fn optional(self) -> Parser<'a, T> {
    let run_fn = self.run_fn;

    Parser::new(move |input| {
      let result = (run_fn)(input);

      match result {
        Ok(output) => Ok(output),
        Err(_) => Ok(None),
      }
    })
  }

  pub fn where_fn<F>(&self, predicate: F) -> Parser<'a, T>
  where
    F: Fn(&T) -> bool + 'a,
    T: Clone,
  {
    let run_fn = self.run_fn.clone();

    Parser::new(move |input| {
      let output = (run_fn)(input)?;

      match &output {
        Some(value) if predicate(value) => Ok(output),
        Some(_) => Err(ParseError {
          message: "Predicate failed".to_string(),
        }),
        None => Ok(None),
      }
    })
  }

  pub fn zero_or_more(parser: Parser<T>) -> Parser<Vec<T>>
  where
    T: Clone + 'a,
  {
    Parser::new(move |input| {
      let mut results = Vec::new();

      loop {
        let start_index = input.start_index;
        match parser.run(input) {
          Ok(value) => results.push(value),
          Err(_) => {
            input.start_index = start_index;
            break;
          }
        }
      }

      Ok(Some(results))
    })
  }

  pub fn regex(regex: &'a Regex) -> Parser<'a, String> {
    Parser::new(move |input| {
      let start_index = input.start_index;
      let end_index = input.end_index;

      if start_index > end_index {
        return Err(ParseError {
          message: "End of input".to_string(),
        });
      }

      let rest = &input.string[start_index..];

      if let Some(mat) = regex.find(rest) {
        if mat.start() == 0 {
          input.start_index += mat.end();

          return Ok(Some(mat.as_str().to_string()));
        }
      }

      Err(ParseError {
        message: format!("Expected {}, got {}", regex.as_str(), rest),
      })
    })
  }

  pub fn surrounded_by<U>(
    &self,
    prefix: Parser<'a, U>,
    suffix: Option<Parser<'a, U>>,
  ) -> Parser<'a, T>
  where
    U: Clone + Debug + 'a,
  {
    let actual_suffix = suffix.unwrap_or_else(|| prefix.clone());
    self.clone().prefix(prefix).skip(actual_suffix)
  }
}

pub trait SequenceResult<'a>: Clone + 'a {}

impl<'a, T: Clone + 'a> SequenceResult<'a> for T {}

#[derive(Clone)]
pub struct ParserSequence<
  'a,
  S: std::clone::Clone,
  T: std::clone::Clone,
  U: std::clone::Clone,
  P: std::clone::Clone,
> {
  parsers: (
    Option<Parser<'a, S>>,
    Option<Parser<'a, T>>,
    Option<Parser<'a, U>>,
    Option<Parser<'a, P>>,
  ),
}

impl<
    'a,
    S: SequenceResult<'a> + std::fmt::Debug,
    R: SequenceResult<'a> + std::fmt::Debug,
    U: SequenceResult<'a> + std::fmt::Debug,
    P: SequenceResult<'a> + std::fmt::Debug,
  > ParserSequence<'a, S, R, U, P>
{
  pub fn new(
    parsers: (
      Option<Parser<'a, S>>,
      Option<Parser<'a, R>>,
      Option<Parser<'a, U>>,
      Option<Parser<'a, P>>,
    ),
  ) -> Self {
    Self { parsers }
  }

  pub fn to_parser(&self) -> Parser<'a, (Option<S>, Option<R>, Option<U>, Option<P>)> {
    let parsers = self.parsers.clone();

    Parser::new(move |input| {
      let start_index = input.start_index;
      let end_index = input.end_index;
      let mut results = (None, None, None, None);

      if let Some(parser) = &parsers.0 {
        match parser.run(input) {
          Ok(result) => {
            results.0 = Some(result);
          }
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      if let Some(parser) = &parsers.1 {
        match parser.run(input) {
          Ok(result) => {
            results.1 = Some(result);
          }
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      if let Some(parser) = &parsers.2 {
        match parser.run(input) {
          Ok(result) => {
            results.2 = Some(result);
          }
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      if let Some(parser) = &parsers.3 {
        match parser.run(input) {
          Ok(result) => {
            results.3 = Some(result);
          }
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      Ok(Some(results))
    })
  }

  pub fn separated_by<SEP>(self, separator: Parser<'a, SEP>) -> Self
  where
    SEP: Clone + Debug + 'a,
  {
    let separator = separator;
    let (p0, p1, p2, p3) = self.parsers;

    let new_parsers: (
      Option<Parser<'a, S>>,
      Option<Parser<'a, R>>,
      Option<Parser<'a, U>>,
      Option<Parser<'a, P>>,
    ) = (
      p0.clone(),
      p1.clone().map(|parser| {
        if p0.is_some() {
          parser.prefix(separator.clone())
        } else {
          parser
        }
      }),
      p2.clone().map(|parser| {
        if p0.is_some() || p1.is_some() {
          parser.prefix(separator.clone())
        } else {
          parser
        }
      }),
      p3.map(|parser| {
        if p0.is_some() || p1.is_some() || p2.is_some() {
          parser.prefix(separator.clone())
        } else {
          parser
        }
      }),
    );

    Self {
      parsers: new_parsers,
    }
  }
}

impl<'a, T: 'a + std::fmt::Debug> Parser<'a, T>
where
  T: Clone,
{
  pub fn prefix<U>(self, prefix_parser: Parser<'a, U>) -> Parser<'a, T>
  where
    U: Clone + Debug + 'a,
  {
    let run_fn = self.run_fn;
    let prefix_run_fn = prefix_parser.run_fn;

    Parser::new(move |input| {
      let start_index = input.start_index;

      match (prefix_run_fn)(input) {
        Ok(_) => (),
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      match (run_fn)(input) {
        Ok(result) => Ok(result),
        Err(e) => {
          input.start_index = start_index;
          Err(e)
        }
      }
    })
  }
}

impl<'a, T: SequenceResult<'a> + std::fmt::Debug> Parser<'a, T> {
  pub fn sequence<S, R, U, P>(
    parser1: Option<Parser<'a, S>>,
    parser2: Option<Parser<'a, R>>,
    parser3: Option<Parser<'a, U>>,
    parser4: Option<Parser<'a, P>>,
  ) -> ParserSequence<'a, S, R, U, P>
  where
    S: SequenceResult<'a> + std::fmt::Debug,
    R: SequenceResult<'a> + std::fmt::Debug,
    U: SequenceResult<'a> + std::fmt::Debug,
    P: SequenceResult<'a> + std::fmt::Debug,
  {
    ParserSequence::new((parser1, parser2, parser3, parser4))
  }
}

impl<'a, T: Clone + 'a + std::fmt::Debug> Parser<'a, T> {
  pub fn one_or_more(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser::new(move |input| {
      let mut results = Vec::new();
      let start_index = input.start_index;

      match parser.run(input) {
        Ok(value) => results.push(value),
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      loop {
        let current_index = input.start_index;
        match parser.run(input) {
          Ok(value) => results.push(value),
          Err(_) => {
            input.start_index = current_index;
            break;
          }
        }
      }

      Ok(Some(results))
    })
  }
  pub fn never() -> Parser<'a, T> {
    Parser::new(|_| {
      Err(ParseError {
        message: "Never".to_string(),
      })
    })
  }

  pub fn always(output: T) -> Parser<'a, T>
  where
    T: Clone,
  {
    let output = output.clone();
    Parser::new(move |_| Ok(Some(output.clone())))
  }

  pub fn one_of<S>(parsers: Vec<Parser<'a, S>>) -> Parser<'a, T>
  where
    S: 'a + Into<T> + Clone + std::fmt::Debug,
    T: 'a + Clone + std::fmt::Debug,
  {
    Parser::new(move |input| {
      let mut errors = Vec::new();

      for parser in &parsers {
        let start_index = input.start_index;
        let end_index = input.end_index;

        match parser.run(input) {
          Ok(output) => return Ok(Some(output.into())),
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            errors.push(e);
          }
        }
      }

      Err(ParseError {
        message: format!(
          "No parser matched\n{}",
          errors
            .iter()
            .map(|err| format!("- {}", err))
            .collect::<Vec<_>>()
            .join("\n")
        ),
      })
    })
  }

  pub fn flat_map<U, F>(&self, f: F) -> Parser<'a, U>
  where
    F: Fn(T) -> Parser<'a, U> + 'a,
    U: 'a + std::clone::Clone + std::fmt::Debug + std::fmt::Debug,
  {
    let run_fn = self.run_fn.clone();

    Parser::new(move |input| {
      let start_index = input.start_index;
      let end_index = input.end_index;

      let output1 = (run_fn)(input)?;

      if output1.is_none() {
        return Ok(None);
      }

      let second_parser = f(output1.expect("Output should not be None"));

      match second_parser.run(input) {
        Ok(output2) => Ok(Some(output2)),
        Err(e) => {
          input.start_index = start_index;
          input.end_index = end_index;
          Err(e)
        }
      }
    })
  }

  pub fn skip<S>(&self, skip_parser: Parser<'a, S>) -> Parser<'a, T>
  where
    S: 'a + std::clone::Clone + std::fmt::Debug,
  {
    self.flat_map(move |output| {
      let output_clone = output.clone();
      skip_parser.map(move |_| output_clone.clone())
    })
  }

  pub fn separated_by<Sep>(&self, separator: Parser<'a, Sep>) -> Parser<'a, Vec<T>>
  where
    Sep: 'a + Clone + Debug,
  {
    let parser = self.clone();

    Parser::new(move |input| {
      let mut results = Vec::new();
      let start_index = input.start_index;

      // Parse the first element
      match parser.run(input) {
        Ok(first) => {
          results.push(first);
        }
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      // Keep parsing: separator followed by element
      loop {
        let sep_index = input.start_index;

        // Try to parse separator
        match separator.run(input) {
          Ok(_) => {
            // Separator found, now parse an element
            match parser.run(input) {
              Ok(value) => {
                // Successfully parsed element after separator
                results.push(value);
              }
              Err(e) => {
                // Found separator but not element after it
                input.start_index = sep_index;
                return Err(ParseError {
                  message: format!("Expected element after separator: {}", e.message),
                });
              }
            }
          }
          Err(_) => {
            // No more separators, we're done
            break;
          }
        }
      }

      Ok(Some(results))
    })
  }
}

impl<'a, T: 'a + Debug + std::clone::Clone> Parser<'a, T> {
  pub fn string(s: &'a str) -> Parser<'a, String> {
    Parser::new(move |input| {
      if input.start_index + s.len() > input.end_index {
        return Err(ParseError {
          message: "End of input".to_string(),
        });
      }

      if input.starts_with(s) {
        input.start_index += s.len();
        Ok(Some(s.to_string()))
      } else {
        Err(ParseError {
          message: format!("Expected {}, got {}", s, &input.string[input.start_index..]),
        })
      }
    })
  }

  pub fn digit() -> Parser<'a, String> {
    Parser::new(|input| {
      if let Some(c) = input.first() {
        if c.is_ascii_digit() {
          input.start_index += c.len_utf8();
          Ok(Some(c.to_string()))
        } else {
          Err(ParseError {
            message: format!("Expected digit, got {}", c),
          })
        }
      } else {
        Err(ParseError {
          message: "End of input".to_string(),
        })
      }
    })
  }

  pub fn natural() -> Parser<'a, u32> {
    // First digit cannot be 0
    let first_digit = Parser::<'a, String>::digit().where_fn(|digit| digit != "0");
    // Rest can be any digits
    let rest_digits = Parser::zero_or_more(Parser::<'a, String>::digit());

    Parser::<'a, String>::sequence::<String, Vec<String>, (), ()>(
      Some(first_digit),
      Some(rest_digits),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (first, rest, _, _) = values.expect("Expected values to be present");
      let first_digit = first.unwrap();
      let rest_digits = rest.unwrap();

      let num_str = format!("{}{}", first_digit, rest_digits.join(""));
      num_str.parse::<u32>().unwrap_or_else(|_| {
        panic!("Failed to parse natural number: {}", num_str);
      })
    })
  }

  pub fn whole() -> Parser<'a, i32> {
    Parser::one_or_more(Parser::<'a, String>::digit()).map(|digits| {
      let num_str = digits.expect("Digits should not be empty").join("");
      num_str.parse::<i32>().unwrap_or_else(|_| {
        panic!("Failed to parse whole number: {}", num_str);
      })
    })
  }

  pub fn integer() -> Parser<'a, i32> {
    let sign_parser = Parser::<'a, String>::string("-")
      .optional()
      .map(|minus_sign| if minus_sign.is_some() { -1 } else { 1 });

    let whole_parser = Parser::<'a, String>::whole();

    Parser::<'a, i32>::sequence::<i32, i32, String, String>(
      Some(sign_parser),
      Some(whole_parser),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (values0, values1, _, _) = values.expect("Values should not be empty");

      values0.unwrap() * values1.unwrap()
    })
  }
  pub fn float() -> Parser<'a, f32> {
    let sign_parser = Parser::<'a, String>::string("-")
      .optional()
      .map(|minus_sign| if minus_sign.is_some() { -1.0 } else { 1.0 });

    Parser::one_of(vec![
      {
        let whole_parser = Parser::zero_or_more(Parser::<'a, String>::digit()).map(|digits| {
          let s = digits.expect("Digits should not be empty").join("");
          if s.is_empty() {
            0.0
          } else {
            s.parse::<f32>()
              .unwrap_or_else(|_| panic!("Failed to parse whole part: {}", s))
          }
        });

        let dot_parser = Parser::<'a, String>::string(".").map(|_| 0.0);
        let frac_parser = Parser::one_or_more(Parser::<'a, String>::digit()).map(|digits| {
          let s = digits.expect("Digits should not be empty").join("");
          let denominator = 10.0_f32.powi(s.len() as i32);
          s.parse::<f32>()
            .unwrap_or_else(|_| panic!("Failed to parse whole part: {}", s))
            / denominator
        });

        Parser::<'a, i32>::sequence(
          Some(sign_parser.clone()),
          Some(whole_parser),
          Some(dot_parser),
          Some(frac_parser),
        )
        .to_parser()
        .map(|values| {
          let (values0, values1, _, values3) = values.expect("Values should not be empty");

          let sign = Decimal::from_f32_retain(values0.unwrap()).unwrap();
          let whole = Decimal::from_f32_retain(values1.unwrap()).unwrap();
          let frac = Decimal::from_f32_retain(values3.unwrap()).unwrap();

          let result = sign * (whole + frac);

          result.round_dp(5).to_f32().unwrap()
        })
      },
      Parser::<'a, i32>::sequence::<f32, f32, String, String>(
        Some(sign_parser),
        Some(Parser::<'a, String>::whole().map(|n| n.expect("Expected a whole number") as f32)),
        None,
        None,
      )
      .to_parser()
      .map(|values| {
        let (values0, values1, _, _) = values.expect("Expected values to be present");

        values0.unwrap() * values1.unwrap()
      }),
    ])
  }

  pub fn whitespace() -> Parser<'a, ()> {
    Parser::one_or_more(Parser::<'a, ()>::one_of(vec![
      Parser::<'a, String>::string(" ").map(|_| ()),
      Parser::<'a, String>::string("\n").map(|_| ()),
      Parser::<'a, String>::string("\r\n").map(|_| ()),
    ]))
    .map(|_| ())
  }
}
