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
