use crate::shared::regex::{
    CSS_RULE_REGEX, WHITESPACE_NORMALIZER_BRACE_REGEX, WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX,
    WHITESPACE_NORMALIZER_SPACES_REGEX,
};

pub(crate) fn whitespace_normalizer(result: String) -> String {
    let css_string = CSS_RULE_REGEX
        .captures(result.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let normalized_css_string =
        WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX.replace_all(css_string, " $1 $2");

    let normalized_css_string =
        WHITESPACE_NORMALIZER_BRACE_REGEX.replace_all(&normalized_css_string, "$1 $2");

    let normalized_css_string = WHITESPACE_NORMALIZER_SPACES_REGEX
        .replace_all(&normalized_css_string, "$1$2")
        .to_string();

    normalized_css_string
}
