use rustc_hash::FxHashSet;
use swc_core::{
  atoms::Atom,
  ecma::ast::{ArrowExpr, CallExpr, Expr, ExprOrSpread, KeyValueProp, Lit, Pat, VarDeclarator},
};

use crate::shared::{
  constants::{
    common::THEME_NAME_KEY,
    messages::{
      DUPLICATE_CONDITIONAL, ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE,
      INVALID_PSEUDO_OR_AT_RULE, NO_OBJECT_SPREADS, NON_OBJECT_KEYFRAME,
      NON_STATIC_SECOND_ARG_CREATE_THEME_VALUE, ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS,
      POSITION_TRY_INVALID_PROPERTY, illegal_argument_length, non_export_named_declaration,
      non_static_value, non_style_object, unbound_call_value,
    },
  },
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
  },
  structures::state_manager::StateManager,
  utils::{
    ast::{convertors::string_to_expression, factories::key_value_factory},
    common::get_import_from,
    log::build_code_frame_error::build_code_frame_error_and_panic,
  },
};

use super::{
  ast::convertors::{key_value_to_str, lit_to_string},
  common::get_key_values_from_object,
};

// TODO: Once we have a reliable validator, these property checks should be replaced with
// validators that can also validate the values.
const VALID_POSITION_TRY_PROPERTIES: &[&str] = &[
  // anchor Properties
  "anchorName",
  // position Properties
  "positionAnchor",
  "positionArea",
  // inset Properties
  "top",
  "right",
  "bottom",
  "left",
  "inset",
  "insetBlock",
  "insetBlockEnd",
  "insetBlockStart",
  "insetInline",
  "insetInlineEnd",
  "insetInlineStart",
  // margin Properties
  "margin",
  "marginBlock",
  "marginBlockEnd",
  "marginBlockStart",
  "marginInline",
  "marginInlineEnd",
  "marginInlineStart",
  "marginTop",
  "marginBottom",
  "marginLeft",
  "marginRight",
  // size properties
  "width",
  "height",
  "minWidth",
  "minHeight",
  "maxWidth",
  "maxHeight",
  "blockSize",
  "inlineSize",
  "minBlockSize",
  "minInlineSize",
  "maxBlockSize",
  "maxInlineSize",
  // self alignment properties
  "alignSelf",
  "justifySelf",
  "placeSelf",
];

pub(crate) fn validate_stylex_create(call: &CallExpr, state: &mut StateManager) {
  if !is_create_call(call, state) {
    return;
  }

  match state.find_top_level_expr(
    call,
    |tpe: &TopLevelExpression| matches!(tpe.1, Expr::Array(_)),
    None,
  ) {
    Some(_) => {}
    None => build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &unbound_call_value("create"),
      state,
    ),
  }

  if call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &illegal_argument_length("create", 1),
      state,
    );
  }

  let first_arg = &call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &first_arg.expr,
      &non_style_object("create"),
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
    None => panic!("{}", non_static_value("keyframes")),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &non_static_value("keyframes"),
      state,
    );
  });

  match state.find_top_level_expr(init_call, |_| false, None) {
    Some(_) => {}
    None => build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &unbound_call_value("keyframes"),
      state,
    ),
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &illegal_argument_length("keyframes", 1),
      state,
    );
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      &non_style_object("keyframes"),
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
    None => panic!("{}", non_static_value("positionTry")),
  };

  let init_call = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &non_static_value("positionTry"),
      state,
    );
  });

  match state.find_top_level_expr(init_call, |_| false, None) {
    Some(_) => {}
    None => build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &unbound_call_value("positionTry"),
      state,
    ),
  }

  if init_call.args.len() != 1 {
    build_code_frame_error_and_panic(
      &init_expr,
      &init_expr,
      &illegal_argument_length("positionTry", 1),
      state,
    );
  }

  let first_arg: &_ = &init_call.args[0];
  if !first_arg.expr.is_object() {
    build_code_frame_error_and_panic(
      &init_expr,
      &first_arg.expr,
      &non_style_object("positionTry"),
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
      &unbound_call_value("createTheme"),
      state,
    );
  });

  let init_expr = var_decl.init.as_ref().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      &Expr::Call(call.clone()),
      &Expr::Call(call.clone()),
      &unbound_call_value("createTheme"),
      state,
    );
  });

  let init = init_expr.as_call().unwrap_or_else(|| {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &non_static_value("createTheme"),
      state,
    );
  });

  match state.find_top_level_expr(call, |_| false, None) {
    Some(_) => {}
    None => build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &unbound_call_value("createTheme"),
      state,
    ),
  };

  if init.args.len() != 2 {
    build_code_frame_error_and_panic(
      init_expr,
      &Expr::Call(call.clone()),
      &illegal_argument_length("createTheme", 1),
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
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      &unbound_call_value("defineVars"),
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
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      &illegal_argument_length("defineVars", 1),
      state,
    );
  }

  let TopLevelExpression(kind, _, _) = stylex_create_theme_top_level_expr;

  if !matches!(kind, TopLevelExpressionKind::NamedExport) {
    build_code_frame_error_and_panic(
      &call_expr,
      &call_expr,
      &non_export_named_declaration("defineVars"),
      state,
    );
  }

  Some(stylex_create_theme_top_level_expr.clone())
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
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      &unbound_call_value("defineConsts"),
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
        .unwrap_or_else(|| ExprOrSpread {
          spread: None,
          expr: Box::new(call_expr.clone()),
        })
        .expr,
      &illegal_argument_length("defineConsts", 1),
      state,
    );
  }

  if !matches!(
    define_consts_top_level_expr.0,
    TopLevelExpressionKind::NamedExport
  ) {
    build_code_frame_error_and_panic(
      &call_expr,
      &call_expr,
      &non_export_named_declaration("defineConsts"),
      state,
    );
  }

  Some(define_consts_top_level_expr.clone())
}

pub(crate) fn is_create_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(("create", &state.stylex_create_import), call, state)
}

pub(crate) fn is_props_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(("props", &state.stylex_props_import), call, state)
}

pub(crate) fn is_keyframes_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  match init {
    Some(call) => is_target_call(("keyframes", &state.stylex_keyframes_import), &call, state),
    _ => false,
  }
}

pub(crate) fn is_position_try_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
  let init = var_decl.init.as_ref().and_then(|init| init.clone().call());

  match init {
    Some(call) => is_target_call(
      ("positionTry", &state.stylex_position_try_import),
      &call,
      state,
    ),
    _ => false,
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

pub(crate) fn is_define_consts_call(call: &CallExpr, state: &StateManager) -> bool {
  is_target_call(
    ("defineConsts", &state.stylex_define_consts_import),
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
        let key = key_value_to_str(namespace);

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
  let inner_key = key_value_to_str(inner_key_value);
  let inner_value = inner_key_value.value.clone();

  if !(inner_key.starts_with(':')
      || inner_key.starts_with('@')
      // This is a placeholder for `defineConsts` values that are later inlined
      || inner_key.starts_with("var(--")
      || inner_key == "default")
  {
    panic!("{}", INVALID_PSEUDO_OR_AT_RULE);
  }

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

pub(crate) fn assert_valid_keyframes(obj: &EvaluateResultValue, state: &mut StateManager) {
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
        build_code_frame_error_and_panic(expr, expr, &non_style_object("keyframes"), state);
      }
    },
    _ => panic!("{}", non_static_value("keyframes")),
  }
}

pub(crate) fn assert_valid_properties(obj: &EvaluateResultValue, state: &mut StateManager) {
  if let EvaluateResultValue::Expr(expr) = obj {
    if let Expr::Object(object) = expr {
      let key_values = get_key_values_from_object(object);

      for key_value in key_values.iter() {
        let key = key_value_to_str(key_value);
        if !VALID_POSITION_TRY_PROPERTIES.contains(&key.as_str()) {
          build_code_frame_error_and_panic(expr, expr, POSITION_TRY_INVALID_PROPERTY, state);
        }
      }
    }
  }
}

pub(crate) fn assert_valid_position_try(obj: &EvaluateResultValue, state: &mut StateManager) {
  if let EvaluateResultValue::Expr(expr) = obj {
    if !expr.is_object() {
      build_code_frame_error_and_panic(expr, expr, &non_style_object("positionTry"), state);
    }
  } else {
    panic!("{}", non_static_value("positionTry"));
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
        let key = key_value_to_str(&key_value);

        if key == "__themeName__" {
          let value = &key_value.value;

          if let Some(lit) = value.as_lit() {
            let value = lit_to_string(lit);

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
