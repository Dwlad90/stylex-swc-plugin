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

/// Returns `true` when the `(` at `open_paren_index` opens a `url()` function.
///
/// `url()` bodies are copied verbatim by [`normalize_spacing`], so this guard
/// must not fire for identifiers that merely *end* in `url` (`blurl(`,
/// `--icon-url(`). The name is matched case-insensitively (`URL(` is valid
/// CSS) and the character preceding it must not continue an identifier.
///
/// The scan walks Unicode scalar values backwards rather than indexing raw
/// bytes: `open_paren_index - 3` is not necessarily a UTF-8 character
/// boundary, and slicing a `&str` there panics (e.g. `éab(`).
fn starts_url_function(css: &str, open_paren_index: usize) -> bool {
  // `open_paren_index` comes from `char_indices`, so this slice is in bounds
  // and on a character boundary.
  let mut preceding = css[..open_paren_index].chars().rev();

  for expected in ['l', 'r', 'u'] {
    if !preceding
      .next()
      .is_some_and(|ch| ch.eq_ignore_ascii_case(&expected))
    {
      return false;
    }
  }

  !preceding
    .next()
    .is_some_and(|ch| ch.is_alphanumeric() || matches!(ch, '-' | '_' | '\\'))
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
/// - either side of `/` → space (division and the CSS slash separator, e.g.
///   `calc-size(fit-content,size/2)` → `… size / 2`)
/// - `*` + digit/`.`/`(`/`-` → space (calc operators)
///
/// Three regions are copied through verbatim, in this precedence order:
/// `url()` bodies (which are not tokenized as CSS, so a `)` inside a quoted URL
/// does not end them), `/* … */` comments, and quoted strings (so `""` is not
/// split into `" "`). The region checks run before any spacing rule, including
/// at offset 0, so a value that *starts* with a comment or a quote is handled
/// the same as one that does not.
pub fn normalize_spacing(css: &str) -> String {
  let mut result = String::with_capacity(css.len() + 16);
  let mut chars = css.char_indices().peekable();

  // Track quoted strings so spacing rules are not applied to string contents.
  let mut in_quote: Option<char> = None;
  let mut escaped = false;
  let mut after_closing_quote = false;
  let mut in_comment = false;
  // The previous character *within the current comment*. Starts as `None` so
  // the `*` of the opening `/*` cannot also close it — `/*/` is not a complete
  // comment.
  let mut comment_prev: Option<char> = None;
  let mut url_depth: usize = 0;
  let mut url_quote: Option<char> = None;
  let mut url_escaped = false;
  // `None` until the first character has been emitted. Keeping this an `Option`
  // rather than pre-consuming the first character is what lets a value that
  // *starts* with `/*` or a quote take the same path as one that does not.
  let mut prev: Option<char> = None;

  while let Some((idx, cur)) = chars.next() {
    // `url()` bodies are not CSS-tokenized: `/*` and a bare `/` inside them are
    // ordinary URL characters, so this must be checked before comments. Quotes
    // are still tracked, because a `)` inside a quoted URL does not close the
    // function.
    if url_depth > 0 {
      if url_escaped {
        url_escaped = false;
      } else if cur == '\\' {
        url_escaped = true;
      } else if let Some(quote) = url_quote {
        if cur == quote {
          url_quote = None;
        }
      } else if cur == '"' || cur == '\'' {
        url_quote = Some(cur);
      } else if cur == '(' {
        url_depth += 1;
      } else if cur == ')' {
        url_depth -= 1;
      }

      result.push(cur);
      prev = Some(cur);
      continue;
    }

    if in_comment {
      result.push(cur);

      if comment_prev == Some('*') && cur == '/' {
        in_comment = false;
        comment_prev = None;
      } else {
        comment_prev = Some(cur);
      }

      prev = Some(cur);
      continue;
    }

    if let Some(quote) = in_quote {
      result.push(cur);

      if escaped {
        escaped = false;
      } else if cur == '\\' {
        escaped = true;
      } else if cur == quote {
        in_quote = None;
        after_closing_quote = true;
      }

      prev = Some(cur);
      continue;
    }

    if cur == '(' && starts_url_function(css, idx) {
      url_depth = 1;
      url_quote = None;
      url_escaped = false;
      result.push(cur);
      prev = Some(cur);
      continue;
    }

    if cur == '/' && chars.peek().is_some_and(|(_, next)| *next == '*') {
      // Consume the `*` of the opening `/*` together with the `/` so neither
      // can be split apart by the `/` spacing rules below.
      chars.next();
      result.push('/');
      result.push('*');
      in_comment = true;
      comment_prev = None;
      prev = Some('*');
      continue;
    }

    if cur == '"' || cur == '\'' {
      if after_closing_quote {
        result.push(' ');
      }
      in_quote = Some(cur);
      after_closing_quote = false;
      result.push(cur);
      prev = Some(cur);
      continue;
    }

    // Non-quote character clears the closing-quote flag
    after_closing_quote = false;

    if prev.is_some_and(|prev| needs_space(css, prev, cur, idx)) {
      result.push(' ');
    }
    result.push(cur);
    prev = Some(cur);
  }

  result
}

/// Returns `true` when a space must be inserted between `prev` and `cur`, where
/// `cur` starts at byte offset `idx` in `css`.
///
/// Only called from the "normal" region of [`normalize_spacing`] — never inside
/// a string, comment or `url()` body.
fn needs_space(css: &str, prev: char, cur: char, idx: usize) -> bool {
  match (prev, cur) {
    // After `)` before a letter: space unless followed by a CSS unit
    (')', c) if c.is_alphabetic() => {
      if !c.is_ascii_alphabetic() {
        true
      } else {
        let word_end = css[idx..]
          .find(|c: char| !c.is_ascii_alphanumeric())
          .map_or(css.len(), |offset| idx + offset);
        !is_css_unit(&css[idx..word_end])
      }
    },
    // After `)` before digit, `#`, or `(`
    (')', '0'..='9' | '#' | '(') => true,
    // After `)` before `/` or `*` (calc operators)
    (')', '/' | '*') => true,
    // Around `/` (division and CSS slash separator)
    (c, '/') if !c.is_whitespace() => true,
    ('/', c) if !c.is_whitespace() => true,
    // After alphanumeric or `%` before `#` (hex color)
    (c, '#') if c.is_alphanumeric() || c == '%' => true,
    // After `%` before a number (e.g. `40.101%.1147` → `40.101% .1147`)
    ('%', '0'..='9' | '.') => true,
    // After `*` before operand (calc context)
    ('*', '0'..='9' | '.' | '(' | '-') => true,
    _ => false,
  }
}

#[cfg(test)]
#[path = "../../tests/whitespace_normalizer_tests.rs"]
mod tests;
