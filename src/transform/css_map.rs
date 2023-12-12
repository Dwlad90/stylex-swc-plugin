use std::collections::hash_map::DefaultHasher;
use std::fs::Metadata;
use std::hash::{Hash, Hasher};

use convert_case::{Case, Casing};
use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Ident;
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, Lit, MemberProp, Prop, PropOrSpread},
};

use crate::shared::structures::{MetaData, StyleWithDirections};
use crate::shared::utils::{
    hash_css, object_expression_factory, prop_or_spread_string_creator, string_to_expression,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub fn target_call_expression_to_css_map_expr(&mut self, ex: &CallExpr) -> Option<Expr> {
        if let Callee::Expr(callee) = &ex.callee {
            if let Expr::Member(member) = callee.as_ref() {
                match member.prop.clone() {
                    MemberProp::Ident(ident) => {
                        match format!("{}", ident.sym).as_str() {
                            "create" => {
                                if let Some(value) = self.proccess_create_call(ex) {
                                    return value;
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

    fn proccess_create_call(&mut self, ex: &CallExpr) -> Option<Option<Expr>> {
        let mut props: Vec<PropOrSpread> = vec![];
        let mut css_class_has_map: IndexMap<String, String> = IndexMap::new();
        let decl_name = self
            .props_declaration
            .clone()
            .unwrap_or_default()
            .0
            .to_string();

        for arg in ex.args.iter() {
            match &arg.spread {
                Some(_) => todo!(),
                None => match &arg.expr.as_ref() {
                    Expr::Object(object) => {
                        for prop in &object.props {
                            match &prop {
                                PropOrSpread::Prop(prop) => match &prop.as_ref() {
                                    Prop::Shorthand(_) => todo!(),
                                    Prop::KeyValue(key_value) => {
                                        self.process_css_key_value(
                                            key_value,
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
                                &format!("{}", key),
                                value.clone().trim_end().to_string(),
                            ));
                        }
                    }
                    _ => {}
                },
            }
            return Some(object_expression_factory(props));
        }
        None
    }

    fn process_css_key_value(
        &mut self,
        key_value: &swc_core::ecma::ast::KeyValueProp,
        css_class_has_map: &mut IndexMap<String, String>,
        decl_name: &String,
    ) {
        let key = key_value.key.clone();
        let class_name = &*key.as_ident().unwrap().sym.clone();

        let class_name = format!("{}", class_name);

        *css_class_has_map
            .entry("className".to_string())
            .or_default() += format!("{}__{}.{} ", self.file_name, decl_name, class_name).as_str();

        let value = key_value.value.clone();

        match value.as_ref() {
            Expr::Object(css_object) => {
                for prop in &css_object.props {
                    match &prop {
                        PropOrSpread::Prop(prop) => match prop.as_ref() {
                            Prop::Shorthand(_) => todo!(),
                            Prop::KeyValue(key_value) => {
                                let key = key_value.key.clone();
                                let css_property = &*key.as_ident().unwrap().sym.clone();

                                let css_property_value = key_value.value.as_lit().unwrap();

                                match &css_property_value {
                                    Lit::Str(str) => {
                                        let css_property_key =
                                            css_property.to_string().to_case(Case::Kebab);

                                        let css_property_value = str.value.clone();

                                        let css_style = format!(
                                            "{{{}:{}}}",
                                            css_property_key, css_property_value
                                        );

                                        let value_to_hash = format!(
                                            "<>{}{}{}",
                                            css_property_key,
                                            css_property_value,
                                            "null" //pseudoHashString
                                        );

                                        let class_name_hashed = format!(
                                            "x{}",
                                            hash_css(value_to_hash.as_str())
                                        );

                                        let metadata = MetaData(
                                            class_name_hashed.clone(),
                                            StyleWithDirections {
                                                ltr: format!(".{}{}", class_name_hashed, css_style),
                                                rtl: Option::None,
                                            },
                                            3000,
                                        );

                                        self.css_output.push(metadata);

                                        *css_class_has_map
                                            .entry("className".to_string())
                                            .or_default() +=
                                            format!("{} ", class_name_hashed.clone()).as_str();
                                    }
                                    _ => panic!(),
                                }
                            }
                            _ => panic!(),
                        },
                        PropOrSpread::Spread(_) => todo!(),
                    };
                }
            }
            _ => {}
        }
    }
}
