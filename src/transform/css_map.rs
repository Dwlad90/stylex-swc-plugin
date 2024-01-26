use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Ident, KeyValueProp};
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, MemberProp, Prop, PropOrSpread},
};

use crate::shared::constants;
use crate::shared::structures::meta_data::MetaData;
use crate::shared::utils::common::{
    object_expression_factory, prop_or_spread_string_creator, push_css_anchor_prop,
    string_to_expression,
};
use crate::shared::utils::validators::validate_and_return_namespace;
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
            if let Expr::Member(member) = callee.as_ref() {
                match member.prop.clone() {
                    MemberProp::Ident(ident) => {
                        match format!("{}", ident.sym).as_str() {
                            "create" => {
                                if let Some(value) = self.transform_create_call_to_style(ex) {
                                    return Option::Some(value);
                                }
                            }
                            "props" => {
                                todo!("target_call_expression_to_styles_expr: props")
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
                    _ => panic!("{}", constants::common::ILLEGAL_NAMESPACE_VALUE),
                },
            }
            return object_expression_factory(props);
        }
        None
    }

    fn transform_create_call_to_style(&mut self, ex: &CallExpr) -> Option<Expr> {
        for arg in ex.args.iter() {
            match &arg.spread {
                Some(_) => todo!(),
                None => match &arg.expr.as_ref() {
                    Expr::Object(object) => {
                        let mut style_object = object.clone();

                        for mut prop in &mut style_object.props {
                            match &mut prop {
                                PropOrSpread::Prop(prop) => match &mut prop.as_mut() {
                                    Prop::Shorthand(_) => todo!(),
                                    Prop::KeyValue(namespace) => {
                                        let namespace_name =
                                            validate_and_return_namespace(namespace);

                                        let namespace_name = format!("{}", namespace_name);

                                        match namespace.value.as_mut() {
                                            Expr::Object(object) => {
                                                for mut target_prop in &mut object.props {
                                                    match &mut target_prop {
                                                        PropOrSpread::Spread(_) => todo!(),
                                                        PropOrSpread::Prop(prop) => {
                                                            match &mut prop.as_mut(){
                                                                Prop::KeyValue(target_key_value) => {
                                                                    let stylex_set = MetaData::fabric(
                                                                        namespace_name.as_str(),
                                                                        target_key_value,
                                                                        self.config.class_name_prefix.as_str(),
                                                                        &self.declarations,
                                                                        &mut self.var_decl_count_map);

                                                                    let injected_styles_map = stylex_set.1;
                                                                    println!("!!!!__1111 injected_styles_map: {:#?}", injected_styles_map);

                                                                    let metadatas = MetaData::from_injected_styles_map(injected_styles_map);
                                                                    println!("!!!!__1111 metadatas: {:#?}", metadatas);

                                                                    for metadata in metadatas {
                                                                        *target_key_value.value = string_to_expression(metadata.get_class_name().to_string()).unwrap();


                                                                        self.push_to_css_output(metadata);
                                                                    }
                                                                },
                                                                _=> todo!("transform_create_call_to_style: KeyValueProp")
                                                            }
                                                        }
                                                    }
                                                }

                                                push_css_anchor_prop(object);
                                            }
                                            _ => {
                                                panic!(
                                                    "{}",
                                                    constants::common::ILLEGAL_NAMESPACE_VALUE
                                                )
                                            }
                                        }
                                    }
                                    _ => panic!(),
                                },
                                PropOrSpread::Spread(_) => todo!(),
                            };
                        }

                        return Option::Some(Expr::Object(style_object));
                    }
                    _ => {}
                },
            }
        }

        None
    }

    fn process_css_key_value(
        &mut self,
        namespace: &KeyValueProp,
        css_class_has_map: &mut IndexMap<String, String>,
        decl_name: &String,
    ) {
        let namespace_name = validate_and_return_namespace(namespace);

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
                                let stylex_set = MetaData::fabric(
                                    namespace_name.as_str(),
                                    key_value,
                                    self.config.class_name_prefix.as_str(),
                                    &self.declarations,
                                    &mut self.var_decl_count_map,
                                );

                                println!("!!!!__2 stylex_set: {:#?}", stylex_set);

                                let injected_styles_map = stylex_set.1;

                                println!(
                                    "!!!!__222 injected_styles_map: {:#?}",
                                    injected_styles_map
                                );

                                let metadatas =
                                    MetaData::from_injected_styles_map(injected_styles_map);

                                println!("!!!!__2222 metadatas: {:#?}", metadatas);

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
            _ => panic!("{}", constants::common::ILLEGAL_NAMESPACE_VALUE),
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
