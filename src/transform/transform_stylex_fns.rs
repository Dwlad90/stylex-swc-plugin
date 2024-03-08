use core::panic;
use std::collections::HashMap;

use colored::Colorize;
use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident, KeyValueProp, Lit, Null};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, MemberProp, Prop, PropOrSpread},
};

use crate::shared::constants;
use crate::shared::enums::ModuleCycle;
use crate::shared::structures::evaluate_result::EvaluateResultValue;
use crate::shared::structures::flat_compiled_styles::{
    FlatCompiledStyles, FlatCompiledStylesValue,
};
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType, Functions};
use crate::shared::structures::injectable_style::InjectableStyle;
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::structures::state_manager::StateManager;
use crate::shared::utils::common::{
    prop_or_spread_expression_creator, prop_or_spread_string_creator,
};
use crate::shared::utils::css::factories::object_expression_factory;
use crate::shared::utils::css::stylex::evaluate_style_x_create_arg::evaluate_style_x_create_arg;
use crate::shared::utils::js::stylex::stylex_create::stylex_create_set;
use crate::shared::utils::js::stylex::stylex_first_that_works::stylex_first_that_works;
use crate::shared::utils::js::stylex::stylex_include::stylex_include;
use crate::shared::utils::stylex::js_to_expr::{
    convert_object_to_ast, remove_objects_with_spreads, NestedStringObject,
};
use crate::shared::utils::validators::validate_namespace;
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    // pub(crate) fn transform_call_expression_to_css_map_expr(
    //     &mut self,
    //     ex: &CallExpr,
    // ) -> Option<Expr> {
    //     if let Callee::Expr(callee) = &ex.callee {
    //         if let Expr::Member(member) = callee.as_ref() {
    //             match member.prop.clone() {
    //                 MemberProp::Ident(ident) => {
    //                     match format!("{}", ident.sym).as_str() {
    //                         "create" => {
    //                             if let Some(value) = self.proccess_create_css_call(ex) {
    //                                 return Option::Some(value);
    //                             }
    //                         }
    //                         "props" => {
    //                             return Option::Some(Expr::Ident(Ident::new(
    //                                 "_stylex$props".into(),
    //                                 DUMMY_SP,
    //                             )));
    //                         }
    //                         _ => {}
    //                     };
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }

    //     return Option::None;
    // }

    pub(crate) fn transform_call_expression_to_stylex_expr(
        &mut self,
        ex: &CallExpr,
    ) -> Option<Expr> {
        if let Callee::Expr(callee) = &ex.callee {
            match callee.as_ref() {
                Expr::Member(member) => match member.prop.clone() {
                    MemberProp::Ident(ident) => {
                        if let Some(value) = self.transform_styles_ident_create(ident, ex) {
                            return value;
                        }
                    }
                    _ => {}
                },
                Expr::Ident(ident) => {
                    if let Some(value) = self.transform_styles_ident_create(ident.clone(), ex) {
                        return value;
                    }
                }
                _ => {}
            }
        }

        Option::None
    }

    fn transform_styles_ident_create(
        &mut self,
        ident: Ident,
        ex: &CallExpr,
    ) -> Option<Option<Expr>> {
        if self.cycle == ModuleCycle::TransformEnter {
            if self.state.stylex_create_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_styles_create(ex, &mut self.state.clone()) {
                    return Some(Option::Some(value));
                }
            }

            if let Some(value) = self.transform_styles_create(ex, &mut self.state.clone()) {
                return Some(Option::Some(value));
            }
        }

        if self.cycle == ModuleCycle::TransformExit {
            eprintln!(
                "{}",
                Colorize::yellow("!!!! ModuleCycle::TransformExit !!!!")
            );
        }

        None
    }

    fn proccess_create_css_call(&mut self, ex: &CallExpr) -> Option<Expr> {
        let mut props: Vec<PropOrSpread> = vec![];
        let mut css_class_has_map: IndexMap<String, String> = IndexMap::new();
        let decl_name = self.get_props_desclaration_as_string();

        for arg in ex.args.iter() {
            match &arg.spread {
                Some(_) => todo!(),
                None => match &arg.expr.as_ref() {
                    Expr::Object(object) => {
                        for prop in &object.props {
                            match &prop {
                                PropOrSpread::Prop(prop) => match &prop.as_ref() {
                                    Prop::Shorthand(_) => todo!(),
                                    Prop::KeyValue(namespace) => {
                                        self.process_css_key_value(
                                            namespace,
                                            &mut css_class_has_map,
                                            &decl_name,
                                        );
                                    }
                                    _ => panic!(),
                                },
                                PropOrSpread::Spread(_) => todo!(),
                            };
                        }

                        for (key, value) in css_class_has_map.iter() {
                            let value = (*value).clone();

                            props.push(prop_or_spread_string_creator(
                                key.to_string(),
                                value.clone().trim_end().to_string(),
                            ));
                        }
                    }
                    _ => panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE),
                },
            }
            return object_expression_factory(props);
        }
        None
    }

    fn transform_styles_create(&mut self, ex: &CallExpr, state: &mut StateManager) -> Option<Expr> {
        let first_arg = ex.args.get(0);

        state.in_style_x_create = true;
        let is_create_ident = ex
            .callee
            .as_expr()
            .and_then(|expr| expr.as_ident())
            .and_then(|ident| {
                if state.stylex_create_import.contains(&ident.to_id()) {
                    Option::Some(true)
                } else {
                    Option::None
                }
            })
            .is_some();
        let is_create_member = ex
            .callee
            .as_expr()
            .and_then(|expr| expr.as_member())
            .and_then(|member| {
                if member.obj.is_ident()
                    && member
                        .prop
                        .as_ident()
                        .and_then(|ident| {
                            println!(
                                "!!!!&member.obj.as_ident()?.sym.to_string()!!!: {:?}",
                                &member.obj.as_ident()?.sym.to_string()
                            );
                            if ident.sym.eq("create")
                                && state
                                    .stylex_import_stringified()
                                    .contains(&member.obj.as_ident()?.sym.to_string())
                            {
                                Option::Some(true)
                            } else {
                                Option::None
                            }
                        })
                        .is_some()
                {
                    Option::Some(true)
                } else {
                    Option::None
                }
            })
            .is_some();

        let is_create = is_create_ident || is_create_member;

        println!(
            "\n\n!!!!!call: {:?}, is_create_ident: {}, is_create_member: {}\n\n\n",
            ex, is_create_ident, is_create_member
        );

        let result = if is_create {
            match first_arg {
                Some(first_arg) => {
                    match &first_arg.spread {
                        Some(_) => todo!(),
                        None => {
                            let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> =
                                IndexMap::new();

                            let injected_keyframes: IndexMap<String, InjectableStyle> =
                                IndexMap::new();

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
                                fn_ptr: FunctionType::OneArg(|arg| {
                                    panic!("Keyframes not implemented")
                                }),
                                takes_path: false,
                            };

                            for name in &state.stylex_include_import {
                                identifiers.insert(name.clone(), include_fn.clone());
                            }

                            for name in &state.stylex_first_that_works_import {
                                identifiers.insert(name.clone(), first_that_works_fn.clone());
                            }

                            for name in &state.stylex_keyframes_import {
                                identifiers.insert(name.clone(), keyframes_fn.clone());
                            }

                            for name in &state.stylex_import {
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
                                &state,
                                &function_map,
                                &self.declarations,
                                &mut self.var_decl_count_map,
                            );

                            dbg!(&evaluated_arg);

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

                            let (compiled_styles, injected_styles_sans_keyframes) =
                                stylex_create_set(
                                    &value,
                                    self.state.options.class_name_prefix.as_str(),
                                    &self.declarations,
                                    &mut self.var_decl_count_map,
                                    &state.options,
                                    &function_map,
                                );

                            compiled_styles.clone().into_iter().for_each(
                                |(namespace, properties)| {
                                    resolved_namespaces
                                        .entry(namespace)
                                        .or_default()
                                        .extend(properties);
                                },
                            );

                            let mut injected_styles = injected_keyframes.clone();
                            injected_styles.extend(injected_styles_sans_keyframes);

                            let mut var_name: Option<String> = Option::None;

                            let parent_var_decl =
                                &self.declarations.clone().into_iter().find(|decl| {
                                    decl.init
                                        .as_ref()
                                        .unwrap()
                                        .eq(&Box::new(Expr::Call(ex.clone())))
                                });

                            if let Some(parent_var_decl) = &parent_var_decl {
                                if let Some(ident) = parent_var_decl.name.as_ident() {
                                    var_name = Option::Some(ident.sym.clone().to_string());
                                }
                            }

                            if state.is_test() {
                                todo!("convertToTestStyles")
                            }

                            if state.is_dev() {
                                todo!("injectDevClassNames")
                            }

                            if let Option::Some(var_name) = var_name {
                                let styles_to_remember =
                                    remove_objects_with_spreads(&compiled_styles);

                                state.style_map.insert(var_name.clone(), styles_to_remember);
                                state
                                    .style_vars
                                    .insert(var_name, parent_var_decl.clone()?.clone());
                            }

                            let result_ast = convert_object_to_ast(
                                &NestedStringObject::FlatCompiledStyles(compiled_styles),
                            );

                            let metadatas =
                                MetaData::convert_from_injected_styles_map(injected_styles);

                            for metadata in metadatas {
                                // *target_key_value.value = string_to_expression(metadata.get_class_name().to_string()).unwrap();

                                self.push_to_css_output(metadata);
                            }

                            // let mut new_props: Vec<PropOrSpread> = vec![];

                            // resolved_namespaces.clone().into_iter().for_each(
                            //     |(namespace, properties)| {
                            //         let mut new_inner_props: Vec<PropOrSpread> = vec![];

                            //         properties.into_iter().for_each(|(key, value)| match value {
                            //             FlatCompiledStylesValue::String(value) => {
                            //                 let new_key_values =
                            //                     prop_or_spread_string_creator(key, value);

                            //                 new_inner_props.push(new_key_values);
                            //             }
                            //             FlatCompiledStylesValue::Null(_) => {
                            //                 let new_key_values = prop_or_spread_expression_creator(
                            //                     key,
                            //                     Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                            //                 );

                            //                 new_inner_props.push(new_key_values);
                            //             }
                            //             FlatCompiledStylesValue::IncludedStyle(_) => {
                            //                 todo!()
                            //             }
                            //             FlatCompiledStylesValue::Bool(_) => {
                            //                 todo!()
                            //             }
                            //         });
                            //         push_css_anchor_prop(&mut new_inner_props);

                            //         let object =
                            //             object_expression_factory(new_inner_props).unwrap();

                            //         new_props
                            //             .push(prop_or_spread_expression_creator(namespace, object));
                            //     },
                            // );

                            Option::Some(result_ast)
                        }
                    }
                }
                None => Option::None,
            }
        } else {
            Option::Some(Expr::Call(ex.clone()))
        };

        state.in_style_x_create = false;

        result
    }

    fn process_css_key_value(
        &mut self,
        namespace: &KeyValueProp,
        css_class_has_map: &mut IndexMap<String, String>,
        decl_name: &String,
    ) {
        validate_namespace(&vec![namespace.clone()], &vec![]);

        let namespace_name = "blah_replace_with_real_code".to_string();

        let namespace_name = format!("{}", namespace_name);

        *css_class_has_map
            .entry("className".to_string())
            .or_default() +=
            format!("{}__{}.{} ", self.file_name, decl_name, namespace_name).as_str();

        let value = namespace.value.clone();

        match value.as_ref() {
            Expr::Object(css_object) => {
                for prop in &css_object.props {
                    match &prop {
                        PropOrSpread::Prop(prop) => match prop.as_ref() {
                            Prop::Shorthand(_) => todo!(),
                            Prop::KeyValue(key_value) => {
                                let stylex_set = stylex_create_set(
                                    &EvaluateResultValue::Map(IndexMap::new()),
                                    self.state.options.class_name_prefix.as_str(),
                                    &self.declarations,
                                    &mut self.var_decl_count_map,
                                    &self.state.options,
                                    &FunctionMap {
                                        identifiers: HashMap::new(),
                                        member_expressions: HashMap::new(),
                                    },
                                );

                                let injected_styles_map = stylex_set.1;

                                let metadatas =
                                    MetaData::convert_from_injected_styles_map(injected_styles_map);

                                for metadata in metadatas {
                                    self.push_to_css_output(metadata.clone());

                                    *css_class_has_map
                                        .entry("className".to_string())
                                        .or_default() +=
                                        format!("{} ", metadata.get_class_name()).as_str();
                                }
                            }
                            _ => panic!(),
                        },
                        PropOrSpread::Spread(_) => todo!(),
                    };
                }
            }
            Expr::Arrow(_) => {
                todo!();
            }
            _ => {
                panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE)
            }
        }
    }

    pub(crate) fn get_props_desclaration_as_string(&mut self) -> String {
        let decl_name = self
            .props_declaration
            .clone()
            .unwrap_or_default()
            .0
            .to_string();
        decl_name
    }
}
