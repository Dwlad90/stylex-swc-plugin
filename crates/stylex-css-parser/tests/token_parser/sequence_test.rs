#[cfg(test)]
mod sequence {

  use cssparser::Token;
  use stylex_css_parser::{token_parser::TokenParser, tokens::TokenType};

  #[test]
  fn parse_a_sequence() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::sequence(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      whitespace_parser.optional(),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("baz".into()), None),
    ])
    .map(
      |tokens| {
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
  fn parses_a_sequence_separated_by_whitespace() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::sequence(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("bar".into()), None),
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
      parser.parse_to_end("foo bar baz").unwrap(),
      vec![
        Token::Ident("foo".into()),
        Token::Ident("bar".into()),
        Token::Ident("baz".into())
      ]
    );
  }

  #[test]
  fn makes_separators_optional_for_optional_parsers() {
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::sequence(vec![
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
      parser.parse_to_end("foo baz").unwrap(),
      vec![
        Some(Token::Ident("foo".into())),
        None,
        Some(Token::Ident("baz".into()))
      ]
    );
  }

  #[test]
  fn parses_a_sequence_separated_commas_and_optional_whitespace() {
    let string_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident);
    let comma_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Comma);
    let whitespace_parser = TokenParser::<Token<'static>>::get_token_parser(TokenType::Whitespace);

    let parser = TokenParser::sequence(vec![
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("foo".into()), None),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("bar".into()), None),
      string_parser
        .map(|t| t, None)
        .where_fn(|t| *t == Token::Ident("baz".into()), None),
    ])
    .separated_by(comma_parser)
    .separated_by(whitespace_parser.optional())
    .to_parser()
    .map(
      |tokens| tokens.into_iter().flatten().collect::<Vec<_>>(),
      None,
    );

    assert_eq!(
      parser.parse_to_end("foo, bar, baz").unwrap(),
      vec![
        Token::Ident("foo".into()),
        Token::Ident("bar".into()),
        Token::Ident("baz".into()),
      ]
    );
  }
}
