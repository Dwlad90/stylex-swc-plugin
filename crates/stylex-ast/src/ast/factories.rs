use stylex_core::stylex_panic;
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

use super::convertors::{
  bool_to_expression, null_to_expression, number_to_expression, string_to_expression,
  string_to_prop_name,
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
/// let object_lit = object_lit_factory(vec![prop_or_spread_factory(key, value)]);
/// ```
pub fn object_lit_factory(props: Vec<PropOrSpread>) -> ObjectLit {
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
/// let array_lit = array_lit_factory(vec![expr_or_spread_factory(value)]);
/// ```
pub fn array_lit_factory(elems: Vec<Option<ExprOrSpread>>) -> ArrayLit {
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
/// let object_expression = object_expression_factory(vec![prop_or_spread_factory(key, value)]);
/// ```
pub fn object_expression_factory(props: Vec<PropOrSpread>) -> Expr {
  Expr::from(object_lit_factory(props))
}

pub fn array_expression_factory(elems: Vec<Option<ExprOrSpread>>) -> Expr {
  Expr::from(array_lit_factory(elems))
}

/// Creates a `PropOrSpread` from a key and value.
///
/// # Arguments
/// * `key` - The key of the property
/// * `value` - The value of the property
///
/// # Example
/// ```ignore
/// let prop_or_spread = prop_or_spread_expression_factory("key", value);
pub fn prop_or_spread_expression_factory(key: &str, value: Expr) -> PropOrSpread {
  PropOrSpread::from(Prop::from(KeyValueProp {
    key: match string_to_prop_name(key) {
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
/// let binding_ident = binding_ident_factory(ident);
/// ```
pub fn binding_ident_factory(ident: Ident) -> BindingIdent {
  BindingIdent::from(ident)
}

/// Creates a `Lit::Str` from a string.
///
/// # Arguments
/// * `value` - The string to create a literal for
///
/// # Example
/// ```ignore
/// let lit_str = lit_str_factory("value");
/// ```
pub fn lit_str_factory(value: &str) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Number` from a number.
///
/// # Arguments
/// * `value` - The number to create a literal for
pub fn lit_number_factory(value: f64) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::BigInt` from a `BigInt`.
///
/// # Arguments
/// * `value` - The big integer to create a literal for
///
/// # Example
/// ```ignore
/// let lit_big_int = lit_big_int_factory(BigInt::from(123));
/// ```
pub fn lit_big_int_factory(value: BigInt) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Boolean` from a boolean.
///
/// # Arguments
/// * `value` - The boolean to create a literal for
///
/// # Example
/// ```ignore
/// let lit_boolean = lit_boolean_factory(true);
/// ```
pub fn lit_boolean_factory(value: bool) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Null` from a `Null`.
///
/// # Arguments
/// * `value` - The null to create a literal for
///
/// # Example
/// ```ignore
/// let lit_null = lit_null_factory();
/// ```
pub fn lit_null_factory() -> Lit {
  Lit::Null(Null { span: DUMMY_SP })
}

/// Creates an `Ident` from a string.
///
/// # Arguments
/// * `name` - The identifier name
///
/// # Example
/// ```ignore
/// let ident = ident_factory("props");
/// ```
pub fn ident_factory(name: &str) -> Ident {
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
pub fn prop_or_spread_prop_name_factory(key: PropName, value: Expr) -> PropOrSpread {
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
/// let key_value = key_value_ident_factory("props", value);
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
pub fn prop_or_spread_ident_factory(key: &str, value: Expr) -> PropOrSpread {
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
/// let prop_or_spread = prop_or_spread_string_factory("key", "value");
pub fn prop_or_spread_string_factory(key: &str, value: &str) -> PropOrSpread {
  let value = string_to_expression(value);

  prop_or_spread_expression_factory(key, value)
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub fn prop_or_spread_array_string_factory(key: &str, value: &[&str]) -> PropOrSpread {
  let array = ArrayLit {
    span: DUMMY_SP,
    elems: value
      .iter()
      .map(|v| Some(expr_or_spread_string_expression_factory(v)))
      .collect::<Vec<Option<ExprOrSpread>>>(),
  };

  prop_or_spread_expression_factory(key, Expr::from(array))
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
pub fn _prop_or_spread_boolean_factory(key: &str, value: Option<bool>) -> PropOrSpread {
  match value {
    Some(value) => prop_or_spread_expression_factory(key, bool_to_expression(value)),
    None => stylex_panic!("Value is not a boolean"),
  }
}

/// Wraps an arbitrary expression in `ExprOrSpread` with no spread.
/// This is the generic counterpart to the typed `expr_or_spread_*_factory` helpers
/// and eliminates the common boilerplate `ExprOrSpread { spread: None, expr: Box::new(e) }`.
pub fn expr_or_spread_factory(expr: Expr) -> ExprOrSpread {
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
/// let expr_or_spread = expr_or_spread_string_expression_factory("value");
/// ```
pub fn expr_or_spread_string_expression_factory(value: &str) -> ExprOrSpread {
  expr_or_spread_factory(string_to_expression(value))
}

pub fn expr_or_spread_number_expression_factory(value: f64) -> ExprOrSpread {
  expr_or_spread_factory(number_to_expression(value))
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub fn create_array(values: &[Expr]) -> ArrayLit {
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
pub fn _create_spreaded_array(values: &[Expr]) -> ArrayLit {
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
/// let ident_name = ident_name_factory("props");
/// ```
#[inline]
pub fn ident_name_factory(name: &str) -> IdentName {
  IdentName::new(name.into(), DUMMY_SP)
}

/// Creates a `SpreadElement` for spreading an expression.
///
/// # Arguments
/// * `expr` - The expression to spread
///
/// # Example
/// ```ignore
/// let spread = spread_element_factory(obj_expr);
/// ```
#[inline]
pub fn spread_element_factory(expr: Expr) -> SpreadElement {
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
/// let spread = prop_or_spread_spread_factory(call_expr);
/// ```
#[inline]
pub fn prop_or_spread_spread_factory(expr: Expr) -> PropOrSpread {
  PropOrSpread::Spread(spread_element_factory(expr))
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
/// let call = call_expr_member_factory(member, vec![arg1, arg2]);
/// ```
#[inline]
pub fn call_expr_member_factory(
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
/// let call = call_expr_ident_factory("merge", vec![arg1]);
/// ```
#[inline]
pub fn call_expr_ident_factory(callee_ident: &str, args: Vec<ExprOrSpread>) -> CallExpr {
  CallExpr {
    span: DUMMY_SP,
    callee: Callee::Expr(Box::new(Expr::Ident(ident_factory(callee_ident)))),
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
/// let arrow = arrow_expr_factory(call_expr);
/// ```
#[inline]
pub fn arrow_expr_factory(body_expr: Expr) -> Expr {
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
/// let jsx_spread = jsx_attr_or_spread_spread_factory(props_call);
/// ```
#[inline]
pub fn jsx_attr_or_spread_spread_factory(expr: Expr) -> JSXAttrOrSpread {
  JSXAttrOrSpread::SpreadElement(spread_element_factory(expr))
}

/// Creates a `JSXAttrOrSpread::JSXAttr` wrapper.
///
/// # Arguments
/// * `attr` - The JSX attribute
///
/// # Example
/// ```ignore
/// let jsx_attr = jsx_attr_or_spread_attr_factory(attr);
/// ```
#[inline]
pub fn jsx_attr_or_spread_attr_factory(attr: JSXAttr) -> JSXAttrOrSpread {
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
/// let jsx_attr = jsx_attr_factory("name", value);
/// ```
#[allow(dead_code)]
pub fn jsx_attr_factory(name: &str, value: JSXAttrValue) -> JSXAttr {
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
/// let decl = var_declarator_factory(my_ident, some_expr);
/// ```
pub fn var_declarator_factory(ident: Ident, init: Expr) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(binding_ident_factory(ident)),
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
pub fn _var_declarator_null_init_factory(ident: Ident) -> VarDeclarator {
  var_declarator_factory(ident, null_to_expression())
}

/// Creates a `VarDeclarator` initialized to a string.
///
/// # Arguments
/// * `ident` - The identifier for the variable name
/// * `value` - The string value
///
/// # Example
/// ```ignore
/// let decl = var_declarator_string_init_factory(my_ident, "value");
/// ```
pub fn var_declarator_string_init_factory(ident: Ident, value: &str) -> VarDeclarator {
  var_declarator_factory(ident, string_to_expression(value))
}
