#[cfg(test)]
mod dashify_tests {
  use crate::string::dashify;

  #[test]
  fn converts_simple_camel_case() {
    assert_eq!(dashify("marginTop"), "margin-top");
  }

  #[test]
  fn converts_multiple_uppercase_letters() {
    assert_eq!(dashify("borderBottomLeftRadius"), "border-bottom-left-radius");
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
  fn returns_code_point_at_index() {
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
  }
}
