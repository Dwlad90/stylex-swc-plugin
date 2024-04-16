use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::utils::stylex::js_to_expr::{convert_object_to_ast, NestedStringObject};
use crate::shared::utils::{
  js::stylex::stylex_create_theme::stylex_create_theme,
  stylex::dev_class_name::convert_theme_to_test_styles,
};
use crate::shared::utils::{
  stylex::dev_class_name::convert_theme_to_dev_styles,
  validators::{
    is_create_theme_call, validate_stylex_create_theme_indent, validate_theme_variables,
  },
};
use crate::shared::{
  constants, structures::functions::FunctionMap, utils::css::stylex::evaluate::evaluate,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create_theme_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_create_theme_call = is_create_theme_call(call, &self.state);

    let result = if is_create_theme_call {
      let (_, parent_var_decl) = &self.get_call_var_name(call);

      validate_stylex_create_theme_indent(parent_var_decl, call, &mut self.state);

      let first_arg = call.args.first();
      let second_arg = call.args.get(1);

      let first_arg = first_arg.and_then(|first_arg| match &first_arg.spread {
        Some(_) => todo!(),
        None => Option::Some(first_arg.expr.clone()),
      })?;

      let second_arg = second_arg.and_then(|second_arg| match &second_arg.spread {
        Some(_) => todo!(),
        None => Option::Some(second_arg.expr.clone()),
      })?;

      // let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> =
      //     IndexMap::new();

      // // let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

      // let mut identifiers: HashMap<Id, FunctionConfig> = HashMap::new();
      // let mut member_expressions: HashMap<
      //     ImportSources,
      //     HashMap<Id, FunctionConfig>,
      // > = HashMap::new();

      // let include_fn = FunctionConfig {
      //     fn_ptr: FunctionType::ArrayArgs(stylex_include),
      //     takes_path: true,
      // };

      // let first_that_works_fn = FunctionConfig {
      //     fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
      //     takes_path: false,
      // };

      // for name in &self.state.stylex_include_import {
      //     identifiers.insert(name.clone(), include_fn.clone());
      // }

      // for name in &self.state.stylex_first_that_works_import {
      //     identifiers.insert(name.clone(), first_that_works_fn.clone());
      // }

      // for name in &self.state.stylex_import {
      //     member_expressions
      //         .entry(name.clone())
      //         .or_insert(HashMap::new());

      //     let member_expression = member_expressions.get_mut(name).unwrap();

      //     member_expression.insert(
      //         Ident::new("include".into(), DUMMY_SP).to_id(),
      //         include_fn.clone(),
      //     );

      //     member_expression.insert(
      //         Ident::new("firstThatWorks".into(), DUMMY_SP).to_id(),
      //         first_that_works_fn.clone(),
      //     );
      // }

      // let function_map: FunctionMap = FunctionMap {
      //     identifiers,
      //     member_expressions,
      // };

      let evaluated_arg1 = evaluate(&first_arg, &mut self.state, &FunctionMap::default());

      assert!(
        evaluated_arg1.confident,
        "{}",
        constants::messages::NON_STATIC_VALUE
      );

      let evaluated_arg2 = evaluate(&second_arg, &mut self.state, &FunctionMap::default());

      assert!(
        evaluated_arg2.confident,
        "{}",
        constants::messages::NON_STATIC_VALUE
      );

      let variables = match evaluated_arg1.value {
        Some(value) => {
          validate_theme_variables(&value);

          value
        }
        None => {
          panic!("Can only override variables theme created with stylex.defineVars().")
        }
      };

      let overrides = match evaluated_arg2.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            constants::messages::NON_OBJECT_FOR_STYLEX_CALL
          );
          value
        }
        None => {
          panic!("{}", constants::messages::NON_OBJECT_FOR_STYLEX_CALL)
        }
      };

      let (mut overrides_obj, inject_styles) =
        stylex_create_theme(&variables, &overrides, &mut self.state);

      let (var_name, _) = self.get_call_var_name(call);

      if self.state.is_test() {
        overrides_obj =
          convert_theme_to_test_styles(&var_name, &overrides_obj, &self.state.get_filename());
      } else if self.state.is_dev() {
        overrides_obj =
          convert_theme_to_dev_styles(&var_name, &overrides_obj, &self.state.get_filename());
      }

      let result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(overrides_obj));

      self
        .state
        .register_styles(call, &inject_styles, &result_ast, &var_name);

      return Option::Some(result_ast);
    } else {
      None
    };

    dbg!(&result);

    result
  }
}
