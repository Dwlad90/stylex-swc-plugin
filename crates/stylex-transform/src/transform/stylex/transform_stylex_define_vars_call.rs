use rustc_hash::FxHashMap;
use stylex_constants::constants::{
  api_names::{STYLEX_DEFINE_VARS, STYLEX_KEYFRAMES, STYLEX_POSITION_TRY, STYLEX_TYPES},
  messages::{SPREAD_NOT_SUPPORTED, cannot_generate_hash, non_static_value, non_style_object},
};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::StyleXTransform;
use crate::shared::{
  structures::{
    functions::{FunctionConfigType, FunctionMap},
    state_manager::ImportKind,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::{
    stylex_define_vars::stylex_define_vars, stylex_keyframes::get_keyframes_fn,
    stylex_position_try::get_position_try_fn, stylex_types::get_types_fn,
  },
  utils::{
    common::gen_file_based_identifier,
    core::js_to_ast::{NestedStringObject, convert_object_to_ast},
    js::evaluate::evaluate,
    log::build_code_frame_error::build_code_frame_error,
    validators::{find_and_validate_stylex_define_vars, is_define_vars_call},
  },
};
use stylex_structures::top_level_expression::TopLevelExpression;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_vars = is_define_vars_call(call, &self.state);

    if is_define_vars {
      let stylex_create_theme_top_level_expr =
        match find_and_validate_stylex_define_vars(call, &mut self.state) {
          Some(expr) => expr,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!("defineVars(): Could not find the top-level variable declaration."),
        };

      let TopLevelExpression(_, _, var_id) = stylex_create_theme_top_level_expr;

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        #[cfg_attr(coverage_nightly, coverage(off))]
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => first_arg.expr.clone(),
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

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Types) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(types_fn.clone())),
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

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          STYLEX_KEYFRAMES.into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        member_expression.insert(
          STYLEX_POSITION_TRY.into(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
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

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value(STYLEX_DEFINE_VARS),
          &mut self.state,
        )
      );

      let value = match evaluated_arg.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
              &non_style_object(STYLEX_DEFINE_VARS),
              &mut self.state,
            )
          );
          value
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", non_static_value(STYLEX_DEFINE_VARS)),
      };

      let file_name = match self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
      {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", cannot_generate_hash(STYLEX_DEFINE_VARS)),
      };

      let export_name = match var_id.map(|decl| decl.to_string()) {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!(
          "defineVars(): The export variable could not be found. Ensure the call is bound to a named export."
        ),
      };

      self.state.export_id = Some(gen_file_based_identifier(&file_name, &export_name, None));

      let (variables_obj, injected_styles_sans_keyframes) =
        stylex_define_vars(&value, &mut self.state);

      let mut injected_styles = self.state.other_injected_css_rules.clone();
      injected_styles.extend(injected_styles_sans_keyframes);

      let result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(variables_obj));

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
