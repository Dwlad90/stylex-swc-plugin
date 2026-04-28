use rustc_hash::FxHashSet;
use stylex_macros::stylex_panic;
use stylex_structures::top_level_expression::TopLevelExpression;
use swc_core::{
  atoms::Atom,
  ecma::ast::{ArrowExpr, CallExpr, Expr, KeyValueProp, Lit, Pat, VarDeclarator},
};

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::state_manager::{ImportKind, StateManager},
  utils::{
    ast::{convertors::create_string_expr, helpers::is_variable_named_exported},
    common::get_import_from,
    log::build_code_frame_error::build_code_frame_error_and_panic,
  },
};
use stylex_ast::ast::factories::{create_expr_or_spread, create_key_value_prop_ident};
use stylex_constants::constants::{
  api_names::{
    STYLEX_ATTRS, STYLEX_CREATE, STYLEX_CREATE_THEME, STYLEX_DEFAULT_MARKER, STYLEX_DEFINE_CONSTS,
    STYLEX_DEFINE_MARKER, STYLEX_DEFINE_VARS, STYLEX_KEYFRAMES, STYLEX_POSITION_TRY, STYLEX_PROPS,
    STYLEX_VIEW_TRANSITION_CLASS,
  },
  common::VAR_GROUP_HASH_KEY,
  messages::{
    DUPLICATE_CONDITIONAL, EXPECTED_CSS_VAR, ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE,
    INVALID_PSEUDO_OR_AT_RULE, MEMBER_OBJ_NOT_IDENT, NO_OBJECT_SPREADS, NON_OBJECT_KEYFRAME,
    NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE, ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS,
    ONLY_OVERRIDE_DEFINE_VARS, illegal_argument_length, non_export_named_declaration,
    non_static_value, non_style_object, unbound_call_value,
  },
};

use super::{
  ast::convertors::{convert_key_value_to_str, convert_lit_to_string},
  common::get_key_values_from_object,
};

pub(crate) fn validate_stylex_create(call: &CallExpr, state: &mut StateManager) {
  if !is_create_call(call, state) {
    return;
  }

  if state.find_call_declaration(call).is_none()
    && state
      .find_top_level_expr(
        call,
        |tpe: &TopLevelExpression| matches!(tpe.1, Expr::Array(_)),
        None,
      )
      .is_none()
  {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &unbound_call_value(STYLEX_CREATE),
      state,
    );
  }

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &illegal_argument_length(STYLEX_CREATE, 1),
      state,
    );
  }

  let first_arg = &call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &first_arg.expr,
      &non_style_object(STYLEX_CREATE),
      state,
    );
  }

  let has_spread = if let Expr::Object(obj) = first_arg.expr.as_ref() {
    obj
      .props
      .iter()
      .any(|prop| matches!(prop, swc_core::ecma::ast::PropOrSpread::Spread(_)))
  } else {
    false
  };

  if has_spread {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &first_arg.expr,
      NO_OBJECT_SPREADS,
      state,
    );
  }
}

pub(crate) fn validate_stylex_keyframes_indent(var_decl: &VarDeclarator, state: &mut StateManager) {
  if !is_keyframes_call(var_decl, state) {
    return;
  }

  let init_expr = match &var_decl.init {
    Some(init) => init.clone(),
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", non_static_value(STYLEX_KEYFRAMES)),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &non_static_value(STYLEX_KEYFRAMES),
      state,
    );
  });

  match state.find_top_level_expr(init_call, |_| false, None) {
    Some(_) => {},
    None => build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &unbound_call_value(STYLEX_KEYFRAMES),
      state,
    ),
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &illegal_argument_length(STYLEX_KEYFRAMES, 1),
      state,
    );
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      &non_style_object(STYLEX_KEYFRAMES),
      state,
    );
  }
}

pub(crate) fn validate_stylex_position_try_indent(
  var_decl: &VarDeclarator,
  state: &mut StateManager,
) {
  if !is_position_try_call(var_decl, state) {
    return;
  }

  let init_expr = match &var_decl.init {
    Some(init) => init.clone(),
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", non_static_value(STYLEX_POSITION_TRY)),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &non_static_value(STYLEX_POSITION_TRY),
      state,
    );
  });

  match state.find_top_level_expr(init_call, |_| false, None) {
    Some(_) => {},
    None => build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &unbound_call_value(STYLEX_POSITION_TRY),
      state,
    ),
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &illegal_argument_length(STYLEX_POSITION_TRY, 1),
      state,
    );
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      &non_style_object(STYLEX_POSITION_TRY),
      state,
    );
  }
}

pub(crate) fn validate_stylex_default_marker_indent(call: &CallExpr, state: &mut StateManager) {
  if !is_default_marker_call(call, state) {
    return;
  }

  let call_expr = Expr::from(call.clone());

  if !call.args.is_empty() {
    build_code_frame_error_and_panic(
      &call_expr,
      &Box::new(call_expr.clone()),
      &illegal_argument_length(STYLEX_DEFAULT_MARKER, 1),
      state,
    );
  }
}

pub(crate) fn validate_stylex_view_transition_class_indent(
  var_decl: &VarDeclarator,
  state: &mut StateManager,
) {
  if !is_view_transition_class_call(var_decl, state) {
    return;
  }

  let init_expr = match &var_decl.init {
    Some(init) => init.clone(),
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", non_static_value(STYLEX_VIEW_TRANSITION_CLASS)),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &non_static_value(STYLEX_VIEW_TRANSITION_CLASS),
      state,
    );
  });

  match state.find_top_level_expr(init_call, |_| false, None) {
    Some(_) => {},
    None => build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &unbound_call_value(STYLEX_VIEW_TRANSITION_CLASS),
      state,
    ),
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &illegal_argument_length(STYLEX_VIEW_TRANSITION_CLASS, 1),
      state,
    );
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      &non_style_object(STYLEX_VIEW_TRANSITION_CLASS),
      state,
    );
  }
}

pub(crate) fn validate_stylex_create_theme_indent(
  var_decl: &Option<VarDeclarator>,
  call: &CallExpr,
  state: &mut StateManager,
) {
  if !is_create_theme_call(call, state) {
    return;
  }

  let var_decl = var_decl.as_ref().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &unbound_call_value(STYLEX_CREATE_THEME),
      state,
    );
  });

  let init_expr = var_decl.init.as_ref().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &unbound_call_value(STYLEX_CREATE_THEME),
      state,
    );
  });

  let init = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &non_static_value(STYLEX_CREATE_THEME),
      state,
    );
  });

  match state.find_top_level_expr(call, |_| false, None) {
    Some(_) => {},
    None => build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &unbound_call_value(STYLEX_CREATE_THEME),
      state,
    ),
  };

  if init.args.len() != 2 {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &illegal_argument_length(STYLEX_CREATE_THEME, 1),
      state,
    );
  }

  let second_arg = &init.args[1];

  let is_valid_second_arg = match second_arg.expr.as_ref() {
    Expr::Ident(ident) => get_import_from(state, ident).is_none(),
    Expr::Object(_) => true,
    _ => false,
  };

  if !is_valid_second_arg {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE,
      state,
    );
  }
}

pub(crate) fn find_and_validate_stylex_define_vars(
  call: &CallExpr,
  state: &mut StateManager,
) -> Option<TopLevelExpression> {
  if !is_define_vars_call(call, state) {
    return None;
  }

  let call_expr = Expr::from(call.clone());

  let stylex_create_theme_top_level_expr = match state.find_top_level_expr(call, |_| false, None) {
    Some(stylex_create_theme_top_level_expr) => stylex_create_theme_top_level_expr,
    None => build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(2)
        .cloned()
        .unwrap_or_else(|| create_expr_or_spread(call_expr.clone()))
        .expr,
      &unbound_call_value(STYLEX_DEFINE_VARS),
      state,
    ),
  };

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(1)
        .cloned()
        .unwrap_or_else(|| create_expr_or_spread(call_expr.clone()))
        .expr,
      &illegal_argument_length(STYLEX_DEFINE_VARS, 1),
      state,
    );
  }

  if !is_variable_named_exported(stylex_create_theme_top_level_expr, state) {
    build_code_frame_error_and_panic(
      &call_expr,
      &call_expr,
      &non_export_named_declaration(STYLEX_DEFINE_VARS),
      state,
    );
  }

  Some(stylex_create_theme_top_level_expr.clone())
}

pub(crate) fn validate_stylex_define_marker_indent(call: &CallExpr, state: &mut StateManager) {
  if !is_define_marker_call(call, state) {
    return;
  }

  let call_expr = Expr::from(call.clone());

  if !call.args.is_empty() {
    build_code_frame_error_and_panic(
      &call_expr,
      &Box::new(call_expr.clone()),
      &illegal_argument_length(STYLEX_DEFINE_MARKER, 0),
      state,
    );
  }

  let define_marker_top_level_expr = match state.find_top_level_expr(call, |_| false, None) {
    Some(define_marker_top_level_expr) => define_marker_top_level_expr,
    None => build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(2)
        .cloned()
        .unwrap_or_else(|| create_expr_or_spread(call_expr.clone()))
        .expr,
      &unbound_call_value(STYLEX_DEFINE_MARKER),
      state,
    ),
  };

  if !is_variable_named_exported(define_marker_top_level_expr, state) {
    build_code_frame_error_and_panic(
      &call_expr,
      &call_expr,
      &non_export_named_declaration(STYLEX_DEFINE_MARKER),
      state,
    );
  }
}

pub(crate) fn find_and_validate_stylex_define_consts(
  call: &CallExpr,
  state: &mut StateManager,
) -> Option<TopLevelExpression> {
  if !is_define_consts_call(call, state) {
    return None;
  }

  let call_expr = Expr::from(call.clone());

  let define_consts_top_level_expr = match state.find_top_level_expr(call, |_| false, None) {
    Some(define_consts_top_level_expr) => define_consts_top_level_expr,
    None => build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(2)
        .cloned()
        .unwrap_or_else(|| create_expr_or_spread(call_expr.clone()))
        .expr,
      &unbound_call_value(STYLEX_DEFINE_CONSTS),
      state,
    ),
  };

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(1)
        .cloned()
        .unwrap_or_else(|| create_expr_or_spread(call_expr.clone()))
        .expr,
      &illegal_argument_length(STYLEX_DEFINE_CONSTS, 1),
      state,
    );
  }

  if !is_variable_named_exported(define_consts_top_level_expr, state) {
    build_code_frame_error_and_panic(
      &call_expr,
      &call_expr,
      &non_export_named_declaration(STYLEX_DEFINE_CONSTS),
      state,
    );
  }

  Some(define_consts_top_level_expr.clone())
}

pub(crate) fn is_create_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_CREATE,
      state.get_stylex_api_import(ImportKind::Create),
    ),
    call,
    state,
  )
}

pub(crate) fn is_props_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (STYLEX_PROPS, state.get_stylex_api_import(ImportKind::Props)),
    call,
    state,
  )
}

pub(crate) fn is_attrs_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (STYLEX_ATTRS, state.get_stylex_api_import(ImportKind::Attrs)),
    call,
    state,
  )
}

pub(crate) fn is_keyframes_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  match init {
    Some(call) => is_target_call(
      (
        STYLEX_KEYFRAMES,
        state.get_stylex_api_import(ImportKind::Keyframes),
      ),
      &call,
      state,
    ),
    _ => false,
  }
}

pub(crate) fn is_position_try_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  match init {
    Some(call) => is_target_call(
      (
        STYLEX_POSITION_TRY,
        state.get_stylex_api_import(ImportKind::PositionTry),
      ),
      &call,
      state,
    ),
    _ => false,
  }
}

pub(crate) fn is_default_marker_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_DEFAULT_MARKER,
      state.get_stylex_api_import(ImportKind::DefaultMarker),
    ),
    call,
    state,
  )
}

pub(crate) fn is_view_transition_class_call(
  var_decl: &VarDeclarator,
  state: &StateManager,
) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  match init {
    Some(call) => is_target_call(
      (
        STYLEX_VIEW_TRANSITION_CLASS,
        state.get_stylex_api_import(ImportKind::ViewTransitionClass),
      ),
      &call,
      state,
    ),
    _ => false,
  }
}

pub(crate) fn is_create_theme_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_CREATE_THEME,
      state.get_stylex_api_import(ImportKind::CreateTheme),
    ),
    call,
    state,
  )
}

pub(crate) fn is_define_vars_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_DEFINE_VARS,
      state.get_stylex_api_import(ImportKind::DefineVars),
    ),
    call,
    state,
  )
}

pub(crate) fn is_define_consts_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_DEFINE_CONSTS,
      state.get_stylex_api_import(ImportKind::DefineConsts),
    ),
    call,
    state,
  )
}

pub(crate) fn is_define_marker_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    (
      STYLEX_DEFINE_MARKER,
      state.get_stylex_api_import(ImportKind::DefineMarker),
    ),
    call,
    state,
  )
}
pub(crate) fn is_target_call(
  (call_name, imports_map): (&str, Option<&FxHashSet<Atom>>),
  call: &CallExpr,
  state: &StateManager,
) -> bool {
  let is_create_ident = call
    .callee
    .as_expr()
    .and_then(|arg| arg.as_ident())
    .is_some_and(|ident| imports_map.is_some_and(|set| set.contains(&ident.sym)));

  let is_create_member = call
    .callee
    .as_expr()
    .and_then(|expr| expr.as_member())
    .is_some_and(|member| {
      member.obj.is_ident()
        && member.prop.as_ident().is_some_and(|ident| {
          ident.sym == call_name
            && state.stylex_import_stringified().contains(
              &match member.obj.as_ident() {
                Some(ident) => ident,
                #[cfg_attr(coverage_nightly, coverage(off))]
                None => stylex_panic!("{}", MEMBER_OBJ_NOT_IDENT),
              }
              .sym
              .to_string(),
            )
        })
    });

  is_create_ident || is_create_member
}
pub(crate) fn validate_namespace(
  namespaces: &[KeyValueProp],
  conditions: &[String],
  state: &mut StateManager,
) {
  for namespace in namespaces {
    match namespace.value.as_ref() {
      Expr::Lit(lit) => {
        if !matches!(
          lit,
          Lit::Str(_) | Lit::Null(_) | Lit::Num(_) | Lit::BigInt(_)
        ) {
          build_code_frame_error_and_panic(
            &Expr::Lit(lit.clone()),
            &Expr::Lit(lit.clone()),
            ILLEGAL_PROP_VALUE,
            state,
          );
        }
      },
      Expr::Array(array) => {
        for elem in array.elems.iter().flatten() {
          if elem.spread.is_some() {
            build_code_frame_error_and_panic(
              &Expr::Array(array.clone()),
              &Expr::Array(array.clone()),
              "Spread operator not implemented",
              state,
            );
          }

          if !matches!(elem.expr.as_ref(), Expr::Lit(_)) {
            build_code_frame_error_and_panic(
              &Expr::Array(array.clone()),
              &Expr::Array(array.clone()),
              ILLEGAL_PROP_ARRAY_VALUE,
              state,
            );
          }
        }
      },
      Expr::Object(object) => {
        let key = convert_key_value_to_str(namespace);

        if key.starts_with('@') || key.starts_with(':') || key.starts_with('[') {
          if conditions.contains(&key) {
            build_code_frame_error_and_panic(
              &Expr::Object(object.clone()),
              &Expr::Object(object.clone()),
              DUPLICATE_CONDITIONAL,
              state,
            );
          }

          let nested_key_values = get_key_values_from_object(object);

          let mut extended_conditions = conditions.to_vec();
          extended_conditions.push(key);

          validate_namespace(&nested_key_values, &extended_conditions, state);
        } else {
          let conditional_styles_key_values = get_key_values_from_object(object);

          for conditional_style in &conditional_styles_key_values {
            validate_conditional_styles(conditional_style, &[], state);
          }
        }
      },
      _ => {},
    }
  }
}

pub(crate) fn validate_dynamic_style_params(
  path: &ArrowExpr,
  params: &[Pat],
  state: &mut StateManager,
) {
  if params.iter().any(|param| !param.is_ident()) {
    let path_expr = Expr::Arrow(path.clone());

    build_code_frame_error_and_panic(
      &path_expr,
      &path_expr,
      ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS,
      state,
    )
  }
}

pub(crate) fn validate_conditional_styles(
  inner_key_value: &KeyValueProp,
  conditions: &[String],
  state: &mut StateManager,
) {
  let inner_key = convert_key_value_to_str(inner_key_value);
  let inner_value = inner_key_value.value.clone();

  if !(inner_key.starts_with(':')
      || inner_key.starts_with('@')
      || inner_key.starts_with('[')
      // This is a placeholder for `defineConsts` values that are later inlined
      || inner_key.starts_with("var(--")
      || inner_key == "default")
  {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", INVALID_PSEUDO_OR_AT_RULE);
    }
  }

  if conditions.contains(&inner_key) {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", DUPLICATE_CONDITIONAL);
    }
  }

  match inner_value.as_ref() {
    Expr::Lit(_) => {},
    Expr::Array(array) => {
      for elem in array.elems.iter().flatten() {
        match elem.expr.as_ref() {
          Expr::Lit(_) => {},
          _ => build_code_frame_error_and_panic(
            &Expr::Array(array.clone()),
            &Expr::Array(array.clone()),
            ILLEGAL_PROP_VALUE,
            state,
          ),
        }
      }
    },
    Expr::Object(object) => {
      let nested_key_values = get_key_values_from_object(object);

      let mut extended_conditions = conditions.to_vec();
      extended_conditions.push(inner_key);

      for nested_key_value in nested_key_values.iter() {
        validate_conditional_styles(nested_key_value, &extended_conditions, state);
      }
    },
    Expr::Ident(_) => {},
    _ => build_code_frame_error_and_panic(&inner_value, &inner_value, ILLEGAL_PROP_VALUE, state),
  }
}

pub(crate) fn assert_valid_keyframes(obj: &EvaluateResultValue, state: &mut StateManager) {
  match obj {
    EvaluateResultValue::Expr(expr) => match expr {
      Expr::Object(object) => {
        let key_values = get_key_values_from_object(object);

        for key_value in key_values.iter() {
          match key_value.value.as_ref() {
            Expr::Object(_) => {},
            _ => {
              build_code_frame_error_and_panic(expr, expr, NON_OBJECT_KEYFRAME, state);
            },
          }
        }
      },
      _ => {
        build_code_frame_error_and_panic(expr, expr, &non_style_object(STYLEX_KEYFRAMES), state);
      },
    },
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => stylex_panic!("{}", non_static_value(STYLEX_KEYFRAMES)),
  }
}

pub(crate) fn assert_valid_properties(
  obj: &EvaluateResultValue,
  valid_keys: &[&str],
  error_message: &str,
  state: &mut StateManager,
) {
  if let EvaluateResultValue::Expr(expr) = obj
    && let Expr::Object(object) = expr
  {
    let key_values = get_key_values_from_object(object);

    for key_value in key_values.iter() {
      let key = convert_key_value_to_str(key_value);
      if !valid_keys.contains(&key.as_str()) {
        build_code_frame_error_and_panic(expr, expr, error_message, state);
      }
    }
  }
}

fn assert_stylex_arg(value: &EvaluateResultValue, state: &mut StateManager, fn_name: &str) {
  if let EvaluateResultValue::Expr(expr) = value {
    if !expr.is_object() {
      build_code_frame_error_and_panic(expr, expr, &non_style_object(fn_name), state);
    }
  } else {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", non_static_value(fn_name));
    }
  }
}

pub(crate) fn assert_valid_position_try(obj: &EvaluateResultValue, state: &mut StateManager) {
  assert_stylex_arg(obj, state, STYLEX_POSITION_TRY);
}

pub(crate) fn assert_valid_view_transition_class(
  obj: &EvaluateResultValue,
  state: &mut StateManager,
) {
  assert_stylex_arg(obj, state, STYLEX_VIEW_TRANSITION_CLASS);
}

pub(crate) fn validate_theme_variables(
  variables: &EvaluateResultValue,
  state: &StateManager,
) -> KeyValueProp {
  if let Some(theme_ref) = variables.as_theme_ref() {
    let mut cloned_theme_ref = theme_ref.clone();

    let value = cloned_theme_ref.get(VAR_GROUP_HASH_KEY, state);

    let key_value = create_key_value_prop_ident(
      VAR_GROUP_HASH_KEY,
      create_string_expr(match value.as_css_var() {
        Some(v) => v,
        None => stylex_panic!("{}", EXPECTED_CSS_VAR),
      }),
    );

    return key_value;
  }

  if !variables.as_expr().is_some_and(|expr| expr.is_object()) {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", ONLY_OVERRIDE_DEFINE_VARS);
    }
  }

  match variables
    .as_expr()
    .and_then(|expr| expr.as_object())
    .map(get_key_values_from_object)
    .and_then(|key_values| {
      for key_value in key_values.into_iter() {
        let key = convert_key_value_to_str(&key_value);

        if key == VAR_GROUP_HASH_KEY {
          let value = &key_value.value;

          if let Some(lit) = value.as_lit() {
            let value = convert_lit_to_string(lit);

            if value.filter(|value| !value.is_empty()).is_some() {
              return Some(key_value);
            }
          }
        }
      }

      None
    }) {
    Some(key_value) => key_value,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", ONLY_OVERRIDE_DEFINE_VARS),
  }
}
