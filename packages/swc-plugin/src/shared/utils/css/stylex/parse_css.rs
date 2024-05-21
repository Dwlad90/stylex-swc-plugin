use cssparser::{
  serialize_identifier, serialize_string, ParseError, Parser, ParserInput, SourcePosition, Token,
};

pub fn format_ident(ident: &str) -> String {
  let mut res: String = "".to_string();
  let _ = serialize_identifier(ident, &mut res);
  res = res.trim_end().to_string();
  res
}

pub fn _format_quoted_string(string: &str) -> String {
  let mut res: String = "".to_string();
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
  rule_name: &str,
  prop_name: &str,
) -> Result<Vec<String>, ParseError<'a, Vec<String>>> {
  let mut result: Vec<String> = vec![];

  let mut curr_rule: String = rule_name.to_string();
  let mut curr_prop: String = prop_name.to_string();
  let mut token: &Token;
  let mut token_offset: SourcePosition;

  loop {
    let mut iter_result: String = "".to_string();

    token_offset = parser.position();
    token = match parser.next_including_whitespace_and_comments() {
      Ok(token) => token,
      Err(_) => {
        break;
      }
    };

    match *token {
      Token::Comment(_) => {
        let token_slice = parser.slice_from(token_offset);
        iter_result.push_str(token_slice);
      }
      Token::Semicolon => iter_result.push(';'),
      Token::Colon => iter_result.push(':'),
      Token::Comma => iter_result.push(','),
      Token::ParenthesisBlock | Token::SquareBracketBlock | Token::CurlyBracketBlock => {
        // if options.no_fonts && curr_rule == "font-face" {
        //   continue;
        // }

        let closure: &str;
        if token == &Token::ParenthesisBlock {
          iter_result.push('(');
          closure = ")";
        } else if token == &Token::SquareBracketBlock {
          iter_result.push('[');
          closure = "]";
        } else {
          iter_result.push('{');
          closure = "}";
        }

        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| {
            parse_css_inner(
              // cache,
              // client,
              // document_url,
              parser,
              // options,
              // depth,
              rule_name,
              curr_prop.as_str(),
              // func_name,
            )
          })
          .unwrap();

        iter_result.push_str(join_css(&block_css).as_str());

        iter_result.push_str(closure);
      }
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
      }
      // div...
      Token::Ident(ref value) => {
        curr_rule = "".to_string();
        curr_prop = value.to_string();
        iter_result.push_str(&format_ident(value));
      }
      // @import, @font-face, @charset, @media...
      Token::AtKeyword(ref value) => {
        curr_rule = value.to_string();
        // if options.no_fonts && curr_rule == "font-face" {
        //   continue;
        // }
        iter_result.push('@');
        iter_result.push_str(value);
      }
      Token::Hash(ref value) => {
        iter_result.push('#');
        iter_result.push_str(value);
      }
      Token::QuotedString(ref _value) => {
        todo!("Handle QuotedString");
        // if curr_rule == "import" {
        //   // Reset current at-rule value
        //   curr_rule = "".to_string();

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
      }
      Token::Number {
        ref has_sign,
        ref value,
        ..
      } => {
        if *has_sign && *value >= 0. {
          iter_result.push('+');
        }
        iter_result.push_str(&value.to_string())
      }
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
      }
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
      }
      // #selector, #id...
      Token::IDHash(ref value) => {
        curr_rule = "".to_string();
        iter_result.push('#');
        iter_result.push_str(&format_ident(value));
      }
      // url()
      Token::UnquotedUrl(ref _value) => {
        todo!("Handle UnquotedUrl");
        //   let is_import: bool = curr_rule == "import";

        //   if is_import {
        //     // Reset current at-rule value
        //     curr_rule = "".to_string();
        //   }

        //   // Skip empty url()'s
        //   if value.len() < 1 {
        //     result.push_str("url()");
        //     continue;
        //   } else if value.starts_with("#") {
        //     result.push_str("url(");
        //     result.push_str(value);
        //     result.push_str(")");
        //     continue;
        //   }

        //   result.push_str("url(");
        //   if is_import {
        //     let full_url: Url = resolve_url(&document_url, value);
        //     match retrieve_asset(cache, client, &document_url, &full_url, options, depth + 1) {
        //       Ok((css, final_url, media_type, charset)) => {
        //         let mut data_url = create_data_url(
        //           &media_type,
        //           &charset,
        //           embed_css(
        //             cache,
        //             client,
        //             &final_url,
        //             &String::from_utf8_lossy(&css),
        //             options,
        //             depth + 1,
        //           )
        //           .as_bytes(),
        //           &final_url,
        //         );
        //         data_url.set_fragment(full_url.fragment());
        //         result.push_str(format_quoted_string(&data_url.to_string()).as_str());
        //       }
        //       Err(_) => {
        //         // Keep remote reference if unable to retrieve the asset
        //         if full_url.scheme() == "http" || full_url.scheme() == "https" {
        //           result.push_str(format_quoted_string(&full_url.to_string()).as_str());
        //         }
        //       }
        //     }
        //   } else {
        //     if is_image_url_prop(curr_prop.as_str()) && options.no_images {
        //       result.push_str(format_quoted_string(EMPTY_IMAGE_DATA_URL).as_str());
        //     } else {
        //       let full_url: Url = resolve_url(&document_url, value);
        //       match retrieve_asset(cache, client, &document_url, &full_url, options, depth + 1) {
        //         Ok((data, final_url, media_type, charset)) => {
        //           let mut data_url = create_data_url(&media_type, &charset, &data, &final_url);
        //           data_url.set_fragment(full_url.fragment());
        //           result.push_str(format_quoted_string(&data_url.to_string()).as_str());
        //         }
        //         Err(_) => {
        //           // Keep remote reference if unable to retrieve the asset
        //           if full_url.scheme() == "http" || full_url.scheme() == "https" {
        //             result.push_str(format_quoted_string(&full_url.to_string()).as_str());
        //           }
        //         }
        //       }
        //     }
        //   }
        //   result.push_str(")");
      }
      Token::Delim(ref value) => iter_result.push_str(&value.to_string()),
      Token::Function(ref name) => {
        let function_name: &str = &name.clone();
        iter_result.push_str(function_name);
        iter_result.push('(');

        let block_css: Vec<String> = parser
          .parse_nested_block(|parser| {
            parse_css_inner(parser, curr_rule.as_str(), curr_prop.as_str())
          })
          .unwrap();

        iter_result.push_str(join_css(&block_css).as_str());

        iter_result.push(')');
      }
      _ => {}
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
  let rule_name = "";
  let prop_name = "";

  match parse_css_inner(&mut parser, rule_name, prop_name) {
    Ok(nodes) => nodes
      .into_iter()
      .filter_map(|s| {
        if !s.is_empty() && s != "," {
          Option::Some(s)
        } else {
          Option::None
        }
      })
      .collect::<Vec<String>>(),
    Err(_) => todo!(),
  }
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
