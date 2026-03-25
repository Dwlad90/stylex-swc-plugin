use fancy_regex::Regex;
use once_cell::sync::Lazy;

// Extracts CSS property value from a rule (e.g., "color: red;" -> "red")
pub static CSS_RULE_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\w+:\s*([^;}]+);?").expect("CSS rule regex is valid"));

// Normalizes whitespace around math operators in CSS calc() and similar functions
// Uses named groups for better readability and maintenance
pub static WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r"(?x)
    (?<left>\d+%?|\))       # Left operand: number with optional %, or closing paren
    (?<lspace>\s*)          # Optional whitespace before operator
    (?<op>[+\-*/%])         # Operator
    (?<rspace>\s*)          # Optional whitespace after operator
    (?<right>\d*\.?\d+|\()  # Right operand: number (with optional decimal), or opening paren
  ",
  )
  .expect("Whitespace normalizer math signs regex is valid")
});

pub static SANITIZE_CLASS_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[^.a-zA-Z0-9_-]").expect("Sanitize class name regex is valid"));

// Normalizes spacing around brackets and quotes: ")a" -> ") a", """" -> ""
pub static WHITESPACE_BRACKET_NORMALIZER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r#"(\))(\S)|(\")(\")"#).expect("Whitespace bracket normalizer regex is valid")
});

// Normalizes function arguments: "( arg )" -> "(arg)"
// Uses possessive quantifiers for better performance
pub static WHITESPACE_FUNC_NORMALIZER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r#"\(\s*+([^)]*?)\s*+\)\s*+,\s*+"#)
    .expect("Whitespace function normalizer regex is valid")
});

// Adds space before hash in color values: "a#fff" -> "a #fff"
pub static HASH_WHITESPACE_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\S)#").expect("Hash whitespace normalizer regex is valid"));

// Updated to include modern CSS units: dvh, dvw, lvh, lvw, svh, svw, cqw, cqh, cqi, cqb, cqmin, cqmax
pub static LENGTH_UNIT_TESTER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^-?\d+(?:px|%|em|rem|ex|ch|vh|vw|vmin|vmax|dvh|dvw|lvh|lvw|svh|svw|cqw|cqh|cqi|cqb|cqmin|cqmax)?$")
    .expect("Length unit tester regex is valid")
});

// Improved: Non-greedy matching to avoid over-matching
pub static CSS_URL_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"url\([^)]+\)"#).expect("CSS URL regex is valid"));

// Note: This is identical to WHITESPACE_BRACKET_NORMALIZER_REGEX - using the same pattern
pub static CSS_PROPERTY_KEY: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).expect("CSS property key regex is valid"));

pub static CLEAN_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\\3(\d) "#).expect("Clean CSS var regex is valid"));

// Updated: More permissive CSS variable name matching (allows dots, underscores, etc.)
pub static IS_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"^var\(--[\w-]+\)$"#).expect("Is CSS var regex is valid"));

pub static MANY_SPACES: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\s+"#).expect("Many spaces regex is valid"));

pub static WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r#"(?x)
      # Empty string quotes
      ^(\")\s(\")$ |
      # Multiple closing parens
      (\))\s+(\))
  "#,
  )
  .expect("Whitespace normalizer extra spaces regex is valid")
});

// Improved: Using positive lookbehind to avoid capturing the prefix
// This simplifies replacement from "$1-$2" to "-$1"
pub static DASHIFY_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(?<=^|[a-z])([A-Z])").expect("Dashify regex is valid"));

pub static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
        r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
    ).expect("URL regex is valid")
});

pub static JSON_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\{|,)\s*([a-zA-Z0-9_$*-]+)\s*:"#).expect("JSON regex is valid"));

pub static NPM_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*$")
    .expect("NPM name regex is valid")
});

// #region Relational selectors for .when() functions
pub static ANCESTOR_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+\*\)$")
    .expect("Ancestor selector regex is valid")
});

pub static DESCENDANT_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(:has\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$")
    .expect("Descendant selector regex is valid")
});

pub static SIBLING_BEFORE_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+~\s+\*\)$")
    .expect("Sibling before selector regex is valid")
});

pub static SIBLING_AFTER_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(:has\(~\s\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$")
    .expect("Sibling after selector regex is valid")
});

pub static ANY_SIBLING_SELECTOR: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^:where\(\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\s+~\s+\*,\s+:has\(~\s\.[0-9a-zA-Z_-]+(:[a-zA-Z-]+)\)\)$")
    .expect("Any sibling selector regex is valid")
});
// #endregion Relational selectors for .when() functions

// Matches pseudo-elements (::after) and pseudo-classes (:hover, :nth-child(2))
pub static PSEUDO_PART_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"::[a-zA-Z-]+|:[a-zA-Z-]+(?:\([^)]*\))?").expect("Pseudo part regex is valid")
});

// Matches .stylex or .consts file imports with optional extensions (.ts, .js, .tsx, .jsx)
pub static STYLEX_CONSTS_IMPORT_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"\.(stylex|consts)(?:\.(.+){2,6})?$").expect("StyleX consts import regex is valid")
});

pub static VAR_EXTRACTION_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"var\((--x-[^,)]+)[^)]*\)").expect("Var extraction regex is valid"));
