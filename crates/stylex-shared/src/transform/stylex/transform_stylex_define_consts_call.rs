use rustc_hash::FxHashMap;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::utils::log::build_code_frame_error::build_code_frame_error;
use crate::shared::{
  constants::messages::cannot_generate_hash, structures::functions::FunctionMap,
};
use crate::shared::{
  constants::messages::{non_static_value, non_style_object},
  enums::data_structures::top_level_expression::TopLevelExpression,
  transformers::stylex_define_consts::stylex_define_consts,
  utils::core::js_to_expr::{NestedStringObject, convert_object_to_ast},
  utils::{
    common::gen_file_based_identifier,
    js::evaluate::evaluate,
    validators::{find_and_validate_stylex_define_consts, is_define_consts_call},
  },
};

use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_consts(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_consts = is_define_consts_call(call, &self.state);

    if is_define_consts {
      let top_level_expr_defined_consts =
        find_and_validate_stylex_define_consts(call, &mut self.state).unwrap();

      let TopLevelExpression(_, _, var_id) = top_level_expr_defined_consts;

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!("Spread"),
        None => first_arg.expr.clone(),
      })?;

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &FunctionMap::default());

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value("defineConsts"),
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
              &non_style_object("defineConsts"),
              &mut self.state,
            )
          );
          value
        }
        None => panic!("{}", non_static_value("defineConsts")),
      };

      let file_name = self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
        .unwrap_or_else(|| panic!("{}", cannot_generate_hash("defineConsts")));

      let export_name = var_id.expect("Export variable not found");

      let theme_name = Some(gen_file_based_identifier(&file_name, &export_name, None));

      self.state.theme_name = theme_name.clone();

      let (transformed_js_output, js_output) = stylex_define_consts(&value, &mut self.state);

      let result_ast = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
        transformed_js_output,
      ));

      self.state.register_styles(call, &js_output, &result_ast);

      Some(result_ast)
    } else {
      None
    }
  }
}
