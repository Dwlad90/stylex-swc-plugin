use cssparser::{
  ParseError, Parser, ParserInput, SourcePosition, Token, serialize_identifier, serialize_string,
};
use stylex_macros::stylex_unreachable;

pub fn format_ident(ident: &str) -> String {
  let mut result = String::default();
  let _ = serialize_identifier(ident, &mut result);
  result.trim_end().to_string()
}

pub fn format_quoted_string(string: &str) -> String {
  let mut result = String::default();
  let _ = serialize_string(string, &mut result);
  result
}

fn parse_css_inner<'a>(parser: &mut Parser) -> Result<Vec<String>, ParseError<'a, Vec<String>>> {
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
        iter_result.push_str(&parse_nested_joined(parser));
        iter_result.push(')');
      },
      Token::SquareBracketBlock => {
        iter_result.push('[');
        iter_result.push_str(&parse_nested_joined(parser));
        iter_result.push(']');
      },
      Token::CurlyBracketBlock => {
        iter_result.push('{');
        iter_result.push_str(&parse_nested_joined(parser));
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
        iter_result.push_str(&value.to_string())
      },
      Token::Percentage {
        ref has_sign,
        ref unit_value,
        ..
      } => {
        if *has_sign && *unit_value >= 0. {
          iter_result.push('+');
        }
        iter_result.push_str(&(unit_value * 100.0).to_string());
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
        iter_result.push_str(&value.to_string());
        iter_result.push_str(unit.as_ref());
      },
      Token::UnquotedUrl(_) | Token::BadUrl(_) | Token::BadString(_) => {
        panic!("Unsupported CSS token: unquoted/bad url or bad string. Use quoted values instead.")
      },
      Token::Delim(ref value) => iter_result.push(*value),
      Token::Function(ref name) => {
        iter_result.push_str(name);
        iter_result.push('(');
        iter_result.push_str(&parse_nested_joined(parser));
        iter_result.push(')');
      },
    }

    if !iter_result.is_empty() && iter_result.trim().is_empty() {
      iter_result = iter_result.trim().to_string()
    }

    if !iter_result.is_empty() {
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
  let nodes = parse_css_inner(&mut parser).unwrap_or_else(parse_css_inner_unreachable);

  nodes
    .into_iter()
    .filter_map(|s| {
      if !s.is_empty() && s != "," {
        Some(s)
      } else {
        None
      }
    })
    .collect::<Vec<String>>()
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

fn parse_nested_joined(parser: &mut Parser) -> String {
  let block_css: Vec<String> = parser
    .parse_nested_block(|parser| parse_css_inner(parser))
    .unwrap_or_default();
  join_css(&block_css)
}

#[cfg(test)]
#[path = "tests/value_parser_tests.rs"]
mod tests;
