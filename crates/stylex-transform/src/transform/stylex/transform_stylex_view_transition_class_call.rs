use std::rc::Rc;
use stylex_constants::constants::messages::{SPREAD_NOT_SUPPORTED, expected_call_expression};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::ecma::ast::VarDeclarator;
use swc_core::{common::comments::Comments, ecma::ast::Expr};

use crate::StyleXTransform;
use crate::shared::structures::functions::FunctionConfigType;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::state_manager::ImportKind;
use crate::shared::structures::types::{FunctionMapIdentifiers, FunctionMapMemberExpression};
use crate::shared::transformers::stylex_first_that_works::stylex_first_that_works;
use crate::shared::transformers::stylex_keyframes::get_keyframes_fn;
use crate::shared::transformers::stylex_view_transition_class::stylex_view_transition_class;
use crate::shared::utils::ast::convertors::create_string_expr;
use crate::shared::utils::js::evaluate::evaluate;
use crate::shared::utils::log::build_code_frame_error::build_code_frame_error;
use crate::shared::utils::validators::assert_valid_properties;
use crate::shared::utils::validators::assert_valid_view_transition_class;
use crate::shared::utils::validators::{
  is_view_transition_class_call, validate_stylex_view_transition_class_indent,
};
use stylex_constants::constants::common::VALID_VIEW_TRANSITION_CLASS_PROPERTIES;
use stylex_constants::constants::messages::{
  VIEW_TRANSITION_CLASS_INVALID_PROPERTY, non_static_value, non_style_object,
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_view_transition_class_call(
    &mut self,
    var_decl: &VarDeclarator,
  ) -> Option<Expr> {
    let is_view_transition_class_call = is_view_transition_class_call(var_decl, &self.state);

    if is_view_transition_class_call {
      validate_stylex_view_transition_class_indent(var_decl, &mut self.state);

      let call = match var_decl.init.as_ref().and_then(|decl| decl.as_call()) {
        Some(call) => call,
        None => stylex_panic!("{}", expected_call_expression("viewTransitionClass")),
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

      let keyframes_fn = get_keyframes_fn();

      if let Some(set) = self.state.get_import(ImportKind::FirstThatWorks) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_import(ImportKind::Keyframes) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
          );
        }
      }

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          "firstThatWorks".into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
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
          &non_static_value("viewTransitionClass"),
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
              &non_style_object("viewTransitionClass"),
              &mut self.state,
            )
          );
          value
        },
        None => stylex_panic!(
          "{}",
          build_code_frame_error(
            &Expr::Call(call.clone()),
            &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
            &non_static_value("viewTransitionClass"),
            &mut self.state,
          )
        ),
      };

      assert_valid_view_transition_class(&plain_object, &mut self.state);
      assert_valid_properties(
        &plain_object,
        &*VALID_VIEW_TRANSITION_CLASS_PROPERTIES,
        VIEW_TRANSITION_CLASS_INVALID_PROPERTY,
        &mut self.state,
      );

      let (view_transition_class_name, injectable_style) =
        stylex_view_transition_class(&plain_object, &mut self.state);

      let mut injected_styles = IndexMap::new();

      injected_styles.insert(
        view_transition_class_name.clone(),
        Rc::new(injectable_style),
      );

      let other_injected_css_rules: IndexMap<
        String,
        Rc<stylex_types::enums::data_structures::injectable_style::InjectableStyleKind>,
      > = self.state.other_injected_css_rules.clone();

      injected_styles.extend(other_injected_css_rules);

      let result_ast = create_string_expr(view_transition_class_name.as_str());

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}
