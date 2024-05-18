use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{
  ArrayLit, ArrowExpr, BlockStmtOrExpr, ExprOrSpread, Id, Ident, ObjectLit, Pat, PropName,
};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr, PropOrSpread},
};

use crate::shared::structures::flat_compiled_styles::FlatCompiledStyles;
use crate::shared::structures::functions::FunctionConfigType;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::{
  get_key_str, get_key_values_from_object, prop_or_spread_expression_creator,
};
use crate::shared::utils::css::factories::object_expression_factory;
use crate::shared::utils::css::stylex::evaluate_stylex_create_arg::evaluate_stylex_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::stylex::dev_class_name::{
  convert_to_test_styles, inject_dev_class_names,
};
use crate::shared::utils::stylex::js_to_expr::{
  convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::{is_create_call, validate_stylex_create};
use crate::shared::{constants, utils::js::stylex::stylex_keyframes::get_keyframes_fn};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
    self.state.in_stylex_create = true;
    let is_create_call = is_create_call(call, &self.state);

    let result = if is_create_call {
      validate_stylex_create(call, &mut self.state);

      let first_arg = call.args.first();

      let first_arg = first_arg.and_then(|first_arg| match &first_arg.spread {
        Some(_) => todo!(),
        None => Option::Some(first_arg.expr.clone()),
      })?;

      let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();

      let mut identifiers: HashMap<Box<Id>, Box<FunctionConfigType>> = HashMap::new();
      let mut member_expressions: HashMap<
        Box<ImportSources>,
        Box<HashMap<Box<Id>, Box<FunctionConfigType>>>,
      > = HashMap::new();

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
          Box::new(Ident::new("include".into(), DUMMY_SP).to_id()),
          Box::new(FunctionConfigType::Regular(include_fn.clone())),
        );

        member_expression.insert(
          Box::new(Ident::new("firstThatWorks".into(), DUMMY_SP).to_id()),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          Box::new(Ident::new("keyframes".into(), DUMMY_SP).to_id()),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg = evaluate_stylex_create_arg(&first_arg, &mut self.state, &function_map);

      // println!("!!!evaluated_arg.value: {:?}\n\n",evaluated_arg.value);

      let value = match evaluated_arg.value {
        Some(value) => value,
        None => {
          panic!("{}", constants::messages::NON_STATIC_VALUE)
        }
      };

      assert!(
        evaluated_arg.confident,
        "{}",
        constants::messages::NON_STATIC_VALUE
      );

      let (mut compiled_styles, injected_styles_sans_keyframes) =
        stylex_create_set(&value, &mut self.state, &function_map);

      // dbg!(&compiled_styles, &injected_styles_sans_keyframes);

      compiled_styles
        .clone()
        .into_iter()
        .for_each(|(namespace, properties)| {
          resolved_namespaces
            .entry(namespace)
            .or_default()
            .extend(*properties);
        });

      let mut injected_styles = self.state.injected_keyframes.clone();

      injected_styles.extend(injected_styles_sans_keyframes);
      // dbg!(&injected_styles);

      let (var_name, parent_var_decl) = &self.get_call_var_name(call);

      if self.state.is_test() {
        compiled_styles = convert_to_test_styles(&compiled_styles, var_name, &self.state);
      }

      if self.state.is_dev() {
        compiled_styles = inject_dev_class_names(&compiled_styles, var_name, &self.state);
      }

      if let Option::Some(var_name) = var_name.clone() {
        let styles_to_remember = Box::new(remove_objects_with_spreads(&compiled_styles));

        self
          .state
          .style_map
          .insert(var_name.clone(), styles_to_remember);

        self
          .state
          .style_vars
          .insert(var_name.clone(), parent_var_decl.clone().unwrap().clone());
      }

      let mut result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStyles(compiled_styles));

      // println!("result_ast: {:?}", result_ast);

      if let Some(fns) = evaluated_arg.fns {
        if let Some(object) = result_ast.as_object() {
          let key_values = get_key_values_from_object(object);

          let props = key_values
            .clone()
            .into_iter()
            .map(|key_value| {
              let orig_key = get_key_str(&key_value);
              let value = key_value.value.clone();

              let key = match &key_value.key {
                PropName::Ident(ident) => Some(ident.sym.as_ref().to_string()),
                PropName::Str(str) => Some(str.value.as_ref().to_string()),
                _ => None,
              };

              let mut prop: Option<PropOrSpread> = Option::None;

              if let Some(key) = key {
                if let Some((params, inline_styles)) = fns.get(&key) {
                  // dbg!(&value);
                  let value = Expr::Arrow(ArrowExpr {
                    span: DUMMY_SP,
                    params: params.clone().into_iter().map(Pat::Ident).collect(), // replace with your parameters
                    body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Array(ArrayLit {
                      span: DUMMY_SP,
                      elems: vec![
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(value.as_ref().clone()),
                        }),
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: inline_styles // replace with your inline_styles
                              .iter()
                              .map(|(key, value)| {
                                prop_or_spread_expression_creator(key.as_str(), value.clone())
                              })
                              .collect(),
                          })),
                        }),
                      ],
                    })))),
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                  });

                  prop = Option::Some(prop_or_spread_expression_creator(
                    orig_key.as_str(),
                    Box::new(value),
                  ));
                }
              }

              let prop =
                prop.unwrap_or(prop_or_spread_expression_creator(orig_key.as_str(), value));

              prop
            })
            .collect::<Vec<PropOrSpread>>();

          result_ast = object_expression_factory(props).unwrap_or(result_ast);
        }
        //// dbg!(&result_ast);
      };

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, var_name);

      Option::Some(result_ast)
    } else {
      None
    };

    self.state.in_stylex_create = false;

    result
  }
}
