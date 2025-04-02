#[cfg(test)]
mod sequence {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_sequence() {
    let parser = Parser::sequence(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
      Parser::<'static, String>::string("baz"),
    ])
    .to_parser(|p| p);
    // .map(|(first, (second, third))| {
    //   // Convert the nested tuple to a flat vector
    //   vec![first, second, third]
    // });
    let a = parser.parse("foobarbaz").unwrap();
    assert_eq!(
      a,
      vec!["foo".to_string(), "bar".to_string(), "baz".to_string(),]
    );
  }

  #[test]
  fn parse_a_sequence_separated_by_whitespace() {
    let parser = Parser::sequence(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
      Parser::<'static, String>::string("baz"),
    ])
    .separated_by(Parser::<'static, String>::whitespace().map(|_| String::new()))
    .to_parser(|p| p);

    let output = vec![
      String::from("foo"),
      String::from("bar"),
      String::from("baz"),
    ];

    assert_eq!(parser.parse("foo bar baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar      baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar\nbaz").unwrap(), output);
  }

  #[test]
  fn parse_a_sequence_separated_by_optional_whitespace() {
    let parser = Parser::sequence(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
      Parser::<'static, String>::string("baz"),
    ])
    .separated_by(
      Parser::<'static, String>::whitespace()
        .map(|_| String::new())
        .optional(),
    )
    .to_parser(|p| p);

    let output = vec![
      String::from("foo"),
      String::from("bar"),
      String::from("baz"),
    ];

    assert_eq!(parser.parse("foobarbaz").unwrap(), output);
    assert_eq!(parser.parse("foo bar baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar      baz").unwrap(), output);
    assert_eq!(parser.parse("foo   bar\nbaz").unwrap(), output);
  }

  #[test]
  #[should_panic(expected = "Expected baz, got qux")]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::sequence(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
      Parser::<'static, String>::string("baz"),
    ])
    .to_parser(|p| p);

    parser.parse("foobarqux").unwrap();
  }
}
