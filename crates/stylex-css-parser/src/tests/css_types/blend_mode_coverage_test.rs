// Additional coverage tests for blend_mode.rs.
// Targets the `else { stylex_unreachable!() }` arm inside the ident-extraction
// closure, which is unreachable through the public parser but coverable by
// calling the extracted named function directly with a non-Ident token.

use super::*;
use crate::token_types::SimpleToken;

#[test]
fn extract_ident_token_returns_value_for_ident() {
  // Happy path: a proper Ident token yields its string value.
  let token = SimpleToken::Ident("multiply".to_string());
  assert_eq!(BlendMode::extract_ident_token(token), "multiply");
}

#[test]
#[should_panic]
fn extract_ident_token_panics_for_non_ident() {
  // The else-branch inside extract_ident_token is unreachable through the
  // public parser (tokens::ident() only yields Ident tokens). Calling the
  // named function directly with a non-Ident token exercises that branch.
  BlendMode::extract_ident_token(SimpleToken::Number(42.0));
}
