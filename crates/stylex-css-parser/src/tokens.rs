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
    static TOKEN_PARSERS: RefCell<HashMap<TokenType, TokenParser<'static, Token<'static>>>> = {
        let mut m = HashMap::new();

        // Add parsers for each token type
        m.insert(TokenType::Comment, TokenParser::<'static, Token<'static>>::token("Comment"));
        m.insert(TokenType::AtKeyword, TokenParser::<'static, Token<'static>>::token("AtKeyword"));
        m.insert(TokenType::BadString, TokenParser::<'static, Token<'static>>::token("BadString"));
        m.insert(TokenType::BadURL, TokenParser::<'static, Token<'static>>::token("BadURL"));
        m.insert(TokenType::CDC, TokenParser::<'static, Token<'static>>::token("CDC"));
        m.insert(TokenType::CDO, TokenParser::<'static, Token<'static>>::token("CDO"));
        m.insert(TokenType::Colon, TokenParser::<'static, Token<'static>>::token("Colon"));
        m.insert(TokenType::Comma, TokenParser::<'static, Token<'static>>::token("Comma"));
        m.insert(TokenType::Delim, TokenParser::<'static, Token<'static>>::token("Delim"));
        m.insert(TokenType::Dimension, TokenParser::<'static, Token<'static>>::token("Dimension"));
        m.insert(TokenType::EOF, TokenParser::<'static, Token<'static>>::token("EOF"));
        m.insert(TokenType::Function, TokenParser::<'static, Token<'static>>::token("Function"));
        m.insert(TokenType::Hash, TokenParser::<'static, Token<'static>>::token("Hash"));
        m.insert(TokenType::Ident, TokenParser::<'static, Token<'static>>::token("Ident"));
        m.insert(TokenType::Number, TokenParser::<'static, Token<'static>>::token("Number"));
        m.insert(TokenType::Percentage, TokenParser::<'static, Token<'static>>::token("Percentage"));
        m.insert(TokenType::Semicolon, TokenParser::<'static, Token<'static>>::token("Semicolon"));
        m.insert(TokenType::String, TokenParser::<'static, Token<'static>>::token("String"));
        m.insert(TokenType::URL, TokenParser::<'static, Token<'static>>::token("URL"));
        m.insert(TokenType::Whitespace, TokenParser::<'static, Token<'static>>::token("Whitespace"));
        m.insert(TokenType::OpenParen, TokenParser::<'static, Token<'static>>::token("OpenParen"));
        m.insert(TokenType::CloseParen, TokenParser::<'static, Token<'static>>::token("CloseParen"));
        m.insert(TokenType::OpenSquare, TokenParser::<'static, Token<'static>>::token("OpenSquare"));
        m.insert(TokenType::CloseSquare, TokenParser::<'static, Token<'static>>::token("CloseSquare"));
        m.insert(TokenType::OpenCurly, TokenParser::<'static, Token<'static>>::token("OpenCurly"));
        m.insert(TokenType::CloseCurly, TokenParser::<'static, Token<'static>>::token("CloseCurly"));
        m.insert(TokenType::UnicodeRange, TokenParser::<'static, Token<'static>>::token("UnicodeRange"));

        RefCell::new(m)
    };
}

// Add tokens accessor to TokenParser
impl<T: 'static> TokenParser<'_, T> {
  // Add this to your existing TokenParser implementation
  pub fn get_token_parser(token_type: TokenType) -> TokenParser<'static, Token<'static>> {
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
pub fn comment() -> TokenParser<'static, Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Comment)
}

pub fn at_keyword() -> TokenParser<'static, Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::AtKeyword)
}

pub fn ident() -> TokenParser<'static, Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Ident)
}

// Add getters for other token types...
pub fn delim() -> TokenParser<'static, Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Delim)
}

pub fn number() -> TokenParser<'static, Token<'static>> {
  TokenParser::<Token<'static>>::get_token_parser(TokenType::Number)
}
