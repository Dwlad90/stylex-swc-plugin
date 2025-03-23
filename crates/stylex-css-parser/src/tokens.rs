use cssparser::Token;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::token_parser::TokenParser;

// Add enum for token types
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
// Use thread local storage instead of lazy_static
thread_local! {
    static TOKEN_PARSERS: RefCell<HashMap<TokenType, TokenParser<Token<'static>>>> = {
        let mut m = HashMap::new();

        // Add parsers for each token type
        m.insert(TokenType::Comment, TokenParser::<Token<'static>>::token("Comment"));
        m.insert(TokenType::AtKeyword, TokenParser::<Token<'static>>::token("AtKeyword"));
        m.insert(TokenType::BadString, TokenParser::<Token<'static>>::token("BadString"));
        m.insert(TokenType::BadURL, TokenParser::<Token<'static>>::token("BadURL"));
        m.insert(TokenType::CDC, TokenParser::<Token<'static>>::token("CDC"));
        m.insert(TokenType::CDO, TokenParser::<Token<'static>>::token("CDO"));
        m.insert(TokenType::Colon, TokenParser::<Token<'static>>::token("Colon"));
        m.insert(TokenType::Comma, TokenParser::<Token<'static>>::token("Comma"));
        m.insert(TokenType::Delim, TokenParser::<Token<'static>>::token("Delim"));
        m.insert(TokenType::Dimension, TokenParser::<Token<'static>>::token("Dimension"));
        m.insert(TokenType::EOF, TokenParser::<Token<'static>>::token("EOF"));
        m.insert(TokenType::Function, TokenParser::<Token<'static>>::token("Function"));
        m.insert(TokenType::Hash, TokenParser::<Token<'static>>::token("Hash"));
        m.insert(TokenType::Ident, TokenParser::<Token<'static>>::token("Ident"));
        m.insert(TokenType::Number, TokenParser::<Token<'static>>::token("Number"));
        m.insert(TokenType::Percentage, TokenParser::<Token<'static>>::token("Percentage"));
        m.insert(TokenType::Semicolon, TokenParser::<Token<'static>>::token("Semicolon"));
        m.insert(TokenType::String, TokenParser::<Token<'static>>::token("String"));
        m.insert(TokenType::URL, TokenParser::<Token<'static>>::token("URL"));
        m.insert(TokenType::Whitespace, TokenParser::<Token<'static>>::token("Whitespace"));
        m.insert(TokenType::OpenParen, TokenParser::<Token<'static>>::token("OpenParen"));
        m.insert(TokenType::CloseParen, TokenParser::<Token<'static>>::token("CloseParen"));
        m.insert(TokenType::OpenSquare, TokenParser::<Token<'static>>::token("OpenSquare"));
        m.insert(TokenType::CloseSquare, TokenParser::<Token<'static>>::token("CloseSquare"));
        m.insert(TokenType::OpenCurly, TokenParser::<Token<'static>>::token("OpenCurly"));
        m.insert(TokenType::CloseCurly, TokenParser::<Token<'static>>::token("CloseCurly"));
        m.insert(TokenType::UnicodeRange, TokenParser::<Token<'static>>::token("UnicodeRange"));

        RefCell::new(m)
    };
}

// Add tokens accessor to TokenParser
impl<T: 'static> TokenParser<T> {
  // Add this to your existing TokenParser implementation
  pub fn get_token_parser(token_type: TokenType) -> TokenParser<Token<'static>> {
    TOKEN_PARSERS.with(|parsers| {
      (*parsers
        .borrow()
        .get(&token_type)
        .expect("Token parser not found"))
      .clone()
    })
  }
}

// Add a tokens namespace

// Create getter functions for each token type
pub fn comment() -> TokenParser<Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Comment)
}

pub fn at_keyword() -> TokenParser<Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::AtKeyword)
}

pub fn ident() -> TokenParser<Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident)
}

// Add getters for other token types...
pub fn delim() -> TokenParser<Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Delim)
}

pub fn number() -> TokenParser<Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Number)
}
