use crate::parser::Parser;
use std::fmt;

use super::number::number;

#[derive(Debug, Clone, PartialEq)]
pub struct Frequency {
  pub value: f32,
  pub unit: FrequencyUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrequencyUnit {
  Hz,
  KHz,
}

impl Frequency {
  pub fn new(value: f32, unit: FrequencyUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Frequency> {
    Parser::<'a, f32>::sequence::<f32, String, (), ()>(
      Some(number()),
      Some(Parser::one_of(vec![
        Parser::<String>::string("Hz"),
        Parser::<String>::string("KHz"),
      ])),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (value_opt, unit_opt, _, _) = values.expect("Frequency parsing failed");
      let value = value_opt.expect("Expected a number value");
      let unit_str = unit_opt.expect("Expected a unit");

      let unit = unit_str.into();

      Frequency::new(value, unit)
    })
  }
}

impl fmt::Display for Frequency {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Always use the shortest representation
    match self.unit {
      FrequencyUnit::KHz => write!(f, "{}s", self.value / 1000.0),
      FrequencyUnit::Hz => write!(f, "{}{}", self.value, self.unit),
    }
  }
}

impl fmt::Display for FrequencyUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FrequencyUnit::Hz => write!(f, "Hz"),
      FrequencyUnit::KHz => write!(f, "KHz"),
    }
  }
}

impl From<String> for FrequencyUnit {
  fn from(unit_str: String) -> Self {
    match unit_str.as_str() {
      "Hz" => FrequencyUnit::Hz,
      "KHz" => FrequencyUnit::KHz,
      _ => panic!("Invalid frequency unit"),
    }
  }
}
