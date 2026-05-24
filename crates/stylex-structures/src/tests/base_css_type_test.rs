//! Tests for `BaseCSSType` and its `From` impls.

use crate::base_css_type::BaseCSSType;
use indexmap::IndexMap;
use stylex_ast::ast::factories::{create_key_value_prop, create_object_lit, create_string_lit};
use stylex_enums::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault};
use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

// ---------- helpers ----------

fn obj_with(props: Vec<(&str, Expr)>) -> ObjectLit {
  create_object_lit(
    props
      .into_iter()
      .map(|(k, v)| create_key_value_prop(k, v))
      .collect(),
  )
}

fn unwrap_props(expr: Expr) -> Vec<KeyValueProp> {
  match expr {
    Expr::Object(o) => o
      .props
      .into_iter()
      .map(|p| match p {
        PropOrSpread::Prop(prop) => match *prop {
          Prop::KeyValue(kv) => kv,
          _ => panic!("expected key-value prop"),
        },
        _ => panic!("expected non-spread prop"),
      })
      .collect(),
    _ => panic!("expected object expression"),
  }
}

fn key_of(kv: &KeyValueProp) -> String {
  match &kv.key {
    PropName::Str(s) => s.value.as_str().unwrap_or("").to_string(),
    PropName::Ident(i) => i.sym.to_string(),
    _ => panic!("unexpected key kind"),
  }
}

// ---------- value_to_props ----------

#[test]
fn value_to_props_number_default_key() {
  let props = BaseCSSType::value_to_props(ValueWithDefault::Number(2.5), None);
  assert_eq!(props.len(), 1);
  let kv = unwrap_props(Expr::Object(create_object_lit(props)))
    .pop()
    .unwrap();
  assert_eq!(key_of(&kv), "value");
}

#[test]
fn value_to_props_number_custom_key() {
  let props = BaseCSSType::value_to_props(ValueWithDefault::Number(7.0), Some("custom".into()));
  let kv = unwrap_props(Expr::Object(create_object_lit(props)))
    .pop()
    .unwrap();
  assert_eq!(key_of(&kv), "custom");
}

#[test]
fn value_to_props_string_default_key() {
  let props = BaseCSSType::value_to_props(ValueWithDefault::String("red".into()), None);
  let kv = unwrap_props(Expr::Object(create_object_lit(props)))
    .pop()
    .unwrap();
  assert_eq!(key_of(&kv), "value");
}

#[test]
fn value_to_props_string_custom_key() {
  let props = BaseCSSType::value_to_props(
    ValueWithDefault::String("blue".into()),
    Some("primary".into()),
  );
  let kv = unwrap_props(Expr::Object(create_object_lit(props)))
    .pop()
    .unwrap();
  assert_eq!(key_of(&kv), "primary");
}

#[test]
fn value_to_props_map_uses_nested_keys() {
  let mut inner = IndexMap::new();
  inner.insert("default".to_string(), ValueWithDefault::String("a".into()));
  inner.insert("hover".to_string(), ValueWithDefault::String("b".into()));
  let props = BaseCSSType::value_to_props(ValueWithDefault::Map(inner), None);
  let kvs = unwrap_props(Expr::Object(create_object_lit(props)));
  assert_eq!(kvs.len(), 1);
  assert_eq!(key_of(&kvs[0]), "value");
}

// ---------- From<BaseCSSType> for Expr ----------

#[test]
fn into_expr_emits_syntax_and_value_props() {
  let mut inner = IndexMap::new();
  inner.insert(
    "default".to_string(),
    ValueWithDefault::String("red".into()),
  );
  let css = BaseCSSType {
    value: ValueWithDefault::Map(inner),
    syntax: CSSSyntax::Color,
  };
  let expr: Expr = css.into();
  let kvs = unwrap_props(expr);
  // Expect at least a "syntax" prop and a "value" prop.
  assert!(kvs.iter().any(|kv| key_of(kv) == "syntax"));
  assert!(kvs.iter().any(|kv| key_of(kv) == "value"));
}

// ---------- From<ObjectLit> for BaseCSSType ----------

#[test]
fn from_object_lit_parses_syntax_and_default_value() {
  let value_obj = obj_with(vec![("default", Expr::Lit(create_string_lit("red")))]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let css: BaseCSSType = outer.into();
  assert_eq!(css.syntax, CSSSyntax::Color);
  if let ValueWithDefault::Map(map) = &css.value {
    assert!(map.contains_key("default"));
  } else {
    panic!("expected Map value");
  }
}

#[test]
fn from_object_lit_promotes_string_value_to_default() {
  // value: "red" → value: { default: "red" }
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Lit(create_string_lit("red"))),
  ]);
  let css: BaseCSSType = outer.into();
  if let ValueWithDefault::Map(map) = &css.value {
    assert!(map.contains_key("default"));
  } else {
    panic!("expected Map value");
  }
}

#[test]
fn from_object_lit_accepts_nested_value_map() {
  let nested = obj_with(vec![("hover", Expr::Lit(create_string_lit("blue")))]);
  let value_obj = obj_with(vec![
    ("default", Expr::Lit(create_string_lit("red"))),
    ("states", Expr::Object(nested)),
  ]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let css: BaseCSSType = outer.into();
  if let ValueWithDefault::Map(map) = &css.value {
    assert!(map.contains_key("states"));
    if let Some(ValueWithDefault::Map(nested_map)) = map.get("states") {
      assert!(nested_map.contains_key("hover"));
    } else {
      panic!("expected nested Map");
    }
  } else {
    panic!("expected Map value");
  }
}

#[test]
#[should_panic(expected = "Key")]
fn from_object_lit_panics_on_unknown_top_key() {
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Lit(create_string_lit("red"))),
    ("extra", Expr::Lit(create_string_lit("nope"))),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Value must")]
fn from_object_lit_panics_on_non_lit_non_object_value() {
  // Use an Ident expr → not a Lit, not an Object → hits the catch-all panic.
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    (
      "value",
      Expr::Ident(stylex_ast::ast::factories::create_ident("noSuchThing")),
    ),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Invalid value")]
fn from_object_lit_panics_on_empty_values_map() {
  // value is an empty object → no entries inserted → assertion fires.
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(create_object_lit(vec![]))),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "default value")]
fn from_object_lit_panics_when_no_default_key() {
  let value_obj = obj_with(vec![("hover", Expr::Lit(create_string_lit("blue")))]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "CSS syntax")]
fn from_object_lit_panics_when_syntax_missing() {
  let value_obj = obj_with(vec![("default", Expr::Lit(create_string_lit("red")))]);
  let outer = obj_with(vec![("value", Expr::Object(value_obj))]);
  let _: BaseCSSType = outer.into();
}

// Helpers for triggering specific panic arms.

fn null_lit_expr() -> Expr {
  Expr::Lit(swc_core::ecma::ast::Lit::Null(swc_core::ecma::ast::Null {
    span: swc_core::common::DUMMY_SP,
  }))
}

fn ident_expr() -> Expr {
  Expr::Ident(stylex_ast::ast::factories::create_ident("bad"))
}

#[test]
#[should_panic(expected = "Expected a string value")]
fn from_object_lit_panics_when_promoted_lit_is_not_string() {
  // value: Null (a literal but not a string) → promote path fails string
  // conversion → VALUE_MUST_BE_STRING.
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", null_lit_expr()),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Expected a string value")]
fn from_object_lit_panics_when_deeply_nested_value_is_null_lit() {
  // value: { default: { hover: null } } → inner-lit path → string conversion fails.
  let inner_inner = obj_with(vec![("hover", null_lit_expr())]);
  let value_obj = obj_with(vec![
    ("default", Expr::Lit(create_string_lit("red"))),
    ("states", Expr::Object(inner_inner)),
  ]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Value must be a string")]
fn from_object_lit_panics_when_deeply_nested_value_is_non_lit_non_object() {
  // value: { default: ..., states: { hover: <ident> } } → catch-all panic in
  // inner loop.
  let inner_inner = obj_with(vec![("hover", ident_expr())]);
  let value_obj = obj_with(vec![
    ("default", Expr::Lit(create_string_lit("red"))),
    ("states", Expr::Object(inner_inner)),
  ]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Expected a string value")]
fn from_object_lit_panics_when_mid_level_value_is_null_lit() {
  // value: { default: null } → middle Lit branch → string conversion fails.
  let value_obj = obj_with(vec![("default", null_lit_expr())]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let _: BaseCSSType = outer.into();
}

#[test]
#[should_panic(expected = "Value must be a string or object")]
fn from_object_lit_panics_when_mid_level_value_is_non_lit_non_object() {
  // value: { default: <ident> } → catch-all panic in middle loop.
  let value_obj = obj_with(vec![("default", ident_expr())]);
  let outer = obj_with(vec![
    ("syntax", Expr::Lit(create_string_lit("<color>"))),
    ("value", Expr::Object(value_obj)),
  ]);
  let _: BaseCSSType = outer.into();
}
