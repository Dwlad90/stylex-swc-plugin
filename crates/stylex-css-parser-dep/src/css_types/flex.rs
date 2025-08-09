use crate::parser::Parser;
/**
 * @flow strict
 */
use std::fmt::{self, Display};

/// e.g. 1fr
#[derive(Debug, Clone, PartialEq)]
pub struct Flex {
  pub fraction: f32,
}

impl Flex {
  pub fn new(fraction: f32) -> Self {
    Self { fraction }
  }

  pub fn parse<'a>() -> Parser<'a, Flex> {
    Parser::<'a, String>::sequence::<f32, String, (), ()>(
      Some(Parser::<'a, f32>::float().where_fn(|&num| num >= 0.0)),
      Some(Parser::<'a, String>::string("fr")),
      None,
      None,
    )
    .to_parser()
    .map(|tuple| {
      let (fraction, _unit, _, _) = tuple.expect("Expected tuple");
      Flex::new(fraction.unwrap())
    })
  }
}

impl Display for Flex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}fr", self.fraction)
  }
}
