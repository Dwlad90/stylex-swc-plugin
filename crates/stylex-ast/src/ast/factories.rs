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

use super::convertors::{
  convert_string_to_prop_name, create_bool_expr, create_null_expr, create_number_expr,
  create_string_expr,
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
pub fn create_object_lit(props: Vec<PropOrSpread>) -> ObjectLit {
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
pub fn create_array_lit(elems: Vec<Option<ExprOrSpread>>) -> ArrayLit {
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
pub fn create_binding_ident(ident: Ident) -> BindingIdent {
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
pub fn create_string_lit(value: &str) -> Lit {
  Lit::from(value)
}

/// Creates a `Lit::Number` from a number.
///
/// # Arguments
/// * `value` - The number to create a literal for
pub fn create_number_lit(value: f64) -> Lit {
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
pub fn create_big_int_lit(value: BigInt) -> Lit {
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
pub fn create_boolean_lit(value: bool) -> Lit {
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
pub fn create_null_lit() -> Lit {
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
pub fn create_ident(name: &str) -> Ident {
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
pub fn create_prop_from_name(key: PropName, value: Expr) -> PropOrSpread {
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
pub fn create_ident_key_value_prop(key: &str, value: Expr) -> PropOrSpread {
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
pub fn create_string_key_value_prop(key: &str, value: &str) -> PropOrSpread {
  let value = create_string_expr(value);

  create_key_value_prop(key, value)
}

// NOTE: Tests only using this function
#[allow(dead_code)]
pub fn create_string_array_prop(key: &str, value: &[&str]) -> PropOrSpread {
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
pub fn create_boolean_prop(key: &str, value: Option<bool>) -> PropOrSpread {
  match value {
    Some(value) => create_key_value_prop(key, create_bool_expr(value)),
    None => stylex_panic!("Value is not a boolean"),
  }
}

/// Wraps an arbitrary expression in `ExprOrSpread` with no spread.
/// This is the generic counterpart to the typed `expr_or_spread_*_factory` helpers
/// and eliminates the common boilerplate `ExprOrSpread { spread: None, expr: Box::new(e) }`.
pub fn create_expr_or_spread(expr: Expr) -> ExprOrSpread {
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
pub fn create_string_expr_or_spread(value: &str) -> ExprOrSpread {
  create_expr_or_spread(create_string_expr(value))
}

pub fn create_number_expr_or_spread(value: f64) -> ExprOrSpread {
  create_expr_or_spread(create_number_expr(value))
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
pub fn create_spreaded_array(values: &[Expr]) -> ArrayLit {
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
pub fn create_ident_name(name: &str) -> IdentName {
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
pub fn create_spread_element(expr: Expr) -> SpreadElement {
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
pub fn create_spread_prop(expr: Expr) -> PropOrSpread {
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
pub fn create_member_call_expr(callee_member: MemberExpr, args: Vec<ExprOrSpread>) -> CallExpr {
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
pub fn create_ident_call_expr(callee_ident: &str, args: Vec<ExprOrSpread>) -> CallExpr {
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
pub fn create_arrow_expression(body_expr: Expr) -> Expr {
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
pub fn create_jsx_spread_attr(expr: Expr) -> JSXAttrOrSpread {
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
pub fn create_jsx_attr_or_spread(attr: JSXAttr) -> JSXAttrOrSpread {
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
pub fn create_jsx_attr(name: &str, value: JSXAttrValue) -> JSXAttr {
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
pub fn create_var_declarator(ident: Ident, init: Expr) -> VarDeclarator {
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
pub fn create_null_var_declarator(ident: Ident) -> VarDeclarator {
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
pub fn create_string_var_declarator(ident: Ident, value: &str) -> VarDeclarator {
  create_var_declarator(ident, create_string_expr(value))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wrap_in_paren_wraps_expression() {
    let expr = create_string_expr("hello");
    let wrapped = wrap_in_paren(expr);
    assert!(matches!(wrapped, Expr::Paren(_)));
  }

  #[test]
  fn wrap_in_paren_ref_clones_and_wraps() {
    let expr = create_number_expr(5.0);
    let wrapped = wrap_in_paren_ref(&expr);
    assert!(matches!(wrapped, Expr::Paren(_)));
  }

  #[test]
  fn create_string_lit_produces_str() {
    let lit = create_string_lit("test");
    assert!(matches!(lit, Lit::Str(_)));
  }

  #[test]
  fn create_number_lit_produces_num() {
    let lit = create_number_lit(7.0);
    match lit {
      Lit::Num(n) => assert!((n.value - 7.0).abs() < f64::EPSILON),
      _ => panic!("Expected Num"),
    }
  }

  #[test]
  fn create_boolean_lit_true() {
    assert!(matches!(create_boolean_lit(true), Lit::Bool(swc_core::ecma::ast::Bool { value: true, .. })));
  }

  #[test]
  fn create_boolean_lit_false() {
    assert!(matches!(create_boolean_lit(false), Lit::Bool(swc_core::ecma::ast::Bool { value: false, .. })));
  }

  #[test]
  fn create_big_int_lit_produces_bigint() {
    use swc_core::ecma::ast::BigInt as SwcBigInt;
    let big = SwcBigInt {
      span: DUMMY_SP,
      value: Box::new(42i64.into()),
      raw: None,
    };
    let lit = create_big_int_lit(big);
    assert!(matches!(lit, Lit::BigInt(_)));
  }

  #[test]
  fn create_null_lit_produces_null() {
    assert!(matches!(create_null_lit(), Lit::Null(_)));
  }

  #[test]
  fn create_ident_produces_ident() {
    let id = create_ident("myVar");
    assert_eq!(id.sym.as_ref(), "myVar");
  }

  #[test]
  fn create_object_lit_empty() {
    let obj = create_object_lit(vec![]);
    assert!(obj.props.is_empty());
  }

  #[test]
  fn create_array_lit_empty() {
    let arr = create_array_lit(vec![]);
    assert!(arr.elems.is_empty());
  }

  #[test]
  fn create_object_expression_wraps_in_expr() {
    let expr = create_object_expression(vec![]);
    assert!(matches!(expr, Expr::Object(_)));
  }

  #[test]
  fn create_array_expression_wraps_in_expr() {
    let expr = create_array_expression(vec![]);
    assert!(matches!(expr, Expr::Array(_)));
  }

  #[test]
  fn create_key_value_prop_creates_prop() {
    let prop = create_key_value_prop("color", create_string_expr("red"));
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_string_array_prop_creates_array() {
    let prop = create_string_array_prop("values", &["a", "b"]);
    if let PropOrSpread::Prop(p) = prop {
      if let Prop::KeyValue(kv) = *p {
        assert!(matches!(*kv.value, Expr::Array(_)));
      } else {
        panic!("Expected KeyValue");
      }
    } else {
      panic!("Expected Prop");
    }
  }

  #[test]
  fn create_boolean_prop_some() {
    let prop = create_boolean_prop("enabled", Some(true));
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_expr_or_spread_no_spread() {
    let eos = create_expr_or_spread(create_string_expr("a"));
    assert!(eos.spread.is_none());
  }

  #[test]
  fn create_string_expr_or_spread_creates() {
    let eos = create_string_expr_or_spread("test");
    assert!(eos.spread.is_none());
  }

  #[test]
  fn create_number_expr_or_spread_creates() {
    let eos = create_number_expr_or_spread(3.0);
    assert!(eos.spread.is_none());
  }

  #[test]
  fn create_array_and_spreaded_array() {
    let exprs = vec![create_string_expr("a"), create_string_expr("b")];
    let arr = create_array(&exprs);
    assert_eq!(arr.elems.len(), 2);
    assert!(arr.elems[0].as_ref().unwrap().spread.is_none());

    let spreaded = create_spreaded_array(&exprs);
    assert_eq!(spreaded.elems.len(), 2);
    assert!(spreaded.elems[0].as_ref().unwrap().spread.is_some());
  }

  #[test]
  fn create_nested_object_prop_works() {
    let inner = create_key_value_prop("a", create_string_expr("b"));
    let prop = create_nested_object_prop("outer", vec![inner]);
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_prop_from_name_works() {
    let key = PropName::Ident(IdentName::new("x".into(), DUMMY_SP));
    let prop = create_prop_from_name(key, create_number_expr(1.0));
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_key_value_prop_ident_works() {
    let kv = create_key_value_prop_ident("foo", create_string_expr("bar"));
    assert!(matches!(kv.key, PropName::Ident(_)));
  }

  #[test]
  fn create_ident_key_value_prop_works() {
    let prop = create_ident_key_value_prop("@media foo", create_string_expr("v"));
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_string_key_value_prop_works() {
    let prop = create_string_key_value_prop("key", "value");
    assert!(matches!(prop, PropOrSpread::Prop(_)));
  }

  #[test]
  fn create_ident_name_works() {
    let name = create_ident_name("prop");
    assert_eq!(name.sym.as_ref(), "prop");
  }

  #[test]
  fn create_spread_element_works() {
    let spread = create_spread_element(create_string_expr("x"));
    assert_eq!(spread.dot3_token, DUMMY_SP);
  }

  #[test]
  fn create_spread_prop_works() {
    let prop = create_spread_prop(create_string_expr("x"));
    assert!(matches!(prop, PropOrSpread::Spread(_)));
  }

  #[test]
  fn create_member_call_expr_works() {
    let member = MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(create_ident("obj").into()),
      prop: swc_core::ecma::ast::MemberProp::Ident(create_ident_name("method")),
    };
    let call = create_member_call_expr(member, vec![]);
    assert!(call.args.is_empty());
    assert!(matches!(call.callee, Callee::Expr(_)));
  }

  #[test]
  fn create_ident_call_expr_works() {
    let call = create_ident_call_expr("myFunc", vec![create_string_expr_or_spread("arg")]);
    assert_eq!(call.args.len(), 1);
  }

  #[test]
  fn create_arrow_expression_works() {
    let arrow = create_arrow_expression(create_string_expr("body"));
    assert!(matches!(arrow, Expr::Arrow(_)));
  }

  #[test]
  fn create_jsx_spread_attr_works() {
    let attr = create_jsx_spread_attr(create_string_expr("p"));
    assert!(matches!(attr, JSXAttrOrSpread::SpreadElement(_)));
  }

  #[test]
  fn create_jsx_attr_works() {
    let attr = create_jsx_attr(
      "className",
      JSXAttrValue::Str("test".into()),
    );
    assert!(matches!(attr.name, JSXAttrName::Ident(_)));
    assert!(attr.value.is_some());
  }

  #[test]
  fn create_jsx_attr_or_spread_works() {
    let jsx = JSXAttr {
      span: DUMMY_SP,
      name: JSXAttrName::Ident(IdentName::from("x")),
      value: None,
    };
    let result = create_jsx_attr_or_spread(jsx);
    assert!(matches!(result, JSXAttrOrSpread::JSXAttr(_)));
  }

  #[test]
  fn create_binding_ident_works() {
    let id = create_ident("x");
    let binding = create_binding_ident(id);
    assert_eq!(binding.id.sym.as_ref(), "x");
  }

  #[test]
  fn create_var_declarator_works() {
    let id = create_ident("x");
    let decl = create_var_declarator(id, create_number_expr(1.0));
    assert!(decl.init.is_some());
    assert!(!decl.definite);
  }

  #[test]
  fn create_null_var_declarator_works() {
    let id = create_ident("y");
    let decl = create_null_var_declarator(id);
    if let Some(init) = decl.init.as_ref() {
      assert!(matches!(init.as_ref(), Expr::Lit(Lit::Null(_))));
    }
  }

  #[test]
  fn create_string_var_declarator_works() {
    let id = create_ident("z");
    let decl = create_string_var_declarator(id, "hello");
    assert!(decl.init.is_some());
  }
}
