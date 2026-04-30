use rustc_hash::FxHashMap;
use stylex_constants::constants::{
  api_names::STYLEX_DEFINE_CONSTS,
  messages::{SPREAD_NOT_SUPPORTED, cannot_generate_hash, non_static_value, non_style_object},
};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::StyleXTransform;
use crate::shared::{
  structures::functions::FunctionMap,
  transformers::stylex_define_consts::stylex_define_consts,
  utils::{
    common::gen_file_based_identifier,
    core::js_to_ast::{NestedStringObject, convert_object_to_ast},
    js::evaluate::evaluate,
    log::build_code_frame_error::build_code_frame_error,
    validators::{find_and_validate_stylex_define_consts, is_define_consts_call},
  },
};
use stylex_structures::top_level_expression::TopLevelExpression;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_consts(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_consts = is_define_consts_call(call, &self.state);

    if is_define_consts {
      let top_level_expr_defined_consts =
        match find_and_validate_stylex_define_consts(call, &mut self.state) {
          Some(expr) => expr,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => {
            stylex_panic!("defineConsts(): Could not find the top-level variable declaration.")
          },
        };

      let TopLevelExpression(_, _, var_id) = top_level_expr_defined_consts;

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        #[cfg_attr(coverage_nightly, coverage(off))]
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers = rustc_hash::FxHashMap::default();
      let mut member_expressions = rustc_hash::FxHashMap::default();
      self
        .state
        .apply_stylex_env(&mut identifiers, &mut member_expressions);

      let function_map = FunctionMap {
        identifiers,
        member_expressions,
        disable_imports: true,
      };

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value(STYLEX_DEFINE_CONSTS),
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
              &non_style_object(STYLEX_DEFINE_CONSTS),
              &mut self.state,
            )
          );
          value
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", non_static_value(STYLEX_DEFINE_CONSTS)),
      };

      let file_name = match self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
      {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", cannot_generate_hash(STYLEX_DEFINE_CONSTS)),
      };

      let export_name = match var_id {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!(
          "defineConsts(): The export variable could not be found. Ensure the call is bound to a named export."
        ),
      };

      let export_id = Some(gen_file_based_identifier(&file_name, &export_name, None));

      self.state.export_id = export_id;

      let (transformed_js_output, js_output) = stylex_define_consts(&value, &mut self.state);

      let result_ast = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
        transformed_js_output,
      ));

      self
        .state
        .register_styles(call, &js_output, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
