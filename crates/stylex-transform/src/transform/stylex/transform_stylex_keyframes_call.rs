use stylex_constants::constants::messages::{SPREAD_NOT_SUPPORTED, expected_call_expression};
use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::ecma::ast::VarDeclarator;
use swc_core::{common::comments::Comments, ecma::ast::Expr};

use crate::StyleXTransform;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::types::{FunctionMapIdentifiers, FunctionMapMemberExpression};
use crate::shared::utils::ast::convertors::create_string_expr;
use crate::shared::utils::validators::{
  assert_valid_keyframes,
  is_keyframes_call,
  validate_stylex_keyframes_indent,
};
use stylex_constants::constants::messages::{non_static_value, non_style_object};
use crate::shared::transformers::stylex_first_that_works::stylex_first_that_works;
use crate::shared::structures::functions::FunctionConfigType;
use crate::shared::utils::log::build_code_frame_error::build_code_frame_error;
use crate::shared::transformers::stylex_keyframes::stylex_keyframes;
use crate::shared::utils::js::evaluate::evaluate;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_keyframes_call(
    &mut self,
    var_decl: &VarDeclarator,
  ) -> Option<Expr> {
    let is_keyframes_call = is_keyframes_call(var_decl, &self.state);

    if is_keyframes_call {
      validate_stylex_keyframes_indent(var_decl, &mut self.state);

      let call = match var_decl.init.as_ref().and_then(|decl| decl.as_call()) {
        Some(call) => call,
        None => stylex_panic!("{}", expected_call_expression("keyframes")),
      };

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      for name in &self.state.stylex_first_that_works_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          "firstThatWorks".into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
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
          &non_static_value("keyframes"),
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
              &non_style_object("keyframes"),
              &mut self.state,
            )
          );
          value
        }
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
            &non_static_value("keyframes"),
            &mut self.state,
          )
        ),
      };

      assert_valid_keyframes(&value, &mut self.state);

      let (animation_name, injectable_style) = stylex_keyframes(&value, &mut self.state);

      let mut injected_styles = IndexMap::new();

      injected_styles.insert(animation_name.clone(), Rc::new(injectable_style));

      let result_ast = create_string_expr(animation_name.as_str());

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
