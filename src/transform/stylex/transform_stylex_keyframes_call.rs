use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident, KeyValueProp, VarDeclarator};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Expr, Prop, PropOrSpread},
};

use crate::shared::constants;
use crate::shared::enums::TopLevelExpression;
use crate::shared::structures::evaluate_result::EvaluateResultValue;
use crate::shared::structures::flat_compiled_styles::FlatCompiledStyles;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::injectable_style::{self, InjectableStyle};
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::{prop_or_spread_string_creator, string_to_expression};
use crate::shared::utils::css::factories::object_expression_factory;
use crate::shared::utils::css::stylex::evaluate::evaluate;
use crate::shared::utils::css::stylex::evaluate_stylex_create_arg::evaluate_stylex_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::js::stylex::stylex_keyframes::stylex_keyframes;
use crate::shared::utils::stylex::dev_class_name::{
    convert_to_test_styles, inject_dev_class_names,
};
use crate::shared::utils::stylex::js_to_expr::{
    convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::{
    assert_valid_keyframes, is_create_call, is_keyframes_call, validate_namespace,
    validate_stylex_create_indent, validate_stylex_keyframes_indent,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_keyframes_call(
        &mut self,
        var_decl: &VarDeclarator,
    ) -> Option<Expr> {
        let is_keyframes_call = is_keyframes_call(var_decl, &mut self.state);

        let result = if is_keyframes_call {
            validate_stylex_keyframes_indent(var_decl, &mut self.state);

            let call = &var_decl.init.clone().unwrap().call().unwrap();

            let first_arg = call.args.get(0);

            let Some(first_arg) = first_arg.and_then(|first_arg| match &first_arg.spread {
                Some(_) => todo!(),
                None => Option::Some(first_arg.expr.clone()),
            }) else {
                return Option::None;
            };

            let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> = IndexMap::new();

            // let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

            let mut identifiers: HashMap<Id, FunctionConfig> = HashMap::new();
            let mut member_expressions: HashMap<ImportSources, HashMap<Id, FunctionConfig>> =
                HashMap::new();

            let include_fn = FunctionConfig {
                fn_ptr: FunctionType::ArrayArgs(stylex_include),
                takes_path: true,
            };

            let first_that_works_fn = FunctionConfig {
                fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
                takes_path: false,
            };

            for name in &self.state.stylex_include_import {
                identifiers.insert(name.clone(), include_fn.clone());
            }

            for name in &self.state.stylex_first_that_works_import {
                identifiers.insert(name.clone(), first_that_works_fn.clone());
            }

            for name in &self.state.stylex_import {
                member_expressions
                    .entry(name.clone())
                    .or_insert(HashMap::new());

                let member_expression = member_expressions.get_mut(name).unwrap();

                member_expression.insert(
                    Ident::new("include".into(), DUMMY_SP).to_id(),
                    include_fn.clone(),
                );

                member_expression.insert(
                    Ident::new("firstThatWorks".into(), DUMMY_SP).to_id(),
                    first_that_works_fn.clone(),
                );
            }

            let function_map: FunctionMap = FunctionMap {
                identifiers,
                member_expressions,
            };

            let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

            dbg!(evaluated_arg.clone());

            assert!(
                evaluated_arg.confident,
                "{}",
                constants::messages::NON_STATIC_VALUE
            );

            let value = match evaluated_arg.value {
                Some(value) => {
                    assert!(
                        value
                            .as_expr()
                            .and_then(|expr| Option::Some(expr.is_object()))
                            .unwrap_or(false),
                        "{}",
                        constants::messages::NON_OBJECT_FOR_STYLEX_CALL
                    );
                    value
                }
                None => {
                    panic!("{}", constants::messages::NON_STATIC_VALUE)
                }
            };

            let plain_object = value;

            assert_valid_keyframes(&plain_object);

            let (animation_name, injectable_style) =
                stylex_keyframes(&plain_object, &mut self.state);

            // compiled_styles
            //     .clone()
            //     .into_iter()
            //     .for_each(|(namespace, properties)| {
            //         resolved_namespaces
            //             .entry(namespace)
            //             .or_default()
            //             .extend(properties);
            //     });

            // let mut injected_styles = injected_keyframes.clone();
            // injected_styles.extend(injected_styles_sans_keyframes);

            let (var_name, parent_var_decl) = &self.get_call_var_name(call);

            // if self.state.is_test() {
            //     compiled_styles = convert_to_test_styles(
            //         &compiled_styles,
            //         &var_name,
            //         &mut self.state,
            //     );
            // }

            // if self.state.is_dev() {
            //     compiled_styles = inject_dev_class_names(
            //         &compiled_styles,
            //         &var_name,
            //         &mut self.state,
            //     );
            // }

            // if let Option::Some(var_name) = var_name.clone() {
            //     let styles_to_remember = remove_objects_with_spreads(&compiled_styles);

            //     self.state
            //         .style_map
            //         .insert(var_name.clone(), styles_to_remember);

            //     self.state
            //         .style_vars
            //         .insert(var_name.clone(), parent_var_decl.clone().unwrap().clone());
            // }

            let mut injected_styles = IndexMap::new();

            injected_styles.insert(animation_name.clone(), injectable_style);

            let result_ast = string_to_expression(animation_name);

            self.state.register_styles(
                call,
                &injected_styles,
                &result_ast.clone().expect("No result ast"),
                &var_name,
            );

            result_ast
        } else {
            None
        };

        dbg!(&result);

        result
    }
}
