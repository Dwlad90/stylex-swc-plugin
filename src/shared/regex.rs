use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static INCLUDED_IDENT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"__included_\d+__").unwrap());

pub(crate) static CSS_RULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r".*:(.*?)}").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_MULTIPLYER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\*)(\S)").unwrap());

pub(crate) static WHITESPACE_NORMALIZER_BRACE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\))(\S)").unwrap());

pub(crate) static DASHIFY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(^|[a-z])([A-Z])").unwrap());
