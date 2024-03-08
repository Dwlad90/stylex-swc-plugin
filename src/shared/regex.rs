use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static INCLUDED_IDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"__included_\d+__").unwrap());
