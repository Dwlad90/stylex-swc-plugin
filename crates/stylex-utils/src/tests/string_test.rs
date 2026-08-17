#[cfg(test)]
mod dashify_tests {
  use crate::string::dashify;

  #[test]
  fn converts_simple_camel_case() {
    assert_eq!(dashify("marginTop"), "margin-top");
  }

  #[test]
  fn converts_multiple_uppercase_letters() {
    assert_eq!(
      dashify("borderBottomLeftRadius"),
      "border-bottom-left-radius"
    );
  }

  #[test]
  fn handles_already_lowercase() {
    assert_eq!(dashify("margin"), "margin");
  }

  #[test]
  fn handles_single_char() {
    assert_eq!(dashify("a"), "a");
  }

  #[test]
  fn handles_empty_string() {
    assert_eq!(dashify(""), "");
  }

  #[test]
  fn handles_vendor_prefix_webkit() {
    assert_eq!(
      dashify("WebkitTapHighlightColor"),
      "-webkit-tap-highlight-color"
    );
  }

  #[test]
  fn handles_vendor_prefix_ms() {
    assert_eq!(dashify("MsTransition"), "-ms-transition");
  }

  #[test]
  fn handles_vendor_prefix_moz() {
    assert_eq!(dashify("MozTransition"), "-moz-transition");
  }

  #[test]
  fn handles_consecutive_uppercase() {
    // CSS-like property with consecutive uppercase e.g. "fontSize" not "fontSIZE"
    assert_eq!(dashify("fontSize"), "font-size");
  }

  #[test]
  fn preserves_all_lowercase() {
    assert_eq!(dashify("opacity"), "opacity");
  }

  /// The lowercasing is unconditional, so it must reach scalars that the
  /// `[A-Z]` hyphenation pass does not match. Expectations produced by running
  /// `str.replace(/(^|[a-z])([A-Z])/g, '$1-$2').toLowerCase()`.
  #[test]
  fn lowercases_non_ascii_without_hyphenating() {
    // Titlecase: neither uppercase nor lowercase, but lowercases to `ǆ`.
    assert_eq!(dashify("ǅ"), "ǆ");
    assert_eq!(dashify("ǅBar"), "ǆbar");
    // Uppercase outside ASCII lowercases without gaining a hyphen, because
    // `[A-Z]` does not match it.
    assert_eq!(dashify("Ä"), "ä");
    // Already-lowercase non-ASCII is unchanged.
    assert_eq!(dashify("é"), "é");
    assert_eq!(dashify("naïveColor"), "naïve-color");
  }

  #[test]
  fn handles_single_uppercase() {
    assert_eq!(dashify("A"), "-a");
  }
}

#[cfg(test)]
mod remove_quotes_tests {
  use crate::string::remove_quotes;

  #[test]
  fn removes_surrounding_double_quotes() {
    assert_eq!(remove_quotes("\"hello\""), "hello");
  }

  #[test]
  fn no_quotes_returns_as_is() {
    assert_eq!(remove_quotes("hello"), "hello");
  }

  #[test]
  fn removes_only_surrounding_quotes() {
    assert_eq!(remove_quotes("\"he\"llo\""), "he\"llo");
  }

  #[test]
  fn handles_empty_string() {
    assert_eq!(remove_quotes(""), "");
  }

  #[test]
  fn handles_only_quotes() {
    assert_eq!(remove_quotes("\"\""), "");
  }
}

#[cfg(test)]
mod wrap_key_in_quotes_tests {
  use crate::string::wrap_key_in_quotes;

  #[test]
  fn wraps_when_flag_is_true() {
    assert_eq!(wrap_key_in_quotes("color", true), "\"color\"");
  }

  #[test]
  fn no_wrap_when_flag_is_false() {
    assert_eq!(wrap_key_in_quotes("color", false), "color");
  }

  #[test]
  fn wraps_empty_string() {
    assert_eq!(wrap_key_in_quotes("", true), "\"\"");
  }
}

#[cfg(test)]
mod char_code_at_tests {
  use crate::string::char_code_at;

  #[test]
  fn returns_code_unit_at_index() {
    assert_eq!(char_code_at("abc", 0), Some(97)); // 'a'
    assert_eq!(char_code_at("abc", 1), Some(98)); // 'b'
    assert_eq!(char_code_at("abc", 2), Some(99)); // 'c'
  }

  #[test]
  fn returns_none_for_out_of_bounds() {
    assert_eq!(char_code_at("abc", 3), None);
    assert_eq!(char_code_at("", 0), None);
  }

  #[test]
  fn handles_unicode() {
    assert_eq!(char_code_at("é", 0), Some(0xe9));
    // `"日本語".charCodeAt(i)` — one code unit per scalar in the BMP.
    assert_eq!(char_code_at("日本語", 0), Some(26085));
    assert_eq!(char_code_at("日本語", 1), Some(26412));
    assert_eq!(char_code_at("日本語", 2), Some(35486));
  }

  #[test]
  fn indexes_astral_scalars_by_code_unit() {
    // `"🎉".length === 2`, and the two indices read back as the surrogate
    // halves rather than the `0x1F389` scalar.
    assert_eq!(char_code_at("🎉", 0), Some(55356)); // 0xD83C
    assert_eq!(char_code_at("🎉", 1), Some(57225)); // 0xDF89
    assert_eq!(char_code_at("🎉", 2), None);
  }

  /// `charCodeAt` coerces its argument with `ToIntegerOrInfinity`, which a bare
  /// `as usize` does not reproduce: the cast saturates, so `-1.0` would land on
  /// index 0.
  #[test]
  fn coerces_a_numeric_index_like_to_integer_or_infinity() {
    use crate::string::char_code_at_f64;

    // `NaN` coerces to 0, so `"abc".charCodeAt(NaN) === 97`.
    assert_eq!(char_code_at_f64("abc", f64::NAN), Some(97));
    // Fractional indices truncate toward zero.
    assert_eq!(char_code_at_f64("abc", 1.9), Some(98));
    assert_eq!(char_code_at_f64("abc", 0.0), Some(97));
    // Negative and infinite indices are out of range — `NaN` in JS — rather
    // than index 0 or a saturated `usize`.
    assert_eq!(char_code_at_f64("abc", -1.0), None);
    assert_eq!(char_code_at_f64("abc", -0.5), None);
    assert_eq!(char_code_at_f64("abc", f64::NEG_INFINITY), None);
    assert_eq!(char_code_at_f64("abc", f64::INFINITY), None);
    assert_eq!(char_code_at_f64("abc", 3.0), None);
  }

  #[test]
  fn astral_scalars_shift_following_indices() {
    // `"a🎉b".charCodeAt(3) === 98` — the surrogate pair consumes indices 1
    // and 2, so `'b'` lands at 3, not at 2.
    assert_eq!(char_code_at("a🎉b", 0), Some(97));
    assert_eq!(char_code_at("a🎉b", 3), Some(98));
  }
}

#[cfg(test)]
mod is_blank_css_text_tests {
  use crate::string::is_blank_css_text;

  #[test]
  fn recognises_text_that_spells_nothing() {
    assert!(is_blank_css_text(""));
    assert!(is_blank_css_text(" "));
    assert!(is_blank_css_text("  \t\n "));
  }

  #[test]
  fn keeps_text_that_spells_a_value() {
    assert!(!is_blank_css_text("0"));
    assert!(!is_blank_css_text("red"));
    // Surrounding whitespace does not make a value blank.
    assert!(!is_blank_css_text(" red "));
    // Empty *quotes* are CSS text, which is what a blank `content` becomes.
    assert!(!is_blank_css_text("\"\""));
  }

  #[test]
  fn unicode_whitespace_is_a_value() {
    // The value scanner reads whitespace as "char code <= 32" and nothing
    // else, so each of these is a *word token* to it, not a gap. The
    // reference compiler emits them: `transformValue("color", "\u{3000}")`
    // produces the declaration rather than dropping the property.
    assert!(!is_blank_css_text("\u{3000}")); // ideographic space
    assert!(!is_blank_css_text("\u{00a0}")); // no-break space
    assert!(!is_blank_css_text("\u{2028}")); // line separator
    // Mixed with scanner whitespace, the word token still carries the value.
    assert!(!is_blank_css_text(" \u{00a0} "));
  }

  /// The other side of the same divergence from `str::trim`, and the side that
  /// changed an answer rather than fixing one.
  ///
  /// A C0 control is not Unicode whitespace, so `trim` used to leave it in
  /// place and the value was rejected downstream by `trim_edges`. Reading the
  /// scanner's rule instead makes it blank, so the property is dropped the way
  /// it already was for `""` and `" "`. Pinned so that undoing it has to be a
  /// decision.
  #[test]
  fn c0_controls_that_are_not_unicode_whitespace_are_blank() {
    assert!(is_blank_css_text("\u{0}")); // NUL
    assert!(is_blank_css_text("\u{1}")); // start of heading
    assert!(is_blank_css_text("\u{1f}")); // unit separator
    // And still true of the controls `trim` already agreed were whitespace.
    assert!(is_blank_css_text("\u{b}")); // vertical tab
    assert!(is_blank_css_text("\u{c}")); // form feed
    // Code 32 is the boundary; 33 is the first character that spells a value.
    assert!(is_blank_css_text("\u{20}"));
    assert!(!is_blank_css_text("\u{21}"));
  }
}
