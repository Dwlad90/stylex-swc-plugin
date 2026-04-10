/// Returns true if `s` is a CSS dimension unit (e.g. `px`, `em`, `rem`).
/// Used to avoid inserting a space between `)` and a unit — e.g.
/// `var(--x)px` must stay as-is (fix for #927).
fn is_css_unit(s: &str) -> bool {
  matches!(
    s,
    // Absolute lengths
    "px" | "cm" | "mm" | "in" | "pt" | "pc" | "Q"
    // Font-relative lengths
    | "em" | "rem" | "ex" | "ch" | "lh" | "rlh" | "cap" | "ic"
    // Viewport-relative lengths
    | "vw" | "vh" | "vi" | "vb" | "vmin" | "vmax"
    | "dvw" | "dvh" | "lvw" | "lvh" | "svw" | "svh"
    // Container-relative lengths
    | "cqw" | "cqh" | "cqi" | "cqb" | "cqmin" | "cqmax"
    // Time
    | "ms" | "s"
    // Angles
    | "deg" | "rad" | "grad" | "turn"
    // Resolution, flex, frequency
    | "dpi" | "dpcm" | "dppx" | "fr" | "Hz" | "kHz"
  )
}

/// Extract the CSS property value from a stringified rule like `*{prop:value}`.
///
/// Handles both `*{prop:value}` and `prop:value` formats. Returns the value
/// portion with leading/trailing whitespace trimmed. For URL values, returns
/// just the `url(...)` portion.
pub fn extract_css_value(css: &str) -> &str {
  // Fast-path: return URL values as-is
  if let Some(start) = css.find("url(")
    && let Some(end) = css[start..].find(')')
  {
    return &css[start..start + end + 1];
  }

  // Find the value inside a rule wrapper
  let search_start = if let Some(brace) = css.find('{') {
    let mut s = brace + 1;
    // Skip nested `{{`
    while css.as_bytes().get(s) == Some(&b'{') {
      s += 1;
    }
    s
  } else {
    0
  };

  let Some(colon) = css[search_start..].find(':') else {
    return css.trim();
  };
  let colon = colon + search_start;

  // Skip whitespace after ':'
  let val_start = css[colon + 1..]
    .find(|c: char| c != ' ')
    .map_or(colon + 1, |p| colon + 1 + p);

  let end = css[val_start..]
    .find(['}', ';'])
    .map_or(css.len(), |p| val_start + p);

  css[val_start..end].trim()
}

/// Single-pass whitespace normalizer. Scans the CSS string once (O(N)) and
/// inserts spaces where SWC's minified codegen omits them:
///
/// - `)` + letter/function → space (unless followed by a CSS unit like `px`)
/// - `)` + digit, `#`, `(`, `/`, `*` → space
/// - alphanumeric/`%` + `#` → space (hex colors)
/// - `%` + digit/`.` → space (e.g. `40%10` → `40% 10`)
/// - closing `"` + opening `"` → space (adjacent strings)
/// - `/` or `*` + digit/`.`/`(`/`-` → space (calc operators)
///
/// Tracks whether we are inside a quoted string so that `""` (empty string)
/// is NOT split into `" "`.
///
/// Returns the input unchanged if it starts with `url(`.
pub fn normalize_spacing(css: &str) -> String {
  // Fast-path: URL values need no spacing changes
  if css.starts_with("url(") {
    return css.to_string();
  }

  let bytes = css.as_bytes();
  let len = bytes.len();
  if len == 0 {
    return String::new();
  }

  let mut result = String::with_capacity(len + 16);
  // Track whether we're inside a `"`-delimited string so we skip spacing
  // rules for string contents, and whether the previous `"` was a closing
  // quote (used to detect adjacent strings like `"a""b"` → `"a" "b"`).
  let mut in_string = bytes[0] == b'"';
  let mut after_closing_quote = false;
  result.push(bytes[0] as char);

  let mut i = 1;
  while i < len {
    let prev = bytes[i - 1];
    let cur = bytes[i];

    // Handle quote tracking
    if cur == b'"' {
      if in_string {
        // Closing quote
        in_string = false;
        after_closing_quote = true;
        result.push(cur as char);
        i += 1;
        continue;
      }
      // Opening quote: insert space if previous was a closing quote
      if after_closing_quote {
        result.push(' ');
      }
      in_string = true;
      after_closing_quote = false;
      result.push(cur as char);
      i += 1;
      continue;
    }

    // Non-quote character clears the closing-quote flag
    after_closing_quote = false;

    // Inside a string: no spacing rules apply
    if in_string {
      result.push(cur as char);
      i += 1;
      continue;
    }

    let need_space = match (prev, cur) {
      // After `)` before a letter: space unless followed by a CSS unit
      (b')', b'a'..=b'z' | b'A'..=b'Z') => {
        let word_end = bytes[i..]
          .iter()
          .position(|&b| !b.is_ascii_alphanumeric())
          .unwrap_or(len - i);
        let word = &css[i..i + word_end];
        !is_css_unit(word)
      },
      // After `)` before digit, `#`, or `(`
      (b')', b'0'..=b'9' | b'#' | b'(') => true,
      // After `)` before `/` or `*` (calc operators)
      (b')', b'/' | b'*') => true,
      // After alphanumeric or `%` before `#` (hex color)
      (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'%', b'#') => true,
      // After `%` before a number (e.g. `40.101%.1147` → `40.101% .1147`)
      (b'%', b'0'..=b'9' | b'.') => true,
      // After `/` or `*` before operand (calc context)
      (b'/' | b'*', b'0'..=b'9' | b'.' | b'(' | b'-') => true,
      _ => false,
    };

    if need_space {
      result.push(' ');
    }
    result.push(cur as char);
    i += 1;
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  // ── is_css_unit ──────────────────────────────────────────────────────

  #[test]
  fn css_unit_common() {
    assert!(is_css_unit("px"));
    assert!(is_css_unit("em"));
    assert!(is_css_unit("rem"));
    assert!(is_css_unit("vh"));
    assert!(is_css_unit("vw"));
    assert!(is_css_unit("ms"));
    assert!(is_css_unit("s"));
    assert!(is_css_unit("deg"));
    assert!(is_css_unit("fr"));
    assert!(is_css_unit("Hz"));
    assert!(is_css_unit("kHz"));
  }

  #[test]
  fn css_unit_container_viewport() {
    assert!(is_css_unit("cqw"));
    assert!(is_css_unit("cqh"));
    assert!(is_css_unit("cqmin"));
    assert!(is_css_unit("dvw"));
    assert!(is_css_unit("svh"));
  }

  #[test]
  fn non_units_rejected() {
    assert!(!is_css_unit("var"));
    assert!(!is_css_unit("calc"));
    assert!(!is_css_unit("rotate"));
    assert!(!is_css_unit("translate3d"));
    assert!(!is_css_unit("solid"));
    assert!(!is_css_unit("auto"));
    assert!(!is_css_unit(""));
  }

  // ── extract_css_value ────────────────────────────────────────────────

  #[test]
  fn extract_simple_rule() {
    assert_eq!(extract_css_value("*{color:red}"), "red");
  }

  #[test]
  fn extract_double_braces() {
    assert_eq!(extract_css_value("*{{color:blue}}"), "blue");
  }

  #[test]
  fn extract_with_spaces() {
    assert_eq!(
      extract_css_value("*{ color: 1px solid #000 }"),
      "1px solid #000"
    );
  }

  #[test]
  fn extract_url() {
    assert_eq!(
      extract_css_value("*{background:url(image.png)}"),
      "url(image.png)"
    );
  }

  #[test]
  fn extract_no_braces() {
    assert_eq!(extract_css_value("color:red"), "red");
  }

  #[test]
  fn extract_no_colon() {
    assert_eq!(extract_css_value("just-text"), "just-text");
  }

  #[test]
  fn extract_semicolon_terminated() {
    assert_eq!(extract_css_value("*{color:red; margin:0}"), "red");
  }

  // ── normalize_spacing ────────────────────────────────────────────────

  // Issue #927: `)` + CSS unit should NOT get a space
  #[test]
  fn paren_before_css_unit_no_space() {
    assert_eq!(normalize_spacing("var(--x)px"), "var(--x)px");
    assert_eq!(normalize_spacing("var(--gap)rem"), "var(--gap)rem");
    assert_eq!(normalize_spacing("calc(1+2)em"), "calc(1+2)em");
    assert_eq!(normalize_spacing("var(--d)ms"), "var(--d)ms");
    assert_eq!(normalize_spacing("var(--a)deg"), "var(--a)deg");
    assert_eq!(normalize_spacing("var(--h)vh"), "var(--h)vh");
  }

  // `)` + function name gets a space
  #[test]
  fn paren_before_function() {
    assert_eq!(
      normalize_spacing("rotate(10deg)translate3d(0,0,0)"),
      "rotate(10deg) translate3d(0,0,0)"
    );
    assert_eq!(
      normalize_spacing("var(--a)var(--b)var(--c)"),
      "var(--a) var(--b) var(--c)"
    );
  }

  // `)` + digit
  #[test]
  fn paren_before_digit() {
    assert_eq!(normalize_spacing(")3"), ") 3");
    assert_eq!(normalize_spacing("calc(a)42"), "calc(a) 42");
  }

  // `)` + `#` (hex color)
  #[test]
  fn paren_before_hash() {
    assert_eq!(normalize_spacing(")#fff"), ") #fff");
  }

  // `)` + `(` (adjacent functions)
  #[test]
  fn paren_before_open_paren() {
    assert_eq!(normalize_spacing(")("), ") (");
  }

  // `)` + `/` or `*` (calc operators)
  #[test]
  fn paren_before_calc_operators() {
    // Both `)→/` and `/→7` insert spaces
    assert_eq!(normalize_spacing(")/7"), ") / 7");
    assert_eq!(normalize_spacing(")*3"), ") * 3");
  }

  // alphanumeric/`%` + `#`
  #[test]
  fn value_before_hash() {
    assert_eq!(normalize_spacing("1px#000"), "1px #000");
    assert_eq!(normalize_spacing("solid#abc"), "solid #abc");
    assert_eq!(normalize_spacing("50%#fff"), "50% #fff");
  }

  // `%` + number
  #[test]
  fn percent_before_number() {
    assert_eq!(normalize_spacing("40%.1147"), "40% .1147");
    assert_eq!(normalize_spacing("50%10"), "50% 10");
  }

  // `/` or `*` + operand
  #[test]
  fn calc_operators_before_operand() {
    assert_eq!(normalize_spacing("/ 7"), "/ 7");
    assert_eq!(normalize_spacing("* 3"), "* 3");
    assert_eq!(normalize_spacing("/.5"), "/ .5");
    assert_eq!(normalize_spacing("*("), "* (");
    assert_eq!(normalize_spacing("/-1"), "/ -1");
  }

  // Adjacent quoted strings
  #[test]
  fn adjacent_strings_get_space() {
    assert_eq!(
      normalize_spacing(r#""content""sidebar""#),
      r#""content" "sidebar""#
    );
  }

  // Empty string stays as-is
  #[test]
  fn empty_string_no_space() {
    assert_eq!(normalize_spacing(r#""""#), r#""""#);
  }

  // Two empty strings (3 pairs of `""` → space between each)
  #[test]
  fn two_empty_strings_get_space() {
    assert_eq!(normalize_spacing(r#""""""""#), r#""" "" """#);
  }

  // URL fast-path
  #[test]
  fn url_passthrough() {
    let url = "url(image.png)";
    assert_eq!(normalize_spacing(url), url);
  }

  // Empty input
  #[test]
  fn empty_input() {
    assert_eq!(normalize_spacing(""), "");
  }

  // No modifications needed
  #[test]
  fn no_changes_needed() {
    assert_eq!(normalize_spacing("1px solid red"), "1px solid red");
    assert_eq!(normalize_spacing("calc(100% - 20px)"), "calc(100% - 20px)");
  }

  // Compound expressions
  #[test]
  fn compound_expressions() {
    assert_eq!(
      normalize_spacing("oklab(40.101%.1147 .0453)"),
      "oklab(40.101% .1147 .0453)"
    );
  }
}
