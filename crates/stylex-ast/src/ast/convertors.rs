use anyhow::anyhow;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use stylex_utils::{string::wrap_key_in_quotes, swc::get_default_expr_ctx};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  ecma::{
    ast::{
      BigInt, Bool, CallExpr, Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName,
      PropOrSpread, Str, Tpl, TplElement,
    },
    parser::Context,
    utils::{ExprExt, quote_ident, quote_str},
  },
};

use stylex_constants::constants::messages::{
  ILLEGAL_PROP_VALUE, INVALID_UTF8, SPREAD_NOT_SUPPORTED,
};

use super::factories::{
  create_big_int_lit, create_boolean_lit, create_ident, create_null_lit, create_number_lit,
  create_string_lit,
};

pub fn convert_lit_to_number(lit_num: &Lit) -> Result<f64, anyhow::Error> {
  match lit_num {
    Lit::Bool(Bool { value, .. }) => Ok(if *value { 1.0 } else { 0.0 }),
    Lit::Num(num) => Ok(num.value),
    Lit::Str(strng) => {
      let string_value = convert_atom_to_string(&strng.value);
      match string_value.parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(anyhow!("Value in not a number: {}", string_value)),
      }
    },
    _ => Err(anyhow!(
      "Value in not a number: {:?}",
      Expr::from(lit_num.clone()).get_type(get_default_expr_ctx())
    )),
  }
}

pub fn convert_tpl_to_string_lit(tpl: &Tpl) -> Option<Lit> {
  if !tpl.exprs.is_empty() || tpl.quasis.len() != 1 {
    return None;
  }

  let quasi = &tpl.quasis[0];
  let value = match quasi.cooked.as_ref() {
    Some(cooked) => match cooked.as_str() {
      Some(value) => value,
      None => stylex_panic!("Failed to extract a string value from the expression."),
    },
    None => stylex_panic!("Failed to extract cooked value from template literal element."),
  };

  Some(create_string_lit(value))
}

pub fn convert_simple_tpl_to_str_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Tpl(ref tpl) => convert_tpl_to_string_lit(tpl).map_or(expr, Expr::Lit),
    _ => expr,
  }
}

pub fn convert_concat_to_tpl_expr(expr: Expr) -> Expr {
  match expr {
    Expr::Call(ref call_expr) => concat_call_to_template_literal(call_expr).unwrap_or(expr),
    _ => expr,
  }
}

pub(crate) fn concat_call_to_template_literal(call_expr: &CallExpr) -> Option<Expr> {
  use swc_core::common::DUMMY_SP;

  let callee = call_expr.callee.as_expr()?;
  let member_expr = callee.as_member()?;
  let prop_ident = member_expr.prop.as_ident()?;

  if prop_ident.sym.as_ref() != "concat" {
    return None;
  }

  let object_lit = member_expr.obj.as_lit()?;
  let base_string = match extract_str_lit_ref(object_lit) {
    Some(base_string) => base_string.to_string(),
    None => return None,
  };

  let mut exprs = Vec::new();
  let mut quasis = Vec::new();

  quasis.push(TplElement {
    span: DUMMY_SP,
    tail: false,
    cooked: Some(base_string.clone().into()),
    raw: base_string.into(),
  });

  for (i, arg) in call_expr.args.iter().enumerate() {
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

  Some(Expr::Tpl(Tpl {
    span: DUMMY_SP,
    exprs,
    quasis,
  }))
}

pub fn create_number_expr(value: f64) -> Expr {
  Expr::from(create_number_lit(value))
}

pub fn create_big_int_expr(value: BigInt) -> Expr {
  Expr::from(create_big_int_lit(value))
}

pub fn create_string_expr(value: &str) -> Expr {
  Expr::Lit(create_string_lit(value))
}

pub fn create_bool_expr(value: bool) -> Expr {
  Expr::Lit(create_boolean_lit(value))
}

pub fn create_ident_expr(value: &str) -> Expr {
  Expr::Ident(create_ident(value))
}

pub fn create_null_expr() -> Expr {
  Expr::Lit(create_null_lit())
}

fn should_wrap_prop_name_key_with_quotes(key: &str) -> bool {
  if Ident::verify_symbol(key).is_ok() {
    return false;
  }

  !Context::default().is_reserved_word(&key.into())
}

pub fn convert_string_to_prop_name(value: &str) -> PropName {
  if should_wrap_prop_name_key_with_quotes(value) {
    PropName::Str(quote_str!(value))
  } else {
    PropName::Ident(quote_ident!(value))
  }
}

pub fn expand_shorthand_prop(prop: &mut Box<Prop>) {
  if let Some(ident) = prop.as_shorthand() {
    **prop = Prop::from(KeyValueProp {
      key: convert_string_to_prop_name(ident.sym.as_ref()),
      value: Box::new(Expr::Ident(ident.clone())),
    });
  }
}

pub fn convert_atom_to_string(atom: &Wtf8Atom) -> String {
  match atom.as_str() {
    Some(value) => value.to_string(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

pub fn convert_wtf8_to_atom(atom: &Wtf8Atom) -> Atom {
  match atom.as_atom() {
    Some(value) => value.clone(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

pub fn convert_str_lit_to_string(str_lit: &Str) -> String {
  match str_lit.value.as_str() {
    Some(value) => value.to_string(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

pub fn convert_str_lit_to_atom(str_lit: &Str) -> Atom {
  match str_lit.value.as_atom() {
    Some(value) => value.clone(),
    None => stylex_panic!("{}", INVALID_UTF8),
  }
}

pub fn extract_tpl_cooked_value(elem: &TplElement) -> &str {
  match elem.cooked.as_ref() {
    Some(cooked) => match cooked.as_str() {
      Some(value) => value,
      None => stylex_panic!("{}", INVALID_UTF8),
    },
    None => stylex_panic!(
      "Template literal element has no cooked value (contains an invalid escape sequence)."
    ),
  }
}

pub fn convert_atom_to_str_ref(atom: &swc_core::atoms::Wtf8Atom) -> &str {
  match atom.as_str() {
    Some(value) => value,
    None => stylex_panic!("Failed to convert SWC Atom to string (invalid WTF-8 encoding)."),
  }
}

pub fn convert_lit_to_string(value: &Lit) -> Option<String> {
  match value {
    Lit::Str(strng) => Some(convert_str_lit_to_string(strng)),
    Lit::Num(num) => Some(format!("{}", num.value)),
    Lit::BigInt(big_int) => Some(format!("{}", big_int.value)),
    _ => None,
  }
}

pub fn extract_str_lit_ref(lit: &Lit) -> Option<&str> {
  match lit {
    Lit::Str(strng) => Some(convert_atom_to_str_ref(&strng.value)),
    _ => None,
  }
}

#[inline]
pub fn convert_key_value_to_str(key_value: &KeyValueProp) -> String {
  let key = &key_value.key;
  let should_wrap_in_quotes = false;

  let key = match key {
    PropName::Ident(ident) => ident.sym.to_string(),
    PropName::Str(strng) => convert_str_lit_to_string(strng),
    PropName::Num(num) => num.value.to_string(),
    PropName::BigInt(big_int) => big_int.value.to_string(),
    PropName::Computed(computed) => match computed.expr.as_ref() {
      Expr::Lit(lit) => match convert_lit_to_string(lit) {
        Some(s) => s,
        None => stylex_panic!("Computed property key must be a string or number literal."),
      },
      Expr::Tpl(tpl) => {
        match convert_tpl_to_string_lit(tpl).and_then(|lit| convert_lit_to_string(&lit)) {
          Some(s) => s,
          None => stylex_unimplemented!("Computed key is not a literal"),
        }
      },
      _ => stylex_unimplemented!("Computed key is not a literal"),
    },
  };

  wrap_key_in_quotes(&key, should_wrap_in_quotes).into_owned()
}

pub fn get_key_values_from_object(object: &ObjectLit) -> Vec<KeyValueProp> {
  object
    .props
    .iter()
    .map(|prop| match prop {
      PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();
        expand_shorthand_prop(&mut prop);
        match prop.as_ref() {
          Prop::KeyValue(key_value) => key_value.clone(),
          _ => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
        }
      },
    })
    .collect()
}
