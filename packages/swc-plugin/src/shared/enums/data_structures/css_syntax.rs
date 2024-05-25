use std::fmt;

#[derive(Debug, PartialEq, Clone, Hash)]
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
      _ => panic!(r#"CSSSyntax "{}" not found"#, value),
    }
  }
}
