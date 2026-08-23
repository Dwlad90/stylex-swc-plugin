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

// ---------------------------------------------------------------------------
// An authored number is echoed, not reprinted
// ---------------------------------------------------------------------------

/// This path passes a value through from the source rather than printing one it
/// computed, and the official compiler does the same, so the target is the
/// author's own bytes.
///
/// It used to read the span into an `f64` and print it back, which respells
/// everything the authored spelling and the shortest round-trip spelling
/// disagree on. Reachable output, not a parser detail: under
/// `styleResolution: 'legacy-expand-shorthands'` these values reach the
/// stylesheet, and each respelling was a different class name.
///
/// The expectations here are this function's own output, which is an
/// intermediate rather than an emitted rule: later passes strip a leading zero
/// and fold a negative zero, so `000.5px` and `-0px` leave here intact and
/// reach a stylesheet as `.5px` and `0px`. What the official compiler emits is
/// asserted where that is observable -- the end-to-end snapshot in
/// `stylex-transform`'s `legacy_deprecated.rs`, whose class names were captured
/// from a side-by-side run against `@stylexjs/babel-plugin@0.19.0`.
///
/// What is comparable at this seam is the digits and their spelling, and those
/// are the assertions below.
#[cfg(test)]
mod an_authored_number_is_echoed {
  use super::*;

  /// The spellings a reprint cannot preserve, because they name the same double
  /// as a shorter spelling does: a trailing zero, an uppercase exponent, an
  /// exponent at all, and an exponent large enough that a reprint went to
  /// twenty-two digits.
  #[test]
  fn a_spelling_a_double_cannot_carry_survives() {
    assert_eq!(parse_css("1.50px"), vec!["1.50px"]);
    assert_eq!(parse_css("1E2px"), vec!["1E2px"]);
    assert_eq!(parse_css("1e2px"), vec!["1e2px"]);
    assert_eq!(parse_css("1e21px"), vec!["1e21px"]);
    assert_eq!(parse_css("1e+21px"), vec!["1e+21px"]);
    assert_eq!(parse_css("000.5px"), vec!["000.5px"]);
  }

  /// A negative zero used to come out as `+-0px`, which is not a CSS value:
  /// the sign-carrying branch tested `value >= 0.`, which a negative zero
  /// satisfies, so a `+` was prepended to a value already carrying a `-`.
  /// Nothing prepends a sign now -- the authored one is inside the literal.
  #[test]
  fn a_negative_zero_is_not_given_a_second_sign() {
    assert_eq!(parse_css("-0px"), vec!["-0px"]);
    assert_eq!(parse_css("-0"), vec!["-0"]);
    assert_eq!(parse_css("-0%"), vec!["-0%"]);
  }

  /// An authored `+` still survives, which is the behaviour the old manual
  /// prepend existed for -- it just comes from the literal now.
  #[test]
  fn an_authored_positive_sign_still_survives() {
    assert_eq!(parse_css("+1px"), vec!["+1px"]);
    assert_eq!(parse_css("+1"), vec!["+1"]);
    assert_eq!(parse_css("+2%"), vec!["+2%"]);
    assert_eq!(parse_css("-3px"), vec!["-3px"]);
    assert_eq!(parse_css("-10%"), vec!["-10%"]);
  }

  /// The digits a reprint dropped or saturated before the widening, which the
  /// echo carries for a different reason: it never reads them as a number.
  #[test]
  fn digits_past_what_a_double_holds_survive_as_authored() {
    assert_eq!(
      parse_css("1.2345678901234567px"),
      vec!["1.2345678901234567px"]
    );
    assert_eq!(
      parse_css("0.12345678901234567px"),
      vec!["0.12345678901234567px"]
    );
    assert_eq!(
      parse_css("1.7976931348623157e308px"),
      vec!["1.7976931348623157e308px"]
    );
    assert_eq!(parse_css("5e-324px"), vec!["5e-324px"]);
    // Past the double range entirely: the author's text, not `Infinity`.
    assert_eq!(parse_css("1e400px"), vec!["1e400px"]);
  }

  /// A percentage carries the authored percent rather than a fraction scaled
  /// back up, and now carries the authored spelling of it too.
  #[test]
  fn a_percentage_echoes_the_authored_percent() {
    assert_eq!(parse_css("7%"), vec!["7%"]);
    assert_eq!(parse_css("0.50%"), vec!["0.50%"]);
    assert_eq!(parse_css("1e2%"), vec!["1e2%"]);
    assert_eq!(
      parse_css("33.333333333333336%"),
      vec!["33.333333333333336%"]
    );
  }

  /// A trailing dot is not part of the number, so `1.px` is three tokens --
  /// unchanged by the echo, and pinned because the number's span is what the
  /// echo depends on.
  #[test]
  fn a_trailing_dot_is_not_part_of_the_number() {
    assert_eq!(parse_css("1.px"), vec!["1", ".", "px"]);
  }

  /// An incomplete exponent is the number followed by an identifier, which is
  /// the same span rule seen from the other side.
  #[test]
  fn an_incomplete_exponent_is_not_part_of_the_number() {
    assert_eq!(parse_css("1epx"), vec!["1epx"]);
    assert_eq!(parse_css("1e"), vec!["1e"]);
    assert_eq!(parse_css("1e+"), vec!["1e", "+"]);
  }

  /// A number inside a function or a list takes the same arm, so the echo is
  /// not confined to a whole-value token.
  #[test]
  fn a_number_nested_in_a_function_or_list_is_echoed_too() {
    assert_eq!(
      parse_css("calc(1.50px + 1E2px)"),
      vec!["calc(1.50px + 1E2px)"]
    );
    assert_eq!(parse_css("1.50px 1E2px"), vec!["1.50px", "1E2px"]);
    assert_eq!(
      parse_css("translateX(1.50px) rotate(1E2deg)"),
      vec!["translateX(1.50px)", "rotate(1E2deg)"]
    );
  }

  /// A known divergence, pinned rather than fixed: the unit comes from the
  /// token rather than the source, so an escaped unit is emitted as what it
  /// escapes to. The official compiler echoes the escape itself, so `1\70x`
  /// stays `1\70x` there and becomes `1px` here.
  ///
  /// This is a lost token rather than a lost spelling -- the same shape of
  /// finding as the `lch()` percent recorded in ticket 04 -- and closing it
  /// means echoing the unit's span too, which is a separate change.
  #[test]
  fn an_escaped_unit_is_still_unescaped_which_the_official_compiler_does_not_do() {
    assert_eq!(parse_css("1\\70x"), vec!["1px"]);
    assert_eq!(parse_css("1.50\\70x"), vec!["1.50px"]);
  }

  /// Malformed input still parses to what it parsed to before: the echo reads a
  /// span the tokenizer already delimited, so it cannot refuse what the
  /// tokenizer accepted.
  #[test]
  fn malformed_input_is_unchanged_by_the_echo() {
    assert_eq!(parse_css("calc(1.50px"), vec!["calc(1.50px)"]);
    assert_eq!(parse_css("\"1.50px"), vec!["\"1.50px\""]);
    assert_eq!(parse_css(""), Vec::<String>::new());
    assert_eq!(parse_css("1.50px)"), vec!["1.50px", ")"]);
  }
}
