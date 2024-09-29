use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{ArrowExpr, BlockStmtOrExpr, ExprOrSpread, Pat, PropName};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr, PropOrSpread},
};

use crate::shared::utils::validators::{is_create_call, validate_stylex_create};
use crate::shared::utils::{
  ast::factories::array_expression_factory,
  core::js_to_expr::{convert_object_to_ast, remove_objects_with_spreads, NestedStringObject},
};
use crate::shared::utils::{
  ast::factories::object_expression_factory,
  common::{get_key_str, get_key_values_from_object},
};
use crate::shared::{
  constants::messages::NON_STATIC_VALUE,
  utils::core::dev_class_name::{convert_to_test_styles, inject_dev_class_names},
};
use crate::shared::{
  structures::functions::{FunctionConfig, FunctionMap, FunctionType},
  transformers::{
    stylex_create::stylex_create_set, stylex_first_that_works::stylex_first_that_works,
    stylex_include::stylex_include, stylex_keyframes::get_keyframes_fn,
  },
};
use crate::shared::{
  structures::types::{FlatCompiledStyles, FunctionMapMemberExpression},
  utils::core::evaluate_stylex_create_arg::evaluate_stylex_create_arg,
};
use crate::shared::{
  structures::{functions::FunctionConfigType, types::FunctionMapIdentifiers},
  utils::ast::factories::prop_or_spread_expression_factory,
};
use crate::StyleXTransform;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
    self.state.in_stylex_create = true;
    let is_create_call = is_create_call(call, &self.state);

    let result = if is_create_call {
      validate_stylex_create(call, &mut self.state);

      let first_arg = call.args.first();

      let mut first_arg = first_arg.map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!(),
        None => first_arg.expr.clone(),
      })?;

      let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();

      let mut identifiers: FunctionMapIdentifiers = HashMap::new();
      let mut member_expressions: FunctionMapMemberExpression = HashMap::new();

      let include_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_include),
        takes_path: true,
      };

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      let keyframes_fn = get_keyframes_fn();

      for name in &self.state.stylex_include_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(include_fn.clone())),
        );
      }

      for name in &self.state.stylex_first_that_works_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
      }

      for name in &self.state.stylex_keyframes_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        member_expressions.entry(name.clone()).or_default();

        let member_expression = member_expressions.get_mut(name).unwrap();

        member_expression.insert(
          "include".into(),
          Box::new(FunctionConfigType::Regular(include_fn.clone())),
        );

        member_expression.insert(
          "firstThatWorks".into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg =
        evaluate_stylex_create_arg(&mut first_arg, &mut self.state, &function_map);

      let value = match evaluated_arg.value {
        Some(value) => value,
        None => {
          panic!("{}", NON_STATIC_VALUE)
        }
      };

      assert!(evaluated_arg.confident, "{}", NON_STATIC_VALUE);

      let (mut compiled_styles, injected_styles_sans_keyframes) =
        stylex_create_set(&value, &mut self.state, &function_map);

      for (namespace, properties) in compiled_styles.iter() {
        resolved_namespaces
          .entry(namespace.clone())
          .or_default()
          .extend(*properties.clone());
      }

      let mut injected_styles = self.state.injected_keyframes.clone();

      injected_styles.extend(injected_styles_sans_keyframes);

      let (var_name, parent_var_decl) = &self.get_call_var_name(call);

      if self.state.is_test() {
        compiled_styles = convert_to_test_styles(&compiled_styles, var_name, &self.state);
      }

      if self.state.is_dev() {
        compiled_styles = inject_dev_class_names(&compiled_styles, var_name, &self.state);
      }

      if let Some(var_name) = var_name.as_ref() {
        let styles_to_remember = Box::new(remove_objects_with_spreads(&compiled_styles));

        self
          .state
          .style_map
          .insert(var_name.clone(), styles_to_remember);

        self
          .state
          .style_vars
          .insert(var_name.clone(), parent_var_decl.clone().unwrap());
      }

      let mut result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStyles(compiled_styles));

      if let Some(fns) = evaluated_arg.fns {
        if let Some(object) = result_ast.as_object() {
          let key_values = get_key_values_from_object(object);

          let props = key_values
            .iter()
            .map(|key_value| {
              let orig_key = get_key_str(key_value);
              let value = key_value.value.clone();

              let key = match &key_value.key {
                PropName::Ident(ident) => Some(ident.sym.to_string()),
                PropName::Str(strng) => Some(strng.value.to_string()),
                _ => None,
              };

              let mut prop: Option<PropOrSpread> = None;

              if let Some(key) = key {
                if let Some((params, inline_styles)) = fns.get(&key) {
                  let value = Expr::from(ArrowExpr {
                    span: DUMMY_SP,
                    params: params.iter().map(|arg| Pat::Ident(arg.clone())).collect(),
                    body: Box::new(BlockStmtOrExpr::from(Box::new(array_expression_factory(
                      vec![
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(*value.clone()),
                        }),
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(object_expression_factory(
                            inline_styles
                              .iter()
                              .map(|(key, value)| {
                                prop_or_spread_expression_factory(key.as_str(), *value.clone())
                              })
                              .collect(),
                          )),
                        }),
                      ],
                    )))),
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                  });

                  prop = Some(prop_or_spread_expression_factory(orig_key.as_str(), value));
                }
              }

              let prop =
                prop.unwrap_or(prop_or_spread_expression_factory(orig_key.as_str(), *value));

              prop
            })
            .collect::<Vec<PropOrSpread>>();

          result_ast = object_expression_factory(props);
        }
      };

      self
        .state
        .register_styles(call, &injected_styles, &result_ast);

      Some(result_ast)
    } else {
      None
    };

    self.state.in_stylex_create = false;

    result
  }
}
