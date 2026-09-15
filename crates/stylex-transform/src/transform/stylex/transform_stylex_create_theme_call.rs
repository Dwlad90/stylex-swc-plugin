use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_constants::constants::messages::ONLY_OVERRIDE_DEFINE_VARS;
use stylex_macros::stylex_panic;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    transformers::{
      stylex_create_theme::stylex_create_theme, stylex_keyframes::get_keyframes_fn,
      stylex_position_try::get_position_try_fn, stylex_types::get_types_fn,
    },
    utils::{
      core::{
        dev_class_name::{convert_theme_to_dev_styles, convert_theme_to_test_styles},
        js_to_ast::{NestedStringObject, convert_object_to_ast},
      },
      validators::{
        argument_at, is_create_theme_call, validate_stylex_create_theme_indent,
        validate_theme_variables,
      },
    },
  },
  transform::stylex::visitor_utils::{apply_unstable_conditional, insert_stylex_identifier_entry},
};
use stylex_constants::constants::{
  api_names::{STYLEX_CREATE_THEME, STYLEX_KEYFRAMES, STYLEX_TYPES},
  messages::{non_static_value, non_style_object},
};
use stylex_diagnostics::code_frame::build_code_frame_error;
use stylex_evaluator::{evaluate::evaluate, evaluate_result::refusal_site};
use stylex_state::{
  functions::{FunctionConfigType, FunctionMap},
  state_manager::ImportKind,
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create_theme_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_create_theme_call = is_create_theme_call(call, &self.state);

    if is_create_theme_call {
      let (_, parent_var_decl) = &self.get_call_var_name(call);

      validate_stylex_create_theme_indent(parent_var_decl, call, &mut self.state);

      let first_arg = argument_at(call, 0, STYLEX_CREATE_THEME);

      let second_arg = argument_at(call, 1, STYLEX_CREATE_THEME);

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let keyframes_fn = get_keyframes_fn();
      let types_fn = get_types_fn();
      let position_try_fn = get_position_try_fn();

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Keyframes) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::PositionTry) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Types) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(types_fn.clone())),
          );
        }
      }

      for name in self.state.stylex_imports() {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          STYLEX_KEYFRAMES.into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        insert_stylex_identifier_entry(
          &mut identifiers,
          name,
          STYLEX_TYPES.into(),
          FunctionConfigType::Regular(types_fn.clone()),
        );
      }

      apply_unstable_conditional(&self.state, &mut identifiers, &mut member_expressions);

      self
        .state
        .apply_stylex_env(&mut identifiers, &mut member_expressions);

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
        disable_imports: false,
      });

      let evaluated_arg1 = evaluate(first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg1.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &refusal_site(evaluated_arg1.deopt.as_ref(), first_arg),
          &non_static_value(STYLEX_CREATE_THEME),
          &mut self.state,
        )
      );

      let evaluated_arg2 = evaluate(second_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg2.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &refusal_site(evaluated_arg2.deopt.as_ref(), second_arg),
          &non_static_value(STYLEX_CREATE_THEME),
          &mut self.state,
        )
      );

      let variables = match evaluated_arg1.value {
        Some(value) => {
          // Asked here so that a first argument that is no variable group is
          // refused with this sentence, before the producer runs and reports
          // the same input in its own words. Both arguments are already read
          // at this point, so this does not change which one is read first.
          validate_theme_variables(&value, &self.state);
          value
        },
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &refusal_site(evaluated_arg1.deopt.as_ref(), first_arg),
            ONLY_OVERRIDE_DEFINE_VARS,
            &mut self.state,
          )
        ),
      };

      let overrides = match evaluated_arg2.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &refusal_site(evaluated_arg2.deopt.as_ref(), second_arg),
              &non_style_object(STYLEX_CREATE_THEME),
              &mut self.state,
            )
          );
          value
        },
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &refusal_site(evaluated_arg2.deopt.as_ref(), second_arg),
            &non_style_object(STYLEX_CREATE_THEME),
            &mut self.state,
          )
        ),
      };

      let (mut overrides_obj, inject_styles) = stylex_create_theme(
        &variables,
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

      self
        .state
        .register_styles(call, &inject_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
