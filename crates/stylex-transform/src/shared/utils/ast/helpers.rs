use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::top_level_expression::TopLevelExpression;
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{
      ArrowExpr, ExportSpecifier, Expr, KeyValueProp, Lit, MemberProp, ModuleExportName, Prop,
      PropName, PropOrSpread,
    },
    visit::{Visit, VisitWith},
  },
};

use crate::shared::structures::state_manager::StateManager;

use super::convertors::{convert_str_lit_to_atom, convert_tpl_to_string_lit};

pub(crate) fn is_variable_named_exported(
  TopLevelExpression(kind, _, variable_name): &TopLevelExpression,
  state: &StateManager,
) -> bool {
  if matches!(kind, TopLevelExpressionKind::NamedExport) {
    return true;
  }

  let Some(var_name) = variable_name else {
    return false;
  };

  for named_export in &state.named_exports {
    for specifier in &named_export.specifiers {
      if let ExportSpecifier::Named(named_specifier) = specifier
        && matches!(&named_specifier.orig, ModuleExportName::Ident(ident) if ident.sym == *var_name)
      {
        return named_export.src.is_none() && named_specifier.exported.is_none();
      }
    }
  }
  false
}

pub fn get_property_by_key<'a>(expr: &'a Expr, key: &str) -> Option<&'a Expr> {
  match expr {
    Expr::Object(obj) => {
      for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
          && let Some(kv) = prop.as_key_value()
        {
          let k = match &kv.key {
            PropName::Ident(id) => Some(id.sym.as_ref()),
            PropName::Str(s) => s.value.as_str(),
            _ => None,
          };
          if k == Some(key) {
            return Some(&kv.value);
          }
        }
      }
      None
    },
    _ => None,
  }
}

pub(crate) fn namespace_name_from_prop_key(key: &PropName) -> Option<Atom> {
  match key {
    PropName::Ident(ident) => Some(ident.sym.clone()),
    PropName::Str(strng) => Some(convert_str_lit_to_atom(strng)),
    PropName::Num(num) => Some(Atom::from(num.value.to_string())),
    PropName::BigInt(big_int) => Some(Atom::from(big_int.value.to_string())),
    PropName::Computed(computed) => namespace_name_from_expr(computed.expr.as_ref()),
  }
}

pub(crate) fn namespace_name_from_member_prop(prop: &MemberProp) -> Option<Atom> {
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

/// Returns `Some(kv)` only for `PropOrSpread::Prop(Box<Prop::KeyValue>)`
/// shapes; any other variant (spread, method, getter, setter, shorthand, …)
/// yields `None`. Callers typically use this to skip props they can't handle in
/// a single pass.
pub(crate) fn prop_as_key_value(prop: &PropOrSpread) -> Option<&KeyValueProp> {
  match prop {
    PropOrSpread::Prop(p) => match p.as_ref() {
      Prop::KeyValue(kv) => Some(kv),
      _ => None,
    },
    _ => None,
  }
}

/// Returns `true` if `expr` contains any `Expr::Arrow` anywhere in its subtree
/// (object values, parens, conditionals, arrays, calls, template literals, …).
/// Used to decide whether a rewrite pass is necessary without paying the cost
/// of cloning the AST eagerly.
pub(crate) fn expr_contains_arrow(expr: &Expr) -> bool {
  let mut finder = ArrowFinder { found: false };
  expr.visit_with(&mut finder);
  finder.found
}

/// Returns `true` if `prop`'s value (or anything nested inside it) is an
/// `Expr::Arrow`. Companion to [`expr_contains_arrow`].
pub(crate) fn prop_contains_arrow(prop: &PropOrSpread) -> bool {
  match prop_as_key_value(prop) {
    Some(kv) => expr_contains_arrow(&kv.value),
    None => false,
  }
}

/// SWC `Visit` implementation that flags `true` on the first `ArrowExpr` it
/// encounters, regardless of the surrounding expression kind. Default
/// `visit_children_with` dispatch covers all variants (Cond, Logical, Array,
/// Seq, TaggedTpl, New, Call, …) so the check is exhaustive.
struct ArrowFinder {
  found: bool,
}

impl Visit for ArrowFinder {
  fn visit_arrow_expr(&mut self, _: &ArrowExpr) {
    self.found = true;
  }

  fn visit_expr(&mut self, expr: &Expr) {
    if self.found {
      return;
    }
    expr.visit_children_with(self);
  }
}
