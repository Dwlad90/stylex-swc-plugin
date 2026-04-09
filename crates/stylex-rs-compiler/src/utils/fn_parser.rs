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
  match value.get_type()? {
    ValueType::String => {
      let s: JsString = unsafe { value.cast()? };
      Ok(EnvEntry::Expr(create_string_expr(s.into_utf8()?.as_str()?)))
    },
    ValueType::Number => {
      let n: JsNumber = unsafe { value.cast()? };
      Ok(EnvEntry::Expr(create_number_expr(n.get_double()?)))
    },
    ValueType::Boolean => {
      let raw_env = env.raw();
      let mut b = false;
      let status = unsafe { napi::sys::napi_get_value_bool(raw_env, value.raw(), &mut b) };
      if status != napi::sys::Status::napi_ok {
        return Err(napi::Error::from_reason("Failed to get boolean value"));
      }
      Ok(EnvEntry::Expr(create_bool_expr(b)))
    },
    ValueType::Null | ValueType::Undefined => Ok(EnvEntry::Expr(create_null_expr())),
    ValueType::Function => parse_env_function(env, value.raw()),
    ValueType::Object => Ok(EnvEntry::Expr(napi_value_to_expr(env.raw(), value.raw()))),
    _ => Ok(EnvEntry::Expr(create_null_expr())),
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
          let key = match &kv.key {
            PropName::Ident(id) => id.sym.to_string(),
            PropName::Str(s) => s.value.as_str().unwrap_or("").to_string(),
            PropName::Num(n) => n.value.to_string(),
            _ => continue,
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

  match val_type {
    napi::sys::ValueType::napi_string => {
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
      buf.truncate(written);
      let s = String::from_utf8(buf).unwrap_or_default();
      Ok(JSFunction::new(move |_args| create_string_expr(&s)))
    },
    napi::sys::ValueType::napi_function => {
      parse_env_function(env, raw_val).and_then(|entry| match entry {
        EnvEntry::Function(f) => Ok(f),
        _ => Err(napi::Error::from_reason(
          "Expected function from parse_env_function",
        )),
      })
    },
    _ => Err(napi::Error::from_reason(
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
  buf.truncate(written);
  String::from_utf8(buf).unwrap_or_default()
}

fn napi_value_to_expr(raw_env: napi::sys::napi_env, value: napi::sys::napi_value) -> Expr {
  let mut val_type: napi::sys::napi_valuetype = napi::sys::ValueType::napi_undefined;
  unsafe {
    napi::sys::napi_typeof(raw_env, value, &mut val_type);
  }

  match val_type {
    napi::sys::ValueType::napi_string => create_string_expr(read_napi_string(raw_env, value)),
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
