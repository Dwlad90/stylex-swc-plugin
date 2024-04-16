use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static INCLUDED_IDENT_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"__included_\d+__").unwrap());

pub(crate) static CSS_RULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r".*:(.*?)}").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"([\*\/])(\S)").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_SPACES_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\))\s+(\))").unwrap());

pub(crate) static DASHIFY_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(^|[a-z])([A-Z])").unwrap());

pub(crate) static SANITIZE_CLASS_NAME_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[^.a-zA-Z0-9_-]").unwrap());

pub(crate) static IDENT_PROP_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^[a-zA-Z\d$_]*$").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"(\))(\S)|(\")(\")"#).unwrap());

pub(crate) static HASH_WHITESPACE_NORMALIZER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(\S)#").unwrap());

pub(crate) static LENGTH_UNIT_TESTER_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^-?\d+(px|%|em|rem|ex|ch|vh|vw|vmin|vmax)?$").unwrap());
