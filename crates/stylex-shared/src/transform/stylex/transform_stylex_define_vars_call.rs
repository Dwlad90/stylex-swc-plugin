use rustc_hash::FxHashMap;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::utils::{common::gen_file_based_identifier, js::evaluate::evaluate};
use crate::shared::{
  constants::messages::NON_OBJECT_FOR_STYLEX_CALL,
  utils::validators::{is_define_vars_call, validate_stylex_define_vars},
};
use crate::shared::{
  constants::messages::NON_STATIC_VALUE,
  utils::core::js_to_expr::{convert_object_to_ast, NestedStringObject},
};
use crate::shared::{
  enums::data_structures::top_level_expression::TopLevelExpressionKind,
  structures::{
    functions::FunctionMap,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::{
    stylex_define_vars::stylex_define_vars, stylex_keyframes::get_keyframes_fn,
    stylex_types::get_types_fn,
  },
};
use crate::shared::{
  structures::functions::FunctionConfigType,
  utils::log::build_code_frame_error::build_code_frame_error,
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_vars = is_define_vars_call(call, &self.state);

    if is_define_vars {
      validate_stylex_define_vars(call, &self.state);

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!("Spread"),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let keyframes_fn = get_keyframes_fn();
      let types_fn = get_types_fn();

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

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
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
          NON_STATIC_VALUE,
          self.state.get_filename(),
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
              NON_OBJECT_FOR_STYLEX_CALL,
              self.state.get_filename(),
            )
          );
          value
        }
        None => panic!("{}", NON_STATIC_VALUE),
      };

      let file_name = self
        .state
        .get_filename_for_hashing()
        .expect("No filename found for generating theme name.");

      let export_expr = self
        .state
        .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call);

      let export_name = export_expr
        .and_then(|expr| expr.2)
        .map(|decl| decl.to_string())
        .expect("Export variable not found");

      self.state.theme_name = Some(gen_file_based_identifier(&file_name, &export_name, None));

      let (variables_obj, injected_styles_sans_keyframes) =
        stylex_define_vars(&value, &mut self.state);

      let mut injected_styles = self.state.injected_keyframes.clone();
      injected_styles.extend(injected_styles_sans_keyframes);

      let result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(variables_obj));

      self
        .state
        .register_styles(call, &injected_styles, &result_ast);

      Some(result_ast)
    } else {
      None
    }
  }
}
