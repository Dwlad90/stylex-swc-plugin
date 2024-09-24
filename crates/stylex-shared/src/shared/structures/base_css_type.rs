use indexmap::IndexMap;
use swc_core::ecma::{
  ast::{Expr, ObjectLit, PropOrSpread},
  utils::ExprExt,
};

use crate::shared::{
  enums::data_structures::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault},
  utils::{
    ast::factories::{
      object_expression_factory, object_lit_factory, prop_or_spread_expression_factory,
      prop_or_spread_string_factory,
    },
    common::{get_key_str, get_key_values_from_object, get_string_val_from_lit},
  },
};

#[derive(Debug, PartialEq, Clone, Hash)]
pub(crate) struct BaseCSSType {
  pub(crate) value: ValueWithDefault,
  pub(crate) syntax: CSSSyntax,
}

impl BaseCSSType {
  pub(crate) fn value_to_props(
    value: ValueWithDefault,
    top_key: Option<String>,
  ) -> Vec<PropOrSpread> {
    let value = match value {
      ValueWithDefault::Number(n) => {
        let value_prop = prop_or_spread_string_factory(
          top_key.unwrap_or(String::from("value")).as_str(),
          n.to_string().as_str(),
        );
        let props = vec![value_prop];
        props
      }
      ValueWithDefault::String(s) => {
        let value_prop = prop_or_spread_string_factory(
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

        let object_expr = object_expression_factory(local_props);
        let prop = prop_or_spread_expression_factory(
          top_key.unwrap_or("value".to_string()).as_str(),
          object_expr,
        );

        vec![prop]
      }
    };

    value
  }
}

impl From<ObjectLit> for BaseCSSType {
  fn from(obj: ObjectLit) -> BaseCSSType {
    let key_values = get_key_values_from_object(&obj);
    let mut syntax: Option<CSSSyntax> = None;

    let mut values: IndexMap<String, ValueWithDefault> = IndexMap::new();

    for key_value in key_values {
      let key = get_key_str(&key_value);

      match key.as_str() {
        "syntax" => {
          syntax = key_value
            .value
            .as_lit()
            .and_then(get_string_val_from_lit)
            .map(|str_val| str_val.into())
        }
        "value" => {
          let obj_value = match key_value.value.as_ref() {
            Expr::Object(obj) => obj,
            Expr::Lit(obj) => {
              let value = get_string_val_from_lit(obj).expect("Value must be a string");

              let prop = prop_or_spread_string_factory("default", value.as_str());

              &object_lit_factory(vec![prop])
            }
            _ => panic!(
              "Value must be an object or string, but got: {:?}",
              key_value.value.get_type()
            ),
          };

          for key_value in get_key_values_from_object(obj_value) {
            let key = get_key_str(&key_value);

            match key_value.value.as_ref() {
              Expr::Object(obj) => {
                let mut obj_map = IndexMap::new();

                let key_values = get_key_values_from_object(obj);

                for key_value in key_values {
                  let key = get_key_str(&key_value);

                  match key_value.value.as_ref() {
                    Expr::Lit(lit) => {
                      let value = get_string_val_from_lit(lit).expect("Value must be a string");

                      obj_map.insert(key, ValueWithDefault::String(value));
                    }
                    _ => panic!(
                      "Value must be a string, but got: {:?}",
                      key_value.value.get_type()
                    ),
                  }
                }

                let value = ValueWithDefault::Map(obj_map);

                values.insert(key, value);
              }
              Expr::Lit(lit) => {
                let value = get_string_val_from_lit(lit).expect("Value must be a string");

                values.insert(key, ValueWithDefault::String(value));
              }
              _ => panic!(
                "Value must be a string or object, but got: {:?}",
                key_value.value.get_type()
              ),
            }
          }
        }
        _ => {
          panic!(r#"Key "{}" not support by BaseCSSType"#, key)
        }
      }
    }

    assert!(!values.is_empty(), "Invalid value in stylex.defineVars");

    assert!(
      values.contains_key("default"),
      "Default value is not defined for variable."
    );

    BaseCSSType {
      value: ValueWithDefault::Map(values),
      syntax: syntax.expect("Syntax is required"),
    }
  }
}
