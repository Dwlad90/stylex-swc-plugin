use std::{borrow::Borrow, rc::Rc, sync::Arc};

mod cache;
mod deopt;
mod helpers;
mod nodes;

pub(crate) use cache::evaluate_cached;
pub(crate) use deopt::deopt;
use helpers::*;

// Import error handling macros from shared utilities
use crate::{expr_to_str_or_deopt, stylex_panic_with_context};
use stylex_constants::constants::{
  api_names::STYLEX_ENV,
  common::{MUTATING_ARRAY_METHODS, MUTATING_OBJECT_METHODS},
};

use indexmap::IndexMap;
use log::{debug, warn};
use rustc_hash::{FxHashMap, FxHashSet};
use stylex_macros::{stylex_panic, stylex_unimplemented, stylex_unreachable, unwrap_or_panic};
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{
      ArrayLit, AssignTarget, BlockStmtOrExpr, CallExpr, Callee, ComputedPropName, Expr,
      ExprOrSpread, Ident, ImportSpecifier, KeyValueProp, Lit, MemberProp, ModuleExportName,
      Number, ObjectLit, OptChainBase, Pat, Prop, PropName, PropOrSpread, SimpleAssignTarget,
      TplElement, UnaryOp, VarDeclarator,
    },
    utils::{ExprExt, drop_span, ident::IdentLike},
  },
};

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{
    evaluate_result::EvaluateResult,
    functions::{CallbackType, FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    seen_value::SeenValue,
    state::EvaluationState,
    state_manager::{SeenValueWithVarDeclCount, StateManager, add_import_expression},
    theme_ref::ThemeRef,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  utils::{
    ast::convertors::{
      binary_expr_to_num, binary_expr_to_string, convert_atom_to_str_ref, convert_atom_to_string,
      convert_expr_to_bool, convert_expr_to_str, convert_key_value_to_str, convert_lit_to_string,
      create_big_int_expr, create_bool_expr, create_number_expr, create_string_expr,
      expand_shorthand_prop, expr_to_num, extract_tpl_cooked_value,
    },
    common::{
      deep_merge_props, get_import_by_ident, get_key_values_from_object, get_var_decl_by_ident,
      get_var_decl_from, normalize_expr, reduce_ident_count, reduce_member_expression_count,
      remove_duplicates,
    },
    js::native_functions::{evaluate_filter, evaluate_join, evaluate_map},
  },
};
use stylex_ast::ast::factories::{
  create_array_expression, create_expr_or_spread, create_ident_key_value_prop,
  create_object_expression, create_object_lit, create_string_lit,
};
use stylex_constants::constants::{
  common::{INVALID_METHODS, VALID_CALLEES},
  evaluation_errors::{
    IMPORT_PATH_RESOLUTION_ERROR, NON_CONSTANT, OBJECT_METHOD, PATH_WITHOUT_NODE,
    UNEXPECTED_MEMBER_LOOKUP, unsupported_expression, unsupported_operator,
  },
  messages::{
    ARGUMENT_NOT_EXPRESSION, BUILT_IN_FUNCTION, EXPECTED_CSS_VAR, EXPRESSION_IS_NOT_A_STRING,
    ILLEGAL_PROP_ARRAY_VALUE, INVALID_UTF8, KEY_VALUE_EXPECTED, MEMBER_NOT_RESOLVED,
    MEMBER_OBJ_NOT_IDENT, OBJECT_KEY_MUST_BE_IDENT, PROPERTY_NOT_FOUND, SPREAD_MUST_BE_OBJECT,
    SPREAD_NOT_SUPPORTED, THEME_IMPORT_KEY_AS_OBJECT_KEY, VALUE_MUST_BE_LITERAL,
  },
};
use stylex_enums::{
  core::TransformationCycle,
  import_path_resolution::ImportPathResolution,
  js::{ArrayJS, MathJS, ObjectJS, StringJS},
  misc::{BinaryExprType, VarDeclAction},
  value_with_default::ValueWithDefault,
};
use stylex_structures::{named_import_source::ImportSources, stylex_env::EnvEntry};
use stylex_utils::{
  collection::{
    get_hash_map_difference, get_hash_map_value_difference, sort_numbers_factory,
    sum_hash_map_values,
  },
  hash::stable_hash_unspanned,
  string::char_code_at,
  swc::get_default_expr_ctx,
};

use super::check_declaration::{DeclarationType, check_ident_declaration};

/// Resolves an `EnvEntry` to an `EvaluateResultValue`.
///
/// - `Expr` → `EvaluateResultValue::Expr`
/// - `Function` → returns the parent map so callers resolve the function at the
///   call-expression site
#[inline]
fn resolve_env_entry_to_result(
  entry: &EnvEntry,
  parent_map: &IndexMap<String, EnvEntry>,
) -> Option<EvaluateResultValue> {
  match entry {
    EnvEntry::Expr(expr) => Some(EvaluateResultValue::Expr(expr.clone())),
    EnvEntry::Function(_) => Some(EvaluateResultValue::EnvObject(parent_map.clone())),
  }
}

/// Converts `EvaluateResultValue::Vec` items into an `Expr::Array`.
///
/// Each item may itself be a nested `Vec` (converted to a sub-array) or a plain
/// `Expr`. Only `Array`, `Object`, `Lit`, and `Ident` expressions are allowed
/// as element values; any other variant panics with
/// [`ILLEGAL_PROP_ARRAY_VALUE`].
fn evaluate_result_vec_to_array_expr(items: &[EvaluateResultValue]) -> Expr {
  let elems = items
    .iter()
    .map(|entry| {
      let expr = entry
        .as_vec()
        .map(|vec| evaluate_result_vec_to_array_expr(vec))
        .or_else(|| entry.as_expr().cloned())
        .unwrap_or_else(|| {
          #[cfg_attr(coverage_nightly, coverage(off))]
          {
            stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE)
          }
        });

      let expr = match expr {
        Expr::Array(array) => Expr::Array(array),
        Expr::Object(obj) => Expr::Object(obj),
        Expr::Lit(lit) => Expr::Lit(lit),
        Expr::Ident(ident) => Expr::Ident(ident),
        #[cfg_attr(coverage_nightly, coverage(off))]
        _ => stylex_panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
      };

      Some(create_expr_or_spread(expr))
    })
    .collect();

  create_array_expression(elems)
}

/// Helper function to evaluate unary numeric operations (Plus, Minus, Tilde).
/// This reduces code duplication for operations that convert an expression to a
/// number, apply a transformation, and return the result as an expression.
///
/// # Arguments
/// * `arg` - The expression argument to the unary operator
/// * `state` - The evaluation state
/// * `traversal_state` - The state manager for traversal context
/// * `fns` - The function map for evaluating function calls
/// * `transform` - A function to transform the numeric value
///
/// # Example
/// ```ignore
/// UnaryOp::Plus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| v),
/// UnaryOp::Minus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| -v),
/// ```
#[inline]
fn evaluate_unary_numeric(
  arg: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  transform: impl FnOnce(f64) -> f64,
) -> Option<EvaluateResultValue> {
  let value = unwrap_or_panic!(expr_to_num(arg, state, traversal_state, fns));
  Some(EvaluateResultValue::Expr(create_number_expr(transform(
    value,
  ))))
}

pub(crate) fn evaluate_obj_key(
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
          #[cfg_attr(coverage_nightly, coverage(off))]
          _ => stylex_panic!("Expected an expression value from the evaluation result."),
        }
      } else {
        return EvaluateResult {
          confident: false,
          deopt: computed_result.deopt,
          reason: computed_result.reason,
          value: None,
          inline_styles: None,
          fns: None,
        };
      }
    },
    PropName::Str(strng) => create_string_expr(&convert_atom_to_string(&strng.value)),
    PropName::Num(num) => create_number_expr(num.value),
    PropName::BigInt(big_int) => create_big_int_expr(big_int.clone()),
  };

  let key_expr = match convert_expr_to_str(&key, state, functions) {
    Some(ref s) => create_string_expr(s),
    None => {
      return EvaluateResult {
        confident: false,
        deopt: Some(key),
        reason: Some("Key is not a string".to_string()),
        value: None,
        inline_styles: None,
        fns: None,
      };
    },
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
  path: &mut Expr,
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
    Expr::Lit(lit_path) => nodes::literal::evaluate(lit_path),
    Expr::Tpl(tpl) => nodes::template_literal::evaluate_quasis(
      &tpl.exprs,
      &tpl.quasis,
      false,
      state,
      traversal_state,
      fns,
    ),
    Expr::TaggedTpl(_tagged_tpl) => {
      stylex_panic_with_context!(
        path,
        traversal_state,
        "Tagged template literals are not supported in static evaluation."
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
        "Unsupported type of expression: {:?}. If its not enough, please run in debug mode to see more details",
        normalized_path.get_type(get_default_expr_ctx())
      );

      debug!("Unsupported type of expression: {:?}", normalized_path);

      return deopt(
        normalized_path,
        state,
        &unsupported_expression(&format!(
          "{:?}",
          normalized_path.get_type(get_default_expr_ctx())
        )),
      );
    },
  };

  if result.is_none() && normalized_path.is_ident() {
    let Some(ident) = normalized_path.as_ident() else {
      stylex_panic_with_context!(
        path,
        traversal_state,
        "Could not resolve the identifier. Ensure it is defined and in scope."
      )
    };

    let binding = get_var_decl_by_ident(
      ident,
      traversal_state,
      &state.functions,
      if traversal_state.cycle == TransformationCycle::TransformExit {
        // NOTE: We don't want to reduce the binding count of stylex.props arguments
        VarDeclAction::None
      } else {
        VarDeclAction::Reduce
      },
    );

    if let Some(init) = binding.and_then(|mut var_decl| var_decl.init.take()) {
      return evaluate_cached(&init, state, traversal_state, fns);
    }

    let name = ident.sym.to_string();

    if name == "undefined" || name == "Infinity" || name == "NaN" {
      return Some(EvaluateResultValue::Expr(Expr::from(ident.clone())));
    }

    if let Some(import_path) = get_import_by_ident(ident, traversal_state)
      && !state.functions.disable_imports
    {
      let (local_name, imported) = import_path
        .specifiers
        .iter()
        .find_map(|import| {
          let (local_name, imported) = match import {
            ImportSpecifier::Default(default) => (
              default.local.clone(),
              Some(ModuleExportName::Ident(default.local.clone())),
            ),
            ImportSpecifier::Named(named) => (named.local.clone(), named.imported.clone()),
            ImportSpecifier::Namespace(namespace) => (
              namespace.local.clone(),
              Some(ModuleExportName::Ident(namespace.local.clone())),
            ),
          };

          if ident.sym == local_name.sym {
            Some((local_name, imported))
          } else {
            None
          }
        })
        .unwrap_or_else(|| {
          #[cfg_attr(coverage_nightly, coverage(off))]
          {
            stylex_panic!("Could not resolve the import specifier. Ensure the import is correct.")
          }
        });

      let imported = imported.unwrap_or_else(|| ModuleExportName::Ident(local_name.clone()));

      let abs_path = traversal_state.import_path_resolver(
        convert_atom_to_str_ref(&import_path.src.value),
        &mut FxHashMap::default(),
      );

      let imported_name = match imported {
        ModuleExportName::Ident(ident) => ident.sym.to_string(),
        ModuleExportName::Str(strng) => convert_atom_to_string(&strng.value),
      };

      let return_value = match abs_path {
        ImportPathResolution::Resolved { path: value } => {
          evaluate_theme_ref(&value, imported_name, traversal_state)
        },
        ImportPathResolution::Unresolved => {
          return deopt(path, state, IMPORT_PATH_RESOLUTION_ERROR);
        },
      };

      if state.confident {
        let import_path_src = convert_atom_to_string(&import_path.src.value);

        if !state.added_imports.contains(&import_path_src)
          && traversal_state.get_treeshake_compensation()
        {
          let prepend_import_module_item = add_import_expression(&import_path_src);

          if !traversal_state
            .prepend_import_module_items
            .contains(&prepend_import_module_item)
          {
            traversal_state
              .prepend_import_module_items
              .push(prepend_import_module_item);
          }

          state.added_imports.insert(import_path_src);
        }

        return Some(EvaluateResultValue::ThemeRef(return_value));
      }
    }

    return check_ident_declaration(
      ident,
      &[
        (
          DeclarationType::Class,
          traversal_state.class_name_declarations(),
        ),
        (
          DeclarationType::Function,
          traversal_state.function_name_declarations(),
        ),
      ],
      state,
      normalized_path,
    );
  }

  if result.is_none() {
    return deopt(
      normalized_path,
      state,
      &unsupported_expression(&format!(
        "{:?}",
        normalized_path.get_type(get_default_expr_ctx())
      )),
    );
  }

  result
}
