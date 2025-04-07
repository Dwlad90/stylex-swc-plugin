use crate::parser::Parser;
use std::fmt;

use super::frequency::Frequency;
use super::length::Length;
use super::resolution::Resolution;
use super::time::Time;

#[derive(Debug, Clone, PartialEq)]
pub enum Dimension {
  Length(Length),
  Time(Time),
  Frequency(Frequency),
  Resolution(Resolution),
}

impl Dimension {
  pub fn parse<'a>() -> Parser<'a, Dimension> {
    Parser::one_of::<Dimension>(vec![
      Length::parse().map(|p| p.unwrap().into()),
      Time::parse().map(|p| p.unwrap().into()),
      Frequency::parse().map(|p| p.unwrap().into()),
      Resolution::parse().map(|p| p.unwrap().into()),
    ])
  }
}

impl fmt::Display for Dimension {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Dimension::Length(length) => write!(f, "{}", length),
      Dimension::Time(time) => write!(f, "{}", time),
      Dimension::Frequency(frequency) => write!(f, "{}", frequency),
      Dimension::Resolution(resolution) => write!(f, "{}", resolution),
    }
  }
}

impl From<Length> for Dimension {
  fn from(length: Length) -> Self {
    Dimension::Length(length)
  }
}

impl From<Time> for Dimension {
  fn from(time: Time) -> Self {
    Dimension::Time(time)
  }
}

impl From<Frequency> for Dimension {
  fn from(frequency: Frequency) -> Self {
    Dimension::Frequency(frequency)
  }
}

impl From<Resolution> for Dimension {
  fn from(resolution: Resolution) -> Self {
    Dimension::Resolution(resolution)
  }
}

impl From<String> for Dimension {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Dimension::parse()
      .run(&mut input)
      .expect("Failed to parse dimension")
  }
}

impl From<Dimension> for String {
  fn from(dimension: Dimension) -> Self {
    dimension.to_string()
  }
}
