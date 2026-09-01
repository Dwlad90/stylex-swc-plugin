use std::{borrow::Borrow, rc::Rc, sync::Arc};

mod binding;
mod cache;
mod deopt;
mod engine_fold;
mod engine_stylex_functions;
mod helpers;
mod nodes;

pub(crate) use cache::{Memoized, evaluate_cached, folded_once};
pub(crate) use deopt::{deopt, deopt_at_declaration};
pub use helpers::evaluate_result_is_nullish;
use helpers::*;
pub(crate) use nodes::binary_expression::binary_expr_to_num_or_str;
pub use nodes::object_expression::spread_own_properties;

use stylex_constants::constants::api_names::FUNCTION_CONFIG_FN_KEY;

use indexmap::IndexMap;
use log::{debug, warn};
use rustc_hash::{FxHashMap, FxHashSet};
use stylex_macros::{
  deopt_unsupported, expr_to_str_or_deopt, stylex_panic, stylex_panic_with_context,
  stylex_unreachable,
};
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{
      ArrayLit, BlockStmtOrExpr, CallExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Ident,
      ImportSpecifier, KeyValueProp, Lit, MemberProp, ModuleExportName, ObjectLit, OptChainBase,
      Pat, Prop, PropName, PropOrSpread, TplElement, VarDeclarator,
    },
    utils::ident::IdentLike,
  },
};

use crate::convertors::expr_to_num;
use crate::{evaluate_result::EvaluateResult, state::EvaluationState};
use stylex_ast::ast::convertors::{
  convert_atom_to_str_ref, convert_atom_to_string, convert_key_value_to_str, convert_lit_to_string,
  create_big_int_expr, create_bool_expr, create_null_expr, create_number_expr, create_string_expr,
  expand_shorthand_prop, extract_tpl_cooked_value, is_js_undefined, normalize_expr,
};
use stylex_ast::ast::factories::{
  create_array_expression, create_arrow_expression, create_expr_or_spread,
  create_ident_key_value_prop, create_key_value_prop, create_object_lit, wrap_in_paren_ref,
};
use stylex_ast::ast::objects::{assign_props, order_own_keys, remove_duplicates};
use stylex_constants::constants::{
  evaluation_errors::{
    CONCATENATION, FUNCTION_BODY_WITHOUT_VALUE, IMPORT_FILE_EVAL_ERROR,
    IMPORT_PATH_RESOLUTION_ERROR, NON_CONSTANT, NUMERIC_CONVERSION, OBJECT_METHOD,
    PATH_WITHOUT_NODE, SPREAD_ELEMENT, TEMPLATE_LITERAL, UNDEFINED_CONST, UNEXPECTED_MEMBER_LOOKUP,
    UNINITIALIZED_CONST, USED_BEFORE_DECLARATION, grown_string_too_large, unfoldable_call,
    unsupported_expression, unsupported_operator,
  },
  messages::{
    ARGUMENT_NOT_EXPRESSION, EXPECTED_CSS_VAR, EXPRESSION_IS_NOT_A_STRING,
    ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE, KEY_VALUE_EXPECTED, MEMBER_NOT_RESOLVED,
    MEMBER_OBJ_NOT_IDENT, NULLISH_TO_OBJECT, OBJECT_KEY_MUST_BE_IDENT, PROPERTY_NOT_FOUND,
    SPREAD_HIDES_OBJECT_KEYS, SPREAD_NOT_SUPPORTED, SPREAD_PROPERTIES_UNREADABLE,
    THEME_IMPORT_KEY_AS_OBJECT_KEY, VALUE_MUST_BE_LITERAL,
  },
};
use stylex_declarations::convertors::convert_expr_to_str;
use stylex_declarations::lookup::{
  get_import_by_ident, get_var_decl_from, get_var_decl_parts_by_ident,
};
use stylex_enums::{
  import_path_resolution::ImportPathResolution, misc::BinaryExprType,
  value_with_default::ValueWithDefault,
};
use stylex_js::coercions;
use stylex_js::coercions::{global_identifier_to_value, is_global_spelled_as_an_identifier};
use stylex_js::helpers::{
  get_callee_name, get_method_name, is_id_prop, is_invalid_method, is_mutating_object_method,
  is_mutation_expr, is_valid_callee,
};
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
  seen_value::SeenValue,
  state_manager::{StateManager, add_import_expression},
  theme_ref::ThemeRef,
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
};
use stylex_structures::{named_import_source::ImportSources, stylex_env::EnvEntry};
use stylex_utils::string::utf16_length;
use stylex_utils::{hash::stable_hash_unspanned, swc::get_expr_node_kind};

use crate::check_declaration::check_ident_declaration;
use stylex_diagnostics::code_frame::build_code_frame_error_and_panic;

/// Resolves an `EnvEntry` to an `EvaluateResultValue`.
///
/// - `Expr` → `EvaluateResultValue::Expr`
/// - `Function` → returns the parent map so callers resolve the function at the
///   call-expression site
#[inline]
fn resolve_env_entry_to_result(
  entry: &EnvEntry,
  parent_map: &Rc<IndexMap<String, EnvEntry>>,
) -> Option<EvaluateResultValue> {
  match entry {
    EnvEntry::Expr(expr) => Some(EvaluateResultValue::Expr(expr.clone())),
    EnvEntry::Function(_) => Some(EvaluateResultValue::EnvObject(Rc::clone(parent_map))),
  }
}

/// Converts `EvaluateResultValue::Vec` items into an `Expr::Array`.
///
/// Each item may itself be a nested `Vec` (converted to a sub-array) or a plain
/// `Expr`. Only `Array`, `Object`, `Lit` and `Ident` expressions can stand as
/// element values.
///
/// `None` means an item has no array-element form at all — a callback, a theme
/// reference, an evaluator-internal map. That is an array the evaluator does
/// not fold rather than a broken invariant, so no caller aborts on it: an
/// author can write one, and one written in an operand of `&&` must not fail
/// the build.
///
/// No caller answers a shorter array either. A refusal has to travel all the
/// way to a deopt, because a silently dropped element writes a value the
/// source does not describe — which is worse than a declaration that falls to
/// the runtime.
pub fn evaluate_result_vec_to_array_expr(items: &[EvaluateResultValue]) -> Option<Expr> {
  let mut elems = Vec::with_capacity(items.len());

  for entry in items {
    let expr = match entry.as_vec() {
      Some(vec) => evaluate_result_vec_to_array_expr(vec)?,
      None => entry.as_expr().cloned()?,
    };

    if !matches!(
      expr,
      Expr::Array(_) | Expr::Object(_) | Expr::Lit(_) | Expr::Ident(_)
    ) {
      return None;
    }

    elems.push(Some(create_expr_or_spread(expr)));
  }

  Some(create_array_expression(elems))
}

/// The expression form of an evaluated value, if it has one.
///
/// An array has two spellings -- the evaluator's own list, and the literal it
/// was written as -- so a reader that knows only the second finds no form for
/// half of the arrays it is handed. The values with no form at all are the
/// functions: `String(fn)` is its source text, and this evaluator keeps none.
pub(crate) fn evaluate_result_as_expr(value: &EvaluateResultValue) -> Option<Expr> {
  match value {
    EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(items),
    value => value.as_expr().cloned(),
  }
}

/// Whether an argument has a form an arrow's parameter can be bound to.
///
/// Two of them. Most values bind as the expression they write down. A theme
/// reference writes none — it is this compiler's own value — and binds through
/// the same factory a module's own token import binds through, so a parameter
/// holding one answers a member read exactly as the imported name does.
///
/// An argument with neither form binds nothing and leaves the parameter unbound,
/// which is what the language does with an argument nobody passed. This is asked
/// only to tell the two refusals apart afterwards: a body that then failed to
/// fold has an argument to name, where a body that failed with everything bound
/// has only the call.
pub(crate) fn binds_a_parameter(value: &EvaluateResultValue) -> bool {
  match value {
    EvaluateResultValue::ThemeRef(_) => true,
    value => evaluate_result_as_expr(value).is_some(),
  }
}

/// An object of the given keys, each carrying a function.
///
/// Ordered, because the object's first key is the one a refusal names. The
/// placeholder is a function because that is what the entry holds: the reference
/// implementation maps every one of these names to a function, or to an object
/// of them. A position that refuses on the key never reads the value -- but a
/// spread copies the entry onto the style object, where a function is refused
/// for not being a style value and `null` would be an absent value that
/// declares nothing.
fn object_of_functions<'a>(keys: impl Iterator<Item = &'a str>) -> ObjectLit {
  create_object_lit(
    keys
      .map(|key| create_key_value_prop(key, fold_placeholder_function()))
      .collect(),
  )
}

/// The function a folded entry stands for.
///
/// The reference implementation's `identifiers` maps every one of these names
/// to a function, or to an object of them, and what a reader needs to know is
/// that a function is there -- never which one, because no position that reads
/// one calls it. The body is `null` so the arrow carries no reference of its
/// own to anything the evaluator would then have to resolve.
///
/// One function, because the entry `defaultMarker` stands for *is* this and the
/// wrapped entries stand for an object of it: a placeholder that differed
/// between the two would make one of them refuse for a shape the other does not
/// have.
pub(crate) fn fold_placeholder_function() -> Expr {
  create_arrow_expression(create_null_expr())
}

/// The object one entry of a folded function map stands for.
///
/// Two shapes, and the reference implementation's registration is what decides
/// which: a marker map is the `when` surface, registered as the object of the
/// marker functions themselves, so its keys are the marker names. Every other
/// entry is registered as the wrapper `{ fn }`, so its one key is `fn`.
fn fold_entry_to_object(entry: &FunctionConfigType) -> ObjectLit {
  match entry {
    FunctionConfigType::Regular(config) => match &config.fn_ptr {
      FunctionType::DefaultMarker(marker_map) => {
        object_of_functions(marker_map.keys().map(String::as_str))
      },
      _ => object_of_functions(std::iter::once(FUNCTION_CONFIG_FN_KEY)),
    },
    // The `env` option's object, whose keys are the names it was configured
    // with. Its values are the option's own -- a string, a number, a function --
    // and none of them is read here: every position that reads this object reads
    // it through the `EnvObject` result variant, and the only question asked of
    // the *fold's* object form is which keys it has.
    FunctionConfigType::EnvObject(env_map) => {
      object_of_functions(env_map.keys().map(String::as_str))
    },
    // A map nested inside a map, which the API surface does not have today. Its
    // keys are the inner map's, which is the answer that stays true if it ever
    // does.
    FunctionConfigType::Map(nested) => object_of_functions(nested.keys().map(Atom::as_str)),
    FunctionConfigType::IndexMap(styles) => object_of_functions(styles.keys().map(String::as_str)),
  }
}

/// The object form of a folded function map, for the positions that need one.
///
/// A fold has no expression form, so a position that wants one used to refuse
/// with a message about the value's shape. An object built from the fold's keys
/// asks whatever validates that position the question the reference
/// implementation asks of the plain object it folds to: its `identifiers` is a
/// JavaScript object, so every entry carries keys and there is nothing to
/// materialize.
///
/// Built here rather than at any one consumer, because a style value, a
/// namespace, a spread
/// operand and a `defineVars` value all ask it and the answer has to be the
/// same object every time -- the sentence a build stops on is derived from the
/// first key and from what that key carries. Not built where the identifier
/// resolves, because `stylex.when` as a callee reads the map through its own
/// form and has to keep finding it there.
///
/// `FunctionConfigType::Map`, the config-table spelling, needs no arm of its
/// own: `nodes/identifier.rs` is the only reader that answers it as an
/// `EvaluateResultValue`, and it answers the result spelling. An index map has
/// no arm either -- it is `defaultMarker`, which the reference implementation
/// registers as a bare function rather than as an object -- so it refuses in
/// every position rather than materializing, and the sentence it refuses with
/// names this compiler's shape rather than the input.
///
/// An `ObjectLit` and not an `Expr`, because every answer is an object and a
/// caller that had to unwrap one back down would turn an impossible mismatch
/// into a refusal an author could read.
///
/// `None` is every other evaluated value, including the ones with no expression
/// form that are not folds of a function -- a theme reference stands for a
/// `defineVars` group whose keys live in another file, and is refused rather
/// than invented.
pub fn function_fold_to_object(value: &EvaluateResultValue) -> Option<ObjectLit> {
  match value {
    EvaluateResultValue::FunctionConfigMap(func_map) => Some(create_object_lit(
      func_map
        .iter()
        .map(|(key, config)| create_key_value_prop(key, Expr::from(fold_entry_to_object(config))))
        .collect(),
    )),
    EvaluateResultValue::FunctionConfig(config) => Some(fold_entry_to_object(
      &FunctionConfigType::Regular(config.clone()),
    )),
    _ => None,
  }
}

pub fn evaluate_obj_key(
  prop_kv: &KeyValueProp,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> EvaluateResult {
  let key_path = &prop_kv.key;

  let key = match key_path {
    PropName::Ident(ident) => create_string_expr(&ident.sym),
    PropName::Computed(computed) => {
      let computed_result = evaluate(&computed.expr, state, functions);
      if computed_result.confident {
        match computed_result.value {
          Some(EvaluateResultValue::Expr(value)) => value,
          // A key that folded to a value with no expression form — an
          // evaluator-internal map, a callback — is a key this does not read,
          // which is an ordinary refusal rather than a broken invariant.
          _ => {
            return EvaluateResult::refused(
              Some(*computed.expr.clone()),
              Some(ILLEGAL_PROP_VALUE.to_string()),
            );
          },
        }
      } else {
        return EvaluateResult::refused(computed_result.deopt, computed_result.reason);
      }
    },
    PropName::Str(strng) => create_string_expr(&convert_atom_to_string(&strng.value)),
    PropName::Num(num) => create_number_expr(num.value),
    PropName::BigInt(big_int) => create_big_int_expr(big_int.clone()),
  };

  let key_expr = match convert_expr_to_str(&key, state, functions) {
    Some(ref s) => create_string_expr(s),
    None => return EvaluateResult::refused(Some(key), Some("Key is not a string".to_string())),
  };

  EvaluateResult {
    confident: true,
    deopt: None,
    reason: None,
    value: Some(EvaluateResultValue::Expr(key_expr)),
    inline_styles: None,
    fns: None,
  }
}

pub fn evaluate(
  path: &Expr,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Box<EvaluateResult> {
  evaluate_with_functions(path, traversal_state, Rc::new(fns.clone()))
}

fn evaluate_with_functions(
  path: &Expr,
  traversal_state: &mut StateManager,
  fns: Rc<FunctionMap>,
) -> Box<EvaluateResult> {
  let mut state = Box::new(EvaluationState {
    confident: true,
    deopt_path: None,
    deopt_reason: None,
    added_imports: FxHashSet::default(),
    functions: Rc::clone(&fns),
  });

  let mut value = evaluate_cached(path, &mut state, traversal_state, &fns);

  if !state.confident {
    value = None;
  }

  Box::new(EvaluateResult {
    confident: state.confident,
    value,
    deopt: state.deopt_path,
    reason: state.deopt_reason,
    inline_styles: None,
    fns: None,
  })
}

fn _evaluate(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  if !state.confident {
    return None;
  }

  let normalized_path = normalize_expr(path);

  if is_mutation_expr(normalized_path) {
    return deopt(path, state, NON_CONSTANT);
  }

  let result: Option<EvaluateResultValue> = match normalized_path {
    Expr::Arrow(arrow) => nodes::arrow_function::evaluate(arrow, state),
    Expr::Ident(ident) => nodes::identifier::evaluate(ident, state),
    Expr::TsSatisfies(ts_satisfaies) => {
      nodes::typescript_expression::evaluate(&ts_satisfaies.expr, state, traversal_state, fns)
    },
    Expr::TsConstAssertion(ts_const) => {
      nodes::typescript_expression::evaluate(&ts_const.expr, state, traversal_state, fns)
    },
    Expr::TsAs(ts_as) => {
      nodes::typescript_expression::evaluate(&ts_as.expr, state, traversal_state, fns)
    },
    Expr::TsNonNull(ts_non_null) => {
      nodes::typescript_expression::evaluate(&ts_non_null.expr, state, traversal_state, fns)
    },
    Expr::TsTypeAssertion(ts_type) => {
      nodes::typescript_expression::evaluate(&ts_type.expr, state, traversal_state, fns)
    },
    Expr::TsInstantiation(ts_instantiation) => {
      nodes::typescript_expression::evaluate(&ts_instantiation.expr, state, traversal_state, fns)
    },
    Expr::Seq(sec) => nodes::sequence_expression::evaluate(sec, state, traversal_state, fns),
    // Only string, numeric, boolean and null literals evaluate to a value.
    // A BigInt is none of those — it has no CSS spelling and no safe lossy
    // conversion to one — so it deopts rather than evaluating.
    Expr::Lit(Lit::BigInt(_)) => {
      return deopt(
        normalized_path,
        state,
        &unsupported_expression("BigIntLiteral"),
      );
    },
    // Nor is a regular expression, and the reference implementation refuses one
    // here rather than anywhere later: it folds no `RegExpLiteral` in any
    // position, so `{ color: /a/ }`, `[/a/]`, a binding holding one and
    // `{ .../a/ }` all read the same sentence. Folding it to itself instead left
    // the refusal to whichever downstream reader tripped over the value, and
    // that reader had no name for it — the diagnostic came out as `1`.
    Expr::Lit(Lit::Regex(_)) => {
      return deopt(
        normalized_path,
        state,
        &unsupported_expression("RegExpLiteral"),
      );
    },
    Expr::Lit(lit_path) => nodes::literal::evaluate(lit_path),
    Expr::Tpl(tpl) => nodes::template_literal::evaluate_quasis(
      normalized_path,
      &tpl.exprs,
      &tpl.quasis,
      false,
      state,
      traversal_state,
      fns,
    ),
    Expr::TaggedTpl(_tagged_tpl) => {
      deopt_unsupported!(
        deopt,
        normalized_path,
        state,
        &unsupported_expression("TaggedTemplateExpression")
      )
      // TODO: Uncomment this for implementation of TaggedTpl
      // nodes::template_literal::evaluate_quasis(
      //   &Expr::TaggedTpl(_tagged_tpl.clone()),
      //   &_tagged_tpl.tpl.quasis,
      //   false,
      //   state,
      // )
    },
    Expr::Cond(cond) => nodes::conditional_expression::evaluate(cond, state, traversal_state, fns),
    Expr::Paren(_) => stylex_panic_with_context!(
      wrap_in_paren_ref,
      build_code_frame_error_and_panic,
      path,
      traversal_state,
      "Parenthesized expressions should be unwrapped before evaluation."
    ),
    Expr::Member(member) => nodes::member_expression::evaluate(member, state, traversal_state, fns),
    Expr::Unary(unary) => nodes::unary_expression::evaluate(unary, state, traversal_state, fns),
    Expr::Array(arr_path) => nodes::array_expression::evaluate(arr_path, state, traversal_state),
    Expr::Object(obj_path) => {
      nodes::object_expression::evaluate(obj_path, state, traversal_state, fns)
    },
    Expr::Bin(bin) => nodes::binary_expression::evaluate(bin, state, traversal_state, fns),
    Expr::Call(call) => nodes::call_expression::evaluate(call, state, traversal_state, fns),
    Expr::Await(await_expr) => {
      nodes::await_expression::evaluate(await_expr, state, traversal_state, fns)
    },
    Expr::OptChain(opt_chain) => {
      nodes::optional_chain::evaluate(opt_chain, state, traversal_state, fns)
    },
    _ => {
      warn!(
        "Unsupported type of expression: {}. If its not enough, please run in debug mode to see more details",
        get_expr_node_kind(normalized_path)
      );

      debug!("Unsupported type of expression: {:?}", normalized_path);

      return deopt(
        normalized_path,
        state,
        &unsupported_expression(get_expr_node_kind(normalized_path)),
      );
    },
  };

  if result.is_none() && normalized_path.is_ident() {
    let Some(ident) = normalized_path.as_ident() else {
      stylex_panic_with_context!(
        wrap_in_paren_ref,
        build_code_frame_error_and_panic,
        path,
        traversal_state,
        "Could not resolve the identifier. Ensure it is defined and in scope."
      )
    };

    return binding::resolve_reference(ident, path, normalized_path, state, traversal_state, fns);
  }

  if result.is_none() {
    return deopt(
      normalized_path,
      state,
      &unsupported_expression(get_expr_node_kind(normalized_path)),
    );
  }

  result
}

#[cfg(test)]
#[path = "tests/source_evaluation.rs"]
pub(crate) mod source_evaluation;

#[cfg(test)]
#[path = "tests/array_hole_tests.rs"]
mod array_hole_tests;

#[cfg(test)]
#[path = "tests/member_length_tests.rs"]
mod member_length_tests;

#[cfg(test)]
#[path = "tests/array_index_tests.rs"]
mod array_index_tests;

#[cfg(test)]
#[path = "tests/unsupported_shape_tests.rs"]
mod unsupported_shape_tests;

#[cfg(test)]
#[path = "tests/callee_shape_dispatch_tests.rs"]
mod callee_shape_dispatch_tests;

#[cfg(test)]
#[path = "tests/engine_fold_tests.rs"]
mod engine_fold_tests;

#[cfg(test)]
#[path = "tests/function_fold_object_tests.rs"]
mod function_fold_object_tests;

#[cfg(test)]
#[path = "tests/applied_global_tests.rs"]
mod applied_global_tests;

#[cfg(test)]
#[path = "tests/parameter_binding_tests.rs"]
mod parameter_binding_tests;

#[cfg(test)]
#[path = "tests/fall_through_tests.rs"]
mod fall_through_tests;

#[cfg(test)]
#[path = "tests/short_circuited_walk_tests.rs"]
mod short_circuited_walk_tests;

#[cfg(test)]
#[path = "tests/thread_isolation_tests.rs"]
mod thread_isolation_tests;
