use core::fmt;
use cssparser::Token;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::token_parser::TokenParser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
  Comment,
  AtKeyword,
  BadString,
  BadURL,
  CDC,
  CDO,
  Colon,
  Comma,
  Delim,
  Dimension,
  EOF,
  Function,
  Hash,
  Ident,
  Number,
  Percentage,
  Semicolon,
  String,
  URL,
  Whitespace,
  OpenParen,
  CloseParen,
  OpenSquare,
  CloseSquare,
  OpenCurly,
  CloseCurly,
  UnicodeRange,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = match self {
      TokenType::Comment => "Comment",
      TokenType::AtKeyword => "AtKeyword",
      TokenType::BadString => "BadString",
      TokenType::BadURL => "BadURL",
      TokenType::CDC => "CDC",
      TokenType::CDO => "CDO",
      TokenType::Colon => "Colon",
      TokenType::Comma => "Comma",
      TokenType::Delim => "Delim",
      TokenType::Dimension => "Dimension",
      TokenType::EOF => "EOF",
      TokenType::Function => "Function",
      TokenType::Hash => "Hash",
      TokenType::Ident => "Ident",
      TokenType::Number => "Number",
      TokenType::Percentage => "Percentage",
      TokenType::Semicolon => "Semicolon",
      TokenType::String => "String",
      TokenType::URL => "URL",
      TokenType::Whitespace => "Whitespace",
      TokenType::OpenParen => "OpenParen",
      TokenType::CloseParen => "CloseParen",
      TokenType::OpenSquare => "OpenSquare",
      TokenType::CloseSquare => "CloseSquare",
      TokenType::OpenCurly => "OpenCurly",
      TokenType::CloseCurly => "CloseCurly",
      TokenType::UnicodeRange => "UnicodeRange",
    };
    write!(f, "{}", name)
  }
}
thread_local! {
    pub static TOKEN_PARSERS: RefCell<HashMap<TokenType, TokenParser<'static, Token<'static>>>> = {
        let mut m = HashMap::new();

        m.insert(TokenType::Comment, TokenParser::<'static, Token<'static>>::token(&TokenType::Comment, None));
        m.insert(TokenType::AtKeyword, TokenParser::<'static, Token<'static>>::token(&TokenType::AtKeyword, None));
        m.insert(TokenType::BadString, TokenParser::<'static, Token<'static>>::token(&TokenType::BadString, None));
        m.insert(TokenType::BadURL, TokenParser::<'static, Token<'static>>::token(&TokenType::BadURL, None));
        m.insert(TokenType::CDC, TokenParser::<'static, Token<'static>>::token(&TokenType::CDC, None));
        m.insert(TokenType::CDO, TokenParser::<'static, Token<'static>>::token(&TokenType::CDO, None));
        m.insert(TokenType::Colon, TokenParser::<'static, Token<'static>>::token(&TokenType::Colon, None));
        m.insert(TokenType::Comma, TokenParser::<'static, Token<'static>>::token(&TokenType::Comma, None));
        m.insert(TokenType::Delim, TokenParser::<'static, Token<'static>>::token(&TokenType::Delim, None));
        m.insert(TokenType::Dimension, TokenParser::<'static, Token<'static>>::token(&TokenType::Dimension, None));
        m.insert(TokenType::EOF, TokenParser::<'static, Token<'static>>::token(&TokenType::EOF, None));
        m.insert(TokenType::Function, TokenParser::<'static, Token<'static>>::token(&TokenType::Function, None));
        m.insert(TokenType::Hash, TokenParser::<'static, Token<'static>>::token(&TokenType::Hash, None));
        m.insert(TokenType::Ident, TokenParser::<'static, Token<'static>>::token(&TokenType::Ident, None));
        m.insert(TokenType::Number, TokenParser::<'static, Token<'static>>::token(&TokenType::Number, None));
        m.insert(TokenType::Percentage, TokenParser::<'static, Token<'static>>::token(&TokenType::Percentage, None));
        m.insert(TokenType::Semicolon, TokenParser::<'static, Token<'static>>::token(&TokenType::Semicolon, None));
        m.insert(TokenType::String, TokenParser::<'static, Token<'static>>::token(&TokenType::String, None));
        m.insert(TokenType::URL, TokenParser::<'static, Token<'static>>::token(&TokenType::URL, None));
        m.insert(TokenType::Whitespace, TokenParser::<'static, Token<'static>>::token(&TokenType::Whitespace, None));
        m.insert(TokenType::OpenParen, TokenParser::<'static, Token<'static>>::token(&TokenType::OpenParen, None));
        m.insert(TokenType::CloseParen, TokenParser::<'static, Token<'static>>::token(&TokenType::CloseParen, None));
        m.insert(TokenType::OpenSquare, TokenParser::<'static, Token<'static>>::token(&TokenType::OpenSquare, None));
        m.insert(TokenType::CloseSquare, TokenParser::<'static, Token<'static>>::token(&TokenType::CloseSquare, None));
        m.insert(TokenType::OpenCurly, TokenParser::<'static, Token<'static>>::token(&TokenType::OpenCurly, None));
        m.insert(TokenType::CloseCurly, TokenParser::<'static, Token<'static>>::token(&TokenType::CloseCurly, None));
        m.insert(TokenType::UnicodeRange, TokenParser::<'static, Token<'static>>::token(&TokenType::UnicodeRange, None));

        RefCell::new(m)
    };
}
