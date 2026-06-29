// Additional coverage tests for token_types.rs.
// Targets branches not yet exercised by the existing suites:
//   - map_css_token() arms for special token variants (lines 80-100)
//   - tokenize_nested_content() nested-function and nested-paren paths (lines 115, 132)
//   - tokenize_nested_content() `_ =>` fallthrough for special tokens (line 147)
//   - tokenize_all() `_ =>` fallthrough for special tokens (line 201)
//   - TokenList::first() alias method (lines 261-263)
//
// Lines 80, 82, 83 are exercised by calling the private map_css_token()
// directly (via super::) with the token variants that are never emitted via
// the public tokenize_all / tokenize_nested_content paths.
//
// Genuinely unreachable arms (noted below but NOT tested since they require
// impossible inputs):
//   - lines 115, 132, 166, 184 (cols 20-21): The `e)` pattern binding in
//               `if let Err(e) = parse_nested_block(...)` — only reachable when
//               parse_nested_block returns Err, which requires the inner closure
//               to return Err. The inner closure always returns Ok(()), so this
//               binding is dead.
//   - lines 119-120, 136-137: Err branch bodies inside tokenize_nested_content
//               parse_nested_block — same reason.
//   - lines 171-172, 190-191: Err branch bodies inside tokenize_all — same reason.
//   - lines 147, 201 (cols 9-10): The closing `}` of
//               `if let Some(mapped) = map_css_token(...)` — represents the None
//               branch; map_css_token always returns Some(_), so it is dead.

use super::*;

// Allow direct access to the private map_css_token function from this child module.
use cssparser::Token as CssToken;

// ---------------------------------------------------------------------------
// map_css_token(): direct calls for token variants that can never reach
// map_css_token through normal tokenize_all / tokenize_nested_content paths.
// Child modules in Rust can access private parent items directly.
// ---------------------------------------------------------------------------

#[test]
fn map_css_token_function_variant_produces_function_token() {
  // CssToken::Function arm (line 80) — map_css_token is reachable from child
  // modules as a private fn. This variant never reaches map_css_token through
  // tokenize_all/tokenize_nested_content (both handle Function explicitly), so
  // we exercise it by calling map_css_token directly.
  use cssparser::CowRcStr;
  let token = CssToken::Function(CowRcStr::from("rgb"));
  let result = super::map_css_token(&token);
  assert_eq!(result, Some(SimpleToken::Function("rgb".to_string())));
}

#[test]
fn map_css_token_delim_open_paren_produces_left_paren() {
  // CssToken::Delim('(') arm (line 82) — cssparser never emits this variant
  // at the token level (it emits ParenthesisBlock instead), so we call
  // map_css_token directly.
  let token = CssToken::Delim('(');
  let result = super::map_css_token(&token);
  assert_eq!(result, Some(SimpleToken::LeftParen));
}

#[test]
fn map_css_token_delim_close_paren_produces_right_paren() {
  // CssToken::Delim(')') arm (line 83) — cssparser emits CloseParenthesis
  // instead; we call map_css_token directly.
  let token = CssToken::Delim(')');
  let result = super::map_css_token(&token);
  assert_eq!(result, Some(SimpleToken::RightParen));
}

// ---------------------------------------------------------------------------
// map_css_token arms via TokenList::new (top-level tokens fall through to the
// `_ =>` arm in tokenize_all, which calls map_css_token).
// ---------------------------------------------------------------------------

#[test]
fn map_css_token_unquoted_url_produces_string_token() {
  // url(foo) => CssToken::UnquotedUrl => SimpleToken::String   (line 90)
  let list = TokenList::new("url(foo)");
  // The tokenizer sees url( and produces UnquotedUrl for bare-word urls
  assert!(!list.tokens.is_empty());
  // Verify at least one token is a String (from UnquotedUrl)
  let has_string = list
    .tokens
    .iter()
    .any(|t| matches!(t, SimpleToken::String(_)));
  assert!(
    has_string,
    "url(foo) should produce a String token from UnquotedUrl"
  );
}

#[test]
fn map_css_token_bad_url_produces_unknown_token() {
  // url(bad url) => CssToken::BadUrl => SimpleToken::Unknown   (line 89)
  let list = TokenList::new("url(bad url)");
  assert!(!list.tokens.is_empty());
  let has_unknown = list
    .tokens
    .iter()
    .any(|t| matches!(t, SimpleToken::Unknown(_)));
  assert!(
    has_unknown,
    "url(bad url) should produce an Unknown token from BadUrl"
  );
}

#[test]
fn map_css_token_bad_string_produces_unknown_token() {
  // "bad\nstring" => CssToken::BadString => SimpleToken::Unknown   (line 89)
  let list = TokenList::new("\"bad\nstring\"");
  assert!(!list.tokens.is_empty());
  let has_unknown = list
    .tokens
    .iter()
    .any(|t| matches!(t, SimpleToken::Unknown(_)));
  assert!(
    has_unknown,
    "newline-inside-string should produce an Unknown token from BadString"
  );
}

#[test]
fn map_css_token_close_parenthesis_produces_right_paren() {
  // Unmatched ) at top level => CssToken::CloseParenthesis => SimpleToken::RightParen   (line 91)
  let list = TokenList::new(")");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::RightParen,
    "unmatched ) should produce RightParen"
  );
}

#[test]
fn map_css_token_square_bracket_block_produces_delim_bracket() {
  // [a] => CssToken::SquareBracketBlock => SimpleToken::Delim('[')   (line 92)
  let list = TokenList::new("[a]");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim('['),
    "[ block should produce Delim('[') token"
  );
}

#[test]
fn map_css_token_close_square_bracket_produces_delim_close() {
  // Unmatched ] at top level => CssToken::CloseSquareBracket => SimpleToken::Delim(']')  (line 93)
  let list = TokenList::new("]");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim(']'),
    "unmatched ] should produce Delim(']') token"
  );
}

#[test]
fn map_css_token_curly_bracket_block_produces_delim_brace() {
  // {a} => CssToken::CurlyBracketBlock => SimpleToken::Delim('{')   (line 94)
  let list = TokenList::new("{a}");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim('{'),
    "{{ block should produce Delim('{{') token"
  );
}

#[test]
fn map_css_token_close_curly_bracket_produces_delim_close_brace() {
  // Unmatched } at top level => CssToken::CloseCurlyBracket => SimpleToken::Delim('}')  (line 95)
  let list = TokenList::new("}");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim('}'),
    "unmatched }} should produce Delim('}}') token"
  );
}

#[test]
fn map_css_token_cdc_produces_delim_gt() {
  // --> (CSS comment-close) => CssToken::CDC => SimpleToken::Delim('>')   (line 96)
  let list = TokenList::new("-->");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim('>'),
    "--> should produce Delim('>') token"
  );
}

#[test]
fn map_css_token_cdo_produces_delim_lt() {
  // <!-- (HTML comment-open) => CssToken::CDO => SimpleToken::Delim('<')   (line 97)
  let list = TokenList::new("<!--");
  assert!(!list.tokens.is_empty());
  assert_eq!(
    list.tokens[0],
    SimpleToken::Delim('<'),
    "<!-- should produce Delim('<') token"
  );
}

#[test]
fn map_css_token_wildcard_produces_unknown_for_comment() {
  // /* comment */ => CssToken::Comment (no explicit arm) => SimpleToken::Unknown   (line 100)
  let list = TokenList::new("/* hello */");
  assert!(!list.tokens.is_empty());
  let has_unknown = list
    .tokens
    .iter()
    .any(|t| matches!(t, SimpleToken::Unknown(_)));
  assert!(
    has_unknown,
    "CSS comment should produce an Unknown token via the wildcard arm"
  );
}

#[test]
fn map_css_token_wildcard_produces_unknown_for_include_match() {
  // ~= => CssToken::IncludeMatch (no explicit arm) => SimpleToken::Unknown   (line 100)
  let list = TokenList::new("~=");
  assert!(!list.tokens.is_empty());
  let has_unknown = list
    .tokens
    .iter()
    .any(|t| matches!(t, SimpleToken::Unknown(_)));
  assert!(
    has_unknown,
    "~= should produce an Unknown token via the wildcard arm"
  );
}

// ---------------------------------------------------------------------------
// tokenize_nested_content: nested ParenthesisBlock inside a function (line 115)
// ---------------------------------------------------------------------------

#[test]
fn nested_paren_inside_function_is_tokenized() {
  // rgb((1), 0, 0) — the outer Function token triggers tokenize_nested_content,
  // then the inner ( triggers the ParenthesisBlock arm (line 115).
  let list = TokenList::new("rgb((1), 0, 0)");
  assert!(!list.tokens.is_empty());
  // Expect tokens: Function("rgb"), LeftParen, Number(1.0), RightParen, ...
  let first = &list.tokens[0];
  assert_eq!(*first, SimpleToken::Function("rgb".to_string()));
  // The nested ( inside the function body must also appear
  let has_left_paren = list.tokens.contains(&SimpleToken::LeftParen);
  assert!(
    has_left_paren,
    "nested ( inside function should produce LeftParen token"
  );
}

// ---------------------------------------------------------------------------
// tokenize_nested_content: nested Function inside a function (line 132)
// ---------------------------------------------------------------------------

#[test]
fn nested_function_inside_function_is_tokenized() {
  // rgb(calc(1px + 2em), 0, 0) — the outer Function triggers tokenize_nested_content,
  // then the inner calc( triggers the Function arm (line 132).
  let list = TokenList::new("rgb(calc(1px + 2em), 0, 0)");
  assert!(!list.tokens.is_empty());
  assert_eq!(list.tokens[0], SimpleToken::Function("rgb".to_string()));
  // calc should appear as a nested Function token
  let has_calc = list
    .tokens
    .iter()
    .any(|t| *t == SimpleToken::Function("calc".to_string()));
  assert!(
    has_calc,
    "nested calc() inside rgb() should produce a Function token"
  );
}

// ---------------------------------------------------------------------------
// tokenize_nested_content: `_ =>` arm called with misc tokens (line 147)
// This is already exercised by rgb(1, 2, 3) but we add an explicit assertion
// for the Whitespace and Comma tokens that appear in the nested block.
// ---------------------------------------------------------------------------

#[test]
fn nested_content_misc_tokens_mapped_via_map_css_token() {
  // Inside rgb(255 0 0), the whitespace tokens fall to the `_ =>` arm which
  // calls map_css_token — exercising line 147.
  let list = TokenList::new("rgb(255 0 0)");
  let has_whitespace = list.tokens.contains(&SimpleToken::Whitespace);
  assert!(
    has_whitespace,
    "whitespace inside rgb() should be included as Whitespace tokens"
  );
}

// ---------------------------------------------------------------------------
// TokenList::first() — alias for peek() (lines 261-263)
// ---------------------------------------------------------------------------

#[test]
fn first_returns_same_as_peek_for_non_empty_list() {
  let mut list = TokenList::new("hello");
  let via_peek = list.peek().unwrap();
  let via_first = list.first().unwrap();
  assert_eq!(
    via_peek, via_first,
    "first() should return the same token as peek()"
  );
}

#[test]
fn first_returns_none_for_empty_list() {
  let mut list = TokenList::new("");
  let result = list.first().unwrap();
  assert!(
    result.is_none(),
    "first() on empty TokenList should return None"
  );
}

#[test]
fn first_does_not_consume_token() {
  let mut list = TokenList::new("color");
  let index_before = list.current_index;
  let _ = list.first().unwrap();
  assert_eq!(
    list.current_index, index_before,
    "first() should not advance the current_index"
  );
}
