#[cfg(test)]
mod zero_or_more {

  use cssparser::Token;
  use stylex_css_parser::{token_parser::TokenParser, tokens::TokenType};

  #[test]
  fn parses_zero_or_more() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::zero_or_more(
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
    )
    .separated_by(whitespace_parser)
    .to_parser();

    assert_eq!(parser.parse("").unwrap(), Some(vec![]));

    assert_eq!(
      parser.parse("foo").unwrap(),
      Some(vec![Token::Ident("foo".into())])
    );

    assert_eq!(
      parser.parse("foo foo").unwrap(),
      Some(vec![Token::Ident("foo".into()), Token::Ident("foo".into())])
    );

    assert_eq!(
      parser.parse("foo foo foo").unwrap(),
      Some(vec![
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into())
      ])
    );
    assert_eq!(
      parser.parse("foo foo foo bar").unwrap(),
      Some(vec![
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into())
      ])
    );

    assert_eq!(
      parser.parse("foo foo foo for").unwrap(),
      Some(vec![
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into())
      ])
    );
    assert_eq!(
      parser.parse("foo foo foo foo").unwrap(),
      Some(vec![
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into())
      ])
    );
    assert_eq!(
      parser.parse("foo foo foo foo foo").unwrap(),
      Some(vec![
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into()),
        Token::Ident("foo".into())
      ])
    );
  }

  #[test]
  #[should_panic(expected = r#""Expected end of input, got Ident(\"for\") instead\nConsumed tokens: [Some(Ident(\"foo\")), Some(Ident(\"foo\")), Some(Ident(\"foo\"))]"#)]
  fn fails_to_parse_a_different_string() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::zero_or_more(
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
    )
    .separated_by(whitespace_parser)
    .to_parser();

    parser.parse_to_end("foo foo foo for").unwrap();
  }
}
