use swc_core::{
  common::{Span, DUMMY_SP},
  ecma::ast::{ArrayLit, Expr, ExprOrSpread, ObjectLit, PropOrSpread},
};
use swc_ecma_ast::{Bool, Ident, KeyValueProp, Lit, Prop, PropName};

use crate::shared::constants::messages::NON_STATIC_VALUE;

use super::convertors::{number_to_expression, string_to_expression, string_to_prop_name};

pub fn object_expression_factory(props: Vec<PropOrSpread>) -> Option<Expr> {
  Some(Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props,
  }))
}

pub(crate) fn array_expression_factory(elems: Vec<Option<ExprOrSpread>>) -> Option<Expr> {
  Some(Expr::Array(ArrayLit {
    span: DUMMY_SP,
    elems,
  }))
}

pub fn prop_or_spread_expression_factory(key: &str, value: Box<Expr>) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: string_to_prop_name(key).unwrap(),
    value,
  })))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn prop_or_spread_expr_factory(key: &str, values: Vec<PropOrSpread>) -> PropOrSpread {
  let object = ObjectLit {
    span: DUMMY_SP,
    props: values,
  };

  prop_or_spread_expression_factory(key, Box::new(Expr::Object(object)))
}

pub fn key_value_factory(key: &str, value: Expr) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Ident(Ident::new(key.into(), DUMMY_SP)),
    value: Box::new(value),
  }
}

pub(crate) fn prop_or_spread_string_factory(key: &str, value: &str) -> PropOrSpread {
  let value = string_to_expression(value);

  match value {
    Some(value) => prop_or_spread_expression_factory(key, Box::new(value)),
    None => panic!("Value is not a string"),
  }
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn prop_or_spread_array_string_factory(key: &str, value: &[&str]) -> PropOrSpread {
  let array = ArrayLit {
    span: DUMMY_SP,
    elems: value
      .iter()
      .map(|v| Option::Some(expr_or_spread_string_expression_factory(v)))
      .collect::<Vec<Option<ExprOrSpread>>>(),
  };

  prop_or_spread_expression_factory(key, Box::new(Expr::Array(array)))
}

pub(crate) fn _prop_or_spread_boolean_factory(key: &str, value: Option<bool>) -> PropOrSpread {
  match value {
    Some(value) => prop_or_spread_expression_factory(
      key,
      Box::new(Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value,
      }))),
    ),
    None => panic!("Value is not a boolean"),
  }
}

pub(crate) fn expr_or_spread_string_expression_factory(value: &str) -> ExprOrSpread {
  let expr = Box::new(string_to_expression(value).expect(NON_STATIC_VALUE));

  ExprOrSpread {
    expr,
    spread: Option::None,
  }
}

pub(crate) fn expr_or_spread_number_expression_factory(value: f64) -> ExprOrSpread {
  let expr = Box::new(number_to_expression(value).unwrap());

  ExprOrSpread {
    expr,
    spread: Option::None,
  }
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn create_array(values: &[Expr]) -> Option<ArrayLit> {
  array_fabric(values, Option::None)
}

pub(crate) fn _create_spreaded_array(values: &[Expr]) -> Option<ArrayLit> {
  array_fabric(values, Option::Some(DUMMY_SP))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
fn array_fabric(values: &[Expr], spread: Option<Span>) -> Option<ArrayLit> {
  let array = ArrayLit {
    span: DUMMY_SP,
    elems: values
      .iter()
      .map(|value| {
        Some(ExprOrSpread {
          spread,
          expr: Box::new(value.clone()),
        })
      })
      .collect(),
  };

  Option::Some(array)
}
