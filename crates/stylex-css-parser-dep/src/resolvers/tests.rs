#[cfg(test)]
mod css_parser_tests {
  use crate::resolvers::parse_css;

  #[test]
  fn test_empty_input() {
    let result = parse_css("");
    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert!(tokens.is_empty(), "Expected no tokens for empty input");
  }

  #[test]
  fn test_simple_identifier() {
    // Simple identifier input like a property name.
    let input = "color";
    let result = parse_css(input);
    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert!(
      !tokens.is_empty(),
      "Expected some tokens for a single identifier"
    );
  }

  #[test]
  fn test_complex_css() {
    // CSS with selectors, properties, and punctuation.
    let input = "body {{ margin: 0; padding: 0; }}";
    let result = parse_css(input);
    dbg!(&result);
    assert!(result.is_ok());
    let tokens = result.unwrap();
    // Expect multiple tokens to be parsed from a more complex input.
    assert!(
      tokens.len() >= 8,
      "Expected at least 8 tokens, got {}",
      tokens.len()
    );
  }

  #[test]
  fn test_invalid_css() {
    // Some malformed CSS; depending on cssparser behavior it may either return tokens or an error.
    let input = "div { invalid-css ";
    let result = parse_css(input);
    if let Err(err) = result {
      assert!(
        err.contains("Error parsing CSS"),
        "Error message not as expected"
      );
    } else {
      let tokens = result.unwrap();
      assert!(
        !tokens.is_empty(),
        "Expected tokens even if input is partially invalid"
      );
    }
  }
}
