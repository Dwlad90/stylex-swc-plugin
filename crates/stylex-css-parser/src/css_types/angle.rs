use anyhow::bail;

use std::convert::TryFrom;
use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};

use crate::parser::Parser;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AngleUnit {
  Deg,
  Grad,
  Rad,
  Turn,
  #[default]
  Default,
}

impl AngleUnit {
  fn as_str(&self) -> &'static str {
    match self {
      AngleUnit::Deg => "deg",
      AngleUnit::Grad => "grad",
      AngleUnit::Rad => "rad",
      AngleUnit::Turn => "turn",
      AngleUnit::Default => "",
    }
  }
}

impl Display for AngleUnit {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str(self.as_str())
  }
}

impl FromStr for AngleUnit {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "deg" => Ok(AngleUnit::Deg),
      "grad" => Ok(AngleUnit::Grad),
      "rad" => Ok(AngleUnit::Rad),
      "turn" => Ok(AngleUnit::Turn),
      "" => Ok(AngleUnit::Default),
      _ => bail!("Invalid angle unit: {}", s),
    }
  }
}

impl From<AngleUnit> for String {
  fn from(unit: AngleUnit) -> Self {
    unit.as_str().to_string()
  }
}

impl TryFrom<String> for AngleUnit {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    value.parse()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
  pub value: f32,
  pub unit: AngleUnit,
}

impl Angle {
  pub fn new(value: f32, unit: Option<String>) -> Self {
    Angle {
      value,
      unit: match unit {
        Some(u) => AngleUnit::try_from(u).unwrap_or_default(),
        None => AngleUnit::default(),
      },
    }
  }

  /// Create a parser that recognizes any valid angle
  pub fn parse<'a>() -> Parser<'a, Angle> {
    Parser::one_of(vec![
      Deg::parse(),
      Grad::parse(),
      Rad::parse(),
      Turn::parse(),
      Parser::<String>::string("0").map(|_| Angle::new(0.0, None)),
    ])
  }
}

impl Display for Angle {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Angle> for String {
  fn from(val: Angle) -> Self {
    val.to_string()
  }
}

impl From<String> for Angle {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Angle::parse()
      .run(&mut input)
      .expect("Failed to parse angle")
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deg {
  pub value: f32,
  pub unit: AngleUnit,
}

impl Deg {
  pub fn new(value: f32) -> Self {
    Deg {
      value,
      unit: AngleUnit::Deg,
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

impl From<Deg> for String {
  fn from(val: Deg) -> Self {
    val.to_string()
  }
}

impl From<Deg> for Angle {
  fn from(deg: Deg) -> Self {
    Angle {
      value: deg.value,
      unit: deg.unit,
    }
  }
}

impl From<Angle> for Deg {
  fn from(angle: Angle) -> Self {
    Deg {
      value: angle.value,
      unit: angle.unit,
    }
  }
}

impl From<String> for Deg {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Angle::parse()
      .run(&mut input)
      .expect("Failed to parse angle")
      .into()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grad {
  pub value: f32,
  pub unit: AngleUnit,
}

impl Grad {
  pub fn new(value: f32) -> Self {
    Grad {
      value,
      unit: AngleUnit::Grad,
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

impl From<Grad> for String {
  fn from(val: Grad) -> Self {
    val.to_string()
  }
}

impl From<Angle> for Grad {
  fn from(angle: Angle) -> Self {
    Grad {
      value: angle.value,
      unit: angle.unit,
    }
  }
}

impl From<String> for Grad {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Angle::parse()
      .run(&mut input)
      .expect("Failed to parse angle")
      .into()
  }
}
impl From<Grad> for Angle {
  fn from(grad: Grad) -> Self {
    Angle {
      value: grad.value,
      unit: grad.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rad {
  pub value: f32,
  pub unit: AngleUnit,
}

impl Rad {
  pub fn new(value: f32) -> Self {
    Rad {
      value,
      unit: AngleUnit::Rad,
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

impl From<Rad> for String {
  fn from(val: Rad) -> Self {
    val.to_string()
  }
}

impl From<Angle> for Rad {
  fn from(angle: Angle) -> Self {
    Rad {
      value: angle.value,
      unit: angle.unit,
    }
  }
}
impl From<String> for Rad {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Angle::parse()
      .run(&mut input)
      .expect("Failed to parse angle")
      .into()
  }
}
impl From<Rad> for Angle {
  fn from(rad: Rad) -> Self {
    Angle {
      value: rad.value,
      unit: rad.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
  pub value: f32,
  pub unit: AngleUnit,
}

impl Turn {
  pub fn new(value: f32) -> Self {
    Turn {
      value,
      unit: AngleUnit::Turn,
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

impl From<Turn> for String {
  fn from(val: Turn) -> Self {
    val.to_string()
  }
}

impl From<Angle> for Turn {
  fn from(angle: Angle) -> Self {
    Turn {
      value: angle.value,
      unit: angle.unit,
    }
  }
}
impl From<String> for Turn {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Angle::parse()
      .run(&mut input)
      .expect("Failed to parse angle")
      .into()
  }
}
impl From<Turn> for Angle {
  fn from(turn: Turn) -> Self {
    Angle {
      value: turn.value,
      unit: turn.unit,
    }
  }
}
