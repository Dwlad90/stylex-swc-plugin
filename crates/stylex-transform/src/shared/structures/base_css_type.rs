use indexmap::IndexMap;
use stylex_macros::stylex_panic;
use swc_core::ecma::{
  ast::{Expr, ObjectLit, PropOrSpread},
  utils::ExprExt,
};

use crate::shared::{
  constants::messages::VALUE_MUST_BE_STRING,
  enums::data_structures::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault},
  swc::get_default_expr_ctx,
  utils::{
    ast::{
      convertors::{key_value_to_str, convert_lit_to_string},
      factories::{
        create_object_expression, create_object_lit, create_key_value_prop,
        create_string_key_value_prop,
      },
    },
    common::get_key_values_from_object,
  },
};

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct BaseCSSType {
  pub value: ValueWithDefault,
  pub syntax: CSSSyntax,
}

impl BaseCSSType {
  pub fn value_to_props(value: ValueWithDefault, top_key: Option<String>) -> Vec<PropOrSpread> {
    match value {
      ValueWithDefault::Number(n) => {
        let value_prop = create_string_key_value_prop(
          top_key.unwrap_or(String::from("value")).as_str(),
          n.to_string().as_str(),
        );
        let props = vec![value_prop];
        props
      }
      ValueWithDefault::String(s) => {
        let value_prop = create_string_key_value_prop(
          top_key.unwrap_or(String::from("value")).as_str(),
          s.as_str(),
        );
        let props = vec![value_prop];

        props
      }
      ValueWithDefault::Map(map) => {
        let mut local_props = vec![];

        for (key, val) in map {
          let props_to_extend = BaseCSSType::value_to_props(val, Some(key));

          local_props.extend(props_to_extend);
        }

        let object_expr = create_object_expression(local_props);
        let prop = create_key_value_prop(
          top_key.unwrap_or("value".to_string()).as_str(),
          object_expr,
        );

        vec![prop]
      }
    }
  }
}

impl From<ObjectLit> for BaseCSSType {
  fn from(obj: ObjectLit) -> BaseCSSType {
    let key_values = get_key_values_from_object(&obj);
    let mut syntax: Option<CSSSyntax> = None;

    let mut values: IndexMap<String, ValueWithDefault> = IndexMap::new();

    for key_value in key_values {
      let key = key_value_to_str(&key_value);

      match key.as_str() {
        "syntax" => {
          syntax = key_value
            .value
            .as_lit()
            .and_then(convert_lit_to_string)
            .map(|str_val| str_val.into())
        }
        "value" => {
          let obj_value = match key_value.value.as_ref() {
            Expr::Object(obj) => obj,
            Expr::Lit(obj) => {
              let value = match convert_lit_to_string(obj) {
                Some(v) => v,
                None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
              };

              let prop = create_string_key_value_prop("default", value.as_str());

              &create_object_lit(vec![prop])
            }
            _ => stylex_panic!(
              "Value must be an object or string, but got: {:?}",
              key_value.value.get_type(get_default_expr_ctx())
            ),
          };

          for key_value in get_key_values_from_object(obj_value) {
            let key = key_value_to_str(&key_value);

            match key_value.value.as_ref() {
              Expr::Object(obj) => {
                let mut obj_map = IndexMap::new();

                let key_values = get_key_values_from_object(obj);

                for key_value in key_values {
                  let key = key_value_to_str(&key_value);

                  match key_value.value.as_ref() {
                    Expr::Lit(lit) => {
                      let value = match convert_lit_to_string(lit) {
                        Some(v) => v,
                        None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
                      };

                      obj_map.insert(key, ValueWithDefault::String(value));
                    }
                    _ => stylex_panic!(
                      "Value must be a string, but got: {:?}",
                      key_value.value.get_type(get_default_expr_ctx())
                    ),
                  }
                }

                let value = ValueWithDefault::Map(obj_map);

                values.insert(key, value);
              }
              Expr::Lit(lit) => {
                let value = match convert_lit_to_string(lit) {
                  Some(v) => v,
                  None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
                };

                values.insert(key, ValueWithDefault::String(value));
              }
              _ => stylex_panic!(
                "Value must be a string or object, but got: {:?}",
                key_value.value.get_type(get_default_expr_ctx())
              ),
            }
          }
        }
        _ => {
          stylex_panic!(r#"Key "{}" not support by BaseCSSType"#, key)
        }
      }
    }

    assert!(!values.is_empty(), "Invalid value in defineVars");

    assert!(
      values.contains_key("default"),
      "CSS type requires a default value but none was provided."
    );

    BaseCSSType {
      value: ValueWithDefault::Map(values),
      syntax: match syntax {
        Some(s) => s,
        None => stylex_panic!("CSS syntax definition is required for this type."),
      },
    }
  }
}
