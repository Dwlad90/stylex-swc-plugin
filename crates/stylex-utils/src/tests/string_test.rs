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

  #[test]
  fn astral_scalars_shift_following_indices() {
    // `"a🎉b".charCodeAt(3) === 98` — the surrogate pair consumes indices 1
    // and 2, so `'b'` lands at 3, not at 2.
    assert_eq!(char_code_at("a🎉b", 0), Some(97));
    assert_eq!(char_code_at("a🎉b", 3), Some(98));
  }
}
