// Additional coverage tests for custom_ident.rs.
// Targets the `else { stylex_unreachable!() }` arm inside the ident-extraction
// closure, which is unreachable through the public parser but coverable by
// calling the extracted named function directly with a non-Ident token.

use super::*;
use crate::token_types::SimpleToken;

#[test]
fn extract_ident_token_returns_value_for_ident() {
  // Happy path: a proper Ident token yields its string value.
  let token = SimpleToken::Ident("myIdentifier".to_string());
  assert_eq!(CustomIdentifier::extract_ident_token(token), "myIdentifier");
}

#[test]
#[should_panic]
fn extract_ident_token_panics_for_non_ident() {
  // The else-branch inside extract_ident_token is unreachable through the
  // public parser. Calling the named function directly with a non-Ident token
  // exercises that defensive branch.
  CustomIdentifier::extract_ident_token(SimpleToken::Number(1.0));
}
