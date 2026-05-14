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

  let chars = css.chars().collect::<Vec<_>>();
  if chars.is_empty() {
    return String::new();
  }

  let mut result = String::with_capacity(css.len() + 16);
  // Track quoted strings so spacing rules are not applied to string contents.
  let mut in_quote = if chars[0] == '"' || chars[0] == '\'' {
    Some(chars[0])
  } else {
    None
  };
  let mut escaped = false;
  let mut after_closing_quote = false;
  result.push(chars[0]);

  let mut i = 1;
  while i < chars.len() {
    let prev = chars[i - 1];
    let cur = chars[i];

    if let Some(quote) = in_quote {
      if escaped {
        escaped = false;
        result.push(cur);
        i += 1;
        continue;
      }

      if cur == '\\' {
        escaped = true;
        result.push(cur);
        i += 1;
        continue;
      }

      if cur == quote {
        in_quote = None;
        after_closing_quote = true;
        result.push(cur);
        i += 1;
        continue;
      }

      result.push(cur);
      i += 1;
      continue;
    }

    if cur == '"' || cur == '\'' {
      if after_closing_quote {
        result.push(' ');
      }
      in_quote = Some(cur);
      after_closing_quote = false;
      result.push(cur);
      i += 1;
      continue;
    }

    // Non-quote character clears the closing-quote flag
    after_closing_quote = false;

    let need_space = match (prev, cur) {
      // After `)` before a letter: space unless followed by a CSS unit
      (')', 'a'..='z' | 'A'..='Z') => {
        let word_end = chars[i..]
          .iter()
          .position(|c| !c.is_ascii_alphanumeric())
          .unwrap_or(chars.len() - i);
        let word = chars[i..i + word_end].iter().collect::<String>();
        !is_css_unit(&word)
      },
      // After `)` before digit, `#`, or `(`
      (')', '0'..='9' | '#' | '(') => true,
      // After `)` before `/` or `*` (calc operators)
      (')', '/' | '*') => true,
      // After alphanumeric or `%` before `#` (hex color)
      ('a'..='z' | 'A'..='Z' | '0'..='9' | '%', '#') => true,
      // After `%` before a number (e.g. `40.101%.1147` → `40.101% .1147`)
      ('%', '0'..='9' | '.') => true,
      // After `/` or `*` before operand (calc context)
      ('/' | '*', '0'..='9' | '.' | '(' | '-') => true,
      _ => false,
    };

    if need_space {
      result.push(' ');
    }
    result.push(cur);
    i += 1;
  }

  result
}

#[cfg(test)]
#[path = "../../tests/whitespace_normalizer_tests.rs"]
mod tests;
