//! A JavaScript value as data, and the coercions the language applies to it.
//!
//! `coercions` reads an expression the evaluator has already folded. This
//! module reads a value that arrives from JavaScript at run time: plain data
//! with no source text.
//! The data has no functions, symbols or big integers, so each coercion here is
//! total. The method names match the expression-level functions in
//! `coercions`, so the two readings of one coercion sit side by side.

use std::borrow::Cow;

use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use stylex_utils::number;

use crate::coercions::{OBJECT_TO_STRING, StringSink, string_to_js_number, write_js_join};

/// One JavaScript value as data.
///
/// An object keeps its properties in the order JavaScript enumerates them. It
/// keeps no identity: two reads of one object are two values, and an object
/// coerces as `Object.prototype` says, so an own `toString` or `valueOf` has
/// no effect here.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum JsValue {
  /// `undefined`, and a property that is not there.
  #[default]
  Undefined,
  /// An array index that holds no element (`[a, , b]`). Every coercion and
  /// every destructuring reads it as `undefined`; only a built-in that skips
  /// holes, such as `Array.prototype.filter`, tells the two apart.
  Hole,
  /// `null`.
  Null,
  /// `true` or `false`.
  Boolean(bool),
  /// A number, `NaN`, `-0` and the infinities included.
  Number(f64),
  /// A string. It holds no lone surrogate, which a Rust `String` cannot hold.
  String(String),
  /// An array, with its holes.
  Array(Vec<JsValue>),
  /// Boxed, so that every value stays small: a rule holds five of them.
  Object(Box<JsObject>),
}

/// The properties of an object, in the order JavaScript enumerates them.
pub type JsObject = IndexMap<String, JsValue, FxBuildHasher>;

/// A key of a `Map` or a `Set`: two values are one key when `SameValueZero`
/// says they are the same. `NaN` is one key, and `0` and `-0` are one key.
///
/// An array or an object is a key by its identity, which [`JsValue`] does not
/// keep, so it has no `MapKey`: each one is a key of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapKey<'a> {
  /// `undefined`, and a hole, which reads as `undefined`.
  Undefined,
  /// `null`.
  Null,
  /// `true` or `false`.
  Boolean(bool),
  /// The bits of the number, with every `NaN` and both zeros made one.
  Number(u64),
  /// A string, by its text.
  String(&'a str),
}

impl JsValue {
  /// An object with these properties. A key given twice keeps its first
  /// position and its last value, as an object literal does.
  pub fn object(properties: impl IntoIterator<Item = (String, JsValue)>) -> Self {
    JsValue::Object(Box::new(properties.into_iter().collect()))
  }

  /// `value == null`: `undefined`, a hole and `null`.
  pub fn is_nullish(&self) -> bool {
    matches!(self, JsValue::Undefined | JsValue::Hole | JsValue::Null)
  }

  /// `value[key]` for a string key: the property of an object, else
  /// `undefined`. An array, a string and a number have no named property this
  /// model holds (`length` is the one a caller reads, and it reads it itself).
  pub fn property(&self, key: &str) -> &JsValue {
    match self {
      JsValue::Object(properties) => properties.get(key).unwrap_or(&JsValue::Undefined),
      _ => &JsValue::Undefined,
    }
  }

  /// Takes the property out of an object and leaves `undefined` in its place,
  /// so a reader that consumes a value can keep a long string without a copy.
  pub fn take_property(&mut self, key: &str) -> JsValue {
    match self {
      JsValue::Object(properties) => properties
        .get_mut(key)
        .map(std::mem::take)
        .unwrap_or_default(),
      _ => JsValue::Undefined,
    }
  }

  /// `String(value)` (ECMA-262 `ToString`).
  ///
  /// An array is joined by a recursive walk, so the caller bounds how deep
  /// its arrays nest. The addon builds strings only, and `serde_json` stops
  /// at 128 levels of nesting.
  pub fn to_js_string(&self) -> Cow<'_, str> {
    match self {
      JsValue::Undefined | JsValue::Hole => Cow::Borrowed("undefined"),
      JsValue::Null => Cow::Borrowed("null"),
      JsValue::Boolean(true) => Cow::Borrowed("true"),
      JsValue::Boolean(false) => Cow::Borrowed("false"),
      JsValue::Number(value) => Cow::Owned(number::to_js_string(*value)),
      JsValue::String(text) => Cow::Borrowed(text),
      JsValue::Array(elements) => {
        let mut text = String::new();
        // Neither the element writer nor a `String` refuses a piece, so the
        // join always completes.
        let _ = write_js_join(elements, &mut text, |element, sink| {
          sink.write_piece(&element.to_js_join_element())
        });
        Cow::Owned(text)
      },
      JsValue::Object(_) => Cow::Borrowed(OBJECT_TO_STRING),
    }
  }

  /// The text `Array.prototype.join` writes for this element: nothing for
  /// `undefined`, a hole and `null`, else `ToString`.
  pub fn to_js_join_element(&self) -> Cow<'_, str> {
    if self.is_nullish() {
      Cow::Borrowed("")
    } else {
      self.to_js_string()
    }
  }

  /// `Number(value)` (ECMA-262 `ToNumber`). An array or an object becomes a
  /// primitive first: an array becomes its joined text, an object becomes
  /// `[object Object]`, which is `NaN`.
  pub fn to_js_number(&self) -> f64 {
    match self {
      JsValue::Undefined | JsValue::Hole | JsValue::Object(_) => f64::NAN,
      JsValue::Null | JsValue::Boolean(false) => 0.0,
      JsValue::Boolean(true) => 1.0,
      JsValue::Number(value) => *value,
      JsValue::String(text) => string_to_js_number(text),
      JsValue::Array(_) => string_to_js_number(&self.to_js_string()),
    }
  }

  /// `Boolean(value)` (ECMA-262 `ToBoolean`): `undefined`, `null`, `false`,
  /// both zeros, `NaN` and the empty string are false.
  pub fn to_js_boolean(&self) -> bool {
    match self {
      JsValue::Undefined | JsValue::Hole | JsValue::Null => false,
      JsValue::Boolean(flag) => *flag,
      JsValue::Number(value) => *value != 0.0 && !value.is_nan(),
      JsValue::String(text) => !text.is_empty(),
      JsValue::Array(_) | JsValue::Object(_) => true,
    }
  }

  /// The key this value is in a `Map` (`SameValueZero`), or `None` for an
  /// array or an object, which is a key of its own.
  pub fn same_value_zero_key(&self) -> Option<MapKey<'_>> {
    match self {
      JsValue::Undefined | JsValue::Hole => Some(MapKey::Undefined),
      JsValue::Null => Some(MapKey::Null),
      JsValue::Boolean(flag) => Some(MapKey::Boolean(*flag)),
      JsValue::Number(value) if value.is_nan() => Some(MapKey::Number(f64::NAN.to_bits())),
      // `-0 + 0` is `0`: the two zeros become one.
      JsValue::Number(value) => Some(MapKey::Number((value + 0.0).to_bits())),
      JsValue::String(text) => Some(MapKey::String(text)),
      JsValue::Array(_) | JsValue::Object(_) => None,
    }
  }

  /// What the value is, for a message: `undefined`, `a number`, `an array`.
  pub fn describe(&self) -> &'static str {
    match self {
      JsValue::Undefined | JsValue::Hole => "undefined",
      JsValue::Null => "null",
      JsValue::Boolean(_) => "a boolean",
      JsValue::Number(_) => "a number",
      JsValue::String(_) => "a string",
      JsValue::Array(_) => "an array",
      JsValue::Object(_) => "an object",
    }
  }
}

impl From<&str> for JsValue {
  fn from(text: &str) -> Self {
    JsValue::String(text.to_string())
  }
}

impl From<String> for JsValue {
  fn from(text: String) -> Self {
    JsValue::String(text)
  }
}

impl From<f64> for JsValue {
  fn from(value: f64) -> Self {
    JsValue::Number(value)
  }
}

impl From<bool> for JsValue {
  fn from(flag: bool) -> Self {
    JsValue::Boolean(flag)
  }
}

/// `Deserialize` for `JsValue`, behind the `serde` feature. Only tests use it:
/// they read values from JSON. The addon builds its values itself.
#[cfg(any(test, feature = "serde"))]
mod deserialize {
  use std::fmt;

  use rustc_hash::FxBuildHasher;
  use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

  use super::{JsObject, JsValue};

  /// Reads any self-describing format. A unit is `null`: JSON has no
  /// `undefined`, and a format that has one hands it over as a unit too, so a
  /// reader that must tell the two apart builds the value itself.
  impl<'de> Deserialize<'de> for JsValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      deserializer.deserialize_any(JsValueVisitor)
    }
  }

  // Visible to the tests only: the module is private.
  pub(super) struct JsValueVisitor;

  impl<'de> Visitor<'de> for JsValueVisitor {
    type Value = JsValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.write_str("a JavaScript value")
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
      Ok(JsValue::Null)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
      Ok(JsValue::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
      JsValue::deserialize(deserializer)
    }

    fn visit_bool<E: de::Error>(self, flag: bool) -> Result<Self::Value, E> {
      Ok(JsValue::Boolean(flag))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
      Ok(JsValue::Number(value as f64))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
      Ok(JsValue::Number(value as f64))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
      Ok(JsValue::Number(value))
    }

    fn visit_str<E: de::Error>(self, text: &str) -> Result<Self::Value, E> {
      Ok(JsValue::String(text.to_string()))
    }

    fn visit_string<E: de::Error>(self, text: String) -> Result<Self::Value, E> {
      Ok(JsValue::String(text))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
      let mut elements = Vec::with_capacity(sequence.size_hint().unwrap_or(0));

      while let Some(element) = sequence.next_element()? {
        elements.push(element);
      }

      Ok(JsValue::Array(elements))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
      let mut properties =
        JsObject::with_capacity_and_hasher(map.size_hint().unwrap_or(0), FxBuildHasher);

      // A key read twice keeps its first position and its last value, as an
      // object literal does.
      while let Some((key, value)) = map.next_entry::<String, JsValue>()? {
        properties.insert(key, value);
      }

      Ok(JsValue::Object(Box::new(properties)))
    }
  }
}

#[cfg(test)]
#[path = "tests/value_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/value_deserialize_tests.rs"]
mod deserialize_tests;
