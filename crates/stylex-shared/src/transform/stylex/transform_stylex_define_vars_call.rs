use rustc_hash::FxHashMap;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::{
  constants::messages::{cannot_generate_hash, non_static_value, non_style_object},
  enums::data_structures::top_level_expression::TopLevelExpression,
  transformers::stylex_position_try::get_position_try_fn,
  utils::{
    common::gen_file_based_identifier,
    core::js_to_expr::{NestedStringObject, convert_object_to_ast},
    js::evaluate::evaluate,
    validators::{find_and_validate_stylex_define_vars, is_define_vars_call},
  },
};
use crate::shared::{
  structures::functions::FunctionConfigType,
  utils::log::build_code_frame_error::build_code_frame_error,
};
use crate::shared::{
  structures::{
    functions::FunctionMap,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::{
    stylex_define_vars::stylex_define_vars, stylex_keyframes::get_keyframes_fn,
    stylex_types::get_types_fn,
  },
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_vars = is_define_vars_call(call, &self.state);

    if is_define_vars {
      let stylex_create_theme_top_level_expr =
        find_and_validate_stylex_define_vars(call, &mut self.state).unwrap();

      let TopLevelExpression(_, _, var_id) = stylex_create_theme_top_level_expr;

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!("Spread"),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let keyframes_fn = get_keyframes_fn();
      let types_fn = get_types_fn();
      let position_try_fn = get_position_try_fn();

      for name in &self.state.stylex_keyframes_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      for name in &self.state.stylex_types_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(types_fn.clone())),
        );
      }

      for name in &self.state.stylex_position_try_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        member_expression.insert(
          "positionTry".into(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );

        let identifier = identifiers
          .entry(name.get_import_str().into())
          .or_insert_with(|| Box::new(FunctionConfigType::Map(FxHashMap::default())));

        if let Some(identifier_map) = identifier.as_map_mut() {
          identifier_map.insert("types".into(), types_fn.clone());
        }
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value("defineVars"),
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
              &non_style_object("defineVars"),
              &mut self.state,
            )
          );
          value
        }
        None => panic!("{}", non_static_value("defineVars")),
      };

      let file_name = self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
        .unwrap_or_else(|| panic!("{}", cannot_generate_hash("defineVars")));

      let export_name = var_id
        .map(|decl| decl.to_string())
        .expect("Export variable not found");

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
