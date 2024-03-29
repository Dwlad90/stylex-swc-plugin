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
use crate::shared::structures::injectable_style::InjectableStyle;
use crate::shared::structures::meta_data::MetaData;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::prop_or_spread_string_creator;
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
use crate::shared::utils::validators::{
    is_create_call, validate_namespace, validate_stylex_create_indent,
};
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

    // fn _proccess_create_css_call(&mut self, ex: &CallExpr) -> Option<Expr> {
    //     let mut props: Vec<PropOrSpread> = vec![];
    //     let mut css_class_has_map: IndexMap<String, String> = IndexMap::new();
    //     let decl_name = self._get_props_declaration_as_string();

    //     for arg in ex.args.iter() {
    //         match &arg.spread {
    //             Some(_) => todo!(),
    //             None => match &arg.expr.as_ref() {
    //                 Expr::Object(object) => {
    //                     for prop in &object.props {
    //                         match &prop {
    //                             PropOrSpread::Prop(prop) => match &prop.as_ref() {
    //                                 Prop::Shorthand(_) => todo!(),
    //                                 Prop::KeyValue(namespace) => {
    //                                     self._process_css_key_value(
    //                                         namespace,
    //                                         &mut css_class_has_map,
    //                                         &decl_name,
    //                                     );
    //                                 }
    //                                 _ => panic!(),
    //                             },
    //                             PropOrSpread::Spread(_) => todo!(),
    //                         };
    //                     }

    //                     for (key, value) in css_class_has_map.iter() {
    //                         let value = (*value).clone();

    //                         props.push(prop_or_spread_string_creator(
    //                             key.to_string(),
    //                             value.clone().trim_end().to_string(),
    //                         ));
    //                     }
    //                 }
    //                 _ => panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE),
    //             },
    //         }
    //         return object_expression_factory(props);
    //     }
    //     None
    // }

    pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
        self.state.in_stylex_create = true;
        let is_create_call = is_create_call(call, &mut self.state);

        let result = if is_create_call {
            validate_stylex_create_indent(call, &mut self.state);

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

                        let evaluated_arg = evaluate_stylex_create_arg(
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

                        let (var_name, parent_var_decl) = &self.get_call_var_name(call);

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
                                .insert(var_name.clone(), parent_var_decl.clone().unwrap().clone());
                        }

                        let result_ast = convert_object_to_ast(
                            &NestedStringObject::FlatCompiledStyles(compiled_styles),
                        );

                        self.state
                            .register_styles(call, &injected_styles, &result_ast, &var_name);

                        Option::Some(result_ast)
                    }
                },
                None => Option::None,
            }
        } else {
            None
        };

        self.state.in_stylex_create = false;

        result
    }

    // fn _process_css_key_value(
    //     &mut self,
    //     namespace: &KeyValueProp,
    //     css_class_has_map: &mut IndexMap<String, String>,
    //     decl_name: &String,
    // ) {
    //     validate_namespace(&vec![namespace.clone()], &vec![]);

    //     let namespace_name = "blah_replace_with_real_code".to_string();

    //     let namespace_name = format!("{}", namespace_name);

    //     *css_class_has_map
    //         .entry("className".to_string())
    //         .or_default() += format!(
    //         "{}__{}.{} ",
    //         self.state.get_filename(),
    //         decl_name,
    //         namespace_name
    //     )
    //     .as_str();

    //     let value = namespace.value.clone();

    //     match value.as_ref() {
    //         Expr::Object(css_object) => {
    //             for prop in &css_object.props {
    //                 match &prop {
    //                     PropOrSpread::Prop(prop) => match prop.as_ref() {
    //                         Prop::Shorthand(_) => todo!(),
    //                         Prop::KeyValue(_) => {
    //                             let stylex_set = stylex_create_set(
    //                                 &EvaluateResultValue::Map(IndexMap::new()),
    //                                 &mut self.state,
    //                                 &FunctionMap::default(),
    //                             );

    //                             let injected_styles_map = stylex_set.1;

    //                             let metadatas =
    //                                 MetaData::convert_from_injected_styles_map(injected_styles_map);

    //                             for metadata in metadatas {
    //                                 self.push_to_css_output(decl_name.clone(), metadata.clone());

    //                                 *css_class_has_map
    //                                     .entry("className".to_string())
    //                                     .or_default() +=
    //                                     format!("{} ", metadata.get_class_name()).as_str();
    //                             }
    //                         }
    //                         _ => panic!(),
    //                     },
    //                     PropOrSpread::Spread(_) => todo!(),
    //                 };
    //             }
    //         }
    //         Expr::Arrow(_) => {
    //             todo!();
    //         }
    //         _ => {
    //             panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE)
    //         }
    //     }
    // }

    pub(crate) fn _get_props_declaration_as_string(&mut self) -> String {
        let decl_name = self
            .props_declaration
            .clone()
            .unwrap_or_default()
            .0
            .to_string();
        decl_name
    }
}
