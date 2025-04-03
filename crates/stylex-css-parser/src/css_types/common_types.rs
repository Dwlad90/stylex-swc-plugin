use crate::parser::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Percentage {
  pub value: f32,
}

impl Percentage {
  pub fn new(value: f32) -> Self {
    Percentage { value }
  }

  // Static parser method
  pub fn parse<'a>() -> Parser<'a, Percentage> {
    // First parse a float
    let float_parser = Parser::<f32>::float();

    // Then expect a percent sign
    let percent_sign = Parser::<String>::string("%");

    // Combine them: parse float then skip percent sign
    let run_fn = float_parser.run_fn.clone();
    let percent_run_fn = percent_sign.run_fn.clone();

    Parser::new(move |input| {
      let start_index = input.start_index;

      // Parse the float value
      let value = match (run_fn)(input) {
        Ok(v) => v,
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      };

      // Skip the percent sign
      match (percent_run_fn)(input) {
        Ok(_) => (),
        Err(e) => {
          input.start_index = start_index;
          return Err(e);
        }
      }

      // Create and return the percentage
      Ok(Some(Percentage::new(
        value.expect("Failed to parse percentage"),
      )))
    })
  }
}

// Implement Display to provide toString() functionality
impl Display for Percentage {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}%", self.value)
  }
}
