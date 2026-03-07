use swc_core::ecma::ast::{
  BigInt, BindingIdent, Ident, IdentName, KeyValueProp, Lit, Null, ParenExpr, Prop, PropName,
};
use swc_core::{
  common::{DUMMY_SP, Span},
  ecma::ast::{ArrayLit, Expr, ExprOrSpread, ObjectLit, PropOrSpread},
};

use super::convertors::{
  bool_to_expression, number_to_expression, string_to_expression, string_to_prop_name,
};

/// Wraps an owned expression in a ParenExpr with DUMMY_SP span.
/// This is commonly used when creating error contexts.
///
/// # Example
/// ```ignore
/// let wrapped = wrap_in_paren(some_expr);
/// ```
#[inline]
pub fn wrap_in_paren(expr: Expr) -> Expr {
  Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(expr),
  })
}

/// Wraps a reference to an expression in a ParenExpr with DUMMY_SP span.
/// This clones the expression and is commonly used when creating error contexts.
///
/// # Example
/// ```ignore
/// let wrapped = wrap_in_paren_ref(&path);
/// ```
#[inline]
pub fn wrap_in_paren_ref(expr: &Expr) -> Expr {
  wrap_in_paren(expr.clone())
}

pub(crate) fn object_lit_factory(props: Vec<PropOrSpread>) -> ObjectLit {
  ObjectLit {
    span: DUMMY_SP,
    props,
  }
}

pub(crate) fn array_lit_factory(elems: Vec<Option<ExprOrSpread>>) -> ArrayLit {
  ArrayLit {
    span: DUMMY_SP,
    elems,
  }
}

pub fn object_expression_factory(props: Vec<PropOrSpread>) -> Expr {
  Expr::from(object_lit_factory(props))
}

pub fn array_expression_factory(elems: Vec<Option<ExprOrSpread>>) -> Expr {
  Expr::from(array_lit_factory(elems))
}

pub fn prop_or_spread_expression_factory(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key: string_to_prop_name(key).unwrap(),
    value: Box::new(value),
  }))
}

pub(crate) fn binding_ident_factory(ident: Ident) -> BindingIdent {
  BindingIdent::from(ident)
}

pub(crate) fn lit_str_factory(value: &str) -> Lit {
  Lit::from(value)
}

pub(crate) fn lit_number_factory(value: f64) -> Lit {
  Lit::from(value)
}

pub(crate) fn lit_big_int_factory(value: BigInt) -> Lit {
  Lit::from(value)
}

pub(crate) fn lit_boolean_factory(value: bool) -> Lit {
  Lit::from(value)
}

pub(crate) fn lit_null_factory() -> Lit {
  Lit::Null(Null { span: DUMMY_SP })
}

pub(crate) fn ident_factory(name: &str) -> Ident {
  Ident::from(name)
}

pub fn prop_or_spread_expr_factory(key: &str, values: Vec<PropOrSpread>) -> PropOrSpread {
  let object = ObjectLit {
    span: DUMMY_SP,
    props: values,
  };

  prop_or_spread_expression_factory(key, Expr::Object(object))
}

/// Creates a `PropOrSpread` from an already-constructed `PropName` and an expression value.
///
/// Use this when the key is an existing `PropName` (e.g. cloned from another prop),
/// avoiding the need to re-stringify it.
pub(crate) fn prop_or_spread_prop_name_factory(key: PropName, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key,
    value: Box::new(value),
  }))
}

pub fn key_value_ident_factory(key: &str, value: Expr) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Ident(IdentName::new(key.into(), DUMMY_SP)),
    value: Box::new(value),
  }
}

/// Creates a `PropOrSpread` with an unconditional `PropName::Ident` key.
///
/// Unlike `prop_or_spread_expression_factory`, this bypasses identifier validation,
/// preserving keys that contain special characters (e.g. `@media …`) as ident nodes.
/// Use this wherever downstream code calls `.as_ident()` on the resulting key.
pub(crate) fn prop_or_spread_ident_factory(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key: PropName::Ident(IdentName::new(key.into(), DUMMY_SP)),
    value: Box::new(value),
  }))
}

pub(crate) fn prop_or_spread_string_factory(key: &str, value: &str) -> PropOrSpread {
  let value = string_to_expression(value);

  prop_or_spread_expression_factory(key, value)
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn prop_or_spread_array_string_factory(key: &str, value: &[&str]) -> PropOrSpread {
  let array = ArrayLit {
    span: DUMMY_SP,
    elems: value
      .iter()
      .map(|v| Some(expr_or_spread_string_expression_factory(v)))
      .collect::<Vec<Option<ExprOrSpread>>>(),
  };

  prop_or_spread_expression_factory(key, Expr::from(array))
}

pub(crate) fn _prop_or_spread_boolean_factory(key: &str, value: Option<bool>) -> PropOrSpread {
  match value {
    Some(value) => prop_or_spread_expression_factory(key, bool_to_expression(value)),
    None => panic!("Value is not a boolean"),
  }
}

/// Wraps an arbitrary expression in `ExprOrSpread` with no spread.
/// This is the generic counterpart to the typed `expr_or_spread_*_factory` helpers
/// and eliminates the common boilerplate `ExprOrSpread { spread: None, expr: Box::new(e) }`.
pub(crate) fn expr_or_spread_factory(expr: Expr) -> ExprOrSpread {
  ExprOrSpread {
    expr: Box::new(expr),
    spread: None,
  }
}

pub(crate) fn expr_or_spread_string_expression_factory(value: &str) -> ExprOrSpread {
  expr_or_spread_factory(string_to_expression(value))
}

pub(crate) fn expr_or_spread_number_expression_factory(value: f64) -> ExprOrSpread {
  expr_or_spread_factory(number_to_expression(value))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn create_array(values: &[Expr]) -> ArrayLit {
  array_fabric(values, None)
}

pub(crate) fn _create_spreaded_array(values: &[Expr]) -> ArrayLit {
  array_fabric(values, Some(DUMMY_SP))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
fn array_fabric(values: &[Expr], spread: Option<Span>) -> ArrayLit {
  ArrayLit {
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
  }
}
