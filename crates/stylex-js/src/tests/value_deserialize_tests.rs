// Tests for `Deserialize` of a JavaScript value, behind the `serde` feature.
// Source: crates/stylex-js/src/value.rs

use serde::Deserialize;

use super::JsValue;
use super::deserialize::JsValueVisitor;

fn object(properties: &[(&str, JsValue)]) -> JsValue {
  JsValue::object(
    properties
      .iter()
      .map(|(key, value)| (key.to_string(), value.clone())),
  )
}

fn array(elements: Vec<JsValue>) -> JsValue {
  JsValue::Array(elements)
}

fn read(json: &str) -> JsValue {
  match serde_json::from_str(json) {
    Ok(value) => value,
    Err(error) => panic!("{json} did not read: {error}"),
  }
}

#[test]
fn json_reads_into_the_value_it_spells() {
  assert_eq!(
    read(r#"{"b": [1, -2, 2.5, 18446744073709551615, null, true, "x"], "a": {}, "b2": {"k": 1}}"#),
    object(&[
      (
        "b",
        array(vec![
          1.0.into(),
          (-2.0).into(),
          2.5.into(),
          (u64::MAX as f64).into(),
          JsValue::Null,
          true.into(),
          "x".into(),
        ])
      ),
      ("a", object(&[])),
      ("b2", object(&[("k", 1.0.into())])),
    ])
  );
  // A key written twice keeps its first position and its last value.
  assert_eq!(
    read(r#"{"a": 1, "b": 2, "a": 3}"#),
    object(&[("a", 3.0.into()), ("b", 2.0.into())])
  );
  // A string with an escape arrives owned.
  assert_eq!(read(r#""a\nb""#), JsValue::from("a\nb"));
  // A fault inside an array or an object is the parse error it is.
  for cut_short in ["[1,", r#"{"a": 1,"#, r#"[{"a": [1, {"#] {
    assert!(
      serde_json::from_str::<JsValue>(cut_short).is_err(),
      "{cut_short}"
    );
  }
  assert_eq!(
    JsValue::object([("a".to_string(), 1.0.into()), ("a".to_string(), 2.0.into())]),
    object(&[("a", 2.0.into())])
  );
}

#[test]
fn an_option_wrapper_reads_none_as_null_and_some_as_its_value() {
  use serde::de::value::{Error, StringDeserializer};
  use serde::de::{IntoDeserializer, Visitor};

  let some: Result<JsValue, Error> =
    JsValueVisitor.visit_some(StringDeserializer::<Error>::new("a".to_string()));
  let none: Result<JsValue, Error> = JsValueVisitor.visit_none();
  let unit: Result<JsValue, Error> = JsValue::deserialize(().into_deserializer());

  assert_eq!(some, Ok(JsValue::from("a")));
  assert_eq!(none, Ok(JsValue::Null));
  assert_eq!(unit, Ok(JsValue::Null));
}

#[test]
fn a_type_outside_the_model_is_refused_by_name() {
  use serde::de::value::{BytesDeserializer, Error};

  let refused: Result<JsValue, Error> = JsValue::deserialize(BytesDeserializer::<Error>::new(b"x"));

  match refused {
    Err(error) => assert!(error.to_string().contains("a JavaScript value"), "{error}"),
    Ok(value) => panic!("bytes read as {value:?}"),
  }
}
