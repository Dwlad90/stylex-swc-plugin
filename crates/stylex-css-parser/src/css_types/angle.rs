use crate::parser::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
  pub value: f32,
  pub unit: String,
}

impl Angle {
  pub fn new(value: f32) -> Self {
    Angle {
      value,
      unit: String::new(),
    }
  }

  /// Create a parser that recognizes any valid angle
  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::one_of(vec![
      Deg::parse(),
      Grad::parse(),
      Rad::parse(),
      Turn::parse(),
      Parser::<String>::string("0").map(|_| Angle::new(0.0)),
    ])
  }
}

impl Display for Angle {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deg {
  pub value: f32,
  pub unit: String,
}

impl Deg {
  pub fn new(value: f32) -> Self {
    Deg {
      value,
      unit: "deg".to_string(),
    }
  }

  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::<f32>::float()
      .skip(Parser::<String>::string("deg"))
      .map(|deg| {
        let deg = Deg::new(deg.expect("Expected float value"));

        Angle {
          value: deg.value,
          unit: deg.unit,
        }
      })
  }
}

impl Display for Deg {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grad {
  pub value: f32,
  pub unit: String,
}

impl Grad {
  pub fn new(value: f32) -> Self {
    Grad {
      value,
      unit: "grad".to_string(),
    }
  }

  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::<f32>::float()
      .skip(Parser::<String>::string("grad"))
      .map(|v| {
        let grad = Grad::new(v.expect("Expected float value"));
        Angle {
          value: grad.value,
          unit: grad.unit,
        }
      })
  }
}

impl Display for Grad {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rad {
  pub value: f32,
  pub unit: String,
}

impl Rad {
  pub fn new(value: f32) -> Self {
    Rad {
      value,
      unit: "rad".to_string(),
    }
  }

  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::<f32>::float()
      .skip(Parser::<String>::string("rad"))
      .map(|v| {
        let rad = Rad::new(v.expect("Expected float value"));

        Angle {
          value: rad.value,
          unit: rad.unit,
        }
      })
  }
}

impl Display for Rad {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
  pub value: f32,
  pub unit: String,
}

impl Turn {
  pub fn new(value: f32) -> Self {
    Turn {
      value,
      unit: "turn".to_string(),
    }
  }

  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::<f32>::float()
      .skip(Parser::<String>::string("turn"))
      .map(|v| {
        let turn = Turn::new(v.expect("Expected float value"));
        Angle {
          value: turn.value,
          unit: turn.unit,
        }
      })
  }
}

impl Display for Turn {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}
