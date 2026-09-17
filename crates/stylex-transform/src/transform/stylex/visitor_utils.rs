use std::rc::Rc;

use rustc_hash::FxHashMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::{
  transformers::{
    stylex_keyframes::get_keyframes_fn, stylex_position_try::get_position_try_fn,
    stylex_types::get_types_fn,
  },
  utils::validators::is_target_call,
};
use stylex_constants::constants::api_names::{
  STYLEX_ENV, STYLEX_FIRST_THAT_WORKS, STYLEX_KEYFRAMES, STYLEX_POSITION_TRY, STYLEX_TYPES,
  STYLEX_UNSTABLE_CONDITIONAL,
};
use stylex_evaluator::stylex_first_that_works::stylex_first_that_works;
use stylex_state::{
  functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType, NestedRuleHelpers},
  state_manager::{ImportKind, StateManager},
  types::{FunctionConfigMap, FunctionMapIdentifiers, FunctionMapMemberExpression},
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
  entry: FunctionConfigType,
) {
  let identifier = identifiers
    .entry(name.get_import_str().into())
    .or_insert_with(|| Box::new(FunctionConfigType::Map(FunctionConfigMap::default())));

  if let Some(identifier_map) = identifier.as_map_mut() {
    identifier_map.insert(key, entry);
  }
}

/// Registers `env` as an entry of the namespace's own fold, which is the map
/// that answers what properties the namespace *has* — its own keys, a spread
/// of it, and the member read that walks it.
///
/// Beside [`insert_stylex_identifier_entry`] rather than on `StateManager`,
/// because it reads one field of the state and writes the caller's map: living on
/// the state meant `shared::structures` importing out of `transform::stylex` to
/// reach the helper, which is the dependency the other way round from every
/// other pair in this crate.
///
/// Apart from `StateManager::apply_stylex_env`, and called only where a `create`
/// call sets its evaluation up, because this is the one registration that
/// makes a *bare* namespace reference resolve. The other calls that build a
/// function map — `keyframes`, `positionTry`, `viewTransitionClass`,
/// `defineConsts` — deliberately leave the namespace name unregistered so a
/// bare `stylex` written where a static value belongs refuses rather than
/// materializing into an object and dropping the declaration silently. Adding
/// the entry there too flipped four of those refusals into silent drops.
///
/// Registered whether or not the option is set. `env` is a key of the StyleX
/// namespace however the compiler is configured, so an unset option is
/// reported by the member step as the option being unset; an absent entry
/// would be reported as a property nobody can find and send an author looking
/// in their source. It is also what keeps the namespace's key list the same
/// list on every configuration.
pub(crate) fn register_env_in_namespace_fold(
  state: &StateManager,
  identifiers: &mut FunctionMapIdentifiers,
) {
  for name in state.stylex_imports() {
    insert_stylex_identifier_entry(
      identifiers,
      name,
      STYLEX_ENV.into(),
      FunctionConfigType::EnvObject(Rc::clone(&state.options.env)),
    );
  }
}

/// The function map a nested-rule call -- `keyframes`, `positionTry` or
/// `viewTransitionClass` -- evaluates its argument with, built on the first
/// call of the module that asks for `helpers` and kept for the rest.
///
/// One shape written three times before this: each of the three call handlers
/// built the same map for itself, on every call, even in a module that never
/// writes one of the helpers.
pub(crate) fn nested_rule_eval_config(
  state: &mut StateManager,
  helpers: NestedRuleHelpers,
) -> Rc<FunctionMap> {
  if let Some(function_map) = state.cached_nested_rule_function_map(helpers) {
    return Rc::clone(function_map);
  }

  let function_map = Rc::new(build_nested_rule_eval_config(state, helpers));

  state.insert_cached_nested_rule_function_map(helpers, Rc::clone(&function_map));

  function_map
}

fn build_nested_rule_eval_config(state: &StateManager, helpers: NestedRuleHelpers) -> FunctionMap {
  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  register_stylex_helper(
    state,
    &mut identifiers,
    &mut member_expressions,
    ImportKind::FirstThatWorks,
    STYLEX_FIRST_THAT_WORKS,
    &FunctionConfig {
      fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
      takes_path: false,
    },
  );

  if helpers == NestedRuleHelpers::FirstThatWorksAndKeyframes {
    register_stylex_helper(
      state,
      &mut identifiers,
      &mut member_expressions,
      ImportKind::Keyframes,
      STYLEX_KEYFRAMES,
      &get_keyframes_fn(),
    );
  }

  state.apply_stylex_env(&mut identifiers, &mut member_expressions);

  FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: false,
  }
}

/// Registers one helper under every local name an import gave it, and as
/// `member_name` on every StyleX namespace the module imports.
///
/// The namespace name itself stays unregistered, for the reason
/// [`register_env_in_namespace_fold`] gives.
///
/// Every map built here registers at least one helper this way, so the
/// create-then-insert pair is spelled once.
fn register_stylex_helper(
  state: &StateManager,
  identifiers: &mut FunctionMapIdentifiers,
  member_expressions: &mut FunctionMapMemberExpression,
  kind: ImportKind,
  member_name: &str,
  config: &FunctionConfig,
) {
  if let Some(set) = state.get_stylex_api_import(kind) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(config.clone())),
      );
    }
  }

  for name in state.stylex_imports() {
    member_expressions.entry(name.clone()).or_default().insert(
      member_name.into(),
      Box::new(FunctionConfigType::Regular(config.clone())),
    );
  }
}

pub(crate) fn build_eval_config(state: &mut StateManager) -> FunctionMap {
  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  let types_fn = get_types_fn();

  register_stylex_helper(
    state,
    &mut identifiers,
    &mut member_expressions,
    ImportKind::Keyframes,
    STYLEX_KEYFRAMES,
    &get_keyframes_fn(),
  );

  register_stylex_helper(
    state,
    &mut identifiers,
    &mut member_expressions,
    ImportKind::PositionTry,
    STYLEX_POSITION_TRY,
    &get_position_try_fn(),
  );

  if let Some(set) = state.get_stylex_api_import(ImportKind::Types) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(types_fn.clone())),
      );
    }
  }

  // `types` is the one helper the namespace carries in its own fold rather than
  // as a member alone, because a `create` call reads `stylex.types` off a
  // namespace it also spreads.
  for name in state.stylex_imports() {
    insert_stylex_identifier_entry(
      &mut identifiers,
      name,
      STYLEX_TYPES.into(),
      FunctionConfigType::Regular(types_fn.clone()),
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
  register_stylex_helper(
    state,
    identifiers,
    member_expressions,
    ImportKind::Conditional,
    STYLEX_UNSTABLE_CONDITIONAL,
    &get_conditional_fn(),
  );
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

fn conditional_identity(expr: Expr, _: &mut StateManager) -> Expr {
  expr
}
