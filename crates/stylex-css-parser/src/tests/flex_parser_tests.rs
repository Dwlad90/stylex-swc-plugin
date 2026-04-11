// Tests extracted for flex_parser.rs behaviors and edge cases.
// Source: crates/stylex-css-parser/src/flex_parser.rs

use stylex_macros::stylex_panic;

use super::*;

#[test]
fn test_mixed_sequence() {
  // This simulates parsing: rgb(255, 0, 0)
  let values = vec![
    CssValue::ident("rgb"),
    CssValue::number(255.0),
    CssValue::ident(","),
    CssValue::number(0.0),
    CssValue::ident(","),
    CssValue::number(0.0),
  ];

  let result = CssValue::sequence(values);

  if let Some(seq) = result.as_sequence() {
    assert_eq!(seq.len(), 6);
    assert_eq!(seq[0].as_string(), Some(&"rgb".to_string()));
    assert_eq!(seq[1].as_number(), Some(255.0));
    assert_eq!(seq[3].as_number(), Some(0.0));
    assert_eq!(seq[5].as_number(), Some(0.0));
  } else {
    stylex_panic!("Expected sequence");
  }
}

#[test]
fn test_try_all_flexibility() {
  // This demonstrates how try_all can handle different types
  let numeric_value = CssValue::number(42.0);
  let string_value = CssValue::string("auto");
  let percentage_value = CssValue::percentage(50.0);

  // try_all could parse any of these successfully
  assert!(numeric_value.is_number());
  assert!(string_value.is_string());
  assert!(percentage_value.is_percentage());
}

#[test]
fn test_smart_tokens() {
  // Test that smart tokens automatically extract values correctly
  let num_token = SimpleToken::Number(42.0);
  let percent_token = SimpleToken::Percentage(50.0);
  let dim_token = SimpleToken::Dimension {
    value: 10.0,
    unit: "px".to_string(),
  };

  let num_value: CssValue = num_token.into();
  let percent_value: CssValue = percent_token.into();
  let dim_value: CssValue = dim_token.into();

  assert_eq!(num_value.as_number(), Some(42.0));
  assert_eq!(percent_value.as_percentage(), Some(50.0));
  assert_eq!(dim_value.as_dimension(), Some((10.0, &"px".to_string())));
}
