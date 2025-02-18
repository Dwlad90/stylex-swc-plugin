use std::fmt::{Debug, Display};
use std::rc::Rc;

// Equivalent to SubString in the original code
pub struct SubString<'a> {
  string: &'a str,
  start_index: usize,
  end_index: usize,
}

impl<'a> SubString<'a> {
  pub fn new(s: &'a str) -> Self {
    Self {
      string: s,
      start_index: 0,
      end_index: s.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.start_index >= self.end_index
  }

  //   pub fn first(&self) -> Option<char> {
  //     if self.is_empty() {
  //       None
  //     } else {
  //       self.string[self.start_index..].chars().next()
  //     }
  //   }

  pub fn starts_with(&self, prefix: &str) -> bool {
    if self.is_empty() {
      return false;
    }

    self.string[self.start_index..].starts_with(prefix)
  }
}

// Custom error type
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
// The Parser struct
pub struct Parser<T> {
  run_fn: Rc<dyn Fn(&mut SubString) -> Result<T, ParseError>>,
}

impl<T> Parser<T> {
  pub fn new<F>(parser: F) -> Self
  where
    F: Fn(&mut SubString) -> Result<T, ParseError> + 'static,
  {
    Self {
      run_fn: Rc::new(parser),
    }
  }

  pub fn run(&self, input: &mut SubString) -> Result<T, ParseError> {
    (self.run_fn)(input)
  }

  pub fn parse(&self, input: &str) -> Result<T, ParseError> {
    let mut substr = SubString::new(input);
    self.run(&mut substr)
  }

  // pub fn parse_to_end(&self, input: &str) -> Result<T, ParseError> {
  //   let mut substr = SubString::new(input);
  //   let output = self.run(&mut substr)?;

  //   if !substr.is_empty() {
  //     return Err(ParseError {
  //       message: format!(
  //         "Expected end of input, got {} instead",
  //         &input[substr.start_index..]
  //       ),
  //     });
  //   }

  //   Ok(output)
  // }

  // pub fn map<U, F>(&self, f: F) -> Parser<U>
  // where
  //   F: Fn(T) -> U + 'static,
  //   T: Clone + 'static,
  // {
  //   let run_fn = self.run_fn.clone();
  //   Parser::new(move |input| {
  //     let old_output = (run_fn)(input)?;
  //     Ok(f(old_output))
  //   })
  // }

  // pub fn flat_map<U, F>(&self, f: F) -> Parser<U>
  // where
  //   F: Fn(T) -> Parser<U> + 'static,
  //   T: Clone + 'static,
  // {
  //   let run_fn = self.run_fn.clone();
  //   Parser::new(move |input| {
  //     let start_index = input.start_index;
  //     let end_index = input.end_index;

  //     let output1 = (run_fn)(input)?;
  //     let second_parser = f(output1);

  //     match second_parser.run(input) {
  //       Ok(output2) => Ok(output2),
  //       Err(e) => {
  //         input.start_index = start_index;
  //         input.end_index = end_index;
  //         Err(e)
  //       }
  //     }
  //   })
  // }

  // pub fn or<U>(&self, parser2: &Parser<U>) -> Parser<Result<T, U>>
  // where
  //   T: Clone + 'static,
  //   U: Clone + 'static,
  // {
  //   let run_fn1 = self.run_fn.clone();
  //   let run_fn2 = parser2.run_fn.clone();

  //   Parser::new(move |input| match (run_fn1)(input) {
  //     Ok(output1) => Ok(Ok(output1)),
  //     Err(_) => match (run_fn2)(input) {
  //       Ok(output2) => Ok(Err(output2)),
  //       Err(e) => Err(e),
  //     },
  //   })
  // }

  // pub fn surrounded_by<U>(&self, prefix: &Parser<U>, suffix: Option<&Parser<U>>) -> Parser<T>
  // where
  //   T: Clone + 'static,
  //   U: Clone + 'static,
  // {
  //   let suffix = suffix.unwrap_or(prefix);
  //   self.prefix(prefix).skip(suffix)
  // }

  // pub fn skip<U>(&self, skip_parser: &Parser<U>) -> Parser<T>
  // where
  //   T: Clone + 'static,
  //   U: Clone + 'static,
  // {
  //   let run_fn = self.run_fn.clone();
  //   let skip_run_fn = skip_parser.run_fn.clone();

  //   Parser::new(move |input| {
  //     let output = (run_fn)(input)?;
  //     (skip_run_fn)(input)?;
  //     Ok(output)
  //   })
  // }

  // pub fn optional(self) -> Parser<Option<T>>
  // where
  //   T: Clone + 'static,
  // {
  //   let run_fn = self.run_fn;

  //   Parser::new(move |input| match (run_fn)(input) {
  //     Ok(output) => Ok(Some(output)),
  //     Err(_) => Ok(None),
  //   })
  // }

  // pub fn prefix<U>(&self, prefix_parser: &Parser<U>) -> Parser<T>
  // where
  //   T: Clone + 'static,
  //   U: Clone + 'static,
  // {
  //   let run_fn = self.run_fn.clone();
  //   let prefix_run_fn = prefix_parser.run_fn.clone();

  //   Parser::new(move |input| {
  //     (prefix_run_fn)(input)?;
  //     (run_fn)(input)
  //   })
  // }

  // pub fn where_fn<F>(&self, predicate: F) -> Parser<T>
  // where
  //   F: Fn(&T) -> bool + 'static,
  //   T: Clone + 'static,
  // {
  //   let run_fn = self.run_fn.clone();

  //   Parser::new(move |input| {
  //     let output = (run_fn)(input)?;

  //     if predicate(&output) {
  //       Ok(output)
  //     } else {
  //       Err(ParseError {
  //         message: "Predicate failed".to_string(),
  //       })
  //     }
  //   })
  // }
}

// Static methods implementation
impl<T: 'static> Parser<T> {
  // pub fn never() -> Parser<T> {
  //   Parser::new(|_| {
  //     Err(ParseError {
  //       message: "Never".to_string(),
  //     })
  //   })
  // }

  // pub fn always(output: T) -> Parser<T>
  // where
  //   T: Clone,
  // {
  //   let output = output.clone();
  //   Parser::new(move |_| Ok(output.clone()))
  // }

  // pub fn one_of(parsers: Vec<Parser<T>>) -> Parser<T>
  // where
  //   T: Clone,
  // {
  //   Parser::new(move |input| {
  //     let mut errors = Vec::new();

  //     for parser in &parsers {
  //       let start_index = input.start_index;
  //       let end_index = input.end_index;

  //       match parser.run(input) {
  //         Ok(output) => return Ok(output),
  //         Err(e) => {
  //           input.start_index = start_index;
  //           input.end_index = end_index;
  //           errors.push(e);
  //         }
  //       }
  //     }

  //     Err(ParseError {
  //       message: format!(
  //         "No parser matched\n{}",
  //         errors
  //           .iter()
  //           .map(|err| format!("- {}", err))
  //           .collect::<Vec<_>>()
  //           .join("\n")
  //       ),
  //     })
  //   })
  // }
}

// Basic parser implementations
impl Parser<String> {
  pub fn string(s: &'static str) -> Parser<String> {
    Parser::new(move |input| {
      if input.start_index + s.len() > input.end_index {
        return Err(ParseError {
          message: "End of input".to_string(),
        });
      }

      if input.starts_with(s) {
        input.start_index += s.len();
        Ok(s.to_string())
      } else {
        Err(ParseError {
          message: format!("Expected {}, got {}", s, &input.string[input.start_index..]),
        })
      }
    })
  }

  // pub fn digit() -> Parser<String> {
  //   Parser::new(|input| {
  //     if let Some(c) = input.first() {
  //       if c.is_ascii_digit() {
  //         input.start_index += c.len_utf8();
  //         Ok(c.to_string())
  //       } else {
  //         Err(ParseError {
  //           message: format!("Expected digit, got {}", c),
  //         })
  //       }
  //     } else {
  //       Err(ParseError {
  //         message: "End of input".to_string(),
  //       })
  //     }
  //   })
  // }

  // pub fn letter() -> Parser<String> {
  //   Parser::new(|input| {
  //     if let Some(c) = input.first() {
  //       if c.is_alphabetic() {
  //         input.start_index += c.len_utf8();
  //         Ok(c.to_string())
  //       } else {
  //         Err(ParseError {
  //           message: format!("Expected letter, got {}", c),
  //         })
  //       }
  //     } else {
  //       Err(ParseError {
  //         message: "End of input".to_string(),
  //       })
  //     }
  //   })
  // }
}
