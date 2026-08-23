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
  assert_eq!(format_ident("a"), "a");
  assert_eq!(
    format_ident("#007bff".trim_start_matches('#')),
    "\\30 07bff"
  );
}

/// A name the identifier grammar cannot spell literally is escaped rather than
/// dropped: a leading digit gets the same treatment as the hex colour above,
/// and the double hyphen of a custom property needs none and keeps its
/// spelling.
#[test]
fn format_ident_escapes_a_leading_digit_and_leaves_custom_properties_alone() {
  assert_eq!(format_ident("123abc"), "\\31 23abc");
  assert_eq!(format_ident("--my-var"), "--my-var");
}

/// Both ends of the range an identifier can occupy: every character already
/// legal in one, and no characters at all.
#[test]
fn format_ident_handles_underscores_dashes_and_the_empty_name() {
  assert_eq!(format_ident("_underscore-dash"), "_underscore-dash");
  assert_eq!(format_ident(""), "");
}

#[test]
#[should_panic(expected = "Unsupported CSS token")]
fn parse_css_panics_on_unquoted_url_values() {
  let _ = parse_css("url(foo)");
}

// ---------------------------------------------------------------------------
// Authored numbers are echoed, not re-derived
// ---------------------------------------------------------------------------
//
// `parse_css` echoes an authored value rather than computing one, so the
// number it prints has to be the number the author typed. `cssparser` hands it
// over as an `f32`, which is not wide enough to say so, and this function used
// to print that `f32` straight out. Every expectation below was confirmed
// against `@stylexjs/babel-plugin@0.19.0` run over the same declaration.

/// Seventeen significant digits and a fraction with no short `f32` spelling.
/// These printed as `1.2345679px` and `33.333336%`.
#[test]
fn a_value_past_single_precision_is_echoed_with_its_own_digits() {
  assert_eq!(
    parse_css("1.2345678901234567px 33.333333333333336% 2px"),
    vec!["1.2345678901234567px", "33.333333333333336%", "2px"]
  );
}

/// The worst of it: a magnitude inside the double range but past the single
/// one saturated to infinity, and `infpx` went into the stylesheet.
#[test]
fn a_magnitude_past_the_single_precision_range_is_not_infinity() {
  let printed = parse_css("1.7976931348623157e308px");

  assert_eq!(printed.len(), 1);
  assert!(!printed[0].contains("inf"), "{}", printed[0]);
}

/// A percentage is echoed as authored rather than divided and multiplied back
/// up, which is what made `7%` a candidate for `7.000000000000001%`.
#[test]
fn a_percentage_is_echoed_as_authored() {
  assert_eq!(parse_css("7% 50% 0.0005%"), vec!["7%", "50%", "0.0005%"]);
}

/// The values people actually write are unchanged, so the fix is shown to move
/// only what was wrong.
#[test]
fn ordinary_values_are_unchanged() {
  assert_eq!(
    parse_css("1.2rem 28.81rem 0.0005px +50% -1.5px 0"),
    vec!["1.2rem", "28.81rem", "0.0005px", "+50%", "-1.5px", "0"]
  );
}

/// Numbers nested inside a function take the recursive path, which threads the
/// same source offset through a nested parser.
#[test]
fn numbers_nested_in_a_function_keep_their_digits() {
  assert_eq!(
    parse_css("translate(1.2345678901234567px, 2px)"),
    vec!["translate(1.2345678901234567px,2px)"]
  );
}

/// A number after multibyte text, where a cursor that counted characters
/// instead of bytes would read from the wrong offset.
#[test]
fn a_number_after_multibyte_text_is_read_from_its_own_bytes() {
  assert_eq!(
    parse_css("\"héllo — wörld\" 1.2345678901234567px"),
    vec!["\"héllo — wörld\"", "1.2345678901234567px"]
  );
}
