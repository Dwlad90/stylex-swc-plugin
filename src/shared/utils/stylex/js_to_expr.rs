use indexmap::IndexMap;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{
        Bool, Expr, Ident, KeyValueProp, Lit, Null, Prop, PropName, PropOrSpread, SpreadElement,
    },
};

use crate::shared::{
    structures::flat_compiled_styles::{FlatCompiledStyles, FlatCompiledStylesValue},
    utils::{common::string_to_expression, css::factories::object_expression_factory},
};

pub(crate) fn remove_objects_with_spreads(
    obj: &IndexMap<String, FlatCompiledStyles>,
) -> IndexMap<String, FlatCompiledStyles> {
    let mut obj = obj.clone();

    obj.retain(|_key, value| {
        value
            .values()
            .into_iter()
            .all(|keep_value| match keep_value {
                FlatCompiledStylesValue::IncludedStyle(_) => false,
                _ => true,
            })
    });

    obj
}

pub(crate) enum NestedStringObject {
    FlatCompiledStyles(IndexMap<String, FlatCompiledStyles>),
    FlatCompiledStylesValues(IndexMap<String, FlatCompiledStylesValue>),
}

pub(crate) fn convert_object_to_ast(obj: &NestedStringObject) -> Expr {
    let mut props: Vec<PropOrSpread> = vec![];

    match obj {
        NestedStringObject::FlatCompiledStyles(obj) => {
            for (key, value) in obj.iter() {
                let expr = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
                    value.clone(),
                ));

                let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident::new(key.clone().into(), DUMMY_SP)),
                    value: Box::new(expr),
                })));

                props.push(prop);
            }
        }
        NestedStringObject::FlatCompiledStylesValues(obj) => {
            for (key, value) in obj.iter() {
                let prop = match value {
                    FlatCompiledStylesValue::String(value) => {
                        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(key.clone().into(), DUMMY_SP)),
                            value: Box::new(string_to_expression(value.clone()).unwrap()),
                        })))
                    }
                    FlatCompiledStylesValue::Null(_) => {
                        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(key.clone().into(), DUMMY_SP)),
                            value: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
                        })))
                    }
                    FlatCompiledStylesValue::IncludedStyle(include_style) => {
                        PropOrSpread::Spread(SpreadElement {
                            dot3_token: DUMMY_SP,
                            expr: Box::new(include_style.get_expr().clone()),
                        })
                    }
                    FlatCompiledStylesValue::Bool(value) => {
                        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(key.clone().into(), DUMMY_SP)),
                            value: Box::new(Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: value.clone(),
                            }))),
                        })))
                    }
                };

                props.push(prop);
            }
        }
    }

    object_expression_factory(props).unwrap()
}