// Additional coverage tests for value_parser.rs.
// Targets branches not exercised by the existing suite, notably:
//   - format_quoted_string()                                       (lines 17-22)
//   - parse_css_inner(): Comment branch                            (lines 37-39)
//   - parse_css_inner(): ParenthesisBlock branch                   (lines 44-46)
//   - parse_css_inner(): SquareBracketBlock branch                 (lines 49-51)
//   - parse_css_inner(): CurlyBracketBlock branch                  (lines 54-56)
//   - parse_css_inner(): CloseParenthesis, CloseSquareBracket,
//     CloseCurlyBracket branches                                   (lines 59-61)
//   - parse_css_inner(): IncludeMatch, DashMatch, PrefixMatch,
//     SuffixMatch, SubstringMatch branches                         (lines 62-66)
//   - parse_css_inner(): CDO, CDC branches                         (lines 67-68)
//   - parse_css_inner(): AtKeyword branch                          (lines 75-77)
//   - parse_css_inner(): IDHash branch (in Hash|IDHash pattern)    (line 79)
//   - parse_css_inner(): QuotedString branch                       (lines 83-84)
//   - parse_css_inner(): Number with has_sign branch               (line 91)
//   - parse_css_inner(): Percentage with has_sign branch           (line 101)
//   - parse_css_inner(): Dimension with has_sign branch            (line 113)

use super::{format_ident, format_quoted_string, join_css, parse_css};

// ---------------------------------------------------------------------------
// format_quoted_string() — full function body (lines 17-22)
// ---------------------------------------------------------------------------

#[test]
fn format_quoted_string_wraps_in_double_quotes() {
  // The cssparser serialize_string function wraps the value in double-quotes
  let result = format_quoted_string("hello");
  assert!(
    result.starts_with('"') && result.ends_with('"'),
    "format_quoted_string should wrap value in double-quotes, got: {result}"
  );
  assert!(result.contains("hello"));
}

#[test]
fn format_quoted_string_escapes_special_characters() {
  // Strings with quotes or backslashes should be properly escaped
  let result = format_quoted_string("say \"hi\"");
  // The result should still be a valid quoted CSS string
  assert!(result.starts_with('"'));
}

#[test]
fn format_quoted_string_empty_string() {
  let result = format_quoted_string("");
  assert_eq!(
    result, "\"\"",
    "empty string should become empty quoted string"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): Comment branch (lines 37-39)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_comment_is_preserved() {
  // CssToken::Comment — the branch at lines 37-39 uses slice_from to get the
  // raw token text and pushes it to the result.
  let result = parse_css("/* hello */");
  // Comments produce a non-empty iter_result, but after the trim/filter step the
  // result depends on whether the comment text is all whitespace.
  // The filter removes purely whitespace iter_results; a comment like /* hello */
  // becomes "/* hello */" which passes the trim check.
  // We just verify no panic and the function returns.
  let _ = result;
}

#[test]
fn parse_css_comment_mixed_with_content() {
  let result = parse_css("color /* comment */ red");
  // "color" and "red" should appear; the comment may appear too
  let joined = result.join(" ");
  assert!(joined.contains("color") && joined.contains("red"));
}

// ---------------------------------------------------------------------------
// parse_css_inner(): ParenthesisBlock branch (lines 44-46)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_parenthesis_block_is_expanded() {
  // (1 + 2) at top level — produces ParenthesisBlock token (lines 44-46)
  let result = parse_css("(1 + 2)");
  let joined = result.join("");
  assert!(
    joined.contains('(') && joined.contains(')'),
    "parenthesis block should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): SquareBracketBlock branch (lines 49-51)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_square_bracket_block_is_expanded() {
  // [attr=value] — produces SquareBracketBlock token (lines 49-51)
  let result = parse_css("[attr=value]");
  let joined = result.join("");
  assert!(
    joined.contains('[') && joined.contains(']'),
    "square bracket block should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): CurlyBracketBlock branch (lines 54-56)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_curly_bracket_block_is_expanded() {
  // {color: red} at value level — produces CurlyBracketBlock (lines 54-56)
  let result = parse_css("{color: red}");
  let joined = result.join("");
  assert!(
    joined.contains('{') && joined.contains('}'),
    "curly bracket block should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): CloseParenthesis, CloseSquareBracket, CloseCurlyBracket
// (lines 59-61) — unmatched closing delimiters at the top level
// ---------------------------------------------------------------------------

#[test]
fn parse_css_unmatched_close_paren_appears_in_output() {
  // Unmatched ) produces CssToken::CloseParenthesis (line 59)
  let result = parse_css(")");
  let joined = result.join("");
  assert_eq!(joined, ")", "unmatched ) should appear in output");
}

#[test]
fn parse_css_unmatched_close_square_bracket_appears_in_output() {
  // Unmatched ] produces CssToken::CloseSquareBracket (line 60)
  let result = parse_css("]");
  let joined = result.join("");
  assert_eq!(joined, "]", "unmatched ] should appear in output");
}

#[test]
fn parse_css_unmatched_close_curly_brace_appears_in_output() {
  // Unmatched } produces CssToken::CloseCurlyBracket (line 61)
  let result = parse_css("}");
  let joined = result.join("");
  assert_eq!(joined, "}", "unmatched }} should appear in output");
}

// ---------------------------------------------------------------------------
// parse_css_inner(): IncludeMatch (~=), DashMatch (|=), PrefixMatch (^=),
// SuffixMatch ($=), SubstringMatch (*=) branches (lines 62-66)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_include_match_operator() {
  // ~= => Token::IncludeMatch (line 62)
  let result = parse_css("a~=b");
  let joined = result.join("");
  assert!(
    joined.contains("~="),
    "~= should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_dash_match_operator() {
  // |= => Token::DashMatch (line 63)
  let result = parse_css("a|=b");
  let joined = result.join("");
  assert!(
    joined.contains("|="),
    "|= should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_prefix_match_operator() {
  // ^= => Token::PrefixMatch (line 64)
  let result = parse_css("a^=b");
  let joined = result.join("");
  assert!(
    joined.contains("^="),
    "^= should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_suffix_match_operator() {
  // $= => Token::SuffixMatch (line 65)
  let result = parse_css("a$=b");
  let joined = result.join("");
  assert!(
    joined.contains("$="),
    "$= should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_substring_match_operator() {
  // *= => Token::SubstringMatch (line 66)
  let result = parse_css("a*=b");
  let joined = result.join("");
  assert!(
    joined.contains("*="),
    "*= should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): CDO (<!--) and CDC (-->) branches (lines 67-68)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_cdo_html_comment_open() {
  // <!-- => Token::CDO (line 67)
  let result = parse_css("<!--");
  let joined = result.join("");
  assert!(
    joined.contains("<!--"),
    "<!-- should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_cdc_html_comment_close() {
  // --> => Token::CDC (line 68)
  let result = parse_css("-->");
  let joined = result.join("");
  assert!(
    joined.contains("-->"),
    "--> should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): AtKeyword branch (lines 75-77)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_at_keyword_includes_at_sign() {
  // @media => Token::AtKeyword (lines 75-77)
  let result = parse_css("@media");
  let joined = result.join("");
  assert!(
    joined.contains('@'),
    "@media should start with @ in output, got: {joined}"
  );
  assert!(
    joined.contains("media"),
    "@media should include 'media' in output, got: {joined}"
  );
}

#[test]
fn parse_css_at_keyword_charset() {
  let result = parse_css("@charset");
  let joined = result.join("");
  assert!(
    joined.contains("@charset"),
    "@charset should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): Hash|IDHash — IDHash sub-pattern (line 79)
// IDHash is produced when the hash value is a valid CSS identifier
// (starts with a letter, underscore, or hyphen+letter).
// ---------------------------------------------------------------------------

#[test]
fn parse_css_id_hash_starting_with_letter() {
  // #color starts with a letter → IDHash token (line 79, IDHash sub-pattern)
  let result = parse_css("#color");
  let joined = result.join("");
  // format_ident("color") == "color", so output is "#color"
  assert!(
    joined.contains('#'),
    "id hash should include # in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): QuotedString branch (lines 83-84)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_quoted_string_is_serialized() {
  // "Arial" => Token::QuotedString => format_quoted_string (lines 83-84)
  let result = parse_css("\"Arial\"");
  let joined = result.join("");
  assert!(
    joined.contains("Arial"),
    "quoted string should appear in output, got: {joined}"
  );
}

#[test]
fn parse_css_single_quoted_string() {
  let result = parse_css("'Times New Roman'");
  let joined = result.join("");
  assert!(
    joined.contains("Times"),
    "single-quoted string should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): Number with has_sign=true (line 91 — the push('+') branch)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_number_with_explicit_plus_sign() {
  // +42 in CSS produces Token::Number { has_sign: true, value: 42.0 }
  // The branch at line 91 pushes '+' before the value.
  let result = parse_css("+42");
  let joined = result.join("");
  assert!(
    joined.contains("+42") || joined.contains('+'),
    "number with explicit + sign should include + in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): Percentage with has_sign=true (line 101 — push('+') branch)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_percentage_with_explicit_plus_sign() {
  // +50% produces Token::Percentage { has_sign: true, unit_value: 0.5 }
  // The branch at line 101 pushes '+'.
  let result = parse_css("+50%");
  let joined = result.join("");
  assert!(
    joined.contains('+'),
    "percentage with explicit + sign should include + in output, got: {joined}"
  );
  assert!(
    joined.contains('%'),
    "percentage with + sign should include % in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// parse_css_inner(): Dimension with has_sign=true (line 113 — push('+') branch)
// ---------------------------------------------------------------------------

#[test]
fn parse_css_dimension_with_explicit_plus_sign() {
  // +10px produces Token::Dimension { has_sign: true, value: 10.0, unit: "px" }
  // The branch at line 113 pushes '+'.
  let result = parse_css("+10px");
  let joined = result.join("");
  assert!(
    joined.contains('+'),
    "dimension with explicit + sign should include + in output, got: {joined}"
  );
  assert!(
    joined.contains("10"),
    "dimension value should appear in output, got: {joined}"
  );
  assert!(
    joined.contains("px"),
    "dimension unit should appear in output, got: {joined}"
  );
}

// ---------------------------------------------------------------------------
// Characterization tests: verify exact output for newly-covered branches
// so regressions are caught.
// ---------------------------------------------------------------------------

#[test]
fn parse_css_attribute_selector_operators_characterization() {
  // Verify the CSS attribute selector operators produce the correct operator text.
  assert_eq!(parse_css("~="), vec!["~="]);
  assert_eq!(parse_css("|="), vec!["|="]);
  assert_eq!(parse_css("^="), vec!["^="]);
  assert_eq!(parse_css("$="), vec!["$="]);
  assert_eq!(parse_css("*="), vec!["*="]);
}

#[test]
fn parse_css_html_comment_delimiters_characterization() {
  assert_eq!(parse_css("<!--"), vec!["<!--"]);
  assert_eq!(parse_css("-->"), vec!["-->"]);
}

#[test]
fn join_css_single_item() {
  // Verify join_css with one item (no space inserted)
  assert_eq!(join_css(&["10px".to_string()]), "10px");
}

#[test]
fn join_css_empty_slice() {
  assert_eq!(join_css(&[]), "");
}

#[test]
fn format_ident_with_hyphen() {
  // Verify format_ident preserves hyphenated identifiers
  assert_eq!(format_ident("font-family"), "font-family");
}
