use std::fmt::{Debug, Display};
use std::rc::Rc;

// Equivalent to SubString in the original code
#[derive(Debug)]
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

  pub fn first(&self) -> Option<char> {
    if self.is_empty() {
      None
    } else {
      self.string[self.start_index..].chars().next()
    }
  }

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
#[derive(Clone)]
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

  pub fn map<U, F>(&self, f: F) -> Parser<U>
  where
    F: Fn(T) -> U + 'static,
    T: Clone + 'static,
  {
    let run_fn = self.run_fn.clone();
    Parser::new(move |input| {
      let old_output = (run_fn)(input)?;
      Ok(f(old_output))
    })
  }

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

  pub fn optional(self) -> Parser<Option<T>>
  where
    T: Clone + 'static,
  {
    let run_fn = self.run_fn;

    Parser::new(move |input| match (run_fn)(input) {
      Ok(output) => Ok(Some(output)),
      Err(_) => Ok(None),
    })
  }

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

  pub fn where_fn<F>(&self, predicate: F) -> Parser<T>
  where
    F: Fn(&T) -> bool + 'static,
    T: Clone + 'static,
  {
    let run_fn = self.run_fn.clone();

    Parser::new(move |input| {
      let output = (run_fn)(input)?;

      if predicate(&output) {
        Ok(output)
      } else {
        Err(ParseError {
          message: "Predicate failed".to_string(),
        })
      }
    })
  }

  pub fn zero_or_more(parser: Parser<T>) -> Parser<Vec<T>>
  where
    T: Clone + 'static,
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

      Ok(results)
    })
  }
}

// A trait to define parser results that can be combined in a sequence
pub trait SequenceResult: Clone + 'static {}

impl<T: Clone + 'static> SequenceResult for T {}

// A struct to hold multiple parsers as a sequence
#[derive(Clone)]
pub struct ParserSequence<T> {
  // Store parsers in a Vec for simplicity (unlike JS tuple)
  parsers: Vec<Parser<T>>,
}

impl<T: SequenceResult> ParserSequence<T> {
  pub fn new(parsers: Vec<Parser<T>>) -> Self {
    Self { parsers }
  }

  // Convert to a regular parser that runs all parsers in sequence
  pub fn to_parser<R>(&self, combiner: fn(Vec<T>) -> R) -> Parser<R>
  where
    R: SequenceResult,
  {
    let parsers = self.parsers.clone();

    Parser::new(move |input| {
      let start_index = input.start_index;
      let end_index = input.end_index;
      let mut results = Vec::new();

      // Run each parser in sequence
      for parser in &parsers {
        match parser.run(input) {
          Ok(result) => {
            results.push(result);
          }
          Err(e) => {
            // On failure, reset input position and return error
            input.start_index = start_index;
            input.end_index = end_index;
            return Err(e);
          }
        }
      }

      // Combine all results using the provided function
      Ok(combiner(results))
    })
  }

  // Create a new sequence where all parsers after the first are prefixed with a separator
  pub fn separated_by<S>(self, separator: Parser<S>) -> Self
  where
    S: SequenceResult,
  {
    let separator = separator.map(|_| ());

    // Create new parsers with separators
    let mut new_parsers = Vec::new();

    for (i, parser) in self.parsers.into_iter().enumerate() {
      if i == 0 {
        // First parser doesn't need a separator
        new_parsers.push(parser);
      } else {
        // Add prefix to all other parsers
        new_parsers.push(parser.prefix(separator.clone()));
      }
    }

    Self {
      parsers: new_parsers,
    }
  }
}

// Add the prefix method to Parser
impl<T: 'static> Parser<T>
where
  T: Clone,
{
  pub fn prefix<S>(self, prefix_parser: Parser<S>) -> Parser<T>
  where
    S: Clone + 'static,
  {
    let run_fn = self.run_fn;
    let prefix_run_fn = prefix_parser.run_fn;

    Parser::new(move |input| {
      let start_index = input.start_index;

      // Run the prefix parser first
      match (prefix_run_fn)(input) {
        Ok(_) => (),
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      // Then run the main parser
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

// Add sequence static method to create a ParserSequence
impl<T: SequenceResult> Parser<T> {
  pub fn sequence(parsers: Vec<Parser<T>>) -> ParserSequence<T> {
    ParserSequence::new(parsers)
  }
}

// Helper function to create a sequence from a list of parsers
// pub fn sequence<T: SequenceResult>(parsers: Vec<Parser<T>>) -> ParserSequence<T> {
//   ParserSequence::new(parsers)
// }

// // Add sequence implementation
// impl<T: 'static, U: 'static> Parser<(T, U)> {
//   pub fn sequence(first: Parser<T>, second: Parser<U>) -> Self
//   where
//     T: Clone + 'static,
//     U: Clone + 'static,
//   {
//     Parser::new(move |input| {
//       let start_index = input.start_index;

//       // Parse the first part
//       let first_result = first.run(input)?;

//       // Parse the second part
//       let second_result = match second.run(input) {
//         Ok(second_value) => second_value,
//         Err(e) => {
//           input.start_index = start_index;
//           return Err(e);
//         }
//       };

//       Ok((first_result, second_result))
//     })
//   }
// }

// Add after your sequence implementation for Parser<(T, U)>

// impl<T: 'static, U: 'static> Parser<(T, U)> {
//   pub fn separated_by<S>(self, separator: Parser<S>) -> Parser<(T, U)>
//   where
//     T: Clone + 'static,
//     U: Clone + 'static,
//     S: Clone + 'static,
//   {
//     // First, uncomment the map function from your implementation
//     let prefix_parser = separator.map(|_| ());

//     // We need to implement the prefix method first
//     let run_fn = self.run_fn.clone();

//     Parser::new(move |input| {
//       let start_index = input.start_index;

//       // Parse the first part (no separator needed before first element)
//       let first_parser = Parser::new({
//         let run_fn = run_fn.clone();
//         move |input| match (run_fn)(input) {
//           Ok((first, _)) => Ok(first),
//           Err(e) => Err(e),
//         }
//       });

//       let first_result = match first_parser.run(input) {
//         Ok(val) => val,
//         Err(e) => {
//           input.start_index = start_index;
//           return Err(e);
//         }
//       };

//       // Parse separator followed by second part
//       // This is equivalent to second.prefix(separator.map(() => undefined)) in JS
//       let second_parser = Parser::new({
//         let run_fn = run_fn.clone();
//         let prefix_parser = prefix_parser.clone();
//         move |input| {
//           // First parse the separator
//           let sep_start = input.start_index;
//           match prefix_parser.run(input) {
//             Ok(_) => {
//               // Now parse the second part
//               match (run_fn)(input) {
//                 Ok((_, second)) => Ok(second),
//                 Err(e) => {
//                   input.start_index = sep_start;
//                   Err(e)
//                 }
//               }
//             }
//             Err(_) => {
//               // No separator found, so no second part either
//               Err(ParseError {
//                 message: "Expected separator".to_string(),
//               })
//             }
//           }
//         }
//       });

//       // Parse the second part
//       let second_result = match second_parser.run(input) {
//         Ok(val) => val,
//         Err(e) => {
//           input.start_index = start_index;
//           return Err(e);
//         }
//       };

//       Ok((first_result, second_result))
//     })
//   }
// }

// Static methods implementation
impl<T: 'static> Parser<T> {
  pub fn one_or_more(parser: Parser<T>) -> Parser<Vec<T>>
  where
    T: Clone + 'static,
  {
    Parser::new(move |input| {
      let mut results = Vec::new();
      let start_index = input.start_index;

      // Must have at least one result
      match parser.run(input) {
        Ok(value) => results.push(value),
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      // Then collect any additional matches
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

      Ok(results)
    })
  }
  pub fn never() -> Parser<T> {
    Parser::new(|_| {
      Err(ParseError {
        message: "Never".to_string(),
      })
    })
  }
  // pub fn optional(self) -> Parser<Option<T>> {
  //   let run_fn = self.run_fn;

  //   Parser::new(move |input| match (run_fn)(input) {
  //     Ok(output) => Ok(Some(output)),
  //     Err(_) => Ok(None),
  //   })
  // }

  pub fn always(output: T) -> Parser<T>
  where
    T: Clone,
  {
    let output = output.clone();
    Parser::new(move |_| Ok(output.clone()))
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
          Ok(output) => return Ok(output),
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

      dbg!(&input, s);
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

  pub fn digit() -> Parser<String> {
    Parser::new(|input| {
      if let Some(c) = input.first() {
        if c.is_ascii_digit() {
          input.start_index += c.len_utf8();
          Ok(c.to_string())
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

  pub fn natural() -> Parser<u32> {
    // Parse one or more digits, then check the result
    Parser::one_or_more(Parser::digit())
      .where_fn(|digits| !digits.is_empty() && digits[0] != "0")
      .map(|digits| {
        let num_str = digits.join("");
        num_str.parse::<u32>().unwrap()
      })
  }

  pub fn whole() -> Parser<u32> {
    // Use one_or_more with digit parser, then map to u32
    Parser::one_or_more(Parser::digit()).map(|digits| {
      let num_str = digits.join("");
      num_str.parse::<u32>().unwrap()
    })
  }

  pub fn integer() -> Parser<i32> {
    // Parse optional minus sign, followed by whole number
    let sign_parser = Parser::string("-")
      .optional()
      .map(|minus_sign| if minus_sign.is_some() { -1 } else { 1 });

    let whole_parser = Parser::whole().map(|n| n as i32);

    Parser::sequence(vec![sign_parser, whole_parser]).to_parser(|values| values[0] * values[1])
  }

  pub fn float() -> Parser<f32> {
    // Alternative implementation with flatter structure
    Parser::one_of(vec![
      // Case 1: Full float format like "-123.456"
      {
        let sign_parser =
          Parser::string("-")
            .optional()
            .map(|minus_sign| if minus_sign.is_some() { -1.0 } else { 1.0 });

        let whole_parser = Parser::whole().map(|n| n as f32);
        let dot_parser = Parser::string(".").map(|_| 0.0); // Map to f32 with a placeholder value
        let digits_parser = Parser::one_or_more(Parser::digit()).map(|_| 0.0); // Map to f32 with a placeholder value

        Parser::sequence(vec![sign_parser, whole_parser, dot_parser, digits_parser]).to_parser(
          |values| {
            let sign = values[0];
            let whole = values[1];
            // Skip value[2] which is just the "." string
            let digits = &values[3];
            // let fraction = digits.join("");

            sign * format!("{}.{}", whole, digits).parse::<f32>().unwrap()
          },
        )
      },
      // Case 2: Decimal fraction like "-.456"
      {
        let sign_parser =
          Parser::string("-")
            .optional()
            .map(|minus_sign| if minus_sign.is_some() { -1.0 } else { 1.0 });

        let dot_parser = Parser::string(".").map(|_| 0.0); // Map to f32 with a placeholder value
        let digits_parser = Parser::one_or_more(Parser::digit()).map(|digits| 0.0); // Map to f32 with a placeholder value

        Parser::sequence(vec![sign_parser, dot_parser, digits_parser]).to_parser(|values| {
          let sign = values[0];
          // Skip value[1] which is just the "." string
          let digits = &values[2];
          // let fraction = digits.join("");

          sign * format!("0.{}", digits).parse::<f32>().unwrap()
        })
      },
      // Case 3: Integer as float
      Parser::integer().map(|n| n as f32),
    ])
  }

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

  pub fn whitespace() -> Parser<()> {
    Parser::new(|input| {
      let mut found = false;

      while let Some(c) = input.first() {
        if c.is_whitespace() {
          input.start_index += c.len_utf8();
          found = true;
        } else {
          break;
        }
      }

      if found {
        Ok(())
      } else {
        Err(ParseError {
          message: "Expected whitespace".to_string(),
        })
      }
    })
  }
}

// Add space method to sequences
// impl<T: 'static, U: 'static> Parser<(T, U)> {
//   // Convenience method to separate by whitespace
//   pub fn space(self) -> Parser<(T, U)>
//   where
//     T: Clone + 'static,
//     U: Clone + 'static,
//   {
//     self.separated_b(Parser::whitespace())
//   }
// }
