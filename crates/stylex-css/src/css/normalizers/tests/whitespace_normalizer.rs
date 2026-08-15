//! Coverage of the whitespace repair pass, asserted against the pass itself.
//!
//! **Superseded.** Every input here is re-expressed at the public normalization
//! entry point in `css/tests/spacing_repair_parity_test.rs`, where each case
//! also carries the verdict the parity harness recorded against the reference
//! compiler. That module is the one to add to. This one is kept only until the
//! pass it addresses is deleted, so that both are green across the change.

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
fn css_unit_font_relative() {
  assert!(is_css_unit("ch"));
  assert!(is_css_unit("ex"));
  assert!(is_css_unit("lh"));
  assert!(is_css_unit("rlh"));
  assert!(is_css_unit("cap"));
  assert!(is_css_unit("ic"));
}

#[test]
fn css_unit_viewport_variants() {
  assert!(is_css_unit("vmin"));
  assert!(is_css_unit("vmax"));
  assert!(is_css_unit("vi"));
  assert!(is_css_unit("vb"));
  assert!(is_css_unit("dvh"));
  assert!(is_css_unit("lvw"));
  assert!(is_css_unit("lvh"));
  assert!(is_css_unit("svw"));
}

#[test]
fn css_unit_container_viewport() {
  assert!(is_css_unit("cqw"));
  assert!(is_css_unit("cqh"));
  assert!(is_css_unit("cqmin"));
  assert!(is_css_unit("cqmax"));
  assert!(is_css_unit("cqi"));
  assert!(is_css_unit("cqb"));
  assert!(is_css_unit("dvw"));
  assert!(is_css_unit("svh"));
}

#[test]
fn css_unit_angle_time() {
  assert!(is_css_unit("rad"));
  assert!(is_css_unit("grad"));
  assert!(is_css_unit("turn"));
  assert!(is_css_unit("ms"));
  assert!(is_css_unit("s"));
}

#[test]
fn css_unit_absolute_lengths() {
  assert!(is_css_unit("cm"));
  assert!(is_css_unit("mm"));
  assert!(is_css_unit("in"));
  assert!(is_css_unit("pt"));
  assert!(is_css_unit("pc"));
  assert!(is_css_unit("Q"));
}

#[test]
fn css_unit_resolution_flex_frequency() {
  assert!(is_css_unit("dpi"));
  assert!(is_css_unit("dpcm"));
  assert!(is_css_unit("dppx"));
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

#[test]
fn non_units_common_words() {
  assert!(!is_css_unit("abc"));
  assert!(!is_css_unit("hello"));
  assert!(!is_css_unit("div"));
  assert!(!is_css_unit("span"));
  assert!(!is_css_unit("p"));
  assert!(!is_css_unit("a"));
}

#[test]
fn percent_is_not_a_css_unit() {
  // `%` is intentionally not in the unit list
  assert!(!is_css_unit("%"));
}

#[test]
fn case_sensitive_units() {
  // Only exact case matches
  assert!(!is_css_unit("PX"));
  assert!(!is_css_unit("Px"));
  assert!(!is_css_unit("EM"));
  assert!(is_css_unit("Q")); // Q is uppercase by spec
  assert!(is_css_unit("Hz")); // Hz is mixed-case by spec
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

#[test]
fn extract_multiple_values() {
  assert_eq!(extract_css_value("*{margin:10px 20px}"), "10px 20px");
}

#[test]
fn extract_url_with_protocol() {
  assert_eq!(
    extract_css_value("*{background:url(http://example.com/img.png)}"),
    "url(http://example.com/img.png)"
  );
}

#[test]
fn extract_empty_value() {
  assert_eq!(extract_css_value("*{color:}"), "");
}

#[test]
fn extract_whitespace_only() {
  assert_eq!(extract_css_value("   "), "");
}

#[test]
fn extract_triple_braces() {
  assert_eq!(extract_css_value("*{{{color:green}}}"), "green");
}

#[test]
fn extract_value_with_var() {
  assert_eq!(
    extract_css_value("*{color:var(--my-color)}"),
    "var(--my-color)"
  );
}

#[test]
fn extract_value_with_calc() {
  assert_eq!(
    extract_css_value("*{width:calc(100% - 20px)}"),
    "calc(100% - 20px)"
  );
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
  assert_eq!(normalize_spacing("size/2"), "size / 2");
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

// Complex calc expression
#[test]
fn complex_calc_expression() {
  assert_eq!(normalize_spacing("calc(100% - 2px)"), "calc(100% - 2px)");
}

// `)` + `-` is not a space-triggering pair
#[test]
fn paren_before_minus_no_space() {
  assert_eq!(normalize_spacing(")-1"), ")-1");
}

// Strings inside quotes are not modified
#[test]
fn string_contents_preserved() {
  assert_eq!(normalize_spacing(r#""hello)world""#), r#""hello)world""#);
}

#[test]
fn unicode_string_contents_preserved() {
  assert_eq!(normalize_spacing(r#""•""#), r#""•""#);
}

// Multiple functions chained
#[test]
fn multiple_chained_functions() {
  assert_eq!(
    normalize_spacing("calc(1px)calc(2px)calc(3px)"),
    "calc(1px) calc(2px) calc(3px)"
  );
}

// Already normalized text stays the same
#[test]
fn already_normalized_complex() {
  let input = "var(--color) var(--bg)";
  assert_eq!(normalize_spacing(input), input);
}

// Single character input
#[test]
fn single_character_input() {
  assert_eq!(normalize_spacing("a"), "a");
  assert_eq!(normalize_spacing(")"), ")");
  assert_eq!(normalize_spacing("#"), "#");
}

// Closing paren followed by open paren in real CSS
#[test]
fn adjacent_function_calls() {
  assert_eq!(
    normalize_spacing("rgb(0,0,0)rgba(255,255,255,.5)"),
    "rgb(0,0,0) rgba(255,255,255,.5)"
  );
}

// url() fast-path with complex URL
#[test]
fn url_passthrough_complex() {
  let url = "url(https://fonts.googleapis.com/css2?family=Roboto)";
  assert_eq!(normalize_spacing(url), url);
}

#[test]
fn composite_url_passthrough() {
  let value = "linear-gradient(red,blue),url(http://example.com/a.png)";
  assert_eq!(normalize_spacing(value), value);
}

#[test]
fn url_suffix_in_non_ascii_function_name_is_not_url() {
  assert_eq!(normalize_spacing("éurl(foo/2)"), "éurl(foo / 2)");
}

// Regression: `open_paren_index - 3` is not necessarily a UTF-8 character
// boundary, so the `url(` lookbehind must not slice `&str` by raw byte offset.
#[test]
fn multibyte_identifier_before_paren_does_not_panic() {
  assert_eq!(normalize_spacing("éab(1)"), "éab(1)");
  assert_eq!(normalize_spacing("日本(1/2)"), "日本(1 / 2)");
  assert_eq!(normalize_spacing("é(1)"), "é(1)");
}

// A `/*` inside a `url()` body is part of the URL, not a comment opener.
#[test]
fn url_body_wins_over_comment_and_quote_detection() {
  for value in [
    "url(a/*b.png)",
    r#"url("a/*b.png")"#,
    "url(data:image/svg+xml;utf8,<svg/>)",
    "url(it's-fine.png)",
  ] {
    assert_eq!(normalize_spacing(value), value);
  }
}

// An unquoted `(` inside a `url()` body nests, so the *matching* `)` — not the
// first one — ends the URL. Without depth tracking the body would terminate
// early and the remainder would be re-normalized as CSS.
#[test]
fn nested_parens_inside_url_body_are_balanced() {
  assert_eq!(normalize_spacing("url(a(b).png)"), "url(a(b).png)");
  assert_eq!(
    normalize_spacing("url(a(b(c)).png) 10px"),
    "url(a(b(c)).png) 10px"
  );
  // The value after the URL is still normalized once the body has closed.
  assert_eq!(
    normalize_spacing("url(a(b).png)calc(1px)"),
    "url(a(b).png) calc(1px)"
  );
}

// The `url(` fast path used to skip normalization for the whole value, so a
// trailing component kept SWC's minified (invalid) spacing.
#[test]
fn value_following_a_url_is_still_normalized() {
  assert_eq!(
    normalize_spacing("url(a.png) no-repeat center/cover"),
    "url(a.png) no-repeat center / cover"
  );
  assert_eq!(
    normalize_spacing("url(a.png)calc(1px)"),
    "url(a.png) calc(1px)"
  );
}

// Regression: a `)` inside a *quoted* `url()` body does not close the function.
// Treating it as a closing paren re-entered the spacing rules mid-URL and both
// injected a space into the URL and swallowed the separator after it
// (`url("a)b.png") 10px` → `url("a) b.png")10px`).
#[test]
fn quoted_paren_inside_url_does_not_close_it() {
  assert_eq!(
    normalize_spacing(r#"url("a)b.png") 10px"#),
    r#"url("a)b.png") 10px"#
  );
  assert_eq!(
    normalize_spacing(r#"url('a)b.png')calc(1px)"#),
    r#"url('a)b.png') calc(1px)"#
  );
  assert_eq!(
    normalize_spacing(r#"url("a\")b.png") 10px"#),
    r#"url("a\")b.png") 10px"#
  );
}

// Regression: the first character used to be consumed before the scan loop, so
// a comment (or quote) at offset 0 was never recognised. With spaces now
// inserted on either side of `/`, that turned `/* x */` into the invalid
// `/ * x * /` and dropped the whole declaration.
#[test]
fn comment_at_start_of_value_is_preserved() {
  for value in ["/* a }b */ 1px", "/* a */", "/**/", r#""a" 1px"#] {
    assert_eq!(normalize_spacing(value), value, "value: {value}");
  }

  // A comment already separates tokens, so the inserted space is cosmetic.
  assert_eq!(normalize_spacing("/* a */1px"), "/* a */ 1px");
}

// The `*` of an opening `/*` must not also close the comment: `/*/` is still
// inside the comment, so `x` here is comment text, not a value component.
#[test]
fn slash_star_slash_does_not_close_comment() {
  assert_eq!(normalize_spacing("/*/ x */ 1px"), "/*/ x */ 1px");
  assert_eq!(normalize_spacing("1px /*/ y */"), "1px /*/ y */");
}

// Percent before dot-prefixed decimal
#[test]
fn percent_before_decimal() {
  assert_eq!(normalize_spacing("100%.5"), "100% .5");
}

// Division operator before negative number
#[test]
fn division_before_negative() {
  assert_eq!(normalize_spacing("/-2"), "/ -2");
}

// Multiply operator before parenthesized expression
#[test]
fn multiply_before_paren() {
  assert_eq!(normalize_spacing("*(100%)"), "* (100%)");
}

// No space after paren before CSS unit (comprehensive)
#[test]
fn paren_before_all_common_units() {
  assert_eq!(normalize_spacing("var(--x)px"), "var(--x)px");
  assert_eq!(normalize_spacing("var(--x)em"), "var(--x)em");
  assert_eq!(normalize_spacing("var(--x)rem"), "var(--x)rem");
  assert_eq!(normalize_spacing("var(--x)vh"), "var(--x)vh");
  assert_eq!(normalize_spacing("var(--x)vw"), "var(--x)vw");
  assert_eq!(normalize_spacing("var(--x)ch"), "var(--x)ch");
  assert_eq!(normalize_spacing("var(--x)vmin"), "var(--x)vmin");
  assert_eq!(normalize_spacing("var(--x)vmax"), "var(--x)vmax");
  assert_eq!(normalize_spacing("var(--x)fr"), "var(--x)fr");
  assert_eq!(normalize_spacing("var(--x)s"), "var(--x)s");
  assert_eq!(normalize_spacing("var(--x)turn"), "var(--x)turn");
}

// Extractor: ensure escaped characters inside quoted values are handled and do
// not prematurely terminate the extraction loop. These cases exercise the
// escaped character handling and quote toggling branches in extract_css_value.
#[test]
fn extract_css_value_handles_escaped_quotes() {
  // Single-quoted value with an escaped single quote inside.
  assert_eq!(extract_css_value(r#"*{content:'a\'b'}"#), r#"'a\'b'"#);

  // Double-quoted value with an escaped backslash sequence inside.
  assert_eq!(extract_css_value(r#"*{content:"a\\b"}"#), r#""a\\b""#);
}
