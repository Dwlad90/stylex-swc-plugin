use crate::serialization::{js_object_to_json, serialize_value_to_json_string};

mod serialize_value_to_json_string_tests {
  use super::*;

  #[test]
  fn serializes_number() {
    let result = serialize_value_to_json_string(42);
    assert_eq!(result, "42");
  }

  #[test]
  fn serializes_float() {
    let result = serialize_value_to_json_string(1.75);
    assert_eq!(result, "1.75");
  }

  #[test]
  fn serializes_boolean_true() {
    let result = serialize_value_to_json_string(true);
    assert_eq!(result, "true");
  }

  #[test]
  fn serializes_boolean_false() {
    let result = serialize_value_to_json_string(false);
    assert_eq!(result, "false");
  }

  #[test]
  fn serializes_plain_string() {
    let result = serialize_value_to_json_string("hello");
    assert_eq!(result, "hello");
  }

  #[test]
  fn serializes_numeric_string_as_number() {
    let result = serialize_value_to_json_string("123");
    assert_eq!(result, "123");
  }

  #[test]
  fn serializes_null() {
    let result = serialize_value_to_json_string::<Option<i32>>(None);
    assert_eq!(result, "null");
  }

  #[test]
  fn serializes_array() {
    let result = serialize_value_to_json_string(vec![1, 2, 3]);
    assert_eq!(result, "[1,2,3]");
  }

  #[test]
  fn serializes_empty_string() {
    // Empty string wrapped in quotes, but length <= 2, goes to else branch
    let result = serialize_value_to_json_string("");
    assert_eq!(result, "\"\"");
  }
}

mod serialize_value_to_json_string_extra_tests {
  use super::*;

  #[test]
  fn serializes_js_object_like_string() {
    // A string that starts with '{' and does NOT contain `":`
    // triggers js_object_to_json
    let result = serialize_value_to_json_string("{color: red, size: big}");
    assert!(result.contains('"'));
  }

  #[test]
  fn serializes_json_like_string_passthrough() {
    // A string that starts with '{' and contains `":` is NOT treated
    // as a JS object; it falls through to the plain remove_quotes path
    let result = serialize_value_to_json_string(r#"{"key":"value"}"#);
    assert!(result.contains("key"));
  }
}

mod serialize_value_failure_tests {
  use super::*;

  /// A value whose own `Serialize` refuses, which is the only way the
  /// serializer reports a failure.
  struct Unserializable;

  impl serde::Serialize for Unserializable {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
      Err(serde::ser::Error::custom("no representation"))
    }
  }

  #[test]
  #[should_panic]
  fn panics_when_the_value_cannot_be_serialized() {
    serialize_value_to_json_string(Unserializable);
  }
}

mod js_object_to_json_tests {
  use super::*;

  #[test]
  fn converts_js_object_keys_to_quoted_json() {
    let input = "{color: red}";
    let result = js_object_to_json(input);
    assert!(result.contains('"'));
  }

  #[test]
  fn handles_empty_object() {
    let input = "{}";
    let result = js_object_to_json(input);
    assert_eq!(result, "{}");
  }
}

mod shape_table_tests {
  use super::*;

  /// Every JSON shape a value can take, and the source each one renders as.
  /// Written out as a table so that a change to one branch cannot quietly move
  /// another: this reads the whole surface at once.
  #[test]
  fn renders_every_shape_of_value() {
    assert_eq!(serialize_value_to_json_string(0.0f64), "0.0");
    assert_eq!(serialize_value_to_json_string(-0.5f64), "-0.5");
    // serde_json spells a large float with an explicit sign in the exponent.
    assert_eq!(serialize_value_to_json_string(1e21f64), "1e+21");
    assert_eq!(
      serialize_value_to_json_string(u64::MAX),
      "18446744073709551615"
    );
    assert_eq!(
      serialize_value_to_json_string(i64::MIN),
      "-9223372036854775808"
    );
    assert_eq!(serialize_value_to_json_string(true), "true");
    assert_eq!(serialize_value_to_json_string(()), "null");
    assert_eq!(serialize_value_to_json_string(Option::<i32>::None), "null");

    // A string keeps the source it already is, and the empty one keeps its
    // quotes because it has no source to unwrap to.
    assert_eq!(serialize_value_to_json_string(""), "\"\"");
    assert_eq!(serialize_value_to_json_string(" "), " ");
    assert_eq!(serialize_value_to_json_string("red"), "red");
    assert_eq!(serialize_value_to_json_string("1px"), "1px");
    assert_eq!(serialize_value_to_json_string("42"), "42");
    assert_eq!(serialize_value_to_json_string("-3.5"), "-3.5");
    assert_eq!(serialize_value_to_json_string("\"quoted\""), "quoted");
    assert_eq!(serialize_value_to_json_string("line\nbreak"), "line\nbreak");
    assert_eq!(serialize_value_to_json_string("emoji 🎉"), "emoji 🎉");

    // A string holding a JavaScript object literal is repaired into JSON; one
    // that already spells JSON passes straight through.
    assert_eq!(serialize_value_to_json_string("{ a: 1 }"), "{\"a\": 1 }");
    assert_eq!(
      serialize_value_to_json_string(r#"{"key":"value"}"#),
      r#"{"key":"value"}"#
    );

    // A collection renders as the JSON it already is.
    assert_eq!(serialize_value_to_json_string(vec![1, 2, 3]), "[1,2,3]");
    assert_eq!(serialize_value_to_json_string(Vec::<i32>::new()), "[]");
    assert_eq!(
      serialize_value_to_json_string(serde_json::json!({ "a": 1, "b": { "c": [1, 2] } })),
      r#"{"a":1,"b":{"c":[1,2]}}"#
    );
  }
}
