use convert_case::{Case, Casing};
use indexmap::IndexMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Ident;
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, MemberProp, Prop, PropOrSpread},
};

use crate::shared::structures::{MetaData, StyleWithDirections};
use crate::shared::utils::{
    get_key_str, get_string_val_from_lit, hash_css, object_expression_factory,
    prop_or_spread_string_creator, push_css_anchor_prop, string_to_expression,
};
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
                                key.to_string(),
                                value.clone().trim_end().to_string(),
                            ));
                        }
                    }
                    _ => {}
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
                                    Prop::KeyValue(key_value) => match key_value.value.as_mut() {
                                        Expr::Object(object) => {
                                            for mut target_prop in &mut object.props {
                                                match &mut target_prop {
                                                    PropOrSpread::Spread(_) => todo!(),
                                                    PropOrSpread::Prop(prop) => {
                                                        match &mut prop.as_mut(){
                                                            Prop::KeyValue(target_key_value) => {
                                                                let css_property = get_key_str(target_key_value);

                                                                let css_property_value =
                                                                    get_string_val_from_lit(target_key_value.value.as_lit().unwrap());

                                                                let (css_style, class_name_hashed) =
                                                                    get_hached_class_name(css_property, css_property_value);

                                                                *target_key_value.value = string_to_expression(class_name_hashed.clone()).unwrap();

                                                                let metadata = MetaData(
                                                                    class_name_hashed.clone(),
                                                                    StyleWithDirections {
                                                                        ltr: format!(".{}{}", class_name_hashed, css_style),
                                                                        rtl: Option::None,
                                                                    },
                                                                    3000,
                                                                );

                                                                self.css_output.push(metadata);
                                                            },
                                                            _=> todo!("transform_create_call_to_style: KeyValueProp")
                                                        }
                                                    }
                                                }
                                            }

                                            push_css_anchor_prop(object);
                                        }
                                        _ => todo!("transform_create_call_to_style: KeyValueProp"),
                                    },
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
                                let css_property = get_key_str(key_value);

                                let css_property_value =
                                    get_string_val_from_lit(key_value.value.as_lit().unwrap());

                                let (css_style, class_name_hashed) =
                                    get_hached_class_name(css_property, css_property_value);

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
                        },
                        PropOrSpread::Spread(_) => todo!(),
                    };
                }
            }
            _ => {}
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

fn get_hached_class_name(css_property: String, css_property_value: String) -> (String, String) {
    let css_property_key = css_property.to_string().to_case(Case::Kebab);

    let css_style = format!("{{{}:{}}}", css_property_key, css_property_value);

    let value_to_hash = format!(
        "<>{}{}{}",
        css_property_key,
        css_property_value,
        "null" //pseudoHashString
    );

    let class_name_hashed = format!("x{}", hash_css(value_to_hash.as_str()));
    (css_style, class_name_hashed)
}
