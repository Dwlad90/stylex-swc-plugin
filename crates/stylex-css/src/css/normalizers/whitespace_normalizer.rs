use logos::Logos;

/// CSS value tokens — minimal set needed for spacing decisions.
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Tok {
  /// Identifiers, custom properties, numbers, decimals
  #[regex(r"--[a-zA-Z0-9_][a-zA-Z0-9_-]*")]
  #[regex(r"-?[a-zA-Z_][a-zA-Z0-9_-]*")]
  #[regex(r"[0-9]+(\.[0-9]+)?")]
  #[regex(r"\.[0-9]+")]
  Word,
  /// Quoted string: `"hello"`, `""`
  #[regex(r#""[^"]*""#)]
  Str,
  #[token("(")]
  LP,
  #[token(")")]
  RP,
  #[token(",")]
  Comma,
  #[token("#")]
  Hash,
  #[token("%")]
  Pct,
  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("*")]
  Star,
  #[token("/")]
  Slash,
  #[regex(r"\s+")]
  Ws,
}

/// Returns true if `s` is a CSS dimension unit that may follow `)` without a
/// space — e.g. `var(--x)px` should remain unsplit.
fn is_css_unit(s: &str) -> bool {
  matches!(
    s,
    "px" | "em" | "rem" | "ex" | "ch" | "lh" | "rlh" | "cap" | "ic"
      | "cm" | "mm" | "in" | "pt" | "pc"
      | "vw" | "vh" | "vi" | "vb" | "vmin" | "vmax"
      | "dvw" | "dvh" | "lvw" | "lvh" | "svw" | "svh"
      | "cqw" | "cqh" | "cqi" | "cqb" | "cqmin" | "cqmax"
      | "ms" | "s" | "deg" | "rad" | "grad" | "turn"
      | "dpi" | "dpcm" | "dppx" | "fr" | "Hz" | "kHz" | "Q"
  )
}

/// Decides whether to emit a space between two adjacent non-whitespace tokens.
fn spacing(prev: Tok, cur: Tok, cur_text: &str, had_ws: bool) -> bool {
  use Tok::*;
  match (prev, cur) {
    // Suppress: after `(`, before `)`, before/after `,`
    (LP, _) | (_, RP) | (_, Comma) | (Comma, _) => false,
    // FIX(#927): no space before CSS unit after `)`
    (RP, Word) if is_css_unit(cur_text) => false,
    // Space after `)` before other tokens
    (RP, _) => true,
    // Space between adjacent quoted strings (empty-quote collapse handled separately)
    (Str, Str) => true,
    // Space before `#` when preceded by a value
    (Word | Pct | Str, Hash) => true,
    // `%` attaches to left number, no space
    (Word, Pct) => false,
    // Space after `%` before operand
    (Pct, Word | LP) => true,
    // Space around math operators (`+`, `*`, `/`)
    (Word | Pct, Plus | Star | Slash) => true,
    (Plus | Star | Slash, Word | LP) => true,
    // Space before `-` when acting as an operator
    (Word | Pct, Minus) => true,
    // After `-`: preserve original spacing (keeps negative numbers: `-3` vs `- 3`)
    (Minus, Word | LP) => had_ws,
    // Default: preserve original spacing
    _ => had_ws,
  }
}

/// Extract CSS property value from a rule wrapper like `*{prop:value}`.
fn extract_value(css: &str) -> Option<&str> {
  let brace = css.find('{')?;
  let mut start = brace + 1;
  while css.as_bytes().get(start) == Some(&b'{') {
    start += 1;
  }
  let colon = css[start..].find(':')? + start;
  let val_start = css[colon + 1..]
    .find(|c: char| c != ' ')
    .map_or(colon + 1, |p| colon + 1 + p);
  let end = css[val_start..]
    .find(['}', ';'])
    .map_or(css.len(), |p| val_start + p);
  Some(&css[val_start..end])
}

pub fn whitespace_normalizer(content: String) -> String {
  let raw = content.as_str();

  // Fast-path: return URL values as-is
  if let Some(start) = raw.find("url(")
    && let Some(end) = raw[start..].find(')')
  {
    return raw[start..start + end + 1].to_string();
  }

  // Extract value from CSS rule wrapper
  let css = if raw.contains('{') {
    extract_value(raw).unwrap_or(raw)
  } else {
    raw
  };

  // Phase 1 — tokenize, recording source text and preceding whitespace
  let mut toks: Vec<(Tok, &str, bool)> = Vec::new();
  let mut had_ws = false;

  for (result, span) in Tok::lexer(css).spanned() {
    let text = &css[span];
    match result {
      Ok(Tok::Ws) => had_ws = true,
      Ok(kind) => {
        toks.push((kind, text, had_ws));
        had_ws = false;
      }
      Err(()) => {
        toks.push((Tok::Word, text, had_ws));
        had_ws = false;
      }
    }
  }

  // Phase 2 — rebuild with normalised spacing
  let mut out = String::with_capacity(css.len() + 16);
  let mut skip = false;

  for i in 0..toks.len() {
    if skip {
      skip = false;
      continue;
    }

    let (kind, text, had_ws) = toks[i];

    // Collapse adjacent empty quotes: `""` `""` → `""`
    if kind == Tok::Str
      && text == "\"\""
      && toks
        .get(i + 1)
        .is_some_and(|n| n.0 == Tok::Str && n.1 == "\"\"")
    {
      out.push_str("\"\"");
      skip = true;
      continue;
    }

    if i > 0 {
      let (pk, _, _) = toks[i - 1];
      if spacing(pk, kind, text, had_ws) {
        out.push(' ');
      }
    }

    out.push_str(text);
  }

  let trimmed = out.trim();
  if trimmed.len() == out.len() {
    out
  } else {
    trimmed.to_string()
  }
}
