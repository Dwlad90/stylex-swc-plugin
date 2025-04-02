#[cfg(test)]
mod zero_or_more {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_zero_ore_more() {
    let parser = Parser::zero_or_more(Parser::<'static, String>::string("foo"));

    assert_eq!(parser.parse("").unwrap(), Vec::<String>::new());
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
  fn fails_to_parse_a_different_string() {
    let parser = Parser::zero_or_more(Parser::<'static, String>::string("foo"));

    assert_eq!(parser.parse("bar").unwrap(), Vec::<String>::new());
  }
}
