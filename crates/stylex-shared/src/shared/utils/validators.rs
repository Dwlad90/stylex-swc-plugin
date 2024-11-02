
use rustc_hash::FxHashSet;
use swc_core::{
  atoms::Atom,
  ecma::ast::{CallExpr, Expr, KeyValueProp, Lit, Pat, PropName, VarDeclarator},
};

use crate::shared::{
  constants::{
    common::THEME_NAME_KEY,
    messages::{
      DUPLICATE_CONDITIONAL, ILLEGAL_ARGUMENT_LENGTH, ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE,
      INVALID_PSEUDO_OR_AT_RULE, NON_EXPORT_NAMED_DECLARATION, NON_OBJECT_FOR_STYLEX_CALL,
      NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL, NON_OBJECT_KEYFRAME, NON_STATIC_KEYFRAME_VALUE,
      NON_STATIC_VALUE, ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS, ONLY_TOP_LEVEL_INCLUDES,
      UNBOUND_STYLEX_CALL_VALUE,
    },
  },
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
  },
  regex::INCLUDED_IDENT_REGEX,
  structures::state_manager::StateManager,
  utils::{
    ast::{
      convertors::string_to_expression,
      factories::{ident_factory, key_value_factory},
    },
    common::{get_string_val_from_lit, get_var_decl_by_ident_or_member},
  },
};

use super::common::{get_key_str, get_key_values_from_object};

pub(crate) fn validate_stylex_create(call: &CallExpr, state: &StateManager) {
  if !is_create_call(call, state) {
    return;
  }

  let ident = ident_factory("create");

  let call_expr = Expr::from(call.clone());

  assert!(
    get_var_decl_by_ident_or_member(state, &ident).is_some()
      || state
        .top_level_expressions
        .iter()
        .any(|TopLevelExpression(_, call_item, _)| {
          match call_item {
            Expr::Call(call) => call.eq(
              call_expr
                .as_call()
                .expect("Top level expression is not a call"),
            ),
            Expr::Array(_) => true,
            _ => false,
          }
        }),
    "{}",
    UNBOUND_STYLEX_CALL_VALUE
  );

  assert!(call.args.len() == 1, "{}", ILLEGAL_ARGUMENT_LENGTH);

  let first_args = &call.args[0];

  assert!(
    first_args.expr.is_object(),
    "{}",
    NON_OBJECT_FOR_STYLEX_CALL
  )
}

pub(crate) fn validate_stylex_keyframes_indent(var_decl: &VarDeclarator, state: &StateManager) {
  let init = match &var_decl.init {
    Some(init) => init.clone().call().expect(NON_STATIC_KEYFRAME_VALUE),
    None => panic!("{}", NON_STATIC_KEYFRAME_VALUE),
  };

  if !is_keyframes_call(var_decl, state) {
    return;
  }

  let ident = ident_factory("keyframes");

  let expr = Expr::from(init.clone());

  assert!(
    get_var_decl_by_ident_or_member(state, &ident).is_some()
      || state
        .top_level_expressions
        .iter()
        .any(|TopLevelExpression(_, call_item, _)| { call_item.eq(&expr) }),
    "{}",
    UNBOUND_STYLEX_CALL_VALUE
  );

  assert!(init.args.len() == 1, "{}", ILLEGAL_ARGUMENT_LENGTH);

  let first_args = &init.args[0];

  assert!(
    first_args.expr.is_object(),
    "{}",
    NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL
  )
}

pub(crate) fn validate_stylex_create_theme_indent(
  var_decl: &Option<Box<VarDeclarator>>,
  call: &CallExpr,
  state: &StateManager,
) {
  let Some(var_decl) = var_decl else {
    panic!("{}", UNBOUND_STYLEX_CALL_VALUE)
  };

  let init = match &var_decl.init {
    Some(init) => init.clone().call().expect(NON_STATIC_KEYFRAME_VALUE),
    None => panic!("{}", NON_STATIC_KEYFRAME_VALUE),
  };

  if !is_create_theme_call(call, state) {
    return;
  }

  let ident = ident_factory("keyframes");

  let expr = Expr::from(init.clone());

  assert!(
    get_var_decl_by_ident_or_member(state, &ident).is_some()
      || state
        .top_level_expressions
        .iter()
        .any(|TopLevelExpression(_, call_item, _)| { call_item.eq(&expr) }),
    "{}",
    UNBOUND_STYLEX_CALL_VALUE
  );

  assert!(init.args.len() == 2, "{}", ILLEGAL_ARGUMENT_LENGTH);
}

pub(crate) fn validate_stylex_define_vars(call: &CallExpr, state: &StateManager) {
  if !is_define_vars_call(call, state) {
    return;
  }

  let ident = ident_factory("defineVars");

  let expr = Expr::from(call.clone());

  assert!(
    get_var_decl_by_ident_or_member(state, &ident).is_some()
      || state
        .top_level_expressions
        .iter()
        .any(|TopLevelExpression(_, call_item, _)| { call_item.eq(&expr) }),
    "{}",
    UNBOUND_STYLEX_CALL_VALUE
  );

  assert!(call.args.len() == 1, "{}", ILLEGAL_ARGUMENT_LENGTH);

  assert!(
    state
      .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call)
      .is_some(),
    "{}",
    NON_EXPORT_NAMED_DECLARATION
  );
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
  let init = match &var_decl.init {
    Some(init) => init.clone().call(),
    None => None,
  };

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
  let is_create_ident = call.callee.as_expr().map_or(false, |expr| {
    expr
      .as_ident()
      .map_or(false, |ident| imports_map.contains(&ident.sym))
  });

  let is_create_member = call
    .callee
    .as_expr()
    .and_then(|expr| expr.as_member())
    .map_or(false, |member| {
      member.obj.is_ident()
        && member.prop.as_ident().map_or(false, |ident| {
          ident.sym.eq(call_name)
            && state.stylex_import_stringified().contains(
              &member
                .obj
                .as_ident()
                .expect("Member epression is not an ident")
                .sym
                .to_string(),
            )
        })
    });

  is_create_ident || is_create_member
}
pub(crate) fn validate_namespace(namespaces: &[KeyValueProp], conditions: &[String]) {
  for namespace in namespaces {
    let key = match &namespace.key {
      PropName::Ident(key) => format!("{}", key.sym),
      PropName::Str(key) => {
        if !(key.value.starts_with('@')
          || key.value.starts_with(':')
          || key.value == "default"
          || namespace.value.is_lit())
        {
          panic!("{}", INVALID_PSEUDO_OR_AT_RULE)
        }
        key.value.to_string()
      }
      _ => panic!("{}", NON_STATIC_VALUE),
    };

    match namespace.value.as_ref() {
      Expr::Lit(lit) => {
        if let Lit::Str(_) | Lit::Null(_) | Lit::Num(_) | Lit::BigInt(_) = lit {
        } else {
          panic!("{}", ILLEGAL_PROP_VALUE);
        }
      }
      Expr::Array(array) => {
        for elem in array.elems.iter().flatten() {
          assert!(
            elem.spread.is_none(),
            "{}",
            "Spread operator not implemented"
          );

          if let Expr::Lit(_) = elem.expr.as_ref() {
            // Do nothing
          } else {
            panic!("{}", ILLEGAL_PROP_ARRAY_VALUE);
          }
        }
      }
      Expr::Object(object) => {
        let key = get_key_str(namespace);

        if key.starts_with('@') || key.starts_with(':') {
          if conditions.contains(&key) {
            panic!("{}", DUPLICATE_CONDITIONAL);
          }

          let nested_key_values = get_key_values_from_object(object);

          let mut extended_conditions = conditions.to_vec();
          extended_conditions.push(key);

          validate_namespace(&nested_key_values, &extended_conditions);
        } else {
          let conditional_styles_key_values = get_key_values_from_object(object);

          for conditional_style in &conditional_styles_key_values {
            validate_conditional_styles(conditional_style, conditions);
          }
        }
      }
      _ => {
        if INCLUDED_IDENT_REGEX.is_match(&key) {
          assert!(conditions.is_empty(), "{}", ONLY_TOP_LEVEL_INCLUDES)
        }
      }
    }
  }
}

pub(crate) fn validate_dynamic_style_params(params: &[Pat]) {
  if params.iter().any(|param| !param.is_ident()) {
    panic!("{}", ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS);
  }
}

pub(crate) fn validate_conditional_styles(inner_key_value: &KeyValueProp, conditions: &[String]) {
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
      for elem in array.elems.iter() {
        match elem {
          Some(elem) => match elem.expr.as_ref() {
            Expr::Lit(_) => {}
            _ => panic!("{}", ILLEGAL_PROP_VALUE),
          },
          None => {}
        }
      }
    }
    Expr::Object(object) => {
      let nested_key_values = get_key_values_from_object(object);

      let mut extended_conditions = conditions.to_vec();
      extended_conditions.push(inner_key);

      for nested_key_value in nested_key_values.iter() {
        validate_conditional_styles(nested_key_value, &extended_conditions);
      }
    }
    Expr::Ident(_) => {
      if INCLUDED_IDENT_REGEX.is_match(&inner_key) {
        panic!("{}", ONLY_TOP_LEVEL_INCLUDES);
      }
    }
    _ => panic!("{}", ILLEGAL_PROP_VALUE),
  }
}

pub(crate) fn assert_valid_keyframes(obj: &EvaluateResultValue) {
  match obj {
    EvaluateResultValue::Expr(expr) => match expr {
      Expr::Object(object) => {
        let key_values = get_key_values_from_object(object);

        for key_value in key_values.iter() {
          match key_value.value.as_ref() {
            Expr::Object(_) => {}
            _ => panic!("{}", NON_OBJECT_KEYFRAME),
          }
        }
      }
      _ => panic!("{}", NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL),
    },
    _ => panic!("{}", NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL),
  }
}

pub(crate) fn validate_theme_variables(variables: &EvaluateResultValue) -> KeyValueProp {
  if let Some(theme_ref) = variables.as_theme_ref() {
    let mut cloned_theme_ref = theme_ref.clone();

    let value = cloned_theme_ref.get(THEME_NAME_KEY);

    // state.combine(updated_state);

    let key_value = key_value_factory(THEME_NAME_KEY, string_to_expression(value.as_str()));

    return key_value;
  }

  assert!(
    variables
      .as_expr()
      .map(|expr| expr.is_object())
      .unwrap_or(false),
    "Can only override variables theme created with stylex.defineVars()."
  );

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
