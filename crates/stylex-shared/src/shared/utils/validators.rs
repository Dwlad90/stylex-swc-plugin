use rustc_hash::FxHashSet;
use swc_core::{
  atoms::Atom,
  ecma::ast::{ArrowExpr, CallExpr, Expr, ExprOrSpread, KeyValueProp, Lit, Pat, VarDeclarator},
};

use crate::shared::{
  constants::{
    common::THEME_NAME_KEY,
    messages::{
      DUPLICATE_CONDITIONAL, ILLEGAL_ARGUMENT_LENGTH, ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE,
      INVALID_PSEUDO_OR_AT_RULE, NON_EXPORT_NAMED_DECLARATION, NON_OBJECT_FOR_STYLEX_CALL,
      NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL, NON_OBJECT_KEYFRAME, NON_STATIC_KEYFRAME_VALUE,
      NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE, ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS,
      UNBOUND_STYLEX_CALL_VALUE,
    },
  },
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
  },
  structures::state_manager::StateManager,
  utils::{
    ast::{convertors::string_to_expression, factories::key_value_factory},
    common::get_string_val_from_lit,
    log::build_code_frame_error::build_code_frame_error_and_panic,
  },
};

use super::common::{get_key_str, get_key_values_from_object};

pub(crate) fn validate_stylex_create(call: &CallExpr, state: &StateManager) {
  if !is_create_call(call, state) {
    return;
  }

  if !state
    .top_level_expressions
    .iter()
    .any(|TopLevelExpression(_, call_item, _)| {
      matches!(call_item, Expr::Call(c) if c == call) || matches!(call_item, Expr::Array(_))
    })
  {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      UNBOUND_STYLEX_CALL_VALUE,
      state,
    );
  }

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      ILLEGAL_ARGUMENT_LENGTH,
      state,
    );
  }

  let first_arg = &call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &first_arg.expr,
      NON_OBJECT_FOR_STYLEX_CALL,
      state,
    );
  }
}

pub(crate) fn validate_stylex_keyframes_indent(var_decl: &VarDeclarator, state: &StateManager) {
  if !is_keyframes_call(var_decl, state) {
    return;
  }

  let init_expr = match &var_decl.init {
    Some(init) => init.clone(),
    None => panic!("{}", NON_STATIC_KEYFRAME_VALUE),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(&init_expr, &init_expr, NON_STATIC_KEYFRAME_VALUE, state);
  });

  if !state
    .top_level_expressions
    .iter()
    .any(|TopLevelExpression(_, call_item, _)| call_item.eq(&init_expr))
  {
    build_code_frame_error_and_panic(&init_expr, &init_expr, UNBOUND_STYLEX_CALL_VALUE, state);
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(&init_expr, &init_expr, ILLEGAL_ARGUMENT_LENGTH, state);
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL,
      state,
    );
  }
}

pub(crate) fn validate_stylex_create_theme_indent(
  var_decl: &Option<VarDeclarator>,
  call: &CallExpr,
  state: &StateManager,
) {
  if !is_create_theme_call(call, state) {
    return;
  }

  let var_decl = var_decl.as_ref().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      UNBOUND_STYLEX_CALL_VALUE,
      state,
    );
  });

  let init_expr = var_decl.init.as_ref().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      NON_STATIC_KEYFRAME_VALUE,
      state,
    );
  });

  let init = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      NON_STATIC_KEYFRAME_VALUE,
      state,
    );
  });

  let expr = Expr::Call(init.clone());

  if !state
    .top_level_expressions
    .iter()
    .any(|TopLevelExpression(_, call_item, _)| call_item.eq(&expr))
  {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      UNBOUND_STYLEX_CALL_VALUE,
      state,
    );
  }

  if init.args.len() != 2 {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      ILLEGAL_ARGUMENT_LENGTH,
      state,
    );
  }

  let second_args = &init.args[1];
  if !second_args.expr.is_object() {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE,
      state,
    );
  }
}

pub(crate) fn validate_stylex_define_vars(call: &CallExpr, state: &StateManager) {
  if !is_define_vars_call(call, state) {
    return;
  }

  let call_expr = Expr::from(call.clone());

  if !state
    .top_level_expressions
    .iter()
    .any(|TopLevelExpression(_, call_item, _)| call_item.eq(&call_expr))
  {
    build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(2)
        .cloned()
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      UNBOUND_STYLEX_CALL_VALUE,
      state,
    );
  }

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &call_expr,
      &call
        .args
        .get(1)
        .cloned()
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      ILLEGAL_ARGUMENT_LENGTH,
      state,
    );
  }

  if state
    .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call)
    .is_none()
  {
    build_code_frame_error_and_panic(&call_expr, &call_expr, NON_EXPORT_NAMED_DECLARATION, state);
  }
}

pub(crate) fn is_create_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(("create", &state.stylex_create_import), call, state)
}

pub(crate) fn is_props_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(("props", &state.stylex_props_import), call, state)
}

pub(crate) fn is_attrs_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(("attrs", &state.stylex_attrs_import), call, state)
}

pub(crate) fn is_keyframes_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  if let Some(call) = init {
    is_target_call(("keyframes", &state.stylex_keyframes_import), &call, state)
  } else {
    false
  }
}

pub(crate) fn is_create_theme_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    ("createTheme", &state.stylex_create_theme_import),
    call,
    state,
  )
}

pub(crate) fn is_define_vars_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    ("defineVars", &state.stylex_define_vars_import),
    call,
    state,
  )
}

pub(crate) fn is_target_call(
  (call_name, imports_map): (&str, &FxHashSet<Atom>),
  call: &CallExpr,
  state: &StateManager,
) -> bool {
  let is_create_ident = call
    .callee
    .as_expr()
    .and_then(|arg| arg.as_ident())
    .is_some_and(|ident| imports_map.contains(&ident.sym));

  let is_create_member = call
    .callee
    .as_expr()
    .and_then(|expr| expr.as_member())
    .is_some_and(|member| {
      member.obj.is_ident()
        && member.prop.as_ident().is_some_and(|ident| {
          ident.sym == call_name
            && state.stylex_import_stringified().contains(
              &member
                .obj
                .as_ident()
                .expect("Member expression is not an ident")
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
  state: &StateManager,
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
      }
      Expr::Array(array) => {
        for elem in array.elems.iter().flatten() {
          assert!(
            elem.spread.is_none(),
            "{}",
            "Spread operator not implemented"
          );

          if !matches!(elem.expr.as_ref(), Expr::Lit(_)) {
            build_code_frame_error_and_panic(
              &Expr::Array(array.clone()),
              &Expr::Array(array.clone()),
              ILLEGAL_PROP_ARRAY_VALUE,
              state,
            );
          }
        }
      }
      Expr::Object(object) => {
        let key = get_key_str(namespace);

        if key.starts_with('@') || key.starts_with(':') {
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
      }
      _ => {}
    }
  }
}

pub(crate) fn validate_dynamic_style_params(
  path: &ArrowExpr,
  params: &[Pat],
  state: &StateManager,
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
  state: &StateManager,
) {
  let inner_key = get_key_str(inner_key_value);
  let inner_value = inner_key_value.value.clone();

  assert!(
    (inner_key.starts_with(':') || inner_key.starts_with('@') || inner_key == "default"),
    "{}",
    INVALID_PSEUDO_OR_AT_RULE,
  );

  if conditions.contains(&inner_key) {
    panic!("{}", DUPLICATE_CONDITIONAL);
  }

  match inner_value.as_ref() {
    Expr::Lit(_) => {}
    Expr::Array(array) => {
      for elem in array.elems.iter().flatten() {
        match elem.expr.as_ref() {
          Expr::Lit(_) => {}
          _ => build_code_frame_error_and_panic(
            &Expr::Array(array.clone()),
            &Expr::Array(array.clone()),
            ILLEGAL_PROP_VALUE,
            state,
          ),
        }
      }
    }
    Expr::Object(object) => {
      let nested_key_values = get_key_values_from_object(object);

      let mut extended_conditions = conditions.to_vec();
      extended_conditions.push(inner_key);

      for nested_key_value in nested_key_values.iter() {
        validate_conditional_styles(nested_key_value, &extended_conditions, state);
      }
    }
    Expr::Ident(_) => {}
    _ => build_code_frame_error_and_panic(&inner_value, &inner_value, ILLEGAL_PROP_VALUE, state),
  }
}

pub(crate) fn assert_valid_keyframes(obj: &EvaluateResultValue, state: &StateManager) {
  match obj {
    EvaluateResultValue::Expr(expr) => match expr {
      Expr::Object(object) => {
        let key_values = get_key_values_from_object(object);

        for key_value in key_values.iter() {
          match key_value.value.as_ref() {
            Expr::Object(_) => {}
            _ => {
              build_code_frame_error_and_panic(expr, expr, NON_OBJECT_KEYFRAME, state);
            }
          }
        }
      }
      _ => {
        build_code_frame_error_and_panic(expr, expr, NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL, state);
      }
    },
    _ => panic!("{}", NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL),
  }
}

pub(crate) fn validate_theme_variables(
  variables: &EvaluateResultValue,
  state: &StateManager,
) -> KeyValueProp {
  if let Some(theme_ref) = variables.as_theme_ref() {
    let mut cloned_theme_ref = theme_ref.clone();

    let value = cloned_theme_ref.get(THEME_NAME_KEY, state);

    let key_value = key_value_factory(THEME_NAME_KEY, string_to_expression(value.as_str()));

    return key_value;
  }

  if !variables.as_expr().is_some_and(|expr| expr.is_object()) {
    panic!("Can only override variables theme created with stylex.defineVars().");
  }

  variables
    .as_expr()
    .and_then(|expr| expr.as_object())
    .map(get_key_values_from_object)
    .and_then(|key_values| {
      for key_value in key_values.into_iter() {
        let key = get_key_str(&key_value);

        if key == "__themeName__" {
          let value = &key_value.value;

          if let Some(lit) = value.as_lit() {
            let value = get_string_val_from_lit(lit);

            if value
              .and_then(|value| if value.is_empty() { None } else { Some(value) })
              .is_some()
            {
              return Some(key_value);
            }
          }
        }
      }

      None
    })
    .expect("Can only override variables theme created with stylex.defineVars().")
}
