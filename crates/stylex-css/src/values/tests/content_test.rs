//! Every expectation here is what `@stylexjs/babel-plugin` emits for the same
//! declaration, read from a probe run of both compilers over the same source.
//!
//! The suite holds what only this layer can prove: which node kind makes a
//! component, and what a backslash or a line break does inside the quoted
//! string. The values an author is likely to write -- prose carrying a quote,
//! a string beside a quote keyword, an escape sequence -- are asserted where
//! they arrive, on `transform_value` in `stylex-transform`.

use crate::values::content::transform_content_value;

// ── a value that is already CSS ──────────────────────────────────────

#[test]
fn passes_through_a_string_list_a_separator_stands_in() {
  // A separator between two components is not itself a component, so it does
  // not decide whether the list is CSS.
  let input = "\"icon\" / \"Alt text\"";

  assert_eq!(transform_content_value(input), input);
}

#[test]
fn passes_through_a_string_beside_a_function_it_does_not_know() {
  // `foo(` is in none of the content functions, so the value is admitted for
  // being a closed function rather than for the name it carries.
  assert_eq!(transform_content_value("\"a\" foo(1)"), "\"a\" foo(1)");
}

// ── a value that is not ──────────────────────────────────────────────

#[test]
fn quotes_a_value_whose_components_are_not_all_content() {
  let cases = [
    // A function that never closes.
    ("\"a\" foo(", "\"\\\"a\\\" foo(\""),
    // A word that is not one of the keywords the property names.
    ("\"a\" contents", "\"\\\"a\\\" contents\""),
    // A comment, which is neither a string nor a keyword.
    ("\"a\"/*c*/", "\"\\\"a\\\"/*c*/\""),
    // A unicode range, likewise.
    ("U+26 \"a\"", "\"U+26 \\\"a\\\"\""),
  ];

  for (input, expected) in cases {
    assert_eq!(
      transform_content_value(input),
      expected,
      "quoting `{}`",
      input
    );
  }
}

#[test]
fn quotes_keywords_that_no_string_stands_beside() {
  // Each word is a keyword the property names, but a list of them alone is not
  // the author writing a string, so it is quoted rather than passed through.
  assert_eq!(
    transform_content_value("open-quote close-quote"),
    "\"open-quote close-quote\""
  );
}

// ── escaping ─────────────────────────────────────────────────────────

#[test]
fn writes_every_line_break_as_one_escape() {
  let cases = [
    ("a\nb", "\"a\\A b\""),
    ("a\rb", "\"a\\A b\""),
    // A CRLF is one line break and earns one escape, not two.
    ("a\r\nb", "\"a\\A b\""),
    ("a\u{c}b", "\"a\\A b\""),
  ];

  for (input, expected) in cases {
    assert_eq!(
      transform_content_value(input),
      expected,
      "quoting `{:?}`",
      input
    );
  }
}

#[test]
fn reads_a_backslash_as_the_start_of_an_escape() {
  let cases = [
    // `\e9` is the escape for an e with an acute accent, so doubling its
    // backslash would print the characters instead.
    ("caf\\e9 au lait", "\"caf\\e9 au lait\""),
    // An escaped line break continues the string rather than ending it, so it
    // is copied over whole -- both characters of a CRLF included.
    ("trail\\\r\nx", "\"trail\\\r\nx\""),
    // A trailing backslash escapes nothing but the closing quote, so it is the
    // one backslash that is doubled.
    ("trail\\", "\"trail\\\\\""),
  ];

  for (input, expected) in cases {
    assert_eq!(
      transform_content_value(input),
      expected,
      "quoting `{:?}`",
      input
    );
  }
}

// ── large and awkward input ──────────────────────────────────────────

#[test]
fn reads_a_very_long_component_list_without_help() {
  // Each component is a closed string, so the whole list is CSS however long
  // it gets. The scan must not stop early or run out of stack on it.
  let input = vec!["\"x\""; 5_000].join(" ");

  assert_eq!(transform_content_value(&input), input);
}

#[test]
fn quotes_a_very_long_text_value_once() {
  // One quote character every few words gives the scan the most work it can
  // get and still produces one string, not a list.
  let text = "He said \"hi\" and left. ".repeat(2_000);
  let output = transform_content_value(&text);

  // The value is wrapped one time, and every quote inside it is escaped.
  assert!(output.starts_with('"') && output.ends_with('"'));
  assert_eq!(output.matches("\\\"").count(), text.matches('"').count());
}

#[test]
fn reads_a_deeply_nested_function_as_one_component() {
  // A function node carries its own parentheses, so depth does not turn the
  // value into something other than a component list.
  let nested = format!("{}{}", "calc(".repeat(500), ")".repeat(500));
  let input = format!("\"a\" {}", nested);

  assert_eq!(transform_content_value(&input), input);
}

#[test]
fn quotes_a_lone_quote_character() {
  // The shortest value that is not a closed string.
  assert_eq!(transform_content_value("\""), "\"\\\"\"");
}

#[test]
fn quotes_an_empty_value() {
  // There is no component at all, so no string stands among them.
  assert_eq!(transform_content_value(""), "\"\"");
}
