#[cfg(test)]
mod one_of {

  use cssparser::Token;
  use stylex_css_parser::{
    parser::Parser,
    token_parser::TokenParser,
    tokens::{self, TokenType},
  };

  #[test]
  fn parse_the_first_parser() {
    let a = TokenParser::<Token<'static>>::get_token_parser(TokenType::String);

    // a.
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::String);
    let number_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Number);

    let parser = TokenParser::one_of(vec![
      string_parser.where_fn(|t| *t == Token::Ident("foo".into()), "Description"),
      number_parser.where_fn(
        |t| {
          *t == Token::Number {
            value: 123.0,
            has_sign: false,
            int_value: Some(123),
          }
        },
        "Description",
      ),
    ]);

    // assert_eq!(parser.parse("foo").unwrap(), String::from("foo"));
    // assert_eq!(parser.parse("bar").unwrap(), String::from("bar"));

    // let parser = TokenParser::<String>::string("foo");

    let aaa = parser.parse_to_end("foo").unwrap();
    dbg!(&aaa);

    // Compare with a Token::Ident instead of a String
    // assert_eq!(aaa, "foo".to_string());

    assert_eq!(
      parser.parse_to_end("foo").unwrap(),
      Token::Ident("foo".into())
    );

    assert_eq!(
      parser.parse_to_end("123").unwrap(),
      Token::Number {
        value: 123.0,
        has_sign: false,
        int_value: Some(123)
      }
    );
  }

  // #[test]
  // #[should_panic(expected = "Expected foo, got baz")]
  // fn fails_to_parse_a_different_string() {
  //   let parser = Parser::one_of(vec![Parser::string("foo"), Parser::string("bar")]);

  //   parser.parse("baz").unwrap();
  // }
}
