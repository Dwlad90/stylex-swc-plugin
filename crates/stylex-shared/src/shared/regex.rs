use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static INCLUDED_IDENT_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"__included_\d+__").unwrap());

pub(crate) static CSS_RULE_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\w+:\s*([^;}]+);?").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\d+%?|\))\s*([+\-*/%])\s*(\d*\.?\d+|\()").unwrap());

pub(crate) static SANITIZE_CLASS_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[^.a-zA-Z0-9_-]").unwrap());

pub(crate) static WHITESPACE_BRACKET_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).unwrap());

pub(crate) static WHITESPACE_FUNC_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\(\s*([^)]*?)\s*\)\s*,\s*"#).unwrap());

pub(crate) static HASH_WHITESPACE_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\S)#").unwrap());

pub(crate) static LENGTH_UNIT_TESTER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^-?\d+(px|%|em|rem|ex|ch|vh|vw|vmin|vmax)?$").unwrap());

pub(crate) static CSS_URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"url\(.+\)"#).unwrap());

pub(crate) static CSS_PROPERTY_KEY: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).unwrap());

pub(crate) static CLEAN_CSS_VAR: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\3(\d) "#).unwrap());

pub(crate) static IS_CSS_VAR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"^var\(--[a-zA-Z0-9-_]+\)$"#).unwrap());

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

pub(crate) static DASHIFY_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(^|[a-z])([A-Z])").unwrap());
