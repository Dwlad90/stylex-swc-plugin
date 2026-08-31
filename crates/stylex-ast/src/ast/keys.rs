//! Reading the authored name of an object property or a member.
//!
//! A key is written in several shapes -- an identifier, a string, a number, a
//! big integer, or a computed literal -- and every one of them names the same
//! property at run time. These readers answer with that name, and answer with
//! nothing where the shape carries no static name at all.

use swc_core::{
  atoms::Atom,
  ecma::ast::{Expr, KeyValueProp, Lit, MemberProp, ObjectLit, Prop, PropName, PropOrSpread},
};

use super::convertors::{convert_str_lit_to_atom, convert_tpl_to_string_lit};

pub fn namespace_name_from_prop_key(key: &PropName) -> Option<Atom> {
  match key {
    PropName::Ident(ident) => Some(ident.sym.clone()),
    PropName::Str(strng) => Some(convert_str_lit_to_atom(strng)),
    PropName::Num(num) => Some(Atom::from(num.value.to_string())),
    PropName::BigInt(big_int) => Some(Atom::from(big_int.value.to_string())),
    PropName::Computed(computed) => namespace_name_from_expr(computed.expr.as_ref()),
  }
}

pub fn namespace_name_from_member_prop(prop: &MemberProp) -> Option<Atom> {
  match prop {
    MemberProp::Ident(ident) => Some(ident.sym.clone()),
    MemberProp::Computed(computed) => namespace_name_from_expr(computed.expr.as_ref()),
    MemberProp::PrivateName(_) => None,
  }
}

fn namespace_name_from_lit(lit: &Lit) -> Option<Atom> {
  match lit {
    Lit::Str(strng) => Some(convert_str_lit_to_atom(strng)),
    Lit::Num(num) => Some(Atom::from(num.value.to_string())),
    Lit::BigInt(big_int) => Some(Atom::from(big_int.value.to_string())),
    _ => None,
  }
}

fn namespace_name_from_expr(expr: &Expr) -> Option<Atom> {
  match expr {
    Expr::Lit(lit) => namespace_name_from_lit(lit),
    Expr::Tpl(tpl) => convert_tpl_to_string_lit(tpl)
      .as_ref()
      .and_then(namespace_name_from_lit),
    _ => None,
  }
}

/// The literal keys of an object literal, in source order, duplicates included
/// -- a key written twice is two properties, and a count over them reads it that
/// way.
pub fn collect_object_lit_keys(object: &ObjectLit) -> impl Iterator<Item = Atom> + '_ {
  object.props.iter().filter_map(|prop| {
    prop_as_key_value(prop).and_then(|key_value| namespace_name_from_prop_key(&key_value.key))
  })
}

/// Returns `Some(kv)` only for `PropOrSpread::Prop(Box<Prop::KeyValue>)`
/// shapes; any other variant (spread, method, getter, setter, shorthand, …)
/// yields `None`. Callers typically use this to skip props they can't handle in
/// a single pass.
pub fn prop_as_key_value(prop: &PropOrSpread) -> Option<&KeyValueProp> {
  match prop {
    PropOrSpread::Prop(p) => match p.as_ref() {
      Prop::KeyValue(kv) => Some(kv),
      _ => None,
    },
    _ => None,
  }
}
