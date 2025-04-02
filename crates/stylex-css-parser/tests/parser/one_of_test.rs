#[cfg(test)]
mod one_of {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_the_first_parser() {
    let parser = Parser::one_of(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
    ]);

    assert_eq!(parser.parse("foo").unwrap(), String::from("foo"));
    assert_eq!(parser.parse("bar").unwrap(), String::from("bar"));
  }

  #[test]
  #[should_panic(expected = "Expected foo, got baz")]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::one_of(vec![
      Parser::<'static, String>::string("foo"),
      Parser::<'static, String>::string("bar"),
    ]);

    parser.parse("baz").unwrap();
  }
}
