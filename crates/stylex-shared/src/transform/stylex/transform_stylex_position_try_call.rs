use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::ecma::ast::VarDeclarator;
use swc_core::{common::comments::Comments, ecma::ast::Expr};

use crate::shared::{
  constants::messages::{non_static_value, non_style_object},
  transformers::stylex_first_that_works::stylex_first_that_works,
};
use crate::shared::{
  constants::{common::VALID_POSITION_TRY_PROPERTIES, messages::POSITION_TRY_INVALID_PROPERTY},
  utils::js::evaluate::evaluate,
};
use crate::shared::{
  structures::functions::FunctionConfigType,
  utils::log::build_code_frame_error::build_code_frame_error,
};
use crate::shared::{
  structures::{
    functions::{FunctionConfig, FunctionMap, FunctionType},
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  utils::validators::validate_stylex_position_try_indent,
};
use crate::shared::{
  transformers::stylex_position_try::stylex_position_try,
  utils::{
    ast::convertors::string_to_expression,
    validators::{assert_valid_position_try, assert_valid_properties},
  },
};
use crate::{StyleXTransform, shared::utils::validators::is_position_try_call};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_position_try_call(
    &mut self,
    var_decl: &VarDeclarator,
  ) -> Option<Expr> {
    let is_position_try_call = is_position_try_call(var_decl, &self.state);

    if is_position_try_call {
      validate_stylex_position_try_indent(var_decl, &mut self.state);

      let call = var_decl
        .init
        .as_ref()
        .and_then(|decl| decl.as_call())
        .expect("Expected call expression");

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!("Spread"),
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
          &non_static_value("positionTry"),
          &mut self.state,
        )
      );

      let plain_object = match evaluated_arg.value {
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
              &non_style_object("positionTry"),
              &mut self.state,
            )
          );
          value
        }
        None => panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
            &non_static_value("positionTry"),
            &mut self.state,
          )
        ),
      };

      assert_valid_position_try(&plain_object, &mut self.state);
      assert_valid_properties(
        &plain_object,
        &*VALID_POSITION_TRY_PROPERTIES,
        POSITION_TRY_INVALID_PROPERTY,
        &mut self.state,
      );

      let (position_try_name, injectable_style) =
        stylex_position_try(&plain_object, &mut self.state);

      let mut injected_styles = IndexMap::new();

      injected_styles.insert(position_try_name.clone(), Rc::new(injectable_style));

      let result_ast = string_to_expression(position_try_name.as_str());

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
