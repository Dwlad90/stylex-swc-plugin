use md5::Digest;

use rustc_hash::FxHashMap;
use serde_json::to_string;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::{
  constants::messages::NON_OBJECT_FOR_STYLEX_CALL,
  transformers::stylex_define_consts::stylex_define_consts,
  utils::{common::md5_hash, validators::{find_and_validate_stylex_define_vars, is_define_vars_call}},
};
use crate::shared::{
  constants::messages::NON_STATIC_VALUE,
  utils::core::js_to_expr::{NestedStringObject, convert_object_to_ast},
};
use crate::shared::{
  enums::data_structures::top_level_expression::TopLevelExpression,
  utils::{
    common::gen_file_based_identifier,
    js::evaluate::evaluate,
    validators::{find_and_validate_stylex_define_consts, is_define_consts_call},
  },
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
          NON_STATIC_VALUE,
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
              NON_OBJECT_FOR_STYLEX_CALL,
              &mut self.state,
            )
          );
          value
        }
        None => panic!("{}", NON_STATIC_VALUE),
      };

      let file_name = self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
        .expect("No filename found for generating theme name.");

      let export_name = var_id.expect("Export variable not found");

      let theme_name = Some(gen_file_based_identifier(&file_name, &export_name, None));
      self.state.theme_name = theme_name;
      let a = r#"{"sm":"(min-width: 768px)","md":"(min-width: 1024px)","lg":"(min-width: 1280px)"}"#.to_string();
      dbg!(&self.state.theme_name, &a, md5_hash(&a, 8), &value, md5_hash(value, 8));

      let (variables_obj, injected_styles_sans_keyframes) =
        stylex_define_consts(&value, &mut self.state);

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
