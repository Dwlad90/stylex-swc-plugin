use std::{fmt::Formatter, str::FromStr};

use anyhow::bail;

use crate::parser::Parser;
#[derive(Clone, Debug, PartialEq)]
pub enum CalcConstant {
  Pi,
  E,
  Infinity,
  NegativeInfinity,
  NaN,
}

impl CalcConstant {
  // Helper method for string representation
  fn as_str(&self) -> &'static str {
    match self {
      CalcConstant::Pi => "pi",
      CalcConstant::E => "e",
      CalcConstant::Infinity => "infinity",
      CalcConstant::NegativeInfinity => "-infinity",
      CalcConstant::NaN => "NaN",
    }
  }

  // Add parser method similar to other types
  pub fn parse<'a>() -> Parser<'a, CalcConstant> {
    Parser::one_of(vec![
      Parser::<'a, CalcConstant>::string("pi").map(|_| CalcConstant::Pi),
      Parser::<'a, CalcConstant>::string("e").map(|_| CalcConstant::E),
      Parser::<'a, CalcConstant>::string("infinity").map(|_| CalcConstant::Infinity),
      Parser::<'a, CalcConstant>::string("-infinity").map(|_| CalcConstant::NegativeInfinity),
      Parser::<'a, CalcConstant>::string("NaN").map(|_| CalcConstant::NaN),
    ])
  }
}

impl std::fmt::Display for CalcConstant {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl FromStr for CalcConstant {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "pi" => Ok(CalcConstant::Pi),
      "e" => Ok(CalcConstant::E),
      "infinity" => Ok(CalcConstant::Infinity),
      "-infinity" => Ok(CalcConstant::NegativeInfinity),
      "NaN" => Ok(CalcConstant::NaN),
      _ => bail!("Invalid calc constant: {}", s),
    }
  }
}

impl From<CalcConstant> for String {
  fn from(calc_constant: CalcConstant) -> Self {
    calc_constant.as_str().to_string()
  }
}

impl From<String> for CalcConstant {
  fn from(s: String) -> Self {
    s.parse()
      .unwrap_or_else(|_| panic!("Invalid calc constant: {}", s))
  }
}
