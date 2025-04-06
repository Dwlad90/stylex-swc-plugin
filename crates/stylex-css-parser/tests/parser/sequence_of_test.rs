#[cfg(test)]
mod sequence {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_sequence() {
    let parser = Parser::<'static, String>::sequence::<String, String, String, String>(
      Some(Parser::<'static, String>::string("foo")),
      Some(Parser::<'static, String>::string("bar")),
      Some(Parser::<'static, String>::string("baz")),
      None,
    )
    .to_parser();

    assert_eq!(
      parser.parse("foobarbaz").unwrap(),
      (
        Some("foo".to_string()),
        Some("bar".to_string()),
        Some("baz".to_string()),
        None
      )
    );
  }

  #[test]
  fn parse_a_sequence_separated_by_whitespace() {
    let parser = Parser::<'static, String>::sequence::<String, String, String, String>(
      Some(Parser::<'static, String>::string("foo")),
      Some(Parser::<'static, String>::string("bar")),
      Some(Parser::<'static, String>::string("baz")),
      None,
    )
    .separated_by(Parser::<'static, String>::whitespace().map(|_| String::new()))
    .to_parser();

    let output = (
      Some(String::from("foo")),
      Some(String::from("bar")),
      Some(String::from("baz")),
      None,
    );

    assert_eq!(parser.parse("foo bar baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar      baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar\nbaz").unwrap(), output);
  }

  #[test]
  fn parse_a_sequence_separated_by_optional_whitespace() {
    let parser = Parser::<'static, String>::sequence::<String, String, String, String>(
      Some(Parser::<'static, String>::string("foo")),
      Some(Parser::<'static, String>::string("bar")),
      Some(Parser::<'static, String>::string("baz")),
      None,
    )
    .separated_by(
      Parser::<'static, String>::whitespace()
        .map(|_| String::new())
        .optional(),
    )
    .to_parser();

    let output = (
      Some(String::from("foo")),
      Some(String::from("bar")),
      Some(String::from("baz")),
      None,
    );

    assert_eq!(parser.parse("foobarbaz").unwrap(), output);
    assert_eq!(parser.parse("foo bar baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar      baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar\nbaz").unwrap(), output);
  }

  #[test]
  #[should_panic(expected = "Expected baz, got qux")]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::<'static, String>::sequence::<String, String, String, String>(
      Some(Parser::<'static, String>::string("foo")),
      Some(Parser::<'static, String>::string("bar")),
      Some(Parser::<'static, String>::string("baz")),
      None,
    )
    .to_parser();

    parser.parse("foobarqux").unwrap();
  }
}
