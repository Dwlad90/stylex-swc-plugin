use super::super::*;
use swc_core::ecma::ast::Ident;

pub(in super::super) fn evaluate(
  ident: &Ident,
  state: &mut EvaluationState,
) -> Option<EvaluateResultValue> {
  let atom_ident_id = &ident.sym;

  if let Some(func) = state.functions.identifiers.get(atom_ident_id) {
    match func.as_ref() {
      FunctionConfigType::Regular(func) => match &func.fn_ptr {
        FunctionType::Mapper(func) => {
          return Some(EvaluateResultValue::Expr(func()));
        },
        FunctionType::ThemeRefMapper(f) => {
          return Some(EvaluateResultValue::ThemeRef(f()));
        },
        FunctionType::DefaultMarker(func) => {
          return Some(EvaluateResultValue::FunctionConfig(FunctionConfig {
            fn_ptr: FunctionType::DefaultMarker(Arc::clone(func)),
            takes_path: false,
          }));
        },
        _ => {
          let path = Expr::Ident(ident.clone());
          return deopt(&path, state, "Function not found");
        },
      },
      FunctionConfigType::Map(func_map) => {
        return Some(EvaluateResultValue::FunctionConfigMap(func_map.clone()));
      },
      // An index map carries no value the evaluator reads through an
      // identifier, which is a shape it does not fold rather than a broken
      // invariant.
      FunctionConfigType::IndexMap(_func_map) => {
        let path = Expr::Ident(ident.clone());

        return deopt(&path, state, NON_CONSTANT);
      },
      FunctionConfigType::EnvObject(env_map) => {
        return Some(EvaluateResultValue::EnvObject(env_map.clone()));
      },
    }
  }

  None
}
