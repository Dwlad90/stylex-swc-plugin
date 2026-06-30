use super::*;
use crate::token_types::SimpleToken;

#[test]
fn extract_percentage_token_returns_value_for_percentage() {
  // Happy path: a Percentage token yields an AlphaValue.
  let token = SimpleToken::Percentage(0.75);
  let result = AlphaValue::extract_percentage_token(token);
  assert_eq!(result.value, 0.75_f32);
}

#[test]
#[should_panic]
fn extract_percentage_token_panics_for_non_percentage() {
  // The else-branch inside extract_percentage_token is unreachable through the
  // public parser (the token combinator only yields Percentage tokens). Calling
  // the named function directly with a non-Percentage token exercises that
  // defensive branch.
  AlphaValue::extract_percentage_token(SimpleToken::Number(0.5));
}

#[test]
fn extract_number_token_returns_value_for_number() {
  // Happy path: a Number token yields an AlphaValue.
  let token = SimpleToken::Number(0.25);
  let result = AlphaValue::extract_number_token(token);
  assert_eq!(result.value, 0.25_f32);
}

#[test]
#[should_panic]
fn extract_number_token_panics_for_non_number() {
  // The else-branch inside extract_number_token is unreachable through the
  // public parser (the token combinator only yields Number tokens). Calling
  // the named function directly with a non-Number token exercises that
  // defensive branch.
  AlphaValue::extract_number_token(SimpleToken::Percentage(0.5));
}
