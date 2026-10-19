//! The arguments of `processStylexRules`, read from JavaScript into a
//! [`JsValue`] that `stylex_stylesheet` reads as the reference does.
//!
//! The serde bridge of napi is not used for this: it reads `undefined` and
//! `null` as one unit, and a whole number as an `i64`, which loses `-0` and
//! clamps a number past 2^63. Both change what the reference writes: an
//! `undefined` priority is `NaN` and a `null` one is 0, and `String(1e21)` is
//! `1e+21`. This reader keeps each value as JavaScript holds it.

use napi::bindgen_prelude::{JsObjectValue, Object, Unknown};
use napi::{Env, JsString, JsValue as _, Result, ValueType};
use stylex_js::value::JsValue;

use super::fn_parser::MAX_ENV_NESTING_DEPTH;

/// How deeply the reader descends into the arguments before it stops.
///
/// The reader recurses once per array or object, and a stack overflow aborts
/// the process rather than raising an error that JavaScript can catch. The
/// limit is the one `fn_parser` sets for an `env` value, for the same reason:
/// real rules nest three levels (the list, the tuple, the style object) and a
/// config three (the config, `useLayers`, `before`).
const MAX_ARGUMENT_NESTING_DEPTH: usize = MAX_ENV_NESTING_DEPTH;

/// Reads one argument. `what` names it in a refusal ("the rules").
///
/// A symbol, a function, a big integer or an external value has no
/// [`JsValue`], and is refused wherever it sits. A value that contains itself
/// is refused too: a model with no identity cannot hold it. An object reads
/// as its enumerable string-keyed properties, in the order JavaScript lists
/// them; for a plain object (the only kind a build's rules hold) those are
/// its own properties.
pub(crate) fn read_js_value(env: &Env, value: Unknown<'_>, what: &str) -> Result<JsValue> {
  let mut reader = Reader {
    env,
    what,
    ancestors: Vec::new(),
  };

  reader.read(value)
}

struct Reader<'r, 'env> {
  env: &'r Env,
  what: &'r str,
  /// The arrays and objects that hold the value being read, outermost first.
  ancestors: Vec<Object<'env>>,
}

impl<'env> Reader<'_, 'env> {
  fn read(&mut self, value: Unknown<'env>) -> Result<JsValue> {
    // SAFETY: each cast matches the type `get_type` reported.
    match value.get_type()? {
      ValueType::Undefined => Ok(JsValue::Undefined),
      ValueType::Null => Ok(JsValue::Null),
      ValueType::Boolean => Ok(JsValue::Boolean(unsafe { value.cast::<bool>() }?)),
      ValueType::Number => Ok(JsValue::Number(unsafe { value.cast::<f64>() }?)),
      ValueType::String => Ok(JsValue::String(unsafe { value.cast::<String>() }?)),
      ValueType::Object => self.read_object(unsafe { value.cast::<Object<'env>>() }?),
      ValueType::Symbol => Err(self.refusal("hold a symbol")),
      ValueType::Function => Err(self.refusal("hold a function")),
      ValueType::External => Err(self.refusal("hold an external value")),
      // Without the `napi6` feature, napi reports a big integer as unknown.
      ValueType::Unknown => Err(self.refusal("hold a big integer or a value of an unknown kind")),
    }
  }

  fn read_object(&mut self, object: Object<'env>) -> Result<JsValue> {
    // A value that contains itself nests without end, so it stops here too.
    // Only then are the ancestors compared, to say which of the two it is:
    // a check at every object would cost a call to JavaScript per ancestor.
    if self.ancestors.len() >= MAX_ARGUMENT_NESTING_DEPTH {
      return Err(if self.holds_a_cycle()? {
        self.refusal("hold a value that contains itself")
      } else {
        self.refusal(&format!(
          "nest more than {MAX_ARGUMENT_NESTING_DEPTH} levels deep"
        ))
      });
    }

    self.ancestors.push(object);
    let read = if object.is_array()? {
      self.read_array_elements(object)
    } else {
      self.read_properties(object)
    };
    self.ancestors.pop();

    read
  }

  /// Every index below the length. An index with no element is a hole,
  /// which `Array.prototype.filter` skips and every other read takes as
  /// `undefined`.
  fn read_array_elements(&mut self, array: Object<'env>) -> Result<JsValue> {
    let length = array.get_array_length_unchecked()?;
    let mut elements = Vec::with_capacity(length as usize);

    for index in 0..length {
      let element: Unknown<'env> = array.get_element(index)?;
      let is_hole = element.get_type()? == ValueType::Undefined && !array.has_element(index)?;

      elements.push(if is_hole {
        JsValue::Hole
      } else {
        self.read(element)?
      });
    }

    Ok(JsValue::Array(elements))
  }

  fn read_properties(&mut self, object: Object<'env>) -> Result<JsValue> {
    let names = object.get_property_names()?;
    let length = names.get_array_length_unchecked()?;
    let mut properties = Vec::with_capacity(length as usize);

    for index in 0..length {
      // The key is read with its own JavaScript string, so a name that holds
      // a NUL character reads as it is.
      let name: JsString<'env> = names.get_element(index)?;
      let value: Unknown<'env> = object.get_property(name)?;
      let value = self.read(value)?;

      properties.push((name.into_utf8()?.into_owned()?, value));
    }

    Ok(JsValue::object(properties))
  }

  /// Whether one object is on the ancestor stack twice.
  fn holds_a_cycle(&self) -> Result<bool> {
    for (index, ancestor) in self.ancestors.iter().enumerate() {
      for later in &self.ancestors[index + 1..] {
        if self.env.strict_equals(*ancestor, *later)? {
          return Ok(true);
        }
      }
    }

    Ok(false)
  }

  fn refusal(&self, reason: &str) -> napi::Error {
    napi::Error::from_reason(format!(
      "processStylexRules: {} {reason}, which cannot be read",
      self.what
    ))
  }
}
