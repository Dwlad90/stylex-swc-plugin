use rustc_hash::FxHashMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::{
  structures::{
    functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    state_manager::{ImportKind, StateManager},
    types::{FunctionConfigMap, FunctionMapIdentifiers, FunctionMapMemberExpression},
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
use stylex_structures::named_import_source::ImportSources;

pub(crate) fn is_call_to(
  call: &CallExpr,
  state: &StateManager,
  kind: ImportKind,
  name: &str,
) -> bool {
  is_target_call((name, state.get_stylex_api_import(kind)), call, state)
}

/// Register `entry` under `key` on the identifier map a stylex import's name
/// carries, creating the map when that name has none yet.
///
/// Every registration on a namespace import needs the same create-then-insert,
/// and each of the four wrote it by hand -- one of them as an
/// `and_modify`/`or_insert_with` pair that spelled the insert twice.
///
/// A name already bound to something that is not a map keeps it, and the entry
/// is dropped. That is what all four did, and it is what an import spelling an
/// API name over the namespace has always meant here.
pub(crate) fn insert_stylex_identifier_entry(
  identifiers: &mut FunctionMapIdentifiers,
  name: &ImportSources,
  key: Atom,
  entry: FunctionConfig,
) {
  let identifier = identifiers
    .entry(name.get_import_str().into())
    .or_insert_with(|| Box::new(FunctionConfigType::Map(FunctionConfigMap::default())));

  if let Some(identifier_map) = identifier.as_map_mut() {
    identifier_map.insert(key, entry);
  }
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

    insert_stylex_identifier_entry(
      &mut identifiers,
      name,
      STYLEX_TYPES.into(),
      types_fn.clone(),
    );
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

pub(crate) fn build_env_only_eval_config(state: &mut StateManager) -> FunctionMap {
  let mut identifiers = FxHashMap::default();
  let mut member_expressions = FxHashMap::default();
  state.apply_stylex_env(&mut identifiers, &mut member_expressions);
  FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: true,
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
