use crate::css_types::common_types::Percentage;
use crate::{css_types::angle::Angle, parser::Parser};

/// Parser for angle percentage values, which can be either an Angle or a Percentage
pub fn angle_percentage<'i>() -> Parser<'i, String> {
  Parser::one_of::<String>(vec![
    Percentage::parse().map(|percentage| percentage.unwrap().into()),
    Angle::parse().map(|angle| angle.unwrap().into()),
  ])
}
