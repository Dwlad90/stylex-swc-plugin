use anyhow::anyhow;
use stylex_macros::stylex_panic;
use stylex_utils::swc::get_default_expr_ctx;
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  ecma::{
    ast::{
      BigInt, Bool, CallExpr, Expr, Ident, KeyValueProp, Lit, Prop, PropName, Str, Tpl, TplElement,
    },
    parser::Context,
    utils::{ExprExt, quote_ident, quote_str},
  },
};

use stylex_constants::constants::messages::INVALID_UTF8;

use super::factories::{
  create_big_int_lit, create_boolean_lit, create_ident, create_null_lit, create_number_lit,
  create_string_lit,
};

/// Helper function to convert a Lit to a number
/// # Arguments
/// * `lit_num` - The literal to convert
/// # Returns
/// * `Result<f64, anyhow::Error>` - The number value of the literal
///
/// # Example
/// ```javascript
/// Input: Lit::Num(1.0)
/// Output: 1.0
/// ```
pub fn convert_lit_to_number(lit_num: &Lit) -> Result<f64, anyhow::Error> {
  let result = match &lit_num {
    Lit::Bool(Bool { value, .. }) => {
      if value == &true {
        1.0
      } else {
        0.0
      }
    },
    Lit::Num(num) => num.value,
    Lit::Str(strng) => {
      let Result::Ok(num) = convert_atom_to_string(&strng.value).parse::<f64>() else {
        return Err(anyhow!(
          "Value in not a number: {}",
          convert_atom_to_string(&strng.value)
        ));
      };

      num
    },
    _ => {
      return Err(anyhow!(
        "Value in not a number: {:?}",
        Expr::from(lit_num.clone()).get_type(get_default_expr_ctx())
      ));
    },
  };

  Result::Ok(result)
}

/// Helper function to convert a Tpl to a string literal
/// # Arguments
/// * `tpl` - The template literal to convert
/// # Returns
/// * `Option<Lit>` - The string literal if the template is simple (no interpolations)
/// * `None` - If the template is not simple (has interpolations)
/// # Example
/// ```javascript
/// Input: Tpl { exprs: [], quasis: [TplElement { cooked: Some("hello"), raw: "hello" }] }
/// Output: Some(Lit::Str("hello"))
/// ```
pub fn convert_tpl_to_string_lit(tpl: &Tpl) -> Option<Lit> {
  // Check if it's a simple template (no expressions)
  if tpl.exprs.is_empty() && tpl.quasis.len() == 1 {
    let quasi = &tpl.quasis[0];

    // Get the string value (prefer cooked if available, otherwise use raw)
    let value = match quasi.cooked.as_ref() {
      Some(cooked) => match cooked.as_str() {
        Some(s) => s,
        None => stylex_panic!("Failed to extract a string value from the expression."),
      },
      None => stylex_panic!("Failed to extract cooked value from template literal element."),
    };

    return Some(create_string_lit(value));
  }

  None
}

/// Converts a simple template literal expression to a regular string literal expression.
/// This is a convenience wrapper around `convert_tpl_to_string_lit` that works with `Expr::Tpl`.
///
/// # Arguments
/// * `expr` - The expression to check and potentially convert
///
/// # Returns
/// * `Expr` - The original expression if it's not a simple template literal
/// * A string literal expression if the template is simple (no interpolations)
#[inline]
pub fn convert_simple_tpl_to_str_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Tpl(ref tpl) => {
      if let Some(str_lit) = convert_tpl_to_string_lit(tpl) {
        return Expr::Lit(str_lit);
      }
      expr
    },
    _ => expr,
  }
}

/// Converts a string `.concat()` call expression to a template literal expression.
///
/// # Arguments
/// * `expr` - The expression to check and potentially convert
///
/// # Returns
/// * The original expression if it's not a concat call
/// * A template literal expression if the expression is a valid concat call
///
/// # Example
/// ```javascript
/// Input: "hello".concat(world, "!")
/// Output: `hello${world}!`
/// ```
#[inline]
pub fn convert_concat_to_tpl_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Call(ref call_expr) => {
      if let Some(tpl_expr) = concat_call_to_template_literal(call_expr) {
        return tpl_expr;
      }
      expr
    },
    _ => expr,
  }
}

/// Helper function that converts a CallExpr representing `.concat()` to a template literal.
///
/// # Arguments
/// * `call_expr` - The call expression to convert
///
/// # Returns
/// * `Some(Expr)` - Template literal expression if conversion is successful
/// * `None` - If the call expression is not a valid concat call
pub(crate) fn concat_call_to_template_literal(call_expr: &CallExpr) -> Option<Expr> {
  use swc_core::common::DUMMY_SP;

  // Check if this is a member expression with a "concat" property
  let member_expr = call_expr.callee.as_expr()?.as_member()?;
  let prop_ident = member_expr.prop.as_ident()?;

  if prop_ident.sym.as_ref() != "concat" {
    return None;
  }

  // Get the base string from the object being called
  let base_string = extract_str_lit_ref(member_expr.obj.as_lit()?).map(|s| s.to_string())?;

  let mut exprs = Vec::new();
  let mut quasis = Vec::new();

  // Add the base string as the first quasi
  quasis.push(TplElement {
    span: DUMMY_SP,
    tail: false,
    cooked: Some(base_string.clone().into()),
    raw: base_string.into(),
  });

  // Process each argument
  for (i, arg) in call_expr.args.iter().enumerate() {
    // Skip spread arguments
    if arg.spread.is_some() {
      continue;
    }

    exprs.push(arg.expr.clone());

    let is_last = i == call_expr.args.len() - 1;
    quasis.push(TplElement {
      span: DUMMY_SP,
      tail: is_last,
      cooked: Some("".into()),
      raw: "".into(),
    });
  }

  let template_literal = Tpl {
    span: DUMMY_SP,
    exprs,
    quasis,
  };

  Some(Expr::Tpl(template_literal))
}

#[inline]
pub fn create_number_expr(value: f64) -> Expr {
  Expr::from(create_number_lit(value))
}

#[inline]
pub fn create_big_int_expr(value: BigInt) -> Expr {
  Expr::from(create_big_int_lit(value))
}

#[inline]
pub fn create_string_expr(value: &str) -> Expr {
  Expr::Lit(create_string_lit(value))
}

#[inline]
pub fn create_bool_expr(value: bool) -> Expr {
  Expr::Lit(create_boolean_lit(value))
}

#[inline]
pub fn create_ident_expr(value: &str) -> Expr {
  Expr::Ident(create_ident(value))
}

#[inline]
pub fn create_null_expr() -> Expr {
  Expr::Lit(create_null_lit())
}

fn should_wrap_prop_name_key_with_quotes(key: &str) -> bool {
  Ident::verify_symbol(key).is_err() && {
    let ctx = Context::default();

    !ctx.is_reserved_word(&key.into())
  }
}
#[inline]
pub fn convert_string_to_prop_name(value: &str) -> Option<PropName> {
  if should_wrap_prop_name_key_with_quotes(value) {
    Some(PropName::Str(quote_str!(value)))
  } else {
    Some(PropName::Ident(quote_ident!(value)))
  }
}

pub fn expand_shorthand_prop(prop: &mut Box<Prop>) {
  if let Some(ident) = prop.as_shorthand() {
    **prop = Prop::from(KeyValueProp {
      key: match convert_string_to_prop_name(ident.sym.as_ref()) {
        Some(k) => k,
        None => stylex_panic!("Failed to convert string to a valid property name."),
      },
      value: Box::new(Expr::Ident(ident.clone())),
    });
  }
}

/// Helper function to convert Wtf8Atom to String
/// Note: `.as_str()` returns an `Option<&str>` that only fails when the string contains invalid UTF-8
#[inline]
pub fn convert_atom_to_string(atom: &Wtf8Atom) -> String {
  match atom.as_str() {
    Some(s) => s.to_string(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

pub fn convert_wtf8_to_atom(atom: &Wtf8Atom) -> Atom {
  match atom.as_atom() {
    Some(a) => a.clone(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

/// Helper function to safely get string from Lit::Str
#[inline]
pub fn convert_str_lit_to_string(str_lit: &Str) -> String {
  match str_lit.value.as_str() {
    Some(s) => s.to_string(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

/// Helper function to safely get Atom from Lit::Str
pub fn convert_str_lit_to_atom(str_lit: &Str) -> Atom {
  match str_lit.value.as_atom() {
    Some(a) => a.clone(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

/// Helper function to safely get cooked string from TplElement
#[inline]
pub fn extract_tpl_cooked_value(elem: &TplElement) -> String {
  match elem.cooked.as_ref() {
    Some(cooked) => match cooked.as_str() {
      Some(s) => s.to_string(),
      None => stylex_panic!("{}", INVALID_UTF8),
    },
    None => stylex_panic!(
      "Template literal element has no cooked value (contains an invalid escape sequence)."
    ),
  }
}

/// Helper function to convert Atom to &str (reference, not owned String)
/// Useful when you need a reference instead of an owned String
#[inline]
pub fn convert_atom_to_str_ref(atom: &swc_core::atoms::Wtf8Atom) -> &str {
  match atom.as_str() {
    Some(s) => s,
    None => stylex_panic!("Failed to convert SWC Atom to string (invalid WTF-8 encoding)."),
  }
}

#[inline]
pub fn convert_lit_to_string(value: &Lit) -> Option<String> {
  match value {
    Lit::Str(strng) => Some(convert_str_lit_to_string(strng)),
    Lit::Num(num) => Some(format!("{}", num.value)),
    Lit::BigInt(big_int) => Some(format!("{}", big_int.value)),
    _ => None,
  }
}

/// Helper function to safely extract string from Lit::Str using Option pattern
#[inline]
pub fn extract_str_lit_ref(lit: &Lit) -> Option<&str> {
  match lit {
    Lit::Str(s) => Some(convert_atom_to_str_ref(&s.value)),
    _ => None,
  }
}

