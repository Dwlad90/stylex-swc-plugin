extern crate cssparser;

use cssparser::{ParseError, Parser, ParserInput, Token};

/// Recursively parses tokens from the given parser.
fn parse_tokens_from_parser<'i>(
  parser: &mut Parser<'i, '_>,
) -> Result<Vec<String>, ParseError<'i, ()>> {
  let mut tokens = Vec::new();

  while !parser.is_exhausted() {
      match parser.next() {
          Ok(token) => {
              match token {
                  Token::Ident(ident) => tokens.push(format!("Identifier: {}", ident)),
                  Token::Colon => tokens.push("Colon".to_string()),
                  Token::Semicolon => tokens.push("Semicolon".to_string()),
                  // When a curly bracket block is encountered, recursively parse its inner tokens
                  Token::CurlyBracketBlock => {
                      tokens.push("CurlyBracketBlock Start".to_string());
                      let nested = parser
                          .parse_nested_block(|inner_parser| parse_tokens_from_parser(inner_parser))?;
                      tokens.extend(nested);
                      tokens.push("CurlyBracketBlock End".to_string());
                  }
                  other => tokens.push(format!("Token: {:?}", other)),
              }
          }
          Err(e) => return Err(e.into()),
      }
  }

  Ok(tokens)
}

/// Parses the provided CSS string and returns a vector of token descriptions.
pub fn parse_css(input: &str) -> Result<Vec<String>, ParseError<'_, ()>> {
  let mut input_buffer = ParserInput::new(input);
  let mut parser = Parser::new(&mut input_buffer);
  parse_tokens_from_parser(&mut parser)
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_css() {
    let css = "div { color: red; }";
    let tokens = parse_css(css).unwrap();
    dbg!(&tokens);
    println!("Parsed tokens: {:?}", tokens);
    assert_eq!(tokens.len(), 7);
  }

  #[test]
  fn test_multiple_rules() {
    let css = "div { color: red; } span { font-size: 12px; }";
    let tokens = parse_css(css).unwrap();
    dbg!(&tokens);
    println!("Parsed tokens: {:?}", tokens);
    assert_eq!(tokens.len(), 7);
  }

  #[test]
  fn test_nested_rules() {
    let css = "@media (max-width: 600px) { div { color: blue; } }";
    let tokens = parse_css(css).unwrap();
    println!("Parsed tokens: {:?}", tokens);
    assert!(!tokens.is_empty());
  }

  #[test]
  fn test_empty_css() {
    let css = "";
    let tokens = parse_css(css).unwrap();
    assert!(tokens.is_empty());
  }
}
