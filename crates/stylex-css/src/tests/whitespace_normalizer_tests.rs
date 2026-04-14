use super::{extract_css_value, normalize_spacing};

#[test]
fn extract_css_value_keeps_full_composite_url_rule_value() {
  let css = r#"*{background:url("asset.png") no-repeat center/cover}"#;
  let value = extract_css_value(css);

  assert_eq!(value, r#"url("asset.png") no-repeat center/cover"#);
}

#[test]
fn extract_css_value_keeps_plain_composite_url_value() {
  let css = r#"url("asset.png") no-repeat center/cover"#;
  let value = extract_css_value(css);

  assert_eq!(value, css);
}

#[test]
fn extract_css_value_keeps_data_url_semicolon_content() {
  let css = r#"*{background-image:url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")}"#;
  let value = extract_css_value(css);

  assert_eq!(
    value,
    r#"url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")"#
  );
}

// ── extract_css_value: `;` and `}` terminator at paren_depth == 0 (L82) ──

/// A `;` outside of parens should terminate the value extraction.
#[test]
fn extract_css_value_stops_at_semicolon_outside_parens() {
  // Simulates: `*{color:red;background:blue}` — value of `color` ends at `;`
  let css = "*{color:red;background:blue}";
  let value = extract_css_value(css);
  assert_eq!(value, "red");
}

/// A `}` outside of parens should terminate the value extraction.
#[test]
fn extract_css_value_stops_at_close_brace_outside_parens() {
  let css = "*{color:red}";
  let value = extract_css_value(css);
  assert_eq!(value, "red");
}

// ── normalize_spacing: empty string fast-path (L115) ─────────────────────

/// An empty string input should return an empty string immediately.
#[test]
fn normalize_spacing_empty_string_returns_empty() {
  assert_eq!(normalize_spacing(""), "");
}

// ── normalize_spacing: uppercase letter after `)` (L165) ─────────────────

/// An uppercase letter immediately after `)` should get a space inserted,
/// unless the word is a CSS unit.
#[test]
fn normalize_spacing_uppercase_after_close_paren_gets_space() {
  // `)A` → `)` then `A` — "A" is not a CSS unit, so a space is inserted
  let result = normalize_spacing(")A");
  assert_eq!(result, ") A");
}

/// An uppercase CSS unit `Q` after `)` should NOT get a space.
#[test]
fn normalize_spacing_uppercase_css_unit_after_paren_no_space() {
  // `Q` is the CSS unit for quarter-millimeters; it is a CSS unit
  let result = normalize_spacing(")Q");
  assert_eq!(result, ")Q");
}

// ── normalize_spacing: uppercase/digit/% before `#` (L178) ──────────────

/// An uppercase letter immediately before `#` should get a space inserted.
#[test]
fn normalize_spacing_uppercase_before_hash_gets_space() {
  let result = normalize_spacing("A#fff");
  assert_eq!(result, "A #fff");
}

/// A digit immediately before `#` should get a space inserted.
#[test]
fn normalize_spacing_digit_before_hash_gets_space() {
  let result = normalize_spacing("9#fff");
  assert_eq!(result, "9 #fff");
}

/// A `%` immediately before `#` should get a space inserted.
#[test]
fn normalize_spacing_percent_before_hash_gets_space() {
  let result = normalize_spacing("%#fff");
  assert_eq!(result, "% #fff");
}

/// An opening quote preceded by non-quote content (not by a closing quote)
/// exercises the `after_closing_quote == false` branch in the quote handler.
#[test]
fn normalize_spacing_opening_quote_after_non_quote_content() {
  // The space between `a` and `"b"` resets `after_closing_quote` to false,
  // then encountering `"` takes the `!after_closing_quote` path.
  let result = normalize_spacing(r#""a" x "b""#);
  assert_eq!(result, r#""a" x "b""#);
}
