use crate::css::normalizers::whitespace_normalizer::{
  extract_css_value, is_css_unit, normalize_spacing,
};

// ── is_css_unit ──────────────────────────────────────────────────────

#[test]
fn css_unit_common() {
  assert!(is_css_unit("px"));
  assert!(is_css_unit("em"));
  assert!(is_css_unit("rem"));
  assert!(is_css_unit("vh"));
  assert!(is_css_unit("vw"));
  assert!(is_css_unit("ms"));
  assert!(is_css_unit("s"));
  assert!(is_css_unit("deg"));
  assert!(is_css_unit("fr"));
  assert!(is_css_unit("Hz"));
  assert!(is_css_unit("kHz"));
}

#[test]
fn css_unit_container_viewport() {
  assert!(is_css_unit("cqw"));
  assert!(is_css_unit("cqh"));
  assert!(is_css_unit("cqmin"));
  assert!(is_css_unit("dvw"));
  assert!(is_css_unit("svh"));
}

#[test]
fn non_units_rejected() {
  assert!(!is_css_unit("var"));
  assert!(!is_css_unit("calc"));
  assert!(!is_css_unit("rotate"));
  assert!(!is_css_unit("translate3d"));
  assert!(!is_css_unit("solid"));
  assert!(!is_css_unit("auto"));
  assert!(!is_css_unit(""));
}

// ── extract_css_value ────────────────────────────────────────────────

#[test]
fn extract_simple_rule() {
  assert_eq!(extract_css_value("*{color:red}"), "red");
}

#[test]
fn extract_double_braces() {
  assert_eq!(extract_css_value("*{{color:blue}}"), "blue");
}

#[test]
fn extract_with_spaces() {
  assert_eq!(
    extract_css_value("*{ color: 1px solid #000 }"),
    "1px solid #000"
  );
}

#[test]
fn extract_url() {
  assert_eq!(
    extract_css_value("*{background:url(image.png)}"),
    "url(image.png)"
  );
}

#[test]
fn extract_no_braces() {
  assert_eq!(extract_css_value("color:red"), "red");
}

#[test]
fn extract_no_colon() {
  assert_eq!(extract_css_value("just-text"), "just-text");
}

#[test]
fn extract_semicolon_terminated() {
  assert_eq!(extract_css_value("*{color:red; margin:0}"), "red");
}

// ── normalize_spacing ────────────────────────────────────────────────

// Issue #927: `)` + CSS unit should NOT get a space
#[test]
fn paren_before_css_unit_no_space() {
  assert_eq!(normalize_spacing("var(--x)px"), "var(--x)px");
  assert_eq!(normalize_spacing("var(--gap)rem"), "var(--gap)rem");
  assert_eq!(normalize_spacing("calc(1+2)em"), "calc(1+2)em");
  assert_eq!(normalize_spacing("var(--d)ms"), "var(--d)ms");
  assert_eq!(normalize_spacing("var(--a)deg"), "var(--a)deg");
  assert_eq!(normalize_spacing("var(--h)vh"), "var(--h)vh");
}

// `)` + function name gets a space
#[test]
fn paren_before_function() {
  assert_eq!(
    normalize_spacing("rotate(10deg)translate3d(0,0,0)"),
    "rotate(10deg) translate3d(0,0,0)"
  );
  assert_eq!(
    normalize_spacing("var(--a)var(--b)var(--c)"),
    "var(--a) var(--b) var(--c)"
  );
}

// `)` + digit
#[test]
fn paren_before_digit() {
  assert_eq!(normalize_spacing(")3"), ") 3");
  assert_eq!(normalize_spacing("calc(a)42"), "calc(a) 42");
}

// `)` + `#` (hex color)
#[test]
fn paren_before_hash() {
  assert_eq!(normalize_spacing(")#fff"), ") #fff");
}

// `)` + `(` (adjacent functions)
#[test]
fn paren_before_open_paren() {
  assert_eq!(normalize_spacing(")("), ") (");
}

// `)` + `/` or `*` (calc operators)
#[test]
fn paren_before_calc_operators() {
  // Both `)→/` and `/→7` insert spaces
  assert_eq!(normalize_spacing(")/7"), ") / 7");
  assert_eq!(normalize_spacing(")*3"), ") * 3");
}

// alphanumeric/`%` + `#`
#[test]
fn value_before_hash() {
  assert_eq!(normalize_spacing("1px#000"), "1px #000");
  assert_eq!(normalize_spacing("solid#abc"), "solid #abc");
  assert_eq!(normalize_spacing("50%#fff"), "50% #fff");
}

// `%` + number
#[test]
fn percent_before_number() {
  assert_eq!(normalize_spacing("40%.1147"), "40% .1147");
  assert_eq!(normalize_spacing("50%10"), "50% 10");
}

// `/` or `*` + operand
#[test]
fn calc_operators_before_operand() {
  assert_eq!(normalize_spacing("/ 7"), "/ 7");
  assert_eq!(normalize_spacing("* 3"), "* 3");
  assert_eq!(normalize_spacing("/.5"), "/ .5");
  assert_eq!(normalize_spacing("*("), "* (");
  assert_eq!(normalize_spacing("/-1"), "/ -1");
}

// Adjacent quoted strings
#[test]
fn adjacent_strings_get_space() {
  assert_eq!(
    normalize_spacing(r#""content""sidebar""#),
    r#""content" "sidebar""#
  );
}

// Empty string stays as-is
#[test]
fn empty_string_no_space() {
  assert_eq!(normalize_spacing(r#""""#), r#""""#);
}

// Two empty strings (3 pairs of `""` → space between each)
#[test]
fn two_empty_strings_get_space() {
  assert_eq!(normalize_spacing(r#""""""""#), r#""" "" """#);
}

// URL fast-path
#[test]
fn url_passthrough() {
  let url = "url(image.png)";
  assert_eq!(normalize_spacing(url), url);
}

// Empty input
#[test]
fn empty_input() {
  assert_eq!(normalize_spacing(""), "");
}

// No modifications needed
#[test]
fn no_changes_needed() {
  assert_eq!(normalize_spacing("1px solid red"), "1px solid red");
  assert_eq!(normalize_spacing("calc(100% - 20px)"), "calc(100% - 20px)");
}

// Compound expressions
#[test]
fn compound_expressions() {
  assert_eq!(
    normalize_spacing("oklab(40.101%.1147 .0453)"),
    "oklab(40.101% .1147 .0453)"
  );
}
