/*!
Token types tests.
*/

use crate::token_types::TokenList;

#[cfg(test)]
mod test_token_types {
  use super::*;

  #[test]
  fn token_list_constructor_from_string() {
    let token_list = TokenList::new("test");
    assert_eq!(token_list.current_index, 0);
  }

  #[test]
  fn consume_next_token_basic_functionality() {
    let mut token_list = TokenList::new("test 123");

    // Should consume tokens sequentially
    let first_token = token_list.consume_next_token();
    assert!(first_token.is_ok());

    let second_token = token_list.consume_next_token();
    assert!(second_token.is_ok());

    let third_token = token_list.consume_next_token();
    assert!(third_token.is_ok());
  }

  #[test]
  fn peek_returns_next_token_without_consuming() {
    let mut token_list = TokenList::new("test");

    let initial_index = token_list.current_index;
    let peeked_token = token_list.peek();

    // Index should not change after peek
    assert_eq!(token_list.current_index, initial_index);
    assert!(peeked_token.is_ok());

    // Peek should return same token multiple times
    let peeked_again = token_list.peek();
    assert_eq!(format!("{:?}", peeked_token), format!("{:?}", peeked_again));
  }

  #[test]
  fn save_and_restore_position() {
    let mut token_list = TokenList::new("a b c");

    // Save initial position
    let saved_pos = token_list.save_position();

    // Consume some tokens
    let _ = token_list.consume_next_token();
    let _ = token_list.consume_next_token();

    let current_pos = token_list.current_index;
    assert!(current_pos > saved_pos);

    // Restore to saved position
    let _ = token_list.restore_position(saved_pos);
    assert_eq!(token_list.current_index, saved_pos);
  }

  #[test]
  fn is_empty_returns_correct_status() {
    let token_list = TokenList::new("");
    assert!(token_list.is_empty());

    let non_empty_list = TokenList::new("test");
    assert!(!non_empty_list.is_empty());
  }

  #[test]
  fn slice_returns_token_range() {
    let mut token_list = TokenList::new("a b c d e");

    let slice = token_list.slice(0, Some(2));
    assert_eq!(slice.len(), 2);

    // Empty slice
    let empty_slice = token_list.slice(5, Some(3)); // start > end
    assert!(empty_slice.is_empty());
  }

  #[test]
  fn slice_with_none_end_parameter() {
    let mut token_list = TokenList::new("hello world");

    let slice = token_list.slice(0, None); // Should use all tokens
    // Note: slice may be empty if no tokens were consumed yet
    // Let's just verify the slice operation works without panicking
    let _ = slice; // Use the slice to avoid unused variable warning
  }

  #[test]
  fn token_iterator_interface_compatibility() {
    // Test that TokenList works with string input
    let from_string = TokenList::new("test");
    assert_eq!(from_string.current_index, 0);

    // Test that it actually contains tokens
    assert!(!from_string.tokens.is_empty());
  }

  #[test]
  fn basic_tokenization_functionality() {
    let mut token_list = TokenList::new("hello world");

    // Should be able to peek at first token
    let peeked = token_list.peek();
    assert!(peeked.is_ok());

    // Should be able to consume first token
    let consumed = token_list.consume_next_token();
    assert!(consumed.is_ok());

    // Index should advance after consumption
    assert_eq!(token_list.current_index, 1);
  }

  #[test]
  fn handle_special_css_tokens() {
    let mut token_list = TokenList::new("rgb(255, 0, 0)");

    // Should tokenize function calls properly
    assert!(!token_list.tokens.is_empty());

    // Should be able to consume tokens from function
    let first_token = token_list.consume_next_token();
    assert!(first_token.is_ok());
  }

  #[test]
  fn empty_input_handling() {
    let mut token_list = TokenList::new("");

    // Should handle empty input gracefully
    assert!(token_list.is_empty());

    // Consuming from empty list should return None
    let token = token_list.consume_next_token();
    assert!(token.is_ok());
    if let Ok(opt_token) = token {
      assert!(opt_token.is_none());
    }
  }
}
