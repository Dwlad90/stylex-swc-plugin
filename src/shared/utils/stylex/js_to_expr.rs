use core::panic;

use indexmap::IndexMap;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Bool, Expr, Lit, Null, PropOrSpread, SpreadElement},
};

use crate::shared::{
    enums::FlatCompiledStylesValue,
    structures::flat_compiled_styles::FlatCompiledStyles,
    utils::{
        common::{prop_or_spread_expression_creator, prop_or_spread_string_creator},
        css::factories::object_expression_factory,
    },
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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NestedStringObject {
    FlatCompiledStyles(IndexMap<String, FlatCompiledStyles>),
    FlatCompiledStylesValues(IndexMap<String, FlatCompiledStylesValue>),
}

impl NestedStringObject {
    pub(crate) fn as_styles(&self) -> Option<&IndexMap<String, FlatCompiledStyles>> {
        match self {
            NestedStringObject::FlatCompiledStyles(obj) => Some(obj),
            _ => None,
        }
    }

    pub(crate) fn as_values(&self) -> Option<&IndexMap<String, FlatCompiledStylesValue>> {
        match self {
            NestedStringObject::FlatCompiledStylesValues(obj) => Some(obj),
            _ => None,
        }
    }
}

pub(crate) fn convert_object_to_ast(obj: &NestedStringObject) -> Expr {
    let mut props: Vec<PropOrSpread> = vec![];

    match obj {
        NestedStringObject::FlatCompiledStyles(obj) => {
            for (key, value) in obj.iter() {
                let expr = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
                    value.clone(),
                ));

                let prop = prop_or_spread_expression_creator(key.clone(), expr);

                props.push(prop);
            }
        }
        NestedStringObject::FlatCompiledStylesValues(obj) => {
            for (key, value) in obj.iter() {
                let prop = match value {
                    FlatCompiledStylesValue::String(value) => {
                        prop_or_spread_string_creator(key.clone(), value.clone())
                    }
                    FlatCompiledStylesValue::Null => prop_or_spread_expression_creator(
                        key.clone(),
                        Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                    ),
                    FlatCompiledStylesValue::IncludedStyle(include_style) => {
                        PropOrSpread::Spread(SpreadElement {
                            dot3_token: DUMMY_SP,
                            expr: Box::new(include_style.get_expr().clone()),
                        })
                    }
                    FlatCompiledStylesValue::Bool(value) => prop_or_spread_expression_creator(
                        key.clone(),
                        Expr::Lit(Lit::Bool(Bool {
                            span: DUMMY_SP,
                            value: value.clone(),
                        })),
                    ),
                    FlatCompiledStylesValue::InjectableStyle(_) => todo!("Injectable style"),
                    FlatCompiledStylesValue::Tuple(_, _) => todo!("Tuple"),
                    FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
                };

                dbg!(&prop);

                props.push(prop);
            }
        }
    }

    object_expression_factory(props).unwrap()
}
