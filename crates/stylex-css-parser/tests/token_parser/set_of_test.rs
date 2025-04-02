#[cfg(test)]
mod set_of {

  use cssparser::Token;
  use stylex_css_parser::{token_parser::TokenParser, tokens::TokenType};

  #[test]
  fn parses_a_set() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::set_of(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("baz".into()), None),
    ])
    .separated_by(whitespace_parser)
    .to_parser()
    .map(
      |tokens| tokens.into_iter().flatten().collect::<Vec<_>>(),
      None,
    );

    assert_eq!(
      parser.parse_to_end("foo baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("baz foo").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );
  }

  #[test]
  fn parses_a_set_with_double_separators() {
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);
    let comma_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Comma);

    let parser = TokenParser::set_of(vec![
      TokenParser::<Token<'static>>::string("foo"),
      TokenParser::<Token<'static>>::string("baz"),
    ])
    .separated_by(comma_parser)
    .separated_by(whitespace_parser.optional())
    .to_parser()
    .map(
      |tokens| tokens.into_iter().flatten().collect::<Vec<_>>(),
      None,
    );

    assert_eq!(
      parser.parse_to_end("foo,baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("foo, baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("foo   , baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("foo   ,baz").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("baz,foo").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("baz, foo").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );

    assert_eq!(
      parser.parse_to_end("baz   , foo").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );
    assert_eq!(
      parser.parse_to_end("baz   ,foo").unwrap(),
      vec![Token::Ident("foo".into()), Token::Ident("baz".into())]
    );
  }

  #[test]
  fn makes_separators_optional_for_optional_parsers() {
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);
    let parser = TokenParser::set_of(vec![
      TokenParser::<Token<'static>>::string("foo"),
      TokenParser::<Token<'static>>::string("bar").optional(),
      TokenParser::<Token<'static>>::string("baz"),
    ])
    .separated_by(whitespace_parser)
    .to_parser();

    assert_eq!(
      parser.parse_to_end("foo bar baz").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        Some(Token::Ident("bar".into())),
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("baz foo").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        None,
        Some(Token::Ident("baz".into())),
      ]
    );

    assert_eq!(
      parser.parse_to_end("bar foo baz").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        Some(Token::Ident("bar".into())),
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("bar baz foo").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        Some(Token::Ident("bar".into())),
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("baz bar foo").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        Some(Token::Ident("bar".into())),
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("baz foo bar").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        Some(Token::Ident("bar".into())),
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("foo baz").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        None,
        Some(Token::Ident("baz".into()))
      ]
    );

    assert_eq!(
      parser.parse_to_end("baz foo").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        None,
        Some(Token::Ident("baz".into()))
      ]
    );
  }

  // #[test]
  // #[should_panic(
  //   expected = r#"No parser matched\n- Never\n- Expected token type Number, got Ident"#
  // )]
  // fn fails_to_parse_a_different_string() {
  //   let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
  //   let number_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Number);

  //   let parser = TokenParser::one_of(vec![
  //     string_parser
  //       .map(|t| t, None)
  //       .where_fn(|t| *t == Token::Ident("foo".into()), None),
  //     number_parser.map(|t| t, None),
  //   ]);

  //   parser.parse("baz").unwrap();
  // }
}
