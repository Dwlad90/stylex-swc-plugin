//! Tests for CSSSyntax enum: From<String> conversion and Display formatting.

use crate::css_syntax::CSSSyntax;

/// Verify every valid CSS syntax string converts to the correct enum variant.
#[test]
fn from_string_all_variants() {
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

/// Verify Display output matches the expected CSS syntax string.
#[test]
fn display_all_variants() {
  assert_eq!(CSSSyntax::Angle.to_string(), "<angle>");
  assert_eq!(CSSSyntax::Color.to_string(), "<color>");
  assert_eq!(CSSSyntax::Image.to_string(), "<image>");
  assert_eq!(CSSSyntax::Integer.to_string(), "<integer>");
  assert_eq!(CSSSyntax::Length.to_string(), "<length>");
  assert_eq!(
    CSSSyntax::LengthPercentage.to_string(),
    "<lengthPercentage>"
  );
  assert_eq!(CSSSyntax::Number.to_string(), "<number>");
  assert_eq!(CSSSyntax::Percentage.to_string(), "<percentage>");
  assert_eq!(CSSSyntax::Resolution.to_string(), "<resolution>");
  assert_eq!(CSSSyntax::Time.to_string(), "<time>");
  assert_eq!(
    CSSSyntax::TransformFunction.to_string(),
    "<transformFunction>"
  );
  assert_eq!(CSSSyntax::TransformList.to_string(), "<transformList>");
  assert_eq!(CSSSyntax::Url.to_string(), "<url>");
}
