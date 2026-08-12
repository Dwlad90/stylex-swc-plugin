use anyhow::anyhow;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use stylex_utils::{string::wrap_key_in_quotes, swc::get_default_expr_ctx};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  ecma::{
    ast::{
      BigInt, Bool, CallExpr, Expr, Ident, KeyValueProp, Lit, MemberProp, Number, ObjectLit, Prop,
      PropName, PropOrSpread, Str, Tpl, TplElement, VarDeclarator,
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

/// Renders a numeric AST literal exactly as JS `String(Number)` does, derived
/// from its parsed value rather than the raw source token (e.g. `0x10` ->
/// `"16"`).
///
/// Rust's `f64` `Display` is not a substitute: it never switches to
/// exponential form, so `1e21` would render as `"1000000000000000000000"` where
/// JS renders `"1e+21"`. Since this rendering feeds the class-name hash, the
/// spelling itself is observable and has to match, not merely round-trip to the
/// same `f64`.
pub fn convert_number_to_js_string(n: &Number) -> String {
  let value = n.value;

  if value.is_nan() {
    return "NaN".to_string();
  }
  if value.is_infinite() {
    return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
  }
  if value == 0.0 {
    // Covers `-0`, which JS also renders as `"0"`.
    return "0".to_string();
  }

  let mut result = String::with_capacity(24);

  if value < 0.0 {
    result.push('-');
  }

  // `s` and `n` are ECMA-262's `Number::toString` variables: `s` is the shortest
  // digit string that round-trips, and the value is `s × 10^(n - k)` where `k`
  // is the digit count. Rust's `LowerExp` emits both, as `d.ddde±x`.
  let (s, n) = shortest_digits_and_exponent(value.abs());
  let k = s.len() as i32;

  if k <= n && n <= 21 {
    result.push_str(&s);
    for _ in 0..(n - k) {
      result.push('0');
    }
  } else if 0 < n && n <= 21 {
    let (integral, fractional) = s.split_at(n as usize);
    result.push_str(integral);
    result.push('.');
    result.push_str(fractional);
  } else if -6 < n && n <= 0 {
    result.push_str("0.");
    for _ in 0..(-n) {
      result.push('0');
    }
    result.push_str(&s);
  } else {
    let (first, rest) = s.split_at(1);
    result.push_str(first);
    if !rest.is_empty() {
      result.push('.');
      result.push_str(rest);
    }
    result.push('e');
    result.push(if n >= 1 { '+' } else { '-' });
    result.push_str(&(n - 1).abs().to_string());
  }

  result
}

/// Decomposes a finite, strictly positive `f64` into ECMA-262's `s` (the
/// shortest round-tripping digit string) and `n` (the decimal exponent, such
/// that the value is `s × 10^(n - s.len())`).
///
/// Rust's `LowerExp` already picks the same shortest digits as JS, so this only
/// has to re-read them out of `d.ddde±x`. The scan is hand-rolled rather than
/// `split_once` + `parse` so that every branch is reachable for some input and
/// no unreachable error path is left behind.
fn shortest_digits_and_exponent(value: f64) -> (String, i32) {
  let formatted = format!("{:e}", value);

  let mut digits = String::with_capacity(17);
  let mut exponent = 0i32;
  let mut exponent_is_negative = false;
  let mut in_exponent = false;

  for ch in formatted.chars() {
    match ch {
      'e' => in_exponent = true,
      '.' => {},
      '-' => exponent_is_negative = true,
      _ if in_exponent => exponent = exponent * 10 + i32::from(ch as u8 - b'0'),
      _ => digits.push(ch),
    }
  }

  if exponent_is_negative {
    exponent = -exponent;
  }

  // ECMA's `n` is one past the exponent of the leading digit.
  (digits, exponent + 1)
}

/// Reads the static key of a member property as a string:
/// - non-computed identifier → the identifier name
/// - computed string literal → the string value (`None` on invalid UTF-8)
/// - computed numeric literal → JS `String(Number)` rendering
///
/// Returns `None` for private names and non-literal computed keys, so callers
/// can treat a dynamic key as "not statically resolvable".
pub fn convert_member_prop_to_string(prop: &MemberProp) -> Option<String> {
  match prop {
    MemberProp::Ident(ident) => Some(ident.sym.to_string()),
    MemberProp::Computed(computed) => {
      convert_static_member_key_expr_to_string(normalize_expr(computed.expr.as_ref()))
    },
    MemberProp::PrivateName(_) => None,
  }
}

/// Unwraps parenthesized expressions, returning a reference to the innermost
/// non-paren expression. Spans are preserved. Use [`normalize_expr_mut`] when
/// the caller needs to mutate the unwrapped node.
pub fn normalize_expr(mut expr: &Expr) -> &Expr {
  while let Expr::Paren(paren) = expr {
    expr = paren.expr.as_ref();
  }
  expr
}

/// Mutable counterpart to [`normalize_expr`]: unwraps parenthesized
/// expressions, returning a mutable reference to the innermost non-paren
/// expression. Spans are preserved, so callers that depend on position
/// information (span containment, or span-insensitive comparison via
/// `eq_ignore_span`) keep working on the returned node. This is the
/// read/mutate counterpart to [`crate::ast::factories::wrap_in_paren`].
pub fn normalize_expr_mut(mut expr: &mut Expr) -> &mut Expr {
  while let Expr::Paren(paren) = expr {
    expr = paren.expr.as_mut();
  }
  expr
}

fn convert_static_member_key_expr_to_string(expr: &Expr) -> Option<String> {
  match expr {
    Expr::Lit(Lit::Str(s)) => s.value.as_str().map(|value| value.to_string()),
    Expr::Lit(Lit::Num(n)) => Some(convert_number_to_js_string(n)),
    Expr::Lit(Lit::BigInt(big_int)) => Some(big_int.value.to_string()),
    Expr::Tpl(tpl) => convert_tpl_to_string_lit(tpl).and_then(|lit| convert_lit_to_string(&lit)),
    _ => None,
  }
}

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
  let base_string = {
    let base_string = extract_str_lit_ref(object_lit)?;
    base_string.to_string()
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
    Lit::Num(num) => Some(convert_number_to_js_string(num)),
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

/// Extracts the initializer expression from a variable declarator.
///
/// # Panics
/// Panics (via `stylex_panic!`) when the declarator has no initializer, e.g.
/// `let x;`. Callers that may encounter uninitialized declarators must guard on
/// `var_decl.init.is_some()` first.
pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> &Expr {
  match &var_decl.init {
    Some(var_decl_init) => var_decl_init,
    None => stylex_panic!("Variable declaration must be initialized with an expression."),
  }
}
