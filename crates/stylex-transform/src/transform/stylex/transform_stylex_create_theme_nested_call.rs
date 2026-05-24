use indexmap::IndexMap;
use stylex_constants::constants::{
  api_names::STYLEX_UNSTABLE_CREATE_THEME_NESTED,
  common::VAR_GROUP_HASH_KEY,
  messages::{
    EXPECTED_CSS_VAR, ONLY_OVERRIDE_DEFINE_VARS_NESTED, SPREAD_NOT_SUPPORTED, non_static_value,
    non_style_object,
  },
};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    enums::data_structures::evaluate_result_value::EvaluateResultValue,
    structures::{functions::FunctionMap, state_manager::ImportKind},
    transformers::stylex_create_theme_nested::stylex_create_theme_nested,
    utils::{
      core::{
        dev_class_name::{convert_theme_to_dev_styles, convert_theme_to_test_styles},
        js_to_ast::{NestedStringObject, convert_object_to_ast},
      },
      js::evaluate::evaluate,
      log::build_code_frame_error::build_code_frame_error,
      validators::validate_define_call,
    },
  },
  transform::stylex::visitor_utils::{build_eval_config, is_call_to},
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create_theme_nested_call(
    &mut self,
    call: &CallExpr,
  ) -> Option<Expr> {
    if !is_call_to(
      call,
      &self.state,
      ImportKind::CreateThemeNested,
      STYLEX_UNSTABLE_CREATE_THEME_NESTED,
    ) {
      return None;
    }

    validate_define_call(
      call,
      STYLEX_UNSTABLE_CREATE_THEME_NESTED,
      2,
      false,
      &mut self.state,
    );

    let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
      Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
      None => first_arg.expr.clone(),
    })?;

    let second_arg = call
      .args
      .get(1)
      .map(|second_arg| match &second_arg.spread {
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => second_arg.expr.clone(),
      })?;

    let evaluated_arg1 = evaluate(&first_arg, &mut self.state, &FunctionMap::default());

    if !evaluated_arg1.confident {
      stylex_panic!(
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg1
            .deopt
            .unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value(STYLEX_UNSTABLE_CREATE_THEME_NESTED),
          &mut self.state,
        )
      );
    }

    let mut variables = match evaluated_arg1.value {
      Some(value) => {
        validate_nested_theme_variables(&value, &self.state);
        value
      },
      None => stylex_panic!(
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg1
            .deopt
            .unwrap_or_else(|| *first_arg.to_owned()),
          ONLY_OVERRIDE_DEFINE_VARS_NESTED,
          &mut self.state,
        )
      ),
    };

    let function_map = build_eval_config(&mut self.state);
    let evaluated_arg2 = evaluate(&second_arg, &mut self.state, &function_map);

    if !evaluated_arg2.confident {
      stylex_panic!(
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg2
            .deopt
            .unwrap_or_else(|| *second_arg.to_owned()),
          &non_static_value(STYLEX_UNSTABLE_CREATE_THEME_NESTED),
          &mut self.state,
        )
      );
    }

    let overrides = match evaluated_arg2.value {
      Some(value) => {
        let is_object = value
          .as_expr()
          .map(|expr| expr.is_object())
          .unwrap_or(false);

        if !is_object {
          stylex_panic!(
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &evaluated_arg2
                .deopt
                .unwrap_or_else(|| *second_arg.to_owned()),
              &non_style_object(STYLEX_UNSTABLE_CREATE_THEME_NESTED),
              &mut self.state,
            )
          );
        }

        value
      },
      None => stylex_panic!("{}", non_static_value(STYLEX_UNSTABLE_CREATE_THEME_NESTED)),
    };

    let (mut overrides_obj, injected_styles) = stylex_create_theme_nested(
      &mut variables,
      &overrides,
      &mut self.state,
      &mut IndexMap::default(),
    );

    let (var_name, _) = self.get_call_var_name(call);

    if self.state.is_test() {
      overrides_obj =
        convert_theme_to_test_styles(&var_name, &overrides_obj, self.state.get_filename());
    } else if self.state.is_dev() {
      overrides_obj =
        convert_theme_to_dev_styles(&var_name, &overrides_obj, self.state.get_filename());
    }

    let result_ast =
      convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(overrides_obj));

    let mut injected_styles_with_dependencies = self.state.other_injected_css_rules.clone();
    injected_styles_with_dependencies.extend(injected_styles);

    self
      .state
      .register_styles(call, &injected_styles_with_dependencies, &result_ast, None);

    Some(result_ast)
  }
}

fn validate_nested_theme_variables(
  value: &EvaluateResultValue,
  state: &crate::shared::structures::state_manager::StateManager,
) {
  match value {
    EvaluateResultValue::ThemeRef(theme_ref) => {
      let mut theme_ref = theme_ref.clone();
      let value = theme_ref.get(VAR_GROUP_HASH_KEY, state);
      if value.as_css_var().is_none() {
        stylex_panic!("{}", EXPECTED_CSS_VAR);
      }
    },
    _ => validate_nested_theme_variables_object(value),
  }
}

fn validate_nested_theme_variables_object(value: &EvaluateResultValue) {
  let Some(key_values) = value
    .as_expr()
    .and_then(|expr| expr.as_object())
    .map(stylex_ast::ast::convertors::get_key_values_from_object)
  else {
    stylex_panic!("{}", ONLY_OVERRIDE_DEFINE_VARS_NESTED)
  };

  let has_var_group_hash = key_values.into_iter().any(|key_value| {
    stylex_ast::ast::convertors::convert_key_value_to_str(&key_value) == VAR_GROUP_HASH_KEY
      && key_value
        .value
        .as_lit()
        .and_then(stylex_ast::ast::convertors::convert_lit_to_string)
        .filter(|value| !value.is_empty())
        .is_some()
  });

  if !has_var_group_hash {
    stylex_panic!("{}", ONLY_OVERRIDE_DEFINE_VARS_NESTED);
  }
}
