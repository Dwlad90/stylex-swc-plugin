use indexmap::IndexMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Bool, Expr, Lit, Null, PropOrSpread, SpreadElement},
};

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::types::FlatCompiledStyles,
  utils::ast::factories::{
    object_expression_factory, prop_or_spread_expression_factory, prop_or_spread_string_factory,
  },
};

pub(crate) fn remove_objects_with_spreads(
  obj: &IndexMap<String, Box<FlatCompiledStyles>>,
) -> IndexMap<String, Box<FlatCompiledStyles>> {
  let mut obj = obj.clone();

  obj.retain(|_key, value| {
    value.values().all(|keep_value| {
      !matches!(
        *keep_value.clone(),
        FlatCompiledStylesValue::IncludedStyle(_)
      )
    })
  });

  obj
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NestedStringObject {
  FlatCompiledStyles(IndexMap<String, Box<FlatCompiledStyles>>),
  FlatCompiledStylesValues(IndexMap<String, Box<FlatCompiledStylesValue>>),
}

impl NestedStringObject {
  pub(crate) fn _as_styles(&self) -> Option<&IndexMap<String, Box<FlatCompiledStyles>>> {
    match self {
      NestedStringObject::FlatCompiledStyles(obj) => Some(obj),
      _ => None,
    }
  }

  pub(crate) fn as_values(&self) -> Option<&IndexMap<String, Box<FlatCompiledStylesValue>>> {
    match self {
      NestedStringObject::FlatCompiledStylesValues(obj) => Some(obj),
      _ => None,
    }
  }
}

pub(crate) fn convert_object_to_ast(obj: &NestedStringObject) -> Expr {
  let mut props: Vec<PropOrSpread> = vec![];

  match obj {
    NestedStringObject::FlatCompiledStyles(obj) => {
      for (key, value) in obj.iter() {
        let expr = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
          *value.clone(),
        ));

        let prop = prop_or_spread_expression_factory(key.as_str(), Box::new(expr));

        props.push(prop);
      }
    }
    NestedStringObject::FlatCompiledStylesValues(obj) => {
      for (key, value) in obj.iter() {
        let prop = match value.as_ref() {
          FlatCompiledStylesValue::String(value) => {
            prop_or_spread_string_factory(key.as_str(), value.as_str())
          }
          FlatCompiledStylesValue::Null => prop_or_spread_expression_factory(
            key.as_str(),
            Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
          ),
          FlatCompiledStylesValue::IncludedStyle(include_style) => {
            PropOrSpread::Spread(SpreadElement {
              dot3_token: DUMMY_SP,
              expr: Box::new(include_style.get_expr().clone()),
            })
          }
          FlatCompiledStylesValue::Bool(value) => prop_or_spread_expression_factory(
            key.as_str(),
            Box::new(Expr::Lit(Lit::Bool(Bool {
              span: DUMMY_SP,
              value: *value,
            }))),
          ),
          _ => unreachable!("Unsupported value type"),
        };

        props.push(prop);
      }
    }
  }

  object_expression_factory(props).unwrap()
}
