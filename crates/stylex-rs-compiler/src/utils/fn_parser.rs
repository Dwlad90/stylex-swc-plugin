use std::rc::Rc;

use indexmap::IndexMap;
use log::debug;
use napi::{JsNumber, JsObject, JsString, JsValue, Unknown, ValueType};
use stylex_ast::ast::{
  convertors::{create_bool_expr, create_null_expr, create_number_expr, create_string_expr},
  factories::{create_array_expression, create_key_value_prop, create_object_expression},
};
use stylex_structures::stylex_env::{EnvEntry, JSFunction};
use stylex_utils::swc::get_default_expr_ctx;
use swc_core::ecma::{
  ast::{Expr, ExprOrSpread, Lit, PropName, PropOrSpread},
  utils::ExprExt,
};

thread_local! {
  static NAPI_ENV_RAW: std::cell::Cell<Option<napi::sys::napi_env>> =
    const { std::cell::Cell::new(None) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnvValueKind {
  String,
  Number,
  Boolean,
  Nullish,
  Function,
  Object,
  Unsupported,
}

fn env_value_kind(value_type: ValueType) -> EnvValueKind {
  match value_type {
    ValueType::String => EnvValueKind::String,
    ValueType::Number => EnvValueKind::Number,
    ValueType::Boolean => EnvValueKind::Boolean,
    ValueType::Null | ValueType::Undefined => EnvValueKind::Nullish,
    ValueType::Function => EnvValueKind::Function,
    ValueType::Object => EnvValueKind::Object,
    _ => EnvValueKind::Unsupported,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DebugFilePathKind {
  String,
  Function,
  Unsupported,
}

fn debug_file_path_kind(val_type: napi::sys::napi_valuetype) -> DebugFilePathKind {
  match val_type {
    napi::sys::ValueType::napi_string => DebugFilePathKind::String,
    napi::sys::ValueType::napi_function => DebugFilePathKind::Function,
    _ => DebugFilePathKind::Unsupported,
  }
}

fn utf8_string_from_written_buffer(mut buf: Vec<u8>, written: usize) -> String {
  buf.truncate(written);
  String::from_utf8(buf).unwrap_or_default()
}

fn prop_name_to_string(key: &PropName) -> Option<String> {
  match key {
    PropName::Ident(id) => Some(id.sym.to_string()),
    PropName::Str(s) => Some(s.value.as_str().unwrap_or("").to_string()),
    PropName::Num(n) => Some(n.value.to_string()),
    _ => None,
  }
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

/// Parses a JS object into an `IndexMap<String, EnvEntry>`.
pub(crate) fn parse_env_object(
  env: &napi::Env,
  obj: &JsObject,
) -> napi::Result<IndexMap<String, EnvEntry>> {
  let names = obj.get_property_names()?;
  let len = names.get_array_length()?;
  let mut map = IndexMap::new();

  for i in 0..len {
    let key: JsString = names.get_element(i)?;
    let key_str = key.into_utf8()?.as_str()?.to_string();
    let value: Unknown = obj.get_named_property(&key_str)?;
    let env_entry = parse_env_value(env, value)?;
    map.insert(key_str, env_entry);
  }

  Ok(map)
}

fn parse_env_value(env: &napi::Env, value: Unknown) -> napi::Result<EnvEntry> {
  match env_value_kind(value.get_type()?) {
    EnvValueKind::String => {
      let s: JsString = unsafe { value.cast()? };
      Ok(EnvEntry::Expr(create_string_expr(s.into_utf8()?.as_str()?)))
    },
    EnvValueKind::Number => {
      let n: JsNumber = unsafe { value.cast()? };
      Ok(EnvEntry::Expr(create_number_expr(n.get_double()?)))
    },
    EnvValueKind::Boolean => {
      let raw_env = env.raw();
      let mut b = false;
      let status = unsafe { napi::sys::napi_get_value_bool(raw_env, value.raw(), &mut b) };
      if status != napi::sys::Status::napi_ok {
        return Err(napi::Error::from_reason("Failed to get boolean value"));
      }
      Ok(EnvEntry::Expr(create_bool_expr(b)))
    },
    EnvValueKind::Nullish => Ok(EnvEntry::Expr(create_null_expr())),
    EnvValueKind::Function => parse_env_function(env, value.raw()),
    EnvValueKind::Object => Ok(EnvEntry::Expr(napi_value_to_expr(env.raw(), value.raw()))),
    EnvValueKind::Unsupported => Ok(EnvEntry::Expr(create_null_expr())),
  }
}

fn parse_env_function(env: &napi::Env, js_fn_raw: napi::sys::napi_value) -> napi::Result<EnvEntry> {
  let raw_env = env.raw();

  let mut ref_ptr: napi::sys::napi_ref = std::ptr::null_mut();
  let status = unsafe { napi::sys::napi_create_reference(raw_env, js_fn_raw, 1, &mut ref_ptr) };

  if status != napi::sys::Status::napi_ok {
    return Err(napi::Error::from_reason(
      "Failed to create reference for env function",
    ));
  }

  let ref_ptr = Rc::new(EnvFnRef {
    ref_ptr,
    env: raw_env,
  });

  Ok(EnvEntry::Function(JSFunction::new(
    move |args: Vec<Expr>| {
      let raw_env = NAPI_ENV_RAW
        .get()
        .expect("NAPI env not available during env function call");

      let mut js_fn_raw: napi::sys::napi_value = std::ptr::null_mut();
      unsafe {
        napi::sys::napi_get_reference_value(raw_env, ref_ptr.ref_ptr, &mut js_fn_raw);
      }

      let js_args: Vec<napi::sys::napi_value> = args
        .iter()
        .map(|arg| expr_to_napi_value(raw_env, arg))
        .collect();

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

      napi_value_to_expr(raw_env, result)
    },
  )))
}

fn expr_to_napi_value(raw_env: napi::sys::napi_env, expr: &Expr) -> napi::sys::napi_value {
  let mut result: napi::sys::napi_value = std::ptr::null_mut();
  match expr {
    Expr::Lit(Lit::Str(s)) => {
      let val = s.value.as_str().unwrap_or("");
      unsafe {
        napi::sys::napi_create_string_utf8(
          raw_env,
          val.as_ptr() as *const _,
          val.len() as isize,
          &mut result,
        );
      }
    },
    Expr::Lit(Lit::Num(n)) => unsafe {
      napi::sys::napi_create_double(raw_env, n.value, &mut result);
    },
    Expr::Lit(Lit::Bool(b)) => unsafe {
      napi::sys::napi_get_boolean(raw_env, b.value, &mut result);
    },
    Expr::Object(obj) => unsafe {
      napi::sys::napi_create_object(raw_env, &mut result);
      for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
          && let Some(kv) = prop.as_key_value()
        {
          let Some(key) = prop_name_to_string(&kv.key) else {
            continue;
          };
          let prop_val = expr_to_napi_value(raw_env, &kv.value);
          let mut key_val: napi::sys::napi_value = std::ptr::null_mut();
          napi::sys::napi_create_string_utf8(
            raw_env,
            key.as_ptr() as *const _,
            key.len() as isize,
            &mut key_val,
          );
          napi::sys::napi_set_property(raw_env, result, key_val, prop_val);
        }
      }
    },
    Expr::Array(arr) => unsafe {
      napi::sys::napi_create_array_with_length(raw_env, arr.elems.len(), &mut result);
      for (i, elem) in arr.elems.iter().enumerate() {
        let elem_val = match elem {
          Some(e) => expr_to_napi_value(raw_env, &e.expr),
          None => {
            let mut undef: napi::sys::napi_value = std::ptr::null_mut();
            napi::sys::napi_get_undefined(raw_env, &mut undef);
            undef
          },
        };
        napi::sys::napi_set_element(raw_env, result, i as u32, elem_val);
      }
    },
    _ => {
      debug!("Unsupported napi value type: {:#?}.", expr);

      panic!(
        "Unsupported napi value type: {:?}. If its not enough, please run in debug mode to see more details",
        expr.get_type(get_default_expr_ctx())
      );
    },
  }
  result
}

/// Parses a JS string or function into a `JSFunction` for use as `debugFilePath`.
pub(crate) fn parse_debug_file_path(
  env: &napi::Env,
  unknown_val: Unknown,
) -> napi::Result<JSFunction> {
  let raw_val = unknown_val.raw();
  let raw_env = env.raw();

  let mut val_type: napi::sys::napi_valuetype = napi::sys::ValueType::napi_undefined;
  unsafe { napi::sys::napi_typeof(raw_env, raw_val, &mut val_type) };

  match debug_file_path_kind(val_type) {
    DebugFilePathKind::String => {
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
          buf.as_mut_ptr() as *mut _,
          len + 1,
          &mut written,
        );
      }
      let s = utf8_string_from_written_buffer(buf, written);
      Ok(JSFunction::new(move |_args| create_string_expr(&s)))
    },
    DebugFilePathKind::Function => parse_env_function(env, raw_val).and_then(|entry| match entry {
      EnvEntry::Function(f) => Ok(f),
      _ => Err(napi::Error::from_reason(
        "Expected function from parse_env_function",
      )),
    }),
    DebugFilePathKind::Unsupported => Err(napi::Error::from_reason(
      "debugFilePath must be a string or function",
    )),
  }
}

fn read_napi_string(raw_env: napi::sys::napi_env, value: napi::sys::napi_value) -> String {
  let mut len = 0;
  unsafe {
    napi::sys::napi_get_value_string_utf8(raw_env, value, std::ptr::null_mut(), 0, &mut len);
  }
  let mut buf = vec![0u8; len + 1];
  let mut written = 0;
  unsafe {
    napi::sys::napi_get_value_string_utf8(
      raw_env,
      value,
      buf.as_mut_ptr() as *mut _,
      len + 1,
      &mut written,
    );
  }
  utf8_string_from_written_buffer(buf, written)
}

fn napi_value_to_expr(raw_env: napi::sys::napi_env, value: napi::sys::napi_value) -> Expr {
  let mut val_type: napi::sys::napi_valuetype = napi::sys::ValueType::napi_undefined;
  unsafe {
    napi::sys::napi_typeof(raw_env, value, &mut val_type);
  }

  match val_type {
    napi::sys::ValueType::napi_string => create_string_expr(&read_napi_string(raw_env, value)),
    napi::sys::ValueType::napi_number => {
      let mut n: f64 = 0.0;
      unsafe {
        napi::sys::napi_get_value_double(raw_env, value, &mut n);
      }
      create_number_expr(n)
    },
    napi::sys::ValueType::napi_boolean => {
      let mut b = false;
      unsafe {
        napi::sys::napi_get_value_bool(raw_env, value, &mut b);
      }
      create_bool_expr(b)
    },
    napi::sys::ValueType::napi_object => {
      let mut is_array = false;
      unsafe {
        napi::sys::napi_is_array(raw_env, value, &mut is_array);
      }

      if is_array {
        let mut length: u32 = 0;
        unsafe {
          napi::sys::napi_get_array_length(raw_env, value, &mut length);
        }

        let elems: Vec<Option<ExprOrSpread>> = (0..length)
          .map(|i| {
            let mut elem_val: napi::sys::napi_value = std::ptr::null_mut();
            unsafe {
              napi::sys::napi_get_element(raw_env, value, i, &mut elem_val);
            }
            Some(ExprOrSpread {
              spread: None,
              expr: Box::new(napi_value_to_expr(raw_env, elem_val)),
            })
          })
          .collect();

        create_array_expression(elems)
      } else {
        let mut props = Vec::new();

        let mut property_names: napi::sys::napi_value = std::ptr::null_mut();
        unsafe {
          napi::sys::napi_get_property_names(raw_env, value, &mut property_names);
        }

        let mut length: u32 = 0;
        unsafe {
          napi::sys::napi_get_array_length(raw_env, property_names, &mut length);
        }

        for i in 0..length {
          let mut key_val: napi::sys::napi_value = std::ptr::null_mut();
          unsafe {
            napi::sys::napi_get_element(raw_env, property_names, i, &mut key_val);
          }
          let key = read_napi_string(raw_env, key_val);

          let mut prop_val: napi::sys::napi_value = std::ptr::null_mut();
          unsafe {
            napi::sys::napi_get_property(raw_env, value, key_val, &mut prop_val);
          }

          props.push(create_key_value_prop(
            &key,
            napi_value_to_expr(raw_env, prop_val),
          ));
        }

        create_object_expression(props)
      }
    },
    _ => {
      debug!("Unsupported napi value type: {:#?}.", val_type);

      panic!(
        "Unsupported napi value type: {:?}. If its not enough, please run in debug mode to see more details",
        val_type
      );
    },
  }
}

/// Wrapper for a raw napi_ref with its associated environment.
struct EnvFnRef {
  ref_ptr: napi::sys::napi_ref,
  env: napi::sys::napi_env,
}

impl Drop for EnvFnRef {
  fn drop(&mut self) {
    let status = unsafe { napi::sys::napi_delete_reference(self.env, self.ref_ptr) };
    if status != napi::sys::Status::napi_ok {
      log::warn!(
        "Failed to delete napi_ref during cleanup (status: {:?})",
        status
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use swc_core::common::DUMMY_SP;
  use swc_core::ecma::ast::{ComputedPropName, IdentName, Number, Str};

  use super::*;

  #[test]
  fn env_value_kind_classifies_variants() {
    assert_eq!(env_value_kind(ValueType::String), EnvValueKind::String);
    assert_eq!(env_value_kind(ValueType::Number), EnvValueKind::Number);
    assert_eq!(env_value_kind(ValueType::Boolean), EnvValueKind::Boolean);
    assert_eq!(env_value_kind(ValueType::Null), EnvValueKind::Nullish);
    assert_eq!(env_value_kind(ValueType::Undefined), EnvValueKind::Nullish);
    assert_eq!(env_value_kind(ValueType::Function), EnvValueKind::Function);
    assert_eq!(env_value_kind(ValueType::Object), EnvValueKind::Object);
    assert_eq!(env_value_kind(ValueType::Symbol), EnvValueKind::Unsupported);
  }

  #[test]
  fn debug_file_path_kind_classifies_variants() {
    assert_eq!(
      debug_file_path_kind(napi::sys::ValueType::napi_string),
      DebugFilePathKind::String
    );
    assert_eq!(
      debug_file_path_kind(napi::sys::ValueType::napi_function),
      DebugFilePathKind::Function
    );
    assert_eq!(
      debug_file_path_kind(napi::sys::ValueType::napi_boolean),
      DebugFilePathKind::Unsupported
    );
  }

  #[test]
  fn utf8_string_from_written_buffer_truncates_and_decodes() {
    let buf = vec![b'a', b'b', b'c', b'd'];
    assert_eq!(utf8_string_from_written_buffer(buf, 3), "abc");
  }

  #[test]
  fn utf8_string_from_written_buffer_returns_default_for_invalid_utf8() {
    let buf = vec![0xff, 0xfe, 0xfd];
    assert_eq!(utf8_string_from_written_buffer(buf, 3), "");
  }

  #[test]
  fn prop_name_to_string_supports_ident_str_and_num() {
    let ident = PropName::Ident(IdentName::new("foo".into(), DUMMY_SP));
    let str_key = PropName::Str(Str {
      span: DUMMY_SP,
      value: "bar".into(),
      raw: None,
    });
    let num_key = PropName::Num(Number {
      span: DUMMY_SP,
      value: 42.0,
      raw: None,
    });

    assert_eq!(prop_name_to_string(&ident), Some("foo".to_string()));
    assert_eq!(prop_name_to_string(&str_key), Some("bar".to_string()));
    assert_eq!(prop_name_to_string(&num_key), Some("42".to_string()));
  }

  #[test]
  fn prop_name_to_string_rejects_computed_keys() {
    let computed = PropName::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    });

    assert_eq!(prop_name_to_string(&computed), None);
  }
}
