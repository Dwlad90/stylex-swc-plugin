use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_constants::constants::messages::{ONLY_OVERRIDE_DEFINE_VARS, SPREAD_NOT_SUPPORTED};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    structures::{
      functions::{FunctionConfigType, FunctionMap},
      state_manager::ImportKind,
      types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
    },
    transformers::{
      stylex_create_theme::stylex_create_theme, stylex_keyframes::get_keyframes_fn,
      stylex_position_try::get_position_try_fn, stylex_types::get_types_fn,
    },
    utils::{
      core::{
        dev_class_name::{convert_theme_to_dev_styles, convert_theme_to_test_styles},
        js_to_ast::{NestedStringObject, convert_object_to_ast},
      },
      js::evaluate::evaluate,
      log::build_code_frame_error::build_code_frame_error,
      validators::{
        is_create_theme_call, validate_stylex_create_theme_indent, validate_theme_variables,
      },
    },
  },
};
use stylex_constants::constants::api_names::{STYLEX_CREATE_THEME, STYLEX_KEYFRAMES, STYLEX_TYPES};
use stylex_constants::constants::messages::{non_static_value, non_style_object};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create_theme_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_create_theme_call = is_create_theme_call(call, &self.state);

    if is_create_theme_call {
      let (_, parent_var_decl) = &self.get_call_var_name(call);

      validate_stylex_create_theme_indent(parent_var_decl, call, &mut self.state);

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        #[cfg_attr(coverage_nightly, coverage(off))]
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => first_arg.expr.clone(),
      })?;

      let second_arg = call
        .args
        .get(1)
        .map(|second_arg| match &second_arg.spread {
          #[cfg_attr(coverage_nightly, coverage(off))]
          Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
          None => second_arg.expr.clone(),
        })?;

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

        let identifier = identifiers
          .entry(name.get_import_str().into())
          .or_insert_with(|| Box::new(FunctionConfigType::Map(FxHashMap::default())));

        if let Some(identifier_map) = identifier.as_map_mut() {
          identifier_map.insert(STYLEX_TYPES.into(), types_fn.clone());
        }
      }

      self
        .state
        .apply_stylex_env(&mut identifiers, &mut member_expressions);

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
        disable_imports: false,
      });

      let evaluated_arg1 = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg1.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg1
            .deopt
            .unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value(STYLEX_CREATE_THEME),
          &mut self.state,
        )
      );

      let evaluated_arg2 = evaluate(&second_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg2.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg2
            .deopt
            .unwrap_or_else(|| *second_arg.to_owned()),
          &non_static_value(STYLEX_CREATE_THEME),
          &mut self.state,
        )
      );

      let mut variables = match evaluated_arg1.value {
        Some(ref value) => {
          validate_theme_variables(value, &self.state);
          value.clone()
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &evaluated_arg1
              .deopt
              .unwrap_or_else(|| *first_arg.to_owned()),
            ONLY_OVERRIDE_DEFINE_VARS,
            &mut self.state,
          )
        ),
      };

      let overrides = match evaluated_arg2.value {
        Some(ref value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &evaluated_arg2
                .deopt
                .unwrap_or_else(|| *second_arg.to_owned()),
              &non_style_object(STYLEX_CREATE_THEME),
              &mut self.state,
            )
          );
          value.clone()
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &evaluated_arg2
              .deopt
              .unwrap_or_else(|| *second_arg.to_owned()),
            &non_style_object(STYLEX_CREATE_THEME),
            &mut self.state,
          )
        ),
      };

      let (mut overrides_obj, inject_styles) = stylex_create_theme(
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

      self
        .state
        .register_styles(call, &inject_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
