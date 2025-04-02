#[cfg(test)]
mod one_or_more {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_one_ore_more() {
    let parser = Parser::one_or_more(Parser::<'static, String>::string("foo"));

    assert_eq!(parser.parse("foo").unwrap(), vec![String::from("foo"),]);
    assert_eq!(
      parser.parse("foofoo").unwrap(),
      vec![String::from("foo"), String::from("foo"),]
    );
    assert_eq!(
      parser.parse("foofoofoo").unwrap(),
      vec![
        String::from("foo"),
        String::from("foo"),
        String::from("foo"),
      ]
    );
    assert_eq!(
      parser.parse("foofoofoobar").unwrap(),
      vec![
        String::from("foo"),
        String::from("foo"),
        String::from("foo"),
      ]
    );
    assert_eq!(
      parser.parse("foofoofoofo").unwrap(),
      vec![
        String::from("foo"),
        String::from("foo"),
        String::from("foo"),
      ]
    );
  }

  #[test]
  #[should_panic(expected = "Expected foo, got bar")]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::one_or_more(Parser::<'static, String>::string("foo"));

    parser.parse("bar").unwrap();
  }
}
