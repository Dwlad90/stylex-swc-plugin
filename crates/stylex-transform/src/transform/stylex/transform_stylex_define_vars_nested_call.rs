use rustc_hash::FxHashMap;
use stylex_constants::constants::{
  api_names::STYLEX_UNSTABLE_DEFINE_VARS_NESTED,
  messages::{
    SPREAD_NOT_SUPPORTED, cannot_generate_hash, export_variable_not_found, non_static_value,
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
    structures::state_manager::ImportKind,
    transformers::stylex_define_vars_nested::stylex_define_vars_nested,
    utils::{
      common::gen_file_based_identifier,
      core::stylex_nested_utils::convert_unflattened_object_to_ast, js::evaluate::evaluate,
      log::build_code_frame_error::build_code_frame_error, validators::validate_define_call,
    },
  },
  transform::stylex::visitor_utils::{build_eval_config, is_call_to},
};
use stylex_structures::top_level_expression::TopLevelExpression;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars_nested(&mut self, call: &CallExpr) -> Option<Expr> {
    if !is_call_to(
      call,
      &self.state,
      ImportKind::DefineVarsNested,
      STYLEX_UNSTABLE_DEFINE_VARS_NESTED,
    ) {
      return None;
    }

    let top_level_expr = validate_define_call(
      call,
      STYLEX_UNSTABLE_DEFINE_VARS_NESTED,
      1,
      true,
      &mut self.state,
    );
    let TopLevelExpression(_, _, var_id) = top_level_expr;

    let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
      Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
      None => first_arg.expr.clone(),
    })?;

    let function_map = build_eval_config(&mut self.state);
    let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

    if !evaluated_arg.confident {
      let deopt = evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned());
      stylex_panic!(
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &deopt,
          &non_static_value(STYLEX_UNSTABLE_DEFINE_VARS_NESTED),
          &mut self.state,
        )
      );
    }

    let value = match evaluated_arg.value {
      Some(value) => {
        let is_object = value
          .as_expr()
          .map(|expr| expr.is_object())
          .unwrap_or(false);

        if !is_object {
          let deopt = evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned());
          stylex_panic!(
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &deopt,
              &non_style_object(STYLEX_UNSTABLE_DEFINE_VARS_NESTED),
              &mut self.state,
            )
          );
        }

        value
      },
      None => stylex_panic!("{}", non_static_value(STYLEX_UNSTABLE_DEFINE_VARS_NESTED)),
    };

    let file_name = match self
      .state
      .get_filename_for_hashing(&mut FxHashMap::default())
    {
      Some(name) => name,
      None => stylex_panic!(
        "{}",
        cannot_generate_hash(STYLEX_UNSTABLE_DEFINE_VARS_NESTED)
      ),
    };

    let export_name = match var_id.map(|decl| decl.to_string()) {
      Some(name) => name,
      None => stylex_panic!(
        "{}",
        export_variable_not_found(STYLEX_UNSTABLE_DEFINE_VARS_NESTED)
      ),
    };

    self.state.export_id = Some(gen_file_based_identifier(&file_name, &export_name, None));

    let (variables_obj, injected_styles_sans_keyframes) =
      stylex_define_vars_nested(&value, &mut self.state);

    let mut injected_styles = self.state.other_injected_css_rules.clone();
    injected_styles.extend(injected_styles_sans_keyframes);

    let result_ast = convert_unflattened_object_to_ast(&variables_obj);

    self
      .state
      .register_styles(call, &injected_styles, &result_ast, None);

    Some(result_ast)
  }
}
