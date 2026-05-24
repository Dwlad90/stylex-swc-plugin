use rustc_hash::FxHashMap;
use swc_core::ecma::ast::{CallExpr, Expr};

use crate::shared::{
  structures::{
    functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    state_manager::{ImportKind, StateManager},
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::{
    stylex_keyframes::get_keyframes_fn, stylex_position_try::get_position_try_fn,
    stylex_types::get_types_fn,
  },
  utils::validators::is_target_call,
};
use stylex_constants::constants::api_names::{
  STYLEX_KEYFRAMES, STYLEX_POSITION_TRY, STYLEX_TYPES, STYLEX_UNSTABLE_CONDITIONAL,
};

pub(crate) fn is_call_to(
  call: &CallExpr,
  state: &StateManager,
  kind: ImportKind,
  name: &str,
) -> bool {
  is_target_call((name, state.get_stylex_api_import(kind)), call, state)
}

pub(crate) fn build_eval_config(state: &mut StateManager) -> FunctionMap {
  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  let keyframes_fn = get_keyframes_fn();
  let types_fn = get_types_fn();
  let position_try_fn = get_position_try_fn();

  if let Some(set) = state.get_stylex_api_import(ImportKind::Keyframes) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
      );
    }
  }

  if let Some(set) = state.get_stylex_api_import(ImportKind::PositionTry) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
      );
    }
  }

  if let Some(set) = state.get_stylex_api_import(ImportKind::Types) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(types_fn.clone())),
      );
    }
  }

  for name in state.stylex_imports() {
    let member_expression = member_expressions.entry(name.clone()).or_default();

    member_expression.insert(
      STYLEX_KEYFRAMES.into(),
      Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
    );
    member_expression.insert(
      STYLEX_POSITION_TRY.into(),
      Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
    );

    let identifier = identifiers
      .entry(name.get_import_str().into())
      .or_insert_with(|| Box::new(FunctionConfigType::Map(FxHashMap::default())));

    if let Some(identifier_map) = identifier.as_map_mut() {
      identifier_map.insert(STYLEX_TYPES.into(), types_fn.clone());
    }
  }

  apply_unstable_conditional(state, &mut identifiers, &mut member_expressions);
  state.apply_stylex_env(&mut identifiers, &mut member_expressions);

  FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: false,
  }
}

pub(crate) fn apply_unstable_conditional(
  state: &StateManager,
  identifiers: &mut FunctionMapIdentifiers,
  member_expressions: &mut FunctionMapMemberExpression,
) {
  let conditional_fn = get_conditional_fn();

  if let Some(set) = state.get_stylex_api_import(ImportKind::Conditional) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(conditional_fn.clone())),
      );
    }
  }

  for name in state.stylex_imports() {
    let member_expression = member_expressions.entry(name.clone()).or_default();
    member_expression.insert(
      STYLEX_UNSTABLE_CONDITIONAL.into(),
      Box::new(FunctionConfigType::Regular(conditional_fn.clone())),
    );
  }
}

fn get_conditional_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexExprFn(conditional_identity),
    takes_path: false,
  }
}

fn conditional_identity(expr: Expr, _: &mut dyn stylex_types::traits::StyleOptions) -> Expr {
  expr
}
