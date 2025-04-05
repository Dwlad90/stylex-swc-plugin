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
  // run_fn: Rc<dyn Fn(&mut SubString) -> Result<T, ParseError> + 'a>,
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
}

pub trait SequenceResult<'a>: Clone + 'a {}

impl<'a, T: Clone + 'a> SequenceResult<'a> for T {}

#[derive(Clone)]
pub struct ParserSequence<'a, T: std::clone::Clone> {
  parsers: Vec<Parser<'a, T>>,
}

impl<'a, T: SequenceResult<'a> + std::fmt::Debug> ParserSequence<'a, T> {
  pub fn new(parsers: Vec<Parser<'a, T>>) -> Self {
    Self { parsers }
  }

  pub fn to_parser(&self, combiner: fn(Vec<T>) -> Vec<T>) -> Parser<'a, Vec<T>> {
    let parsers = self.parsers.clone();

    Parser::new(move |input| {
      let start_index = input.start_index;
      let end_index = input.end_index;
      let mut results = Vec::new();

      for parser in &parsers {
        match parser.run(input) {
          Ok(result) => {
            results.push(result);
          }
          Err(e) => {
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      Ok(Some(combiner(results)))
    })
  }

  pub fn separated_by(self, separator: Parser<'a, T>) -> Self {
    let separator = separator; //.map(|_| ());

    let mut new_parsers = Vec::new();

    for (i, parser) in self.parsers.into_iter().enumerate() {
      if i == 0 {
        new_parsers.push(parser);
      } else {
        new_parsers.push(parser.prefix(separator.clone()));
      }
    }

    Self {
      parsers: new_parsers,
    }
  }
}

impl<'a, T: 'a + std::fmt::Debug> Parser<'a, T>
where
  T: Clone,
{
  pub fn prefix(self, prefix_parser: Parser<'a, T>) -> Parser<'a, T> {
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
  pub fn sequence(parsers: Vec<Parser<'a, T>>) -> ParserSequence<'a, T> {
    ParserSequence::new(parsers)
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

  pub fn one_of(parsers: Vec<Parser<T>>) -> Parser<T>
  where
    T: Clone,
  {
    Parser::new(move |input| {
      let mut errors = Vec::new();

      for parser in &parsers {
        let start_index = input.start_index;
        let end_index = input.end_index;

        match parser.run(input) {
          Ok(output) => return Ok(Some(output)),
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

      // Return early if output1 is None
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
    // Implementation follows the JavaScript version:
    // return this.flatMap((output) => skipParser.map(() => output));

    self.flat_map(move |output| {
      // Create a copy of output that can be moved into the closure
      let output_clone = output.clone();

      // Map the skip_parser to always return the original output
      skip_parser.map(move |_| output_clone.clone())
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
    Parser::one_or_more(Parser::<'a, String>::digit())
      .where_fn(|digits| !digits.is_empty() && digits[0] != "0")
      .map(|digits| {
        let num_str = digits.expect("Digits should not be empty").join("");
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

    Parser::sequence(vec![sign_parser, whole_parser])
      .to_parser(|values| values)
      .map(|values| {
        let values = values.expect("Values should not be empty");

        values[0] * values[1]
      })
  }
  pub fn float() -> Parser<'a, f32> {
    let sign_parser = Parser::<'a, String>::string("-")
      .optional()
      .map(|minus_sign| if minus_sign.is_some() { -1.0 } else { 1.0 });

    Parser::one_of(vec![
      // Case 1: Handle both "123.456" and ".456" formats in one parser
      {
        // Optional whole part (may be empty for ".456" style) - map to f32
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

        Parser::sequence(vec![
          sign_parser.clone(),
          whole_parser,
          dot_parser,
          frac_parser,
        ])
        .to_parser(|values| values)
        .map(|values| {
          let values = values.expect("Values should not be empty");

          let sign = Decimal::from_f32_retain(values[0]).unwrap();
          let whole = Decimal::from_f32_retain(values[1]).unwrap();
          let frac = Decimal::from_f32_retain(values[3]).unwrap();

          // dbg!(&sign, &whole, &frac, sign * (whole + frac));
          // let result = sign * (whole + frac);
          // (result * 100000.0).round() / 100000.0

          let result = sign * (whole + frac);
          // dbg!(&result );

          result.round_dp(5).to_f32().unwrap()
        })
      },
      // Case 2: Integer as float (simpler implementation)
      Parser::sequence(vec![
        sign_parser,
        Parser::<'a, String>::whole().map(|n| n.expect("Expected a whole number") as f32),
      ])
      .to_parser(|values| values)
      .map(|values| {
        let values = values.expect("Expected values to be present");
        dbg!(&values);

        values[0] * values[1]
      }),
    ])
  }

  pub fn whitespace() -> Parser<'a, ()> {
    // This implementation returns a parser that matches one or more whitespace characters
    Parser::one_or_more(Parser::one_of(vec![
      // Spaces
      Parser::<'a, String>::string(" ").map(|_| ()),
      // Newlines
      Parser::<'a, String>::string("\n").map(|_| ()),
      // Carriage returns
      Parser::<'a, String>::string("\r\n").map(|_| ()),
    ]))
    .map(|_| ())
  }

  // pub fn whitespace() -> Parser<'a, ()> {
  //   Parser::new(|input| {
  //     let mut found = false;

  //     while let Some(c) = input.first() {
  //       if c.is_whitespace() {
  //         input.start_index += c.len_utf8();
  //         found = true;
  //       } else {
  //         break;
  //       }
  //     }

  //     if found {
  //       Ok(())
  //     } else {
  //       Err(ParseError {
  //         message: "Expected whitespace".to_string(),
  //       })
  //     }
  //   })
  // }
}
