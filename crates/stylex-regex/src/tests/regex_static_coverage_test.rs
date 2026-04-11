//! Coverage-oriented tests for shared static regexes.
//! Each test validates one semantic group and forces Lazy<Regex> initialization.

use crate::regex::{
  ANCESTOR_SELECTOR, ANY_SIBLING_SELECTOR, CLEAN_CSS_VAR, CSS_VALUE_SPLIT_REGEX, DASHIFY_REGEX,
  DESCENDANT_SELECTOR, IS_CSS_VAR, JSON_REGEX, LENGTH_UNIT_TESTER_REGEX, MANY_SPACES,
  PSEUDO_PART_REGEX, SIBLING_AFTER_SELECTOR, SIBLING_BEFORE_SELECTOR, STYLEX_CONSTS_IMPORT_REGEX,
  URL_REGEX, VAR_EXTRACTION_REGEX,
};

/// Value-splitting parsers should detect adjacency patterns.
#[test]
fn css_value_parsers_match_expected_tokens() {
  assert!(CSS_VALUE_SPLIT_REGEX.is_match(")a").unwrap());
  assert!(CSS_VALUE_SPLIT_REGEX.is_match("\"\"").unwrap());
  assert!(LENGTH_UNIT_TESTER_REGEX.is_match("12px").unwrap());
  assert!(LENGTH_UNIT_TESTER_REGEX.is_match("-5").unwrap());
}

/// Core token cleanups should match escaped vars, css var calls, and spacing cases.
#[test]
fn core_cleanup_patterns_match() {
  assert!(CLEAN_CSS_VAR.is_match(r"\31 ").unwrap());
  assert!(IS_CSS_VAR.is_match("var(--token_1)").unwrap());
  assert!(!IS_CSS_VAR.is_match("var(token)").unwrap());
  assert!(MANY_SPACES.is_match("a   b").unwrap());
  assert!(DASHIFY_REGEX.is_match("fooBar").unwrap());
}

/// URL and JSON helper patterns should match their canonical content.
#[test]
fn url_and_json_patterns_match() {
  assert!(URL_REGEX.is_match("https://example.com/path?q=1").unwrap());
  assert!(JSON_REGEX.is_match("{ key: 1 }").unwrap());
}

/// Relational selector patterns should accept normalized `:where(...)` forms.
#[test]
fn relational_selector_patterns_match() {
  assert!(ANCESTOR_SELECTOR.is_match(":where(.foo:hover *)").unwrap());
  assert!(
    DESCENDANT_SELECTOR
      .is_match(":where(:has(.foo:hover))")
      .unwrap()
  );
  assert!(
    SIBLING_BEFORE_SELECTOR
      .is_match(":where(.foo:hover ~ *)")
      .unwrap()
  );
  assert!(
    SIBLING_AFTER_SELECTOR
      .is_match(":where(:has(~ .foo:hover))")
      .unwrap()
  );
  assert!(
    ANY_SIBLING_SELECTOR
      .is_match(":where(.foo:hover ~ *, :has(~ .foo:hover))")
      .unwrap()
  );
}

/// Misc parser helpers should match pseudo parts, stylex imports, and vars.
#[test]
fn pseudo_import_and_var_patterns_match() {
  assert!(PSEUDO_PART_REGEX.is_match("::after").unwrap());
  assert!(PSEUDO_PART_REGEX.is_match(":nth-child(2)").unwrap());
  assert!(
    STYLEX_CONSTS_IMPORT_REGEX
      .is_match("tokens.stylex.ts")
      .unwrap()
  );
  assert!(
    STYLEX_CONSTS_IMPORT_REGEX
      .is_match("colors.consts.js")
      .unwrap()
  );

  let captures = VAR_EXTRACTION_REGEX.captures("var(--x-abc, 10px)").unwrap();
  assert_eq!(
    captures.and_then(|c| c.get(1)).map(|m| m.as_str()),
    Some("--x-abc")
  );
}
