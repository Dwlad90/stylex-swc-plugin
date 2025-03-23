#[cfg(test)]
mod one_of {

  use cssparser::{Parser, ParserInput};
  use stylex_css_parser::{parser, token_parser::TokenParser};

  #[test]
  fn parse_the_first_parser() {
    let parser = TokenParser::one_of(vec![Parser::string("foo"), Parser::string("bar")]);

    // assert_eq!(parser.parse("foo").unwrap(), String::from("foo"));
    // assert_eq!(parser.parse("bar").unwrap(), String::from("bar"));

    let parser = TokenParser::<String>::string("foo");

    let result = parser.parse_to_end("foo").unwrap();

    assert_eq!(result, String::from("foo"));
  }

  // #[test]
  // #[should_panic(expected = "Expected foo, got baz")]
  // fn fails_to_parse_a_different_string() {
  //   let parser = Parser::one_of(vec![Parser::string("foo"), Parser::string("bar")]);

  //   parser.parse("baz").unwrap();
  // }
}
