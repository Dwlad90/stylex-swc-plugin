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
