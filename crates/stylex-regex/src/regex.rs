use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub static SANITIZE_CLASS_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[^.a-zA-Z0-9_-]").expect("Sanitize class name regex is valid"));

// Note: This matches `)` followed by a non-space char, or adjacent quotes.
// Used by flatten_raw_style_object to detect CSS values that need splitting.
pub static CSS_PROPERTY_KEY: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).expect("CSS property key regex is valid"));

// CSS dimension units including modern viewport and container units
pub static LENGTH_UNIT_TESTER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^-?\d+(?:px|%|em|rem|ex|ch|vh|vw|vmin|vmax|dvh|dvw|lvh|lvw|svh|svw|cqw|cqh|cqi|cqb|cqmin|cqmax)?$")
    .expect("Length unit tester regex is valid")
});

pub static CLEAN_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\\3(\d) "#).expect("Clean CSS var regex is valid"));

// Updated: More permissive CSS variable name matching (allows dots, underscores, etc.)
pub static IS_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"^var\(--[\w-]+\)$"#).expect("Is CSS var regex is valid"));

pub static MANY_SPACES: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\s+"#).expect("Many spaces regex is valid"));

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
