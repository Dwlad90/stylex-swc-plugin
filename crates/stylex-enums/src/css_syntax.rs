use std::fmt;

use stylex_macros::stylex_panic;

#[derive(Debug, PartialEq, Clone, Hash, Copy)]
pub enum CSSSyntax {
  Length,
  Number,
  Percentage,
  LengthPercentage,
  Color,
  Image,
  Url,
  Integer,
  Angle,
  Time,
  Resolution,
  TransformFunction,
  TransformList,
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for CSSSyntax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CSSSyntax::Angle => write!(f, "<angle>"),
      CSSSyntax::Color => write!(f, "<color>"),
      CSSSyntax::Image => write!(f, "<image>"),
      CSSSyntax::Integer => write!(f, "<integer>"),
      CSSSyntax::Length => write!(f, "<length>"),
      CSSSyntax::LengthPercentage => write!(f, "<lengthPercentage>"),
      CSSSyntax::Number => write!(f, "<number>"),
      CSSSyntax::Percentage => write!(f, "<percentage>"),
      CSSSyntax::Resolution => write!(f, "<resolution>"),
      CSSSyntax::Time => write!(f, "<time>"),
      CSSSyntax::TransformFunction => write!(f, "<transformFunction>"),
      CSSSyntax::TransformList => write!(f, "<transformList>"),
      CSSSyntax::Url => write!(f, "<url>"),
    }
  }
}

impl From<String> for CSSSyntax {
  fn from(value: String) -> Self {
    match value.as_str() {
      "<angle>" => CSSSyntax::Angle,
      "<color>" => CSSSyntax::Color,
      "<image>" => CSSSyntax::Image,
      "<integer>" => CSSSyntax::Integer,
      "<length>" => CSSSyntax::Length,
      "<lengthPercentage>" => CSSSyntax::LengthPercentage,
      "<number>" => CSSSyntax::Number,
      "<percentage>" => CSSSyntax::Percentage,
      "<resolution>" => CSSSyntax::Resolution,
      "<time>" => CSSSyntax::Time,
      "<transformFunction>" => CSSSyntax::TransformFunction,
      "<transformList>" => CSSSyntax::TransformList,
      "<url>" => CSSSyntax::Url,
      _ => stylex_panic!(r#"CSSSyntax "{}" not found"#, value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_string_all_variants() {
    let cases = vec![
      ("<angle>", CSSSyntax::Angle),
      ("<color>", CSSSyntax::Color),
      ("<image>", CSSSyntax::Image),
      ("<integer>", CSSSyntax::Integer),
      ("<length>", CSSSyntax::Length),
      ("<lengthPercentage>", CSSSyntax::LengthPercentage),
      ("<number>", CSSSyntax::Number),
      ("<percentage>", CSSSyntax::Percentage),
      ("<resolution>", CSSSyntax::Resolution),
      ("<time>", CSSSyntax::Time),
      ("<transformFunction>", CSSSyntax::TransformFunction),
      ("<transformList>", CSSSyntax::TransformList),
      ("<url>", CSSSyntax::Url),
    ];
    for (input, expected) in cases {
      assert_eq!(CSSSyntax::from(input.to_string()), expected);
    }
  }

  #[test]
  fn test_display_all_variants() {
    assert_eq!(CSSSyntax::Angle.to_string(), "<angle>");
    assert_eq!(CSSSyntax::Color.to_string(), "<color>");
    assert_eq!(CSSSyntax::Image.to_string(), "<image>");
    assert_eq!(CSSSyntax::Integer.to_string(), "<integer>");
    assert_eq!(CSSSyntax::Length.to_string(), "<length>");
    assert_eq!(CSSSyntax::LengthPercentage.to_string(), "<lengthPercentage>");
    assert_eq!(CSSSyntax::Number.to_string(), "<number>");
    assert_eq!(CSSSyntax::Percentage.to_string(), "<percentage>");
    assert_eq!(CSSSyntax::Resolution.to_string(), "<resolution>");
    assert_eq!(CSSSyntax::Time.to_string(), "<time>");
    assert_eq!(CSSSyntax::TransformFunction.to_string(), "<transformFunction>");
    assert_eq!(CSSSyntax::TransformList.to_string(), "<transformList>");
    assert_eq!(CSSSyntax::Url.to_string(), "<url>");
  }
}
