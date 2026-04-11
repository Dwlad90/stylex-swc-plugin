use cssparser::{
  ParseError, Parser, ParserInput, SourcePosition, Token, serialize_identifier, serialize_string,
};
use stylex_macros::stylex_unreachable;

pub fn format_ident(ident: &str) -> String {
  let mut res: String = String::default();
  let _ = serialize_identifier(ident, &mut res);
  res = res.trim_end().to_string();
  res
}

pub fn _format_quoted_string(string: &str) -> String {
  let mut res: String = String::default();
  let _ = serialize_string(string, &mut res);
  res
}

// const CSS_PROPS_WITH_IMAGE_URLS: &[&str] = &[
//   // Universal
//   "background",
//   "background-image",
//   "border-image",
//   "border-image-source",
//   "content",
//   "cursor",
//   "list-style",
//   "list-style-image",
//   "mask",
//   "mask-image",
//   // Specific to @counter-style
//   "additive-symbols",
//   "negative",
//   "pad",
//   "prefix",
//   "suffix",
//   "symbols",
// ];

// pub fn is_image_url_prop(prop_name: &str) -> bool {
//   CSS_PROPS_WITH_IMAGE_URLS
//     .iter()
//     .find(|p| prop_name.eq_ignore_ascii_case(p))
//     .is_some()
// }

pub fn parse_css_inner<'a>(
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
        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| parse_css_inner(parser))
          .unwrap_or_default();
        iter_result.push_str(join_css(&block_css).as_str());
        iter_result.push(')');
      },
      Token::SquareBracketBlock => {
        iter_result.push('[');
        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| parse_css_inner(parser))
          .unwrap_or_default();
        iter_result.push_str(join_css(&block_css).as_str());
        iter_result.push(']');
      },
      Token::CurlyBracketBlock => {
        iter_result.push('{');
        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| parse_css_inner(parser))
          .unwrap_or_default();
        iter_result.push_str(join_css(&block_css).as_str());
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
      // div...
      Token::Ident(ref value) => {
        iter_result.push_str(&format_ident(value));
      },
      // @import, @font-face, @charset, @media...
      Token::AtKeyword(ref value) => {
        // if options.no_fonts && curr_rule == "font-face" {
        //   continue;
        // }
        iter_result.push('@');
        iter_result.push_str(value);
      },
      Token::Hash(ref value) | Token::IDHash(ref value) => {
        iter_result.push('#');
        iter_result.push_str(&format_ident(value));
      },
      Token::QuotedString(ref value) => {
        // Add the quoted string with quotes preserved
        iter_result.push_str(&_format_quoted_string(value));

        // if curr_rule == "import" {
        //   // Reset current at-rule value
        //   curr_rule =  String::default();

        //   // Skip empty import values
        //   if value.len() == 0 {
        //     result.push_str("''");
        //     continue;
        //   }

        //   let import_full_url: Url = resolve_url(&document_url, value);
        //   match retrieve_asset(
        //     cache,
        //     client,
        //     &document_url,
        //     &import_full_url,
        //     options,
        //     depth + 1,
        //   ) {
        //     Ok((import_contents, import_final_url, import_media_type, import_charset)) => {
        //       let mut import_data_url = create_data_url(
        //         &import_media_type,
        //         &import_charset,
        //         embed_css(
        //           cache,
        //           client,
        //           &import_final_url,
        //           &String::from_utf8_lossy(&import_contents),
        //           options,
        //           depth + 1,
        //         )
        //         .as_bytes(),
        //         &import_final_url,
        //       );
        //       import_data_url.set_fragment(import_full_url.fragment());
        //       result.push_str(format_quoted_string(&import_data_url.to_string()).as_str());
        //     }
        //     Err(_) => {
        //       // Keep remote reference if unable to retrieve the asset
        //       if import_full_url.scheme() == "http" || import_full_url.scheme() == "https" {
        //         result.push_str(format_quoted_string(&import_full_url.to_string()).as_str());
        //       }
        //     }
        //   }
        // } else {
        //   if func_name == "url" {
        //     // Skip empty url()'s
        //     if value.len() == 0 {
        //       continue;
        //     }

        //     if options.no_images && is_image_url_prop(curr_prop.as_str()) {
        //       result.push_str(format_quoted_string(EMPTY_IMAGE_DATA_URL).as_str());
        //     } else {
        //       let resolved_url: Url = resolve_url(&document_url, value);
        //       match retrieve_asset(
        //         cache,
        //         client,
        //         &document_url,
        //         &resolved_url,
        //         options,
        //         depth + 1,
        //       ) {
        //         Ok((data, final_url, media_type, charset)) => {
        //           let mut data_url = create_data_url(&media_type, &charset, &data, &final_url);
        //           data_url.set_fragment(resolved_url.fragment());
        //           result.push_str(format_quoted_string(&data_url.to_string()).as_str());
        //         }
        //         Err(_) => {
        //           // Keep remote reference if unable to retrieve the asset
        //           if resolved_url.scheme() == "http" || resolved_url.scheme() == "https" {
        //             result.push_str(format_quoted_string(&resolved_url.to_string()).as_str());
        //           }
        //         }
        //       }
        //     }
        //   } else {
        //     result.push_str(format_quoted_string(value).as_str());
        //   }
        // }
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
      // url() — unquoted URLs are explicitly unsupported.
      Token::UnquotedUrl(_) | Token::BadUrl(_) | Token::BadString(_) => {
        panic!("Unsupported CSS token: unquoted/bad url or bad string. Use quoted values instead.")
      },
      Token::Delim(ref value) => iter_result.push(*value),
      Token::Function(ref name) => {
        iter_result.push_str(name);
        iter_result.push('(');

        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| parse_css_inner(parser))
          .unwrap_or_default();

        iter_result.push_str(join_css(&block_css).as_str());

        iter_result.push(')');
      },
    }

    // Ensure empty CSS is really empty
    if !iter_result.is_empty() && iter_result.trim().is_empty() {
      iter_result = iter_result.trim().to_string()
    }

    if !iter_result.is_empty() {
      result.push(iter_result);
    }
  }

  Ok(result)
}

pub fn parse_css(css_string: &str) -> Vec<String> {
  let mut input = ParserInput::new(css_string);

  let mut parser = Parser::new(&mut input);

  let nodes = parse_css_inner(&mut parser).unwrap_or_else(|_| {
    stylex_unreachable!("parse_css_inner returned Err, which should not happen")
  });

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

fn join_css(nodes: &[String]) -> String {
  let mut result = String::new();
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

#[cfg(test)]
mod tests {
  use super::{join_css, parse_css};

  #[test]
  fn join_css_avoids_space_around_slash_and_comma() {
    let nodes = vec![
      "10px".to_string(),
      "/".to_string(),
      "20px".to_string(),
      ",".to_string(),
      "30px".to_string(),
    ];

    assert_eq!(join_css(&nodes), "10px/20px,30px");
  }

  #[test]
  fn parse_css_bad_string_is_tolerated() {
    let result = parse_css("\"unterminated");
    assert!(!result.is_empty());
  }
}
