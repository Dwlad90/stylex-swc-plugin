use std::fmt::Write;

use cssparser::{
  ParseError, Parser, ParserInput, SourcePosition, Token, serialize_identifier, serialize_string,
};
use stylex_macros::stylex_unreachable;

use crate::token_types::leading_f64;

pub fn format_ident(ident: &str) -> String {
  let mut result = String::with_capacity(ident.len());
  // `serialize_identifier` only fails if the underlying writer fails;
  // writing to a `String` is infallible.
  let _ = serialize_identifier(ident, &mut result);
  result.truncate(result.trim_end().len());
  result
}

pub fn format_quoted_string(string: &str) -> String {
  let mut result = String::with_capacity(string.len() + 2);
  // Same rationale as `format_ident`: infallible `String` writer.
  let _ = serialize_string(string, &mut result);
  result
}

/// The number a numeric token was written with, re-read from the source.
///
/// This function echoes an authored value rather than computing one, so the
/// number it prints has to be the number the author typed. `cssparser` hands
/// it over as an `f32`, which is not wide enough to say so: it rounds
/// `1.2345678901234567px` to `1.2345679px` and saturates
/// `1.7976931348623157e308px` to infinity, printing `infpx` into a stylesheet.
///
/// `fallback` is `cssparser`'s own value, widened, and is used only when the
/// slice holds no number to read -- which the token type says cannot happen.
fn authored_number(input: &str, token_offset: SourcePosition, fallback: f32) -> f64 {
  leading_f64(&input[token_offset.byte_index()..]).unwrap_or(fallback as f64)
}

fn parse_css_inner<'a>(
  input: &str,
  parser: &mut Parser,
) -> Result<Vec<String>, ParseError<'a, Vec<String>>> {
  let mut result: Vec<String> = vec![];

  while let Some((token_offset, token)) = {
    let token_offset: SourcePosition = parser.position();
    parser
      .next_including_whitespace_and_comments()
      .ok()
      .map(|token| (token_offset, token))
  } {
    let mut iter_result: String = String::default();

    match *token {
      Token::Comment(_) => {
        let token_slice = parser.slice_from(token_offset);
        iter_result.push_str(token_slice);
      },
      Token::Semicolon => iter_result.push(';'),
      Token::Colon => iter_result.push(':'),
      Token::Comma => iter_result.push(','),
      Token::ParenthesisBlock => {
        iter_result.push('(');
        iter_result.push_str(&parse_nested_joined(input, parser));
        iter_result.push(')');
      },
      Token::SquareBracketBlock => {
        iter_result.push('[');
        iter_result.push_str(&parse_nested_joined(input, parser));
        iter_result.push(']');
      },
      Token::CurlyBracketBlock => {
        iter_result.push('{');
        iter_result.push_str(&parse_nested_joined(input, parser));
        iter_result.push('}');
      },
      Token::CloseParenthesis => iter_result.push(')'),
      Token::CloseSquareBracket => iter_result.push(']'),
      Token::CloseCurlyBracket => iter_result.push('}'),
      Token::IncludeMatch => iter_result.push_str("~="),
      Token::DashMatch => iter_result.push_str("|="),
      Token::PrefixMatch => iter_result.push_str("^="),
      Token::SuffixMatch => iter_result.push_str("$="),
      Token::SubstringMatch => iter_result.push_str("*="),
      Token::CDO => iter_result.push_str("<!--"),
      Token::CDC => iter_result.push_str("-->"),
      Token::WhiteSpace(value) => {
        iter_result.push_str(value);
      },
      Token::Ident(ref value) => {
        iter_result.push_str(&format_ident(value));
      },
      Token::AtKeyword(ref value) => {
        iter_result.push('@');
        iter_result.push_str(value);
      },
      Token::Hash(ref value) | Token::IDHash(ref value) => {
        iter_result.push('#');
        iter_result.push_str(&format_ident(value));
      },
      Token::QuotedString(ref value) => {
        iter_result.push_str(&format_quoted_string(value));
      },
      Token::Number {
        ref has_sign,
        ref value,
        ..
      } => {
        if *has_sign && *value >= 0. {
          iter_result.push('+');
        }
        let _ = write!(
          iter_result,
          "{}",
          authored_number(input, token_offset, *value)
        );
      },
      Token::Percentage {
        ref has_sign,
        ref unit_value,
        ..
      } => {
        if *has_sign && *unit_value >= 0. {
          iter_result.push('+');
        }
        // The authored percent, not the fraction scaled back up.
        let percent = authored_number(input, token_offset, unit_value * 100.0);
        let _ = write!(iter_result, "{percent}");
        iter_result.push('%');
      },
      Token::Dimension {
        ref has_sign,
        ref value,
        ref unit,
        ..
      } => {
        if *has_sign && *value >= 0. {
          iter_result.push('+');
        }
        let _ = write!(
          iter_result,
          "{}",
          authored_number(input, token_offset, *value)
        );
        iter_result.push_str(unit.as_ref());
      },
      Token::UnquotedUrl(_) | Token::BadUrl(_) | Token::BadString(_) => {
        panic!("Unsupported CSS token: unquoted/bad url or bad string. Use quoted values instead.")
      },
      Token::Delim(ref value) => iter_result.push(*value),
      Token::Function(ref name) => {
        iter_result.push_str(name);
        iter_result.push('(');
        iter_result.push_str(&parse_nested_joined(input, parser));
        iter_result.push(')');
      },
    }

    // Drop tokens that consist purely of whitespace; preserve everything else
    // verbatim. This collapses stray whitespace tokens emitted by the parser
    // without disturbing meaningful content.
    if !iter_result.trim().is_empty() {
      result.push(iter_result);
    }
  }

  Ok(result)
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn parse_css_inner_unreachable(_err: ParseError<'_, Vec<String>>) -> Vec<String> {
  stylex_unreachable!("parse_css_inner returned Err, which should not happen")
}

pub fn parse_css(css_string: &str) -> Vec<String> {
  let mut input = ParserInput::new(css_string);
  let mut parser = Parser::new(&mut input);
  let nodes = parse_css_inner(css_string, &mut parser).unwrap_or_else(parse_css_inner_unreachable);

  nodes
    .into_iter()
    .filter(|s| !s.is_empty() && s != ",")
    .collect()
}

pub fn join_css(nodes: &[String]) -> String {
  let capacity = nodes.iter().map(String::len).sum::<usize>() + nodes.len().saturating_sub(1);
  let mut result = String::with_capacity(capacity);
  let mut needs_space = false;

  for node in nodes.iter() {
    if node == "/" || node == "," {
      needs_space = false;
    } else {
      if needs_space {
        result.push(' ');
      }
      needs_space = true;
    }
    result.push_str(node);
  }

  result
}

fn parse_nested_joined(input: &str, parser: &mut Parser) -> String {
  let block_css: Vec<String> = parser
    .parse_nested_block(|parser| parse_css_inner(input, parser))
    .unwrap_or_default();
  join_css(&block_css)
}

#[cfg(test)]
#[path = "tests/value_parser_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/value_parser_coverage_test.rs"]
mod value_parser_coverage_test;
