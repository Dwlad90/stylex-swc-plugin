use swc_core::{
  common::EqIgnoreSpan,
  ecma::ast::{Expr, Ident},
};

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{state::EvaluationState, state_manager::StateManager},
};
use stylex_constants::constants::evaluation_errors::{UNDEFINED_CONST, unsupported_expression};

use super::evaluate::{deopt, deopt_at_declaration};

pub(crate) enum DeclarationType {
  Class,
  Function,
}

/// The tail of the reference chain (`evaluate-path.js:685-690`, 0.19.0): a name
/// whose binding holds no initializer to fold is refused for the kind of
/// declaration it names, or for naming nothing this compiler can resolve.
///
/// The two arms report against different nodes, and upstream's do too. A
/// declaration kind is refused where upstream refuses it — inside
/// `evaluateCached` over the node `path.resolve()` handed back, which is the
/// declaration — so its frame names the `function` or `class` line. Measured on
/// 0.19.0, `create({ x: { color: f } })` under `function f() {}` frames the
/// declaration and carries a caret over the whole of it. `UNDEFINED_CONST` is
/// upstream's one refusal on the reference itself (`:687`), because by then
/// there is no declaration to name: the reference resolved to itself.
pub(crate) fn check_ident_declaration(
  ident: &Ident,
  declarations_map: &[(DeclarationType, &[Ident])],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  path: &Expr,
) -> Option<EvaluateResultValue> {
  for (decl_type, declarations) in declarations_map {
    if declarations.iter().any(|item| item.eq_ignore_span(ident)) {
      return deopt_at_declaration(
        path,
        &ident.sym,
        state,
        traversal_state,
        &match decl_type {
          DeclarationType::Class => unsupported_expression("ClassDeclaration"),
          DeclarationType::Function => unsupported_expression("FunctionDeclaration"),
        },
      );
    }
  }

  deopt(path, state, UNDEFINED_CONST)
}
