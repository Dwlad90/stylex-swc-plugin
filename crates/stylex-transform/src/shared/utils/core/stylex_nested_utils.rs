use std::rc::Rc;

use indexmap::IndexMap;
use stylex_ast::ast::factories::{
  create_key_value_prop, create_object_expression, create_string_key_value_prop,
};
use stylex_constants::constants::common::VAR_GROUP_HASH_KEY;
use stylex_macros::stylex_unreachable;
use stylex_structures::nested::SEPARATOR;
use swc_core::ecma::ast::Expr;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::types::FlatCompiledStyles,
  utils::ast::convertors::{
    create_bool_expr, create_null_expr, create_number_expr, create_string_expr,
  },
};

const COMPILED_SPECIAL_KEY: &str = "$$css";

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UnflattenedCompiledStylesValue {
  Leaf(Rc<FlatCompiledStylesValue>),
  Object(IndexMap<String, UnflattenedCompiledStylesValue>),
}

pub(crate) fn unflatten_object(
  flat_obj: &FlatCompiledStyles,
) -> IndexMap<String, UnflattenedCompiledStylesValue> {
  let mut result = IndexMap::new();

  for (key, value) in flat_obj {
    if is_special_key(key) || !key.contains(SEPARATOR) {
      result.insert(
        key.clone(),
        UnflattenedCompiledStylesValue::Leaf(value.clone()),
      );
      continue;
    }

    let parts = key.split(SEPARATOR).collect::<Vec<_>>();
    let mut current = &mut result;

    for part in parts.iter().take(parts.len() - 1) {
      let entry = current
        .entry((*part).to_string())
        .or_insert_with(|| UnflattenedCompiledStylesValue::Object(IndexMap::new()));

      if !matches!(entry, UnflattenedCompiledStylesValue::Object(_)) {
        *entry = UnflattenedCompiledStylesValue::Object(IndexMap::new());
      }

      match entry {
        UnflattenedCompiledStylesValue::Object(map) => current = map,
        UnflattenedCompiledStylesValue::Leaf(_) => {
          stylex_unreachable!("Expected unflattened intermediate object.")
        },
      }
    }

    let leaf_key = match parts.last() {
      Some(key) => (*key).to_string(),
      None => stylex_unreachable!("Expected at least one key part."),
    };

    current.insert(
      leaf_key,
      UnflattenedCompiledStylesValue::Leaf(value.clone()),
    );
  }

  result
}

pub(crate) fn expr_map_to_evaluate_result(map: IndexMap<String, Expr>) -> EvaluateResultValue {
  EvaluateResultValue::Expr(create_object_expression(
    map
      .into_iter()
      .map(|(key, value)| create_key_value_prop(&key, value))
      .collect(),
  ))
}

pub(crate) fn string_map_to_evaluate_result(map: IndexMap<String, String>) -> EvaluateResultValue {
  EvaluateResultValue::Expr(create_object_expression(
    map
      .into_iter()
      .map(|(key, value)| create_string_key_value_prop(&key, &value))
      .collect(),
  ))
}

pub(crate) fn convert_unflattened_object_to_ast(
  obj: &IndexMap<String, UnflattenedCompiledStylesValue>,
) -> Expr {
  create_object_expression(
    obj
      .iter()
      .map(|(key, value)| create_key_value_prop(key, unflattened_value_to_ast(value)))
      .collect(),
  )
}

fn unflattened_value_to_ast(value: &UnflattenedCompiledStylesValue) -> Expr {
  match value {
    UnflattenedCompiledStylesValue::Object(map) => convert_unflattened_object_to_ast(map),
    UnflattenedCompiledStylesValue::Leaf(value) => match value.as_ref() {
      FlatCompiledStylesValue::String(value) => {
        if let Ok(num) = value.parse::<f64>() {
          create_number_expr(num)
        } else {
          create_string_expr(value)
        }
      },
      FlatCompiledStylesValue::Null => create_null_expr(),
      FlatCompiledStylesValue::Bool(value) => create_bool_expr(*value),
      _ => {
        stylex_unreachable!("Encountered an unsupported value type during nested AST conversion.")
      },
    },
  }
}

fn is_special_key(key: &str) -> bool {
  key == VAR_GROUP_HASH_KEY || key == COMPILED_SPECIAL_KEY
}
