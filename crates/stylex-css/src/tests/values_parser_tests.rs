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
