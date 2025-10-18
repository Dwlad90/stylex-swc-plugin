use fancy_regex::Regex;
use once_cell::sync::Lazy;

// Extracts CSS property value from a rule (e.g., "color: red;" -> "red")
pub(crate) static CSS_RULE_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\w+:\s*([^;}]+);?").unwrap());

// Normalizes whitespace around math operators in CSS calc() and similar functions
// Uses named groups for better readability and maintenance
pub(crate) static WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r"(?x)
    (?<left>\d+%?|\))       # Left operand: number with optional %, or closing paren
    (?<lspace>\s*)          # Optional whitespace before operator
    (?<op>[+\-*/%])         # Operator
    (?<rspace>\s*)          # Optional whitespace after operator
    (?<right>\d*\.?\d+|\()  # Right operand: number (with optional decimal), or opening paren
  ",
  )
  .unwrap()
});

pub(crate) static SANITIZE_CLASS_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[^.a-zA-Z0-9_-]").unwrap());

// Normalizes spacing around brackets and quotes: ")a" -> ") a", """" -> ""
pub(crate) static WHITESPACE_BRACKET_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).unwrap());

// Normalizes function arguments: "( arg )" -> "(arg)"
// Uses possessive quantifiers for better performance
pub(crate) static WHITESPACE_FUNC_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\(\s*+([^)]*?)\s*+\)\s*+,\s*+"#).unwrap());

// Adds space before hash in color values: "a#fff" -> "a #fff"
pub(crate) static HASH_WHITESPACE_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\S)#").unwrap());

// Updated to include modern CSS units: dvh, dvw, lvh, lvw, svh, svw, cqw, cqh, cqi, cqb, cqmin, cqmax
pub(crate) static LENGTH_UNIT_TESTER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^-?\d+(?:px|%|em|rem|ex|ch|vh|vw|vmin|vmax|dvh|dvw|lvh|lvw|svh|svw|cqw|cqh|cqi|cqb|cqmin|cqmax)?$")
    .unwrap()
});

// Improved: Non-greedy matching to avoid over-matching
pub(crate) static CSS_URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"url\([^)]+\)"#).unwrap());

// Note: This is identical to WHITESPACE_BRACKET_NORMALIZER_REGEX - using the same pattern
pub(crate) static CSS_PROPERTY_KEY: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).unwrap());

pub(crate) static CLEAN_CSS_VAR: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\3(\d) "#).unwrap());

// Updated: More permissive CSS variable name matching (allows dots, underscores, etc.)
pub(crate) static IS_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"^var\(--[\w-]+\)$"#).unwrap());

pub(crate) static MANY_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\s+"#).unwrap());

pub(crate) static WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r#"(?x)
      # Empty string quotes
      ^(\")\s(\")$ |
      # Multiple closing parens
      (\))\s+(\))
  "#,
  )
  .unwrap()
});

// Improved: Using positive lookbehind to avoid capturing the prefix
// This simplifies replacement from "$1-$2" to "-$1"
pub(crate) static DASHIFY_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(?<=^|[a-z])([A-Z])").unwrap());

pub(crate) static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
        r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
    ).unwrap()
});

pub(crate) static JSON_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\{|,)\s*([a-zA-Z0-9_$*-]+)\s*:"#).unwrap());

pub static NPM_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*$").unwrap());

// #region Relational selectors for .when() functions
pub(crate) static ANCESTOR_SELECTOR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+\*\)$").unwrap());

pub(crate) static DESCENDANT_SELECTOR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^:where\(:has\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$").unwrap());

pub(crate) static SIBLING_BEFORE_SELECTOR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+~\s+\*\)$").unwrap());

pub(crate) static SIBLING_AFTER_SELECTOR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^:where\(:has\(~\s\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$").unwrap());

pub(crate) static ANY_SIBLING_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+~\s+\*,\s+:has\(~\s\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$")
    .unwrap()
});
// #endregion Relational selectors for .when() functions
