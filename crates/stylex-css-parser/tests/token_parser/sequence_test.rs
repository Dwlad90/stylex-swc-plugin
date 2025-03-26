#[cfg(test)]
mod sequence {

  use cssparser::Token;
  use stylex_css_parser::{token_parser::TokenParser, tokens::TokenType};

  #[test]
  fn parse_a_sequence() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);
    // let number_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Number);

    let parser = TokenParser::sequence(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      whitespace_parser.map(|t| t, None),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("baz".into()), None),
    ])
    .map(
      |tokens| {
        dbg!(&tokens);
        let foo = tokens[0].clone().unwrap();
        let baz = tokens[2].clone().unwrap();

        vec![foo, baz]
      },
      None,
    );

    assert_eq!(
      parser.parse_to_end("foo baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
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
