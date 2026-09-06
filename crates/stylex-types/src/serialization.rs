//! Writing an authored value back out as the JavaScript source it becomes.
//!
//! Serializing gives JSON, and JSON quotes a string. A value that was authored
//! as a string is already the source it has to stay, so the quotes come back
//! off: `1px` is `1px`, not `"1px"`.

use stylex_macros::stylex_panic;
use stylex_regex::regex::JSON_REGEX;
use stylex_utils::string::remove_quotes;

/// The value written back out as JavaScript source.
pub fn serialize_value_to_json_string<T: serde::Serialize>(value: T) -> String {
  render_json(serde_json::to_string(&value))
}

/// The body of [`serialize_value_to_json_string`], held outside the generic so
/// that one refusing type cannot leave the other branches unmeasured -- code
/// coverage counts a generic function once per type it is called with.
fn render_json(result: Result<String, serde_json::Error>) -> String {
  let json = match result {
    Ok(json) => json,
    Err(err) => stylex_panic!("Failed to serialize value. Error: {}", err),
  };

  // Only a JSON string carries a value that has to come back out of its
  // quotes. Every other shape -- a number, a boolean, `null`, an array, an
  // object -- already reads as the source it stays.
  let Ok(inner) = serde_json::from_str::<String>(&json) else {
    return json;
  };

  // The empty string keeps its two quotes. It has no source to unwrap to, and
  // a bare empty value would read as a missing one.
  if inner.is_empty() {
    return json;
  }

  // A string that holds a JavaScript object literal is repaired into JSON,
  // rather than emitted as one long escaped string.
  if inner.trim_start().starts_with('{') && !inner.contains("\":") {
    return js_object_to_json(&inner);
  }

  // A string that spells a number is that number.
  if inner.parse::<f64>().is_ok() {
    return inner;
  }

  remove_quotes(&inner).into_owned()
}

/// Quotes the bare keys of a JavaScript object literal, so `{ a: 1 }` reads as
/// the JSON `{ "a": 1 }`.
pub(crate) fn js_object_to_json(js_str: &str) -> String {
  JSON_REGEX.replace_all(js_str, r#"$1"$2":"#).to_string()
}

#[cfg(test)]
#[path = "tests/serialization_test.rs"]
mod tests;
