use super::{format_ident, join_css, parse_css};

#[test]
fn parse_css_characterizes_current_token_stream_outputs() {
  let cases = [
    ("rgb(255, 0, 0)", vec!["rgb(255,0,0)"]),
    ("calc(100% - 20px)", vec!["calc(100% - 20px)"]),
    ("a, b, c", vec!["a", "b", "c"]),
    (
      "color: red; margin: 0",
      vec!["color", ":", "red", ";", "margin", ":", "0"],
    ),
    ("#007bff", vec!["#\\30 07bff"]),
  ];

  for (input, expected) in cases {
    assert_eq!(parse_css(input), expected);
  }
}

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
fn format_ident_matches_css_identifier_serialization() {
  assert_eq!(format_ident("color"), "color");
  assert_eq!(format_ident("margin-top"), "margin-top");
  assert_eq!(
    format_ident("#007bff".trim_start_matches('#')),
    "\\30 07bff"
  );
}

#[test]
#[should_panic(expected = "Unsupported CSS token")]
fn parse_css_panics_on_unquoted_url_values() {
  let _ = parse_css("url(foo)");
}
