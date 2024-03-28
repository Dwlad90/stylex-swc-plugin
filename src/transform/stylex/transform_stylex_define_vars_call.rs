use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Expr},
};

use crate::shared::constants;
use crate::shared::enums::TopLevelExpression;
use crate::shared::structures::flat_compiled_styles::FlatCompiledStyles;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::injectable_style::InjectableStyle;
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::css::stylex::evaluate_style_x_create_arg::evaluate_style_x_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::stylex::dev_class_name::{
    convert_to_test_styles, inject_dev_class_names,
};
use crate::shared::utils::stylex::js_to_expr::{
    convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::{
    is_create_call, is_define_vars_call, validate_style_x_create_indent,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
        self.state.in_style_x_create = true;
        let is_define_vars = is_define_vars_call(call, &mut self.state);

        let result = if is_define_vars {
            validate_style_x_create_indent(call, &mut self.state);

            let first_arg = call.args.get(0);

            match first_arg {
                Some(first_arg) => match &first_arg.spread {
                    Some(_) => todo!(),
                    None => {
                        let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> =
                            IndexMap::new();

                        let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

                        let mut identifiers: HashMap<Id, FunctionConfig> = HashMap::new();
                        let mut member_expressions: HashMap<
                            ImportSources,
                            HashMap<Id, FunctionConfig>,
                        > = HashMap::new();

                        let include_fn = FunctionConfig {
                            fn_ptr: FunctionType::ArrayArgs(stylex_include),
                            takes_path: true,
                        };

                        let first_that_works_fn = FunctionConfig {
                            fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
                            takes_path: false,
                        };

                        let keyframes_fn = FunctionConfig {
                            fn_ptr: FunctionType::OneArg(|_arg| {
                                panic!("Keyframes not implemented")
                            }),
                            takes_path: false,
                        };

                        for name in &self.state.stylex_include_import {
                            identifiers.insert(name.clone(), include_fn.clone());
                        }

                        for name in &self.state.stylex_first_that_works_import {
                            identifiers.insert(name.clone(), first_that_works_fn.clone());
                        }

                        for name in &self.state.stylex_keyframes_import {
                            identifiers.insert(name.clone(), keyframes_fn.clone());
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

                            member_expression.insert(
                                Ident::new("keyframes".into(), DUMMY_SP).to_id(),
                                keyframes_fn.clone(),
                            );
                        }

                        let function_map: FunctionMap = FunctionMap {
                            identifiers,
                            member_expressions,
                        };

                        let evaluated_arg = evaluate_style_x_create_arg(
                            &first_arg.expr,
                            &mut self.state,
                            &function_map,
                        );

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

                        compiled_styles
                            .clone()
                            .into_iter()
                            .for_each(|(namespace, properties)| {
                                resolved_namespaces
                                    .entry(namespace)
                                    .or_default()
                                    .extend(properties);
                            });

                        let mut injected_styles = injected_keyframes.clone();
                        injected_styles.extend(injected_styles_sans_keyframes);

                        let mut var_name: Option<String> = Option::None;

                        let parent_var_decl =
                            &self.state.declarations.clone().into_iter().find(|decl| {
                                decl.init
                                    .as_ref()
                                    .unwrap()
                                    .eq(&Box::new(Expr::Call(call.clone())))
                            });

                        if let Some(parent_var_decl) = &parent_var_decl {
                            if let Some(ident) = parent_var_decl.name.as_ident() {
                                var_name = Option::Some(ident.sym.clone().to_string());
                            }
                        }

                        if self.state.is_test() {
                            compiled_styles = convert_to_test_styles(
                                &compiled_styles,
                                &var_name,
                                &mut self.state,
                            );
                        }

                        if self.state.is_dev() {
                            compiled_styles = inject_dev_class_names(
                                &compiled_styles,
                                &var_name,
                                &mut self.state,
                            );
                        }

                        if let Option::Some(var_name) = var_name.clone() {
                            let styles_to_remember = remove_objects_with_spreads(&compiled_styles);

                            self.state
                                .style_map
                                .insert(var_name.clone(), styles_to_remember);

                            self.state
                                .style_vars
                                .insert(var_name.clone(), parent_var_decl.clone()?.clone());
                        }

                        let result_ast = convert_object_to_ast(
                            &NestedStringObject::FlatCompiledStyles(compiled_styles),
                        );

                        let metadatas = MetaData::convert_from_injected_styles_map(injected_styles);

                        for metadata in metadatas {
                            dbg!(&metadata);
                            self.push_to_css_output(
                                var_name.clone().unwrap_or("default".to_string()),
                                metadata,
                            );
                        }

                        dbg!(&self.state.declarations.len());
                        if let Some(item) = self.state.declarations.iter_mut().find(|decl| {
                            decl.init
                                .as_ref()
                                .unwrap()
                                .eq(&Box::new(Expr::Call(call.clone())))
                        }) {
                            item.init = Option::Some(Box::new(result_ast.clone()));
                        };

                        if let Some((_, item)) =
                            self.state.style_vars.iter_mut().find(|(_, decl)| {
                                decl.init
                                    .as_ref()
                                    .unwrap()
                                    .eq(&Box::new(Expr::Call(call.clone())))
                            })
                        {
                            item.init = Option::Some(Box::new(result_ast.clone()));
                        };

                        if let Some(TopLevelExpression(_, item)) =
                            self.state.top_level_expressions.iter_mut().find(
                                |TopLevelExpression(_, decl)| decl.eq(&Expr::Call(call.clone())),
                            )
                        {
                            *item = result_ast.clone();
                        };

                        Option::Some(result_ast)
                    }
                },
                None => Option::None,
            }
        } else {
            Option::Some(Expr::Call(call.clone()))
        };

        self.state.in_style_x_create = false;

        result
    }
}
