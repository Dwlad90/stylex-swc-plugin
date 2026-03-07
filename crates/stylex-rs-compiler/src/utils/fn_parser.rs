use std::rc::Rc;

use indexmap::IndexMap;
use napi::{JsNumber, JsObject, JsString, JsValue, NapiRaw, Unknown, ValueType};
use stylex_shared::shared::structures::stylex_env::{EnvValue, JSFunction};

thread_local! {
  static NAPI_ENV_RAW: std::cell::Cell<Option<napi::sys::napi_env>> =
    const { std::cell::Cell::new(None) };
}

/// Sets the NAPI env for the duration of a closure, then clears it.
pub(crate) fn with_napi_env<F, R>(env: &napi::Env, f: F) -> R
where
  F: FnOnce() -> R,
{
  NAPI_ENV_RAW.set(Some(env.raw()));
  let result = f();
  NAPI_ENV_RAW.set(None);
  result
}

/// Parses a JS object into an `IndexMap<String, EnvValue>`.
pub(crate) fn parse_env_object(
  env: &napi::Env,
  obj: &JsObject,
) -> napi::Result<IndexMap<String, EnvValue>> {
  let names = obj.get_property_names()?;
  let len = names.get_array_length()?;
  let mut map = IndexMap::new();

  for i in 0..len {
    let key: JsString = names.get_element(i)?;
    let key_str = key.into_utf8()?.as_str()?.to_string();
    let value: Unknown = obj.get_named_property(&key_str)?;
    let env_value = parse_env_value(env, value)?;
    map.insert(key_str, env_value);
  }

  Ok(map)
}

fn parse_env_value(env: &napi::Env, value: Unknown) -> napi::Result<EnvValue> {
  match value.get_type()? {
    ValueType::String => {
      let s: JsString = unsafe { value.cast()? };
      Ok(EnvValue::String(s.into_utf8()?.as_str()?.to_string()))
    }
    ValueType::Number => {
      let n: JsNumber = unsafe { value.cast()? };
      Ok(EnvValue::Number(n.get_double()?))
    }
    ValueType::Boolean => {
      // JsBoolean doesn't implement FromNapiValue in napi v3, use raw API
      let raw_env = env.raw();
      let mut b = false;
      let status = unsafe { napi::sys::napi_get_value_bool(raw_env, value.raw(), &mut b) };
      if status != napi::sys::Status::napi_ok {
        return Err(napi::Error::from_reason("Failed to get boolean value"));
      }
      Ok(EnvValue::Bool(b))
    }
    ValueType::Null | ValueType::Undefined => Ok(EnvValue::Null),
    ValueType::Function => {
      // JsFunction doesn't implement FromNapiValue in napi v3, use raw value directly
      parse_env_function(env, value.raw())
    }
    ValueType::Object => {
      let obj: JsObject = unsafe { value.cast()? };
      let nested = parse_env_object(env, &obj)?;
      Ok(EnvValue::Object(nested))
    }
    _ => Ok(EnvValue::Null),
  }
}

fn parse_env_function(env: &napi::Env, js_fn_raw: napi::sys::napi_value) -> napi::Result<EnvValue> {
  let raw_env = env.raw();

  // Create a persistent reference to the JS function so it stays alive
  let mut ref_ptr: napi::sys::napi_ref = std::ptr::null_mut();
  let status = unsafe { napi::sys::napi_create_reference(raw_env, js_fn_raw, 1, &mut ref_ptr) };

  if status != napi::sys::Status::napi_ok {
    return Err(napi::Error::from_reason(
      "Failed to create reference for env function",
    ));
  }

  // Wrap in Rc so the closure can be Fn (not just FnOnce)
  let ref_ptr = Rc::new(EnvFnRef(ref_ptr));

  Ok(EnvValue::Function(JSFunction::new(
    move |args: Vec<EnvValue>| {
      let raw_env = NAPI_ENV_RAW
        .get()
        .expect("NAPI env not available during env function call");

      // Get the function from reference
      let mut js_fn_raw: napi::sys::napi_value = std::ptr::null_mut();
      unsafe {
        napi::sys::napi_get_reference_value(raw_env, ref_ptr.0, &mut js_fn_raw);
      }

      // Convert args to JS values
      let js_args: Vec<napi::sys::napi_value> = args
        .iter()
        .map(|arg| env_value_to_napi_value(raw_env, arg))
        .collect();

      // Call the function with undefined as `this`
      let mut result: napi::sys::napi_value = std::ptr::null_mut();
      let mut undefined: napi::sys::napi_value = std::ptr::null_mut();
      unsafe {
        napi::sys::napi_get_undefined(raw_env, &mut undefined);
        napi::sys::napi_call_function(
          raw_env,
          undefined,
          js_fn_raw,
          js_args.len(),
          js_args.as_ptr(),
          &mut result,
        );
      }

      // Coerce result to string
      let mut str_result: napi::sys::napi_value = std::ptr::null_mut();
      unsafe {
        napi::sys::napi_coerce_to_string(raw_env, result, &mut str_result);
      }

      // Read the string value
      let mut len = 0;
      unsafe {
        napi::sys::napi_get_value_string_utf8(
          raw_env,
          str_result,
          std::ptr::null_mut(),
          0,
          &mut len,
        );
      }
      let mut buf = vec![0u8; len + 1];
      let mut written = 0;
      unsafe {
        napi::sys::napi_get_value_string_utf8(
          raw_env,
          str_result,
          buf.as_mut_ptr() as *mut i8,
          len + 1,
          &mut written,
        );
      }
      buf.truncate(written);
      String::from_utf8(buf).unwrap_or_default()
    },
  )))
}

fn env_value_to_napi_value(
  raw_env: napi::sys::napi_env,
  value: &EnvValue,
) -> napi::sys::napi_value {
  let mut result: napi::sys::napi_value = std::ptr::null_mut();
  match value {
    EnvValue::String(s) => unsafe {
      napi::sys::napi_create_string_utf8(
        raw_env,
        s.as_ptr() as *const i8,
        s.len() as isize,
        &mut result,
      );
    },
    EnvValue::Number(n) => unsafe {
      napi::sys::napi_create_double(raw_env, *n, &mut result);
    },
    EnvValue::Bool(b) => unsafe {
      napi::sys::napi_get_boolean(raw_env, *b, &mut result);
    },
    EnvValue::Null | EnvValue::Object(_) | EnvValue::Function(_) => unsafe {
      napi::sys::napi_get_null(raw_env, &mut result);
    },
  }
  result
}

/// Parses a JS string or function into an `EnvFunction` for use as `debugFilePath`.
///
/// - If the value is a string, returns an `EnvFunction` that always returns that string.
/// - If the value is a function `(filePath: string) => string`, wraps it as an `EnvFunction`.
pub(crate) fn parse_debug_file_path(
  env: &napi::Env,
  js_obj: &JsObject,
) -> napi::Result<JSFunction> {
  let raw_val = unsafe { js_obj.raw() };
  let raw_env = env.raw();

  let mut val_type: napi::sys::napi_valuetype = napi::sys::ValueType::napi_undefined;
  unsafe { napi::sys::napi_typeof(raw_env, raw_val, &mut val_type) };

  match val_type {
    napi::sys::ValueType::napi_string => {
      // Static string: wrap as a constant-returning function
      let mut len = 0;
      unsafe {
        napi::sys::napi_get_value_string_utf8(raw_env, raw_val, std::ptr::null_mut(), 0, &mut len);
      }
      let mut buf = vec![0u8; len + 1];
      let mut written = 0;
      unsafe {
        napi::sys::napi_get_value_string_utf8(
          raw_env,
          raw_val,
          buf.as_mut_ptr() as *mut i8,
          len + 1,
          &mut written,
        );
      }
      buf.truncate(written);
      let s = String::from_utf8(buf).unwrap_or_default();
      Ok(JSFunction::new(move |_args| s.clone()))
    }
    napi::sys::ValueType::napi_function => {
      parse_env_function(env, raw_val).and_then(|ev| match ev {
        EnvValue::Function(f) => Ok(f),
        _ => Err(napi::Error::from_reason(
          "Expected function from parse_env_function",
        )),
      })
    }
    _ => Err(napi::Error::from_reason(
      "debugFilePath must be a string or function",
    )),
  }
}

/// Wrapper for a raw napi_ref to allow sharing via Rc.
struct EnvFnRef(napi::sys::napi_ref);

// Safety: EnvFnRef is only used single-threaded within NAPI calls.
// The raw pointer is never sent across threads.
unsafe impl Send for EnvFnRef {}
unsafe impl Sync for EnvFnRef {}
