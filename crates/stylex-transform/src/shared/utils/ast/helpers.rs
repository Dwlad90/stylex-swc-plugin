use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::top_level_expression::TopLevelExpression;
use swc_core::ecma::ast::{ExportSpecifier, Expr, ModuleExportName, PropName, PropOrSpread};

use crate::shared::structures::state_manager::StateManager;

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

pub fn get_property_by_key<'a>(expr: &'a Expr, key: impl AsRef<str>) -> Option<&'a Expr> {
  let key = key.as_ref();
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
