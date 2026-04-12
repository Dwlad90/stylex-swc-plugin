// Tests for CSS value token joining and parser edge cases.
// Source: crates/stylex-css/src/values/parser.rs

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

#[test]
fn parse_css_supports_ident_and_nested_blocks() {
  // Cover identifier and all nested block token variants in one parse pass.
  let result = parse_css("foo(bar)[baz]{qux}");
  let joined = result.join("");

  assert_eq!(joined, "foo(bar)[baz]{qux}");
}

#[test]
fn parse_css_preserves_signed_number_percentage_and_dimension() {
  let result = parse_css("+1 +2% +3px");
  let joined = result.join("");

  assert_eq!(joined, "+1+2%+3px");
}

#[test]
#[should_panic(expected = "Unsupported CSS token")]
fn parse_css_panics_on_unquoted_url_values() {
  // Explicitly unsupported in this parser path.
  let _ = parse_css("url(foo)");
}

// Additional targeted tests to exercise edge branches discovered by tarpaulin

#[test]
fn parse_css_parenthesis_block_only() {
  // Ensure a bare parenthesis block (not a function) is handled
  let result = parse_css("(a)");
  assert_eq!(result.join(""), "(a)");
}

#[test]
fn parse_css_square_bracket_block_only() {
  let result = parse_css("[a]");
  assert_eq!(result.join(""), "[a]");
}

#[test]
fn parse_css_curly_bracket_block_only() {
  let result = parse_css("{a}");
  assert_eq!(result.join(""), "{a}");
}

#[test]
fn parse_css_ident_only() {
  let result = parse_css("foo");
  assert_eq!(result.join(""), "foo");
}

#[test]
fn parse_css_signed_numbers_and_units_explicit() {
  assert_eq!(parse_css("+1").join(""), "+1");
  assert_eq!(parse_css("-1").join(""), "-1");
  assert_eq!(parse_css("+10%").join(""), "+10%");
  assert_eq!(parse_css("-10%").join(""), "-10%");
  assert_eq!(parse_css("+3px").join(""), "+3px");
  assert_eq!(parse_css("-3px").join(""), "-3px");
}
