#[cfg(test)]
mod one_of {

  use cssparser::Token;
  use stylex_css_parser::{token_parser::TokenParser, tokens::TokenType};

  #[test]
  fn parse_the_first_parser() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let number_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Number);

    let parser = TokenParser::one_of(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      number_parser.map(|t| t, None),
    ]);

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

  #[test]
  #[should_panic(
    expected = r#"No parser matched\n- Never\n- Expected token type Number, got Ident"#
  )]
  fn fails_to_parse_a_different_string() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let number_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Number);

    let parser = TokenParser::one_of(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      number_parser.map(|t| t, None),
    ]);

    parser.parse("baz").unwrap();
  }
}
