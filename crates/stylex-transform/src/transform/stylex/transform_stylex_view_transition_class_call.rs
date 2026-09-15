use std::rc::Rc;
use stylex_constants::constants::messages::expected_call_expression;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_ast::ast::convertors::create_string_expr;
use stylex_macros::stylex_panic;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{Expr, VarDeclarator},
};

use crate::{
  StyleXTransform,
  shared::{
    transformers::{
      stylex_keyframes::get_keyframes_fn,
      stylex_view_transition_class::stylex_view_transition_class,
    },
    utils::validators::{
      argument_at, assert_valid_properties, assert_valid_view_transition_class,
      folded_style_object, is_view_transition_class_call,
      validate_stylex_view_transition_class_indent,
    },
  },
};
use stylex_ast::ast::convertors::init_call;
use stylex_constants::constants::{
  api_names::{STYLEX_FIRST_THAT_WORKS, STYLEX_KEYFRAMES, STYLEX_VIEW_TRANSITION_CLASS},
  common::VALID_VIEW_TRANSITION_CLASS_PROPERTIES,
  messages::VIEW_TRANSITION_CLASS_INVALID_PROPERTY,
};
use stylex_evaluator::{evaluate::evaluate, stylex_first_that_works::stylex_first_that_works};
use stylex_state::{
  functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
  state_manager::ImportKind,
  types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
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

      let call = match init_call(var_decl) {
        Some(call) => call,
        None => stylex_panic!("{}", expected_call_expression(STYLEX_VIEW_TRANSITION_CLASS)),
      };

      let first_arg = argument_at(call, 0, STYLEX_VIEW_TRANSITION_CLASS);

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      let keyframes_fn = get_keyframes_fn();

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::FirstThatWorks) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Keyframes) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
          );
        }
      }

      for name in self.state.stylex_imports() {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          STYLEX_FIRST_THAT_WORKS.into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          STYLEX_KEYFRAMES.into(),
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

      let evaluated_arg = evaluate(first_arg, &mut self.state, &function_map);

      let plain_object = folded_style_object(
        evaluated_arg,
        call,
        first_arg,
        STYLEX_VIEW_TRANSITION_CLASS,
        &mut self.state,
      );

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
        view_transition_class_name.clone().into(),
        Rc::new(injectable_style),
      );

      let other_injected_css_rules = self.state.other_injected_css_rules.clone();

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
