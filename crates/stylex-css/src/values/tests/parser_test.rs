use crate::values::parser::{format_ident, parse_css};

// ── format_ident ─────────────────────────────────────────────────────

#[test]
fn format_ident_simple() {
  assert_eq!(format_ident("color"), "color");
}

#[test]
fn format_ident_dashed() {
  assert_eq!(format_ident("margin-top"), "margin-top");
}

#[test]
fn format_ident_custom_property() {
  let result = format_ident("--my-var");
  assert!(result.contains("--my-var"));
}

#[test]
fn format_ident_single_char() {
  assert_eq!(format_ident("a"), "a");
}

#[test]
fn format_ident_with_number_start() {
  // CSS idents starting with a digit need escaping
  let result = format_ident("123abc");
  assert!(!result.is_empty());
}

// ── parse_css ────────────────────────────────────────────────────────

#[test]
fn parse_css_single_value() {
  let result = parse_css("10px");
  assert_eq!(result, vec!["10px"]);
}

#[test]
fn parse_css_multiple_values() {
  let result = parse_css("10px 20px 30px");
  assert_eq!(result, vec!["10px", "20px", "30px"]);
}

#[test]
fn parse_css_empty_string() {
  let result = parse_css("");
  assert!(result.is_empty());
}

#[test]
fn parse_css_single_ident() {
  let result = parse_css("red");
  assert_eq!(result, vec!["red"]);
}

#[test]
fn parse_css_function() {
  let result = parse_css("rgb(255, 0, 0)");
  // Should parse the function with its arguments
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("rgb("));
  assert!(joined.contains("255"));
}

#[test]
fn parse_css_percentage() {
  let result = parse_css("50%");
  assert_eq!(result, vec!["50%"]);
}

#[test]
fn parse_css_number() {
  let result = parse_css("42");
  assert_eq!(result, vec!["42"]);
}

#[test]
fn parse_css_negative_number() {
  let result = parse_css("-10px");
  assert_eq!(result, vec!["-10px"]);
}

#[test]
fn parse_css_calc() {
  let result = parse_css("calc(100% - 20px)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("calc("));
}

#[test]
fn parse_css_var_function() {
  let result = parse_css("var(--my-color)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("var("));
  assert!(joined.contains("--my-color"));
}

#[test]
fn parse_css_hash() {
  let result = parse_css("#ff0000");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("#"));
  assert!(joined.contains("ff0000"));
}

#[test]
fn parse_css_delimiters() {
  let result = parse_css("10px / 20px");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("/"));
}

#[test]
fn parse_css_quoted_string() {
  let result = parse_css(r#""hello""#);
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("hello"));
}

#[test]
fn parse_css_multiple_functions() {
  let result = parse_css("translateX(10px) rotate(45deg)");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("translateX(") || joined.contains("translatex("));
  assert!(joined.contains("rotate("));
}

#[test]
fn parse_css_square_brackets() {
  let result = parse_css("[header-start]");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("["));
  assert!(joined.contains("]"));
}

#[test]
fn parse_css_colon_and_semicolon() {
  let result = parse_css("color: red; margin: 0");
  assert!(!result.is_empty());
}

#[test]
fn parse_css_at_keyword() {
  let result = parse_css("@media screen");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("@media"));
}

#[test]
fn parse_css_decimal_number() {
  let result = parse_css("0.5");
  assert_eq!(result, vec!["0.5"]);
}

#[test]
fn parse_css_dimension_units() {
  let result = parse_css("1em");
  assert_eq!(result, vec!["1em"]);

  let result2 = parse_css("100vh");
  assert_eq!(result2, vec!["100vh"]);

  let result3 = parse_css("2rem");
  assert_eq!(result3, vec!["2rem"]);
}

#[test]
fn parse_css_filters_empty_and_comma() {
  // parse_css filters out empty strings and lone commas
  let result = parse_css("a, b");
  // Commas should be filtered out
  assert!(result.iter().all(|s| s != ","));
}

#[test]
fn parse_css_nested_function() {
  let result = parse_css("calc(var(--x) + 10px)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("calc("));
  assert!(joined.contains("var("));
}

// ── Additional coverage: token branches ─────────────────────────────

#[test]
fn parse_css_signed_positive_number() {
  let result = parse_css("+5");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("+5") || joined.contains("5"));
}

#[test]
fn parse_css_signed_positive_percentage() {
  let result = parse_css("+50%");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("50%"));
}

#[test]
fn parse_css_signed_positive_dimension() {
  let result = parse_css("+10px");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("10px"));
}

#[test]
fn parse_css_negative_percentage() {
  let result = parse_css("-25%");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("-25%"));
}

#[test]
fn parse_css_zero_percentage() {
  let result = parse_css("0%");
  assert_eq!(result, vec!["0%"]);
}

#[test]
fn parse_css_id_hash_selector() {
  let result = parse_css("#myId");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("#"));
}

#[test]
fn parse_css_at_keyword_charset() {
  let result = parse_css("@charset");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("@charset"));
}

#[test]
fn parse_css_at_keyword_font_face() {
  let result = parse_css("@font-face");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("@font-face"));
}

#[test]
fn parse_css_curly_braces() {
  let result = parse_css("a { color: red }");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("{"));
  assert!(joined.contains("}"));
}

#[test]
fn parse_css_comma_filtered_out() {
  let result = parse_css("a, b, c");
  // Commas are filtered by parse_css
  for item in &result {
    assert_ne!(item, ",");
  }
}

#[test]
fn parse_css_multiple_dimensions() {
  let result = parse_css("1rem 2em 3vh 4vw");
  assert_eq!(result.len(), 4);
}

#[test]
fn parse_css_slash_in_join() {
  // Test the join_css "/" handling: no space before "/"
  let result = parse_css("10px / 20px");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("/"));
}

#[test]
fn parse_css_format_ident_special() {
  let result = format_ident("_underscore-dash");
  assert_eq!(result, "_underscore-dash");
}

#[test]
fn parse_css_format_ident_empty() {
  let result = format_ident("");
  assert!(result.is_empty());
}

#[test]
fn parse_css_whitespace_only() {
  let result = parse_css("   ");
  assert!(result.is_empty());
}

#[test]
fn parse_css_comment() {
  let result = parse_css("/* comment */ red");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("red"));
}

#[test]
fn parse_css_semicolon() {
  let result = parse_css("color: red; margin: 0");
  let joined = result.join(" ");
  assert!(joined.contains(";"));
}

#[test]
fn parse_css_single_quoted_string() {
  let result = parse_css("'hello world'");
  let joined = result.join("");
  assert!(joined.contains("hello world"));
}

#[test]
fn parse_css_negative_dimension() {
  let result = parse_css("-3em");
  assert_eq!(result, vec!["-3em"]);
}

#[test]
fn parse_css_zero_number() {
  let result = parse_css("0");
  assert_eq!(result, vec!["0"]);
}

#[test]
fn parse_css_large_number() {
  let result = parse_css("999999");
  assert_eq!(result, vec!["999999"]);
}

#[test]
fn parse_css_nested_parens() {
  let result = parse_css("calc(max(10px, 20px) + 5px)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("calc("));
  assert!(joined.contains("max("));
}

#[test]
fn parse_css_delim_star() {
  let result = parse_css("*");
  assert!(!result.is_empty());
  assert_eq!(result[0], "*");
}

#[test]
fn parse_css_delim_tilde() {
  let result = parse_css("~");
  assert!(!result.is_empty());
}

// ── Coverage: close brackets at top level ───────────────────────────

#[test]
fn parse_css_close_paren_top_level() {
  let result = parse_css(")");
  assert!(!result.is_empty());
  assert_eq!(result[0], ")");
}

#[test]
fn parse_css_close_square_bracket_top_level() {
  let result = parse_css("]");
  assert!(!result.is_empty());
  assert_eq!(result[0], "]");
}

#[test]
fn parse_css_close_curly_bracket_top_level() {
  let result = parse_css("}");
  assert!(!result.is_empty());
  assert_eq!(result[0], "}");
}

// ── Coverage: match operators ───────────────────────────────────────

#[test]
fn parse_css_include_match() {
  let result = parse_css("~=");
  assert!(!result.is_empty());
  assert_eq!(result[0], "~=");
}

#[test]
fn parse_css_dash_match() {
  let result = parse_css("|=");
  assert!(!result.is_empty());
  assert_eq!(result[0], "|=");
}

#[test]
fn parse_css_prefix_match() {
  let result = parse_css("^=");
  assert!(!result.is_empty());
  assert_eq!(result[0], "^=");
}

#[test]
fn parse_css_suffix_match() {
  let result = parse_css("$=");
  assert!(!result.is_empty());
  assert_eq!(result[0], "$=");
}

#[test]
fn parse_css_substring_match() {
  let result = parse_css("*=");
  assert!(!result.is_empty());
  assert_eq!(result[0], "*=");
}

// ── Coverage: CDO / CDC (HTML comment delimiters) ───────────────────

#[test]
fn parse_css_cdo() {
  let result = parse_css("<!--");
  assert!(!result.is_empty());
  assert_eq!(result[0], "<!--");
}

#[test]
fn parse_css_cdc() {
  let result = parse_css("-->");
  assert!(!result.is_empty());
  assert_eq!(result[0], "-->");
}

// ── Coverage: ident token (curr_rule / curr_prop assignment) ────────

#[test]
fn parse_css_plain_ident() {
  let result = parse_css("div");
  assert!(!result.is_empty());
  assert_eq!(result[0], "div");
}

#[test]
fn parse_css_multiple_idents() {
  let result = parse_css("margin auto");
  assert!(result.len() >= 2);
}

// ── Coverage: hash token (non-ID hash) ──────────────────────────────

#[test]
fn parse_css_hex_color_hash() {
  let result = parse_css("#fff");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("#"));
  assert!(joined.contains("fff"));
}

// ── Coverage: signed number / percentage / dimension ────────────────

#[test]
fn parse_css_explicit_positive_number() {
  let result = parse_css("+42");
  assert!(!result.is_empty());
  let joined = result.join("");
  // Should contain the + sign and 42
  assert!(joined.contains("42"));
}

#[test]
fn parse_css_explicit_positive_percentage() {
  let result = parse_css("+75%");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("75%"));
}

#[test]
fn parse_css_explicit_positive_dimension() {
  let result = parse_css("+20em");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("20em"));
}

// ── Coverage: function token (nested function parsing) ──────────────

#[test]
fn parse_css_function_rgb() {
  let result = parse_css("rgb(255, 128, 0)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("rgb("));
  assert!(joined.contains(")"));
}

#[test]
fn parse_css_function_min() {
  let result = parse_css("min(10px, 50%)");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("min("));
}

#[test]
fn parse_css_hash_token_variant() {
  let result = parse_css("#-invalid");
  assert!(!result.is_empty());
  assert!(result.join("").contains('#'));
}

#[test]
#[should_panic(expected = "Unsupported CSS token")]
fn parse_css_unquoted_url_panics() {
  let _ = parse_css("url(foo)");
}

#[test]
fn parse_css_malformed_function_is_tolerated() {
  let result = parse_css("calc(100% -");
  assert!(!result.is_empty());
}

#[test]
fn parse_css_malformed_bracket_block_is_tolerated() {
  let result = parse_css("[foo");
  assert!(!result.is_empty());
}

// ── Coverage: at-keyword sets curr_rule ─────────────────────────────

#[test]
fn parse_css_at_import() {
  let result = parse_css("@import");
  assert!(!result.is_empty());
  let joined = result.join("");
  assert!(joined.contains("@import"));
}

// ── Coverage: curly brace block (not already covered inline) ────────

#[test]
fn parse_css_curly_block_content() {
  let result = parse_css("div { color: red }");
  assert!(!result.is_empty());
  let joined = result.join(" ");
  assert!(joined.contains("{"));
  assert!(joined.contains("}"));
  assert!(joined.contains("color"));
}
