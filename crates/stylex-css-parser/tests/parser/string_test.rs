#[cfg(test)]
mod string {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_string() {
    let parser = Parser::string("foo");

    assert_eq!(parser.parse("foo").unwrap(), String::from("foo"));
  }

  #[test]
  #[should_panic(expected = "Expected foo, got bar")]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::string("foo");

    parser.parse("bar").unwrap();
  }
}
