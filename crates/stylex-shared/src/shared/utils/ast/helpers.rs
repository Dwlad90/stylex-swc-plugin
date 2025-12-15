use swc_core::ecma::ast::{ExportSpecifier, ModuleExportName};

use crate::shared::{
  enums::data_structures::top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
  structures::state_manager::StateManager,
};

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
