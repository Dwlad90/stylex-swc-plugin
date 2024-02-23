use core::panic;
use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Ident, KeyValueProp, Lit, Null};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, MemberProp, Prop, PropOrSpread},
};

use crate::shared::constants;
use crate::shared::structures::evaluate_result::EvaluateResultValue;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, Functions};
use crate::shared::structures::injectable_style::InjectableStyle;
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::structures::state_manager::StateManager;
use crate::shared::utils::common::{
    object_expression_factory, prop_or_spread_expression_creator, prop_or_spread_string_creator,
    push_css_anchor_prop,
};
use crate::shared::utils::css::stylex::{evaluate_style_x_create_arg, stylex_create};
use crate::shared::utils::validators::validate_namespace;
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_call_expression_to_css_map_expr(
        &mut self,
        ex: &CallExpr,
    ) -> Option<Expr> {
        if let Callee::Expr(callee) = &ex.callee {
            if let Expr::Member(member) = callee.as_ref() {
                match member.prop.clone() {
                    MemberProp::Ident(ident) => {
                        match format!("{}", ident.sym).as_str() {
                            "create" => {
                                if let Some(value) = self.proccess_create_css_call(ex) {
                                    return Option::Some(value);
                                }
                            }
                            "props" => {
                                return Option::Some(Expr::Ident(Ident::new(
                                    "_stylex$props".into(),
                                    DUMMY_SP,
                                )));
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                }
            }
        }

        return Option::None;
    }

    pub(crate) fn transform_call_expression_to_styles_expr(
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

        return Option::None;
    }

    fn transform_styles_ident_create(
        &mut self,
        ident: Ident,
        ex: &CallExpr,
    ) -> Option<Option<Expr>> {
        if self.state.stylex_create_import.contains(&ident.to_id()) {
            if let Some(value) = self.transform_styles_create(ex, &mut self.state.clone()) {
                return Some(Option::Some(value));
            }
        }
        match format!("{}", ident.sym).as_str() {
            "create" => {
                if let Some(value) = self.transform_styles_create(ex, &mut self.state.clone()) {
                    return Some(Option::Some(value));
                }
            }
            "props" => {
                return Option::Some(Option::Some(Expr::Ident(Ident::new(
                    "_stylex$props".into(),
                    DUMMY_SP,
                ))));
            }
            _ => {}
        };
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

        let result = match first_arg {
            Some(first_arg) => {
                match &first_arg.spread {
                    Some(_) => todo!(),
                    None => {
                        let mut resolved_namespaces: IndexMap<
                            String,
                            IndexMap<String, Option<String>>,
                        > = IndexMap::new();

                        let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

                        let mut identifiers: HashMap<String, FunctionConfig> = HashMap::new();
                        let mut member_expressions: HashMap<
                            ImportSources,
                            HashMap<String, FunctionConfig>,
                        > = HashMap::new();

                        for name in &state.stylex_include_import {
                            identifiers.insert(
                                name.clone(),
                                FunctionConfig {
                                    fn_ptr: |args| panic!("StylexInclude not implemented"),
                                    takes_path: true,
                                },
                            );
                        }

                        for name in &state.stylex_first_that_works_import {
                            identifiers.insert(
                                name.clone(),
                                FunctionConfig {
                                    fn_ptr: |args| panic!("StylexFirstThatWorks not implemented"),
                                    takes_path: false,
                                },
                            );
                        }

                        for name in &state.stylex_keyframes_import {
                            identifiers.insert(
                                name.clone(),
                                FunctionConfig {
                                    fn_ptr: |a| panic!("Keyframes not implemented"),
                                    takes_path: false,
                                },
                            );
                        }

                        for name in &state.stylex_import {
                            member_expressions.insert(name.clone(), HashMap::new());
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

                        let stylex_set = stylex_create(
                            &value,
                            self.state.options.class_name_prefix.as_str(),
                            &self.declarations,
                            &mut self.var_decl_count_map,
                            &state.options,
                        );

                        stylex_set
                            .0
                            .clone()
                            .into_iter()
                            .for_each(|(namespace, properties)| {
                                resolved_namespaces
                                    .entry(namespace)
                                    .or_default()
                                    .extend(properties);
                            });

                        let injected_styles_map = stylex_set.1;

                        let metadatas =
                            MetaData::convert_from_injected_styles_map(injected_styles_map);

                        for metadata in metadatas {
                            // *target_key_value.value = string_to_expression(metadata.get_class_name().to_string()).unwrap();

                            self.push_to_css_output(metadata);
                        }

                        let mut new_props: Vec<PropOrSpread> = vec![];

                        resolved_namespaces.clone().into_iter().for_each(
                            |(namespace, properties)| {
                                let mut new_inner_props: Vec<PropOrSpread> = vec![];

                                properties.into_iter().for_each(|(key, value)| {
                                    if let Some(value) = value {
                                        let new_key_values =
                                            prop_or_spread_string_creator(key, value);

                                        new_inner_props.push(new_key_values);
                                    } else {
                                        let new_key_values = prop_or_spread_expression_creator(
                                            key,
                                            Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                                        );

                                        new_inner_props.push(new_key_values);
                                    }
                                });
                                push_css_anchor_prop(&mut new_inner_props);

                                let object = object_expression_factory(new_inner_props).unwrap();

                                new_props
                                    .push(prop_or_spread_expression_creator(namespace, object));
                            },
                        );

                        let object = object_expression_factory(new_props).unwrap();

                        Option::Some(object)
                    }
                }
            }
            None => Option::None,
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
        validate_namespace(&vec![namespace.clone()]);

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
                                let stylex_set = stylex_create(
                                    &EvaluateResultValue::Map(IndexMap::new()),
                                    self.state.options.class_name_prefix.as_str(),
                                    &self.declarations,
                                    &mut self.var_decl_count_map,
                                    &self.state.options,
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
