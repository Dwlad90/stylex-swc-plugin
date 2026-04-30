// Tests for function parser expression helpers and mutation detection.
// Source: crates/stylex-rs-compiler/src/utils/fn_parser.rs

use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{ComputedPropName, IdentName, Number, Str},
};

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

#[test]
fn napi_env_guard_clears_thread_local_on_normal_return() {
  // Verify the guard pattern: after with_napi_env-like scope,
  // the thread-local should be cleared.
  // We can't test with real napi::Env, but we can test the guard behavior
  // directly on the thread-local.
  NAPI_ENV_RAW.set(Some(std::ptr::null_mut()));
  {
    let _guard = NapiEnvGuard;
    assert!(NAPI_ENV_RAW.get().is_some());
  }
  assert!(NAPI_ENV_RAW.get().is_none(), "guard should clear on drop");
}

#[test]
fn napi_env_guard_clears_thread_local_on_panic() {
  // Verify the guard clears the thread-local even when unwinding from panic.
  NAPI_ENV_RAW.set(Some(std::ptr::null_mut()));
  let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    let _guard = NapiEnvGuard;
    panic!("simulated panic inside guarded scope");
  }));
  assert!(result.is_err(), "should have caught the panic");
  assert!(
    NAPI_ENV_RAW.get().is_none(),
    "guard should clear on panic unwind"
  );
}

#[test]
fn utf8_string_from_written_buffer_handles_zero_length() {
  let buf = vec![b'a', b'b'];
  assert_eq!(utf8_string_from_written_buffer(buf, 0), "");
}

#[test]
fn utf8_string_from_written_buffer_handles_full_buffer() {
  let buf = vec![b'h', b'e', b'l', b'l', b'o'];
  assert_eq!(utf8_string_from_written_buffer(buf, 5), "hello");
}

#[test]
fn debug_file_path_kind_classifies_all_napi_types() {
  assert_eq!(
    debug_file_path_kind(napi::sys::ValueType::napi_undefined),
    DebugFilePathKind::Unsupported
  );
  assert_eq!(
    debug_file_path_kind(napi::sys::ValueType::napi_null),
    DebugFilePathKind::Unsupported
  );
  assert_eq!(
    debug_file_path_kind(napi::sys::ValueType::napi_number),
    DebugFilePathKind::Unsupported
  );
  assert_eq!(
    debug_file_path_kind(napi::sys::ValueType::napi_object),
    DebugFilePathKind::Unsupported
  );
}

#[test]
fn env_value_kind_exhaustive() {
  // Ensure all ValueType variants are covered
  assert_eq!(env_value_kind(ValueType::String), EnvValueKind::String);
  assert_eq!(env_value_kind(ValueType::Number), EnvValueKind::Number);
  assert_eq!(env_value_kind(ValueType::Boolean), EnvValueKind::Boolean);
  assert_eq!(env_value_kind(ValueType::Null), EnvValueKind::Nullish);
  assert_eq!(env_value_kind(ValueType::Undefined), EnvValueKind::Nullish);
  assert_eq!(env_value_kind(ValueType::Function), EnvValueKind::Function);
  assert_eq!(env_value_kind(ValueType::Object), EnvValueKind::Object);
  assert_eq!(env_value_kind(ValueType::Symbol), EnvValueKind::Unsupported);
  assert_eq!(
    env_value_kind(ValueType::External),
    EnvValueKind::Unsupported
  );
}

#[test]
fn utf8_string_from_written_buffer_handles_unicode() {
  let s = "héllo";
  let buf = s.as_bytes().to_vec();
  assert_eq!(utf8_string_from_written_buffer(buf.clone(), buf.len()), s);
}

#[test]
fn utf8_string_from_written_buffer_handles_single_char() {
  let buf = vec![b'x'];
  assert_eq!(utf8_string_from_written_buffer(buf, 1), "x");
}

#[test]
fn prop_name_to_string_supports_numeric_zero() {
  let num_key = PropName::Num(Number {
    span: DUMMY_SP,
    value: 0.0,
    raw: None,
  });
  assert_eq!(prop_name_to_string(&num_key), Some("0".to_string()));
}

#[test]
fn prop_name_to_string_supports_negative_num() {
  let num_key = PropName::Num(Number {
    span: DUMMY_SP,
    value: -1.5,
    raw: None,
  });
  assert_eq!(prop_name_to_string(&num_key), Some("-1.5".to_string()));
}

#[test]
fn prop_name_to_string_supports_empty_ident() {
  let ident = PropName::Ident(IdentName::new("".into(), DUMMY_SP));
  assert_eq!(prop_name_to_string(&ident), Some(String::new()));
}

#[test]
fn prop_name_to_string_supports_empty_str() {
  let str_key = PropName::Str(Str {
    span: DUMMY_SP,
    value: "".into(),
    raw: None,
  });
  assert_eq!(prop_name_to_string(&str_key), Some(String::new()));
}

#[test]
fn napi_env_guard_no_op_when_already_none() {
  NAPI_ENV_RAW.set(None);
  {
    let _guard = NapiEnvGuard;
  }
  assert!(NAPI_ENV_RAW.get().is_none());
}

#[test]
fn napi_env_guard_multiple_guards_clear_correctly() {
  NAPI_ENV_RAW.set(Some(std::ptr::null_mut()));
  {
    let _guard1 = NapiEnvGuard;
    // Simulate nested guard (though unusual)
    NAPI_ENV_RAW.set(Some(std::ptr::null_mut()));
    {
      let _guard2 = NapiEnvGuard;
    }
    // After inner guard drops, should be None
    assert!(NAPI_ENV_RAW.get().is_none());
  }
  // Outer guard also clears
  assert!(NAPI_ENV_RAW.get().is_none());
}
