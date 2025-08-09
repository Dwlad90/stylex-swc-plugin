use crate::parser::Parser;
use std::fmt;

use super::number::number;

#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
  pub value: f32,
  pub unit: ResolutionUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionUnit {
  Dpi,
  Dpcm,
  Dppx,
  X,
}

impl Resolution {
  pub fn new(value: f32, unit: ResolutionUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Resolution> {
    Parser::<'a, f32>::sequence::<f32, String, (), ()>(
      Some(number()),
      Some(Parser::one_of(vec![
        Parser::<String>::string("dpi"),
        Parser::<String>::string("dpcm"),
        Parser::<String>::string("dppx"),
        Parser::<String>::string("x"),
      ])),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (value_opt, unit_opt, _, _) = values.expect("Resolution parsing failed");
      let value = value_opt.expect("Expected a number value");
      let unit_str = unit_opt.expect("Expected a unit");

      let unit = unit_str.into();

      Resolution::new(value, unit)
    })
  }
}

impl fmt::Display for Resolution {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl fmt::Display for ResolutionUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ResolutionUnit::Dpi => write!(f, "dpi"),
      ResolutionUnit::Dpcm => write!(f, "dpcm"),
      ResolutionUnit::Dppx => write!(f, "dppx"),
      ResolutionUnit::X => write!(f, "x"),
    }
  }
}

impl From<String> for ResolutionUnit {
  fn from(unit_str: String) -> Self {
    match unit_str.as_str() {
      "dpi" => ResolutionUnit::Dpi,
      "dpcm" => ResolutionUnit::Dpcm,
      "dppx" => ResolutionUnit::Dppx,
      "x" => ResolutionUnit::X,
      _ => panic!("Invalid resolution unit"),
    }
  }
}
