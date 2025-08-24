/*!
Error messages for media query parsing and validation.
*/

pub struct MediaQueryErrors;

impl MediaQueryErrors {
  pub const SYNTAX_ERROR: &'static str = "Invalid media query syntax.";
  pub const UNBALANCED_PARENS: &'static str = "Unbalanced parentheses in media query.";
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_messages_exist() {
    assert!(!MediaQueryErrors::SYNTAX_ERROR.is_empty());
    assert!(!MediaQueryErrors::UNBALANCED_PARENS.is_empty());
  }

  #[test]
  fn test_error_message_content() {
    assert!(MediaQueryErrors::SYNTAX_ERROR.contains("syntax"));
    assert!(MediaQueryErrors::UNBALANCED_PARENS.contains("parentheses"));
  }

  #[test]
  fn test_error_message_content_validation() {
    assert_eq!(
      MediaQueryErrors::SYNTAX_ERROR,
      "Invalid media query syntax."
    );
    assert_eq!(
      MediaQueryErrors::UNBALANCED_PARENS,
      "Unbalanced parentheses in media query."
    );
  }
}
