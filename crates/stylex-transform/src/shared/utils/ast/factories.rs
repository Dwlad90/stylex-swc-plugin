use stylex_macros::stylex_panic;
use swc_core::{
  common::SyntaxContext,
  ecma::ast::{
    ArrowExpr, BigInt, BindingIdent, BlockStmtOrExpr, CallExpr, Callee, Ident, IdentName, JSXAttr,
    JSXAttrName, JSXAttrOrSpread, JSXAttrValue, KeyValueProp, Lit, MemberExpr, Null, ParenExpr,
    Pat, Prop, PropName, SpreadElement, VarDeclarator,
  },
};
use swc_core::{
  common::{DUMMY_SP, Span},
  ecma::ast::{ArrayLit, Expr, ExprOrSpread, ObjectLit, PropOrSpread},
};

use crate::shared::utils::ast::convertors::create_null_expr;

use super::convertors::{
  create_bool_expr, create_number_expr, create_string_expr, convert_string_to_prop_name,
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

/// Creates an `ObjectLit` from a vector of `PropOrSpread`.
///
/// # Arguments
/// * `props` - The properties to include in the object literal
///
/// # Example
/// ```ignore
/// let object_lit = create_object_lit(vec![prop_or_spread_factory(key, value)]);
/// ```
pub(crate) fn create_object_lit(props: Vec<PropOrSpread>) -> ObjectLit {
  ObjectLit {
    span: DUMMY_SP,
    props,
  }
}

/// Creates an `ArrayLit` from a vector of `ExprOrSpread`.
///
/// # Arguments
/// * `elems` - The elements to include in the array literal
///
/// # Example
/// ```ignore
/// let array_lit = create_array_lit(vec![create_expr_or_spread(value)]);
/// ```
pub(crate) fn create_array_lit(elems: Vec<Option<ExprOrSpread>>) -> ArrayLit {
  ArrayLit {
    span: DUMMY_SP,
    elems,
  }
}

/// Creates an `Expr::Object` from a vector of `PropOrSpread`.
///
/// # Arguments
/// * `props` - The properties to include in the object literal
///
/// # Example
/// ```ignore
/// let object_expression = create_object_expression(vec![prop_or_spread_factory(key, value)]);
/// ```
pub fn create_object_expression(props: Vec<PropOrSpread>) -> Expr {
  Expr::from(create_object_lit(props))
}

pub fn create_array_expression(elems: Vec<Option<ExprOrSpread>>) -> Expr {
  Expr::from(create_array_lit(elems))
}

/// Creates a `PropOrSpread` from a key and value.
///
/// # Arguments
/// * `key` - The key of the property
/// * `value` - The value of the property
///
/// # Example
/// ```ignore
/// let prop_or_spread = create_key_value_prop("key", value);
pub fn create_key_value_prop(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key: match convert_string_to_prop_name(key) {
      Some(k) => k,
      None => stylex_panic!("Failed to create prop name from key: {}", key),
    },
    value: Box::new(value),
  }))
}

/// Creates a `BindingIdent` from an `Ident`.
///
/// # Arguments
/// * `ident` - The identifier to create a binding for
///
/// # Example
/// ```ignore
/// let binding_ident = create_binding_ident(ident);
/// ```
pub(crate) fn create_binding_ident(ident: Ident) -> BindingIdent {
  BindingIdent::from(ident)
}

/// Creates a `Lit::Str` from a string.
///
/// # Arguments
/// * `value` - The string to create a literal for
///
/// # Example
/// ```ignore
/// let lit_str = create_string_lit("value");
/// ```
pub(crate) fn create_string_lit(value: &str) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Number` from a number.
///
/// # Arguments
/// * `value` - The number to create a literal for
pub(crate) fn create_number_lit(value: f64) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::BigInt` from a `BigInt`.
///
/// # Arguments
/// * `value` - The big integer to create a literal for
///
/// # Example
/// ```ignore
/// let lit_big_int = create_big_int_lit(BigInt::from(123));
/// ```
pub(crate) fn create_big_int_lit(value: BigInt) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Boolean` from a boolean.
///
/// # Arguments
/// * `value` - The boolean to create a literal for
///
/// # Example
/// ```ignore
/// let lit_boolean = create_boolean_lit(true);
/// ```
pub(crate) fn create_boolean_lit(value: bool) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Null` from a `Null`.
///
/// # Arguments
/// * `value` - The null to create a literal for
///
/// # Example
/// ```ignore
/// let lit_null = create_null_lit();
/// ```
pub(crate) fn create_null_lit() -> Lit {
  Lit::Null(Null { span: DUMMY_SP })
}

/// Creates an `Ident` from a string.
///
/// # Arguments
/// * `name` - The identifier name
///
/// # Example
/// ```ignore
/// let ident = create_ident("props");
/// ```
pub(crate) fn create_ident(name: &str) -> Ident {
  Ident::from(name)
}

pub fn create_nested_object_prop(key: &str, values: Vec<PropOrSpread>) -> PropOrSpread {
  let object = ObjectLit {
    span: DUMMY_SP,
    props: values,
  };

  create_key_value_prop(key, Expr::Object(object))
}

/// Creates a `PropOrSpread` from an already-constructed `PropName` and an expression value.
///
/// Use this when the key is an existing `PropName` (e.g. cloned from another prop),
/// avoiding the need to re-stringify it.
pub(crate) fn create_prop_from_name(key: PropName, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key,
    value: Box::new(value),
  }))
}

/// Creates a `KeyValueProp` with an `IdentName` key.
///
/// # Arguments
/// * `key` - The identifier name
/// * `value` - The value of the property
///
/// # Example
/// ```ignore
/// let key_value = create_key_value_prop_ident("props", value);
pub fn create_key_value_prop_ident(key: &str, value: Expr) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Ident(IdentName::new(key.into(), DUMMY_SP)),
    value: Box::new(value),
  }
}

/// Creates a `PropOrSpread` with an unconditional `PropName::Ident` key.
///
/// Unlike `create_key_value_prop`, this bypasses identifier validation,
/// preserving keys that contain special characters (e.g. `@media …`) as ident nodes.
/// Use this wherever downstream code calls `.as_ident()` on the resulting key.
pub(crate) fn create_ident_key_value_prop(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key: PropName::Ident(IdentName::new(key.into(), DUMMY_SP)),
    value: Box::new(value),
  }))
}

/// Creates a `PropOrSpread` with a string key and value.
///
/// # Arguments
/// * `key` - The string key
/// * `value` - The value of the property
///
/// # Example
/// ```ignore
/// let prop_or_spread = create_string_key_value_prop("key", "value");
pub(crate) fn create_string_key_value_prop(key: &str, value: &str) -> PropOrSpread {
  let value = create_string_expr(value);

  create_key_value_prop(key, value)
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn create_string_array_prop(key: &str, value: &[&str]) -> PropOrSpread {
  let array = ArrayLit {
    span: DUMMY_SP,
    elems: value
      .iter()
      .map(|v| Some(create_string_expr_or_spread(v)))
      .collect::<Vec<Option<ExprOrSpread>>>(),
  };

  create_key_value_prop(key, Expr::from(array))
}

/// Creates a `PropOrSpread` with a boolean key and value.
///
/// # Arguments
/// * `key` - The boolean key
/// * `value` - The value of the property
///
/// # Example
/// ```ignore
/// let prop_or_spread = prop_or_spread_boolean_factory("key", true);
pub(crate) fn _create_boolean_prop(key: &str, value: Option<bool>) -> PropOrSpread {
  match value {
    Some(value) => create_key_value_prop(key, create_bool_expr(value)),
    None => stylex_panic!("Value is not a boolean"),
  }
}

/// Wraps an arbitrary expression in `ExprOrSpread` with no spread.
/// This is the generic counterpart to the typed `expr_or_spread_*_factory` helpers
/// and eliminates the common boilerplate `ExprOrSpread { spread: None, expr: Box::new(e) }`.
pub(crate) fn create_expr_or_spread(expr: Expr) -> ExprOrSpread {
  ExprOrSpread {
    expr: Box::new(expr),
    spread: None,
  }
}

/// Creates an `ExprOrSpread` with a string value.
///
/// # Arguments
/// * `value` - The string value
///
/// # Example
/// ```ignore
/// let expr_or_spread = create_string_expr_or_spread("value");
/// ```
pub(crate) fn create_string_expr_or_spread(value: &str) -> ExprOrSpread {
  create_expr_or_spread(create_string_expr(value))
}

pub(crate) fn create_number_expr_or_spread(value: f64) -> ExprOrSpread {
  create_expr_or_spread(create_number_expr(value))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub(crate) fn create_array(values: &[Expr]) -> ArrayLit {
  array_fabric(values, None)
}

/// Creates an `ArrayLit` with a spread.
///
/// # Arguments
/// * `values` - The values to include in the array literal
///
/// # Example
/// ```ignore
/// let array_lit = _create_spreaded_array(vec![expr1, expr2, expr3]);
/// ```
pub(crate) fn _create_spreaded_array(values: &[Expr]) -> ArrayLit {
  array_fabric(values, Some(DUMMY_SP))
}

/// Creates an `ArrayLit` with a spread.
///
/// # Arguments
/// * `values` - The values to include in the array literal
/// * `spread` - The span of the spread
///
/// # Example
/// ```ignore
/// let array_lit = array_fabric(vec![expr1, expr2, expr3], Some(DUMMY_SP));
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

/// Creates an `IdentName` from a string.
///
/// # Arguments
/// * `name` - The identifier name
///
/// # Example
/// ```ignore
/// let ident_name = create_ident_name("props");
/// ```
#[inline]
pub(crate) fn create_ident_name(name: &str) -> IdentName {
  IdentName::new(name.into(), DUMMY_SP)
}

/// Creates a `SpreadElement` for spreading an expression.
///
/// # Arguments
/// * `expr` - The expression to spread
///
/// # Example
/// ```ignore
/// let spread = create_spread_element(obj_expr);
/// ```
#[inline]
pub(crate) fn create_spread_element(expr: Expr) -> SpreadElement {
  SpreadElement {
    dot3_token: DUMMY_SP,
    expr: Box::new(expr),
  }
}

/// Creates a `PropOrSpread::Spread` for spreading properties in an object literal.
///
/// # Arguments
/// * `expr` - The expression to spread
///
/// # Example
/// ```ignore
/// let spread = create_spread_prop(call_expr);
/// ```
#[inline]
pub(crate) fn create_spread_prop(expr: Expr) -> PropOrSpread {
  PropOrSpread::Spread(create_spread_element(expr))
}

/// Creates a `CallExpr` with a member expression callee (e.g., `obj.method(...args)`).
///
/// # Arguments
/// * `callee_member` - The member expression to call
/// * `args` - The call arguments
///
/// # Example
/// ```ignore
/// let member = member_expr_factory("stylex", "props");
/// let call = create_member_call_expr(member, vec![arg1, arg2]);
/// ```
#[inline]
pub(crate) fn create_member_call_expr(
  callee_member: MemberExpr,
  args: Vec<ExprOrSpread>,
) -> CallExpr {
  CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Member(callee_member))),
    args,
    type_args: None,
    ctxt: SyntaxContext::empty(),
  }
}

/// Creates a `CallExpr` with an identifier callee (e.g., `func(...args)`).
///
/// # Arguments
/// * `callee_ident` - The identifier to call
/// * `args` - The call arguments
///
/// # Example
/// ```ignore
/// let call = create_ident_call_expr("merge", vec![arg1]);
/// ```
#[inline]
pub(crate) fn create_ident_call_expr(callee_ident: &str, args: Vec<ExprOrSpread>) -> CallExpr {
  CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Ident(create_ident(callee_ident)))),
    args,
    type_args: None,
    ctxt: SyntaxContext::empty(),
  }
}

/// Creates an arrow expression `() => expr`.
///
/// # Arguments
/// * `body_expr` - The expression to return
///
/// # Example
/// ```ignore
/// let arrow = create_arrow_expression(call_expr);
/// ```
#[inline]
pub(crate) fn create_arrow_expression(body_expr: Expr) -> Expr {
  use ArrowExpr;
  Expr::Arrow(ArrowExpr {
    span: DUMMY_SP,
    params: vec![],
    body: Box::new(BlockStmtOrExpr::Expr(Box::new(body_expr))),
    is_async: false,
    is_generator: false,
    type_params: None,
    return_type: None,
    ctxt: SyntaxContext::empty(),
  })
}

/// Creates a `JSXAttrOrSpread::SpreadElement` for JSX attributes.
///
/// # Arguments
/// * `expr` - The expression to spread
///
/// # Example
/// ```ignore
/// let jsx_spread = create_jsx_spread_attr(props_call);
/// ```
#[inline]
pub(crate) fn create_jsx_spread_attr(expr: Expr) -> JSXAttrOrSpread {
  JSXAttrOrSpread::SpreadElement(create_spread_element(expr))
}

/// Creates a `JSXAttrOrSpread::JSXAttr` wrapper.
///
/// # Arguments
/// * `attr` - The JSX attribute
///
/// # Example
/// ```ignore
/// let jsx_attr = create_jsx_attr_or_spread(attr);
/// ```
#[inline]
pub(crate) fn create_jsx_attr_or_spread(attr: JSXAttr) -> JSXAttrOrSpread {
  use JSXAttrOrSpread;
  JSXAttrOrSpread::JSXAttr(attr)
}

/// Creates a `JSXAttr` from a name and value.
///
/// # Arguments
/// * `name` - The name of the attribute
/// * `value` - The value of the attribute
///
/// # Example
/// ```ignore
/// let jsx_attr = create_jsx_attr("name", value);
/// ```
#[allow(dead_code)]
pub(crate) fn create_jsx_attr(name: &str, value: JSXAttrValue) -> JSXAttr {
  JSXAttr {
    span: DUMMY_SP,
    name: JSXAttrName::Ident(IdentName::from(name)),
    value: Some(value),
  }
}

/// Creates a `VarDeclarator` with an identifier name and an expression initializer.
///
/// # Arguments
/// * `ident` - The identifier for the variable name
/// * `init` - The initializer expression
///
/// # Example
/// ```ignore
/// let decl = create_var_declarator(my_ident, some_expr);
/// ```
pub(crate) fn create_var_declarator(ident: Ident, init: Expr) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(create_binding_ident(ident)),
    init: Some(Box::new(init)),
    definite: false,
  }
}

/// Creates a `VarDeclarator` initialized to `null`.
///
/// Useful when hoisting a variable declaration ahead of its actual assignment,
/// e.g. `var x = null;` before the value is set later.
///
/// # Arguments
/// * `ident` - The identifier for the variable name
///
/// # Example
/// ```ignore
/// let decl = var_declarator_null_init_factory(my_ident);
/// ```
pub(crate) fn _create_null_var_declarator(ident: Ident) -> VarDeclarator {
  create_var_declarator(ident, create_null_expr())
}

/// Creates a `VarDeclarator` initialized to a string.
///
/// # Arguments
/// * `ident` - The identifier for the variable name
/// * `value` - The string value
///
/// # Example
/// ```ignore
/// let decl = create_string_var_declarator(my_ident, "value");
/// ```
pub(crate) fn create_string_var_declarator(ident: Ident, value: &str) -> VarDeclarator {
  create_var_declarator(ident, create_string_expr(value))
}
