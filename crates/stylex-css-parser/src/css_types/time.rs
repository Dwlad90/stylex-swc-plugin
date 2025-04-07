use crate::parser::Parser;
use std::fmt;

use super::number::number;

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
  pub value: f32,
  pub unit: TimeUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
  S,
  Ms,
}

impl Time {
  pub fn new(value: f32, unit: TimeUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Time> {
    Parser::<'a, f32>::sequence::<f32, String, (), ()>(
      Some(number()),
      Some(Parser::one_of(vec![
        Parser::<String>::string("s"),
        Parser::<String>::string("ms"),
      ])),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (value_opt, unit_opt, _, _) = values.expect("Time parsing failed");
      let value = value_opt.expect("Expected a number value");
      let unit_str = unit_opt.expect("Expected a unit");

      let unit = unit_str.into();

      Time::new(value, unit)
    })
  }
}

impl fmt::Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Always use the shortest representation
    match self.unit {
      TimeUnit::Ms => write!(f, "{}s", self.value / 1000.0),
      TimeUnit::S => write!(f, "{}{}", self.value, self.unit),
    }
  }
}

impl fmt::Display for TimeUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TimeUnit::S => write!(f, "s"),
      TimeUnit::Ms => write!(f, "ms"),
    }
  }
}

impl From<String> for TimeUnit {
  fn from(unit_str: String) -> Self {
    match unit_str.as_str() {
      "s" => TimeUnit::S,
      "ms" => TimeUnit::Ms,
      _ => panic!("Invalid time unit"),
    }
  }
}
