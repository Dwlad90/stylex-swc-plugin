/// Returns true if `s` is a CSS dimension unit (e.g. `px`, `em`, `rem`).
/// Used to avoid inserting a space between `)` and a unit — e.g.
/// `var(--x)px` must stay as-is (fix for #927).
pub fn is_css_unit(s: &str) -> bool {
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
/// portion with leading/trailing whitespace trimmed.
pub fn extract_css_value(css: &str) -> &str {
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

  let mut end = css.len();
  let mut paren_depth: usize = 0;
  let mut in_single_quote = false;
  let mut in_double_quote = false;
  let mut escaped = false;

  for (offset, byte) in css.as_bytes()[val_start..].iter().enumerate() {
    let idx = val_start + offset;

    if escaped {
      escaped = false;
      continue;
    }

    match *byte {
      b'\\' if in_single_quote || in_double_quote => {
        escaped = true;
      },
      b'\'' if !in_double_quote => {
        in_single_quote = !in_single_quote;
      },
      b'"' if !in_single_quote => {
        in_double_quote = !in_double_quote;
      },
      b'(' if !in_single_quote && !in_double_quote => {
        paren_depth += 1;
      },
      b')' if !in_single_quote && !in_double_quote => {
        paren_depth = paren_depth.saturating_sub(1);
      },
      b';' | b'}' if !in_single_quote && !in_double_quote && paren_depth == 0 => {
        end = idx;
        break;
      },
      _ => {},
    }
  }

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
#[path = "../../tests/whitespace_normalizer_tests.rs"]
mod tests;
