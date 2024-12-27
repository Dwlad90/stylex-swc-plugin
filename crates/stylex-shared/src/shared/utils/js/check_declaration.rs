use swc_core::{
  common::EqIgnoreSpan,
  ecma::ast::{Expr, Ident},
};

use crate::shared::{
  constants::evaluation_errors::{unsupported_expression, UNDEFINED_CONST},
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::state::EvaluationState,
};

use super::evaluate::deopt;

pub(crate) enum DeclarationType {
  Class,
  Function,
}

pub(crate) fn check_ident_declaration(
  ident: &Ident,
  declarations_map: &[(DeclarationType, &Vec<Ident>)],
  state: &mut EvaluationState,
  path: &Expr,
) -> Option<EvaluateResultValue> {
  for (decl_type, declarations) in declarations_map {
    if declarations.iter().any(|item| item.eq_ignore_span(ident)) {
      return deopt(
        path,
        state,
        &match decl_type {
          DeclarationType::Class => unsupported_expression("ClassDeclaration"),
          DeclarationType::Function => unsupported_expression("FunctionDeclaration"),
        },
      );
    }
  }

  deopt(path, state, UNDEFINED_CONST)
}
