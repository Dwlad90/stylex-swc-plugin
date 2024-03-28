
use indexmap::IndexMap;
use swc_core::ecma::{
    ast::{Expr, KeyValueProp, Prop, PropOrSpread},
    utils::ExprExt,
};

use crate::shared::{
    constants,
    structures::{
        evaluate_result::{EvaluateResult, EvaluateResultValue},
        functions::FunctionMap,
        state_manager::StateManager,
    },
    utils::validators::validate_dynamic_style_params,
};

use super::evaluate::{evaluate, evaluate_obj_key};

pub(crate) fn evaluate_style_x_create_arg(
    path: &Expr,
    traversal_state: &mut StateManager,
    functions: &FunctionMap,
) -> EvaluateResult {
    match path {
        Expr::Object(object) => {
            let mut style_object = object.clone();

            let mut value: IndexMap<Expr, Vec<KeyValueProp>> = IndexMap::new();

            for prop in &mut style_object.props {
                match prop {
                    PropOrSpread::Spread(_) => todo!("Spread not implemented yet"),
                    PropOrSpread::Prop(prop) => {
                        // let obj_prop_path = &prop.clone();

                        match prop.as_ref() {
                            Prop::KeyValue(key_value_prop) => {
                                let key_result =
                                    evaluate_obj_key(key_value_prop, traversal_state, functions);

                                if !key_result.confident {
                                    return EvaluateResult {
                                        confident: false,
                                        deopt: key_result.deopt,
                                        value: None,
                                    };
                                }

                                let key = key_result.value.unwrap();

                                let key = key.as_expr().unwrap();

                                let value_path = &key_value_prop.value;

                                match value_path.as_ref() {
                                    Expr::Arrow(fn_path) => {
                                        let all_params = fn_path.params.clone();

                                        validate_dynamic_style_params(&all_params);
                                    }
                                    _ => {
                                        let val = evaluate(value_path, traversal_state, functions);

                                        if !val.confident {
                                            return val;
                                        }

                                        let value_to_insert = match val.value.unwrap() {
                                            EvaluateResultValue::Expr(expr) => match expr {
                                                Expr::Object(obj_expr) => {
                                                    let mut obj_expr_props: Vec<KeyValueProp> =
                                                        vec![];

                                                    for prop in obj_expr.props {
                                                        match prop {
                                                            PropOrSpread::Spread(_) => todo!(),
                                                            PropOrSpread::Prop(prop) => {
                                                                match prop.as_ref() {
                                                                    Prop::KeyValue(
                                                                        obj_expr_prop_kv,
                                                                    ) => obj_expr_props.push(
                                                                        obj_expr_prop_kv.clone(),
                                                                    ),

                                                                    _ => todo!(),
                                                                }
                                                            }
                                                        }
                                                    }

                                                    obj_expr_props
                                                }
                                                _ => panic!(
                                                    "{}",
                                                    constants::messages::ILLEGAL_NAMESPACE_VALUE
                                                ),
                                            },
                                            _ => panic!(
                                                "{}",
                                                constants::messages::ILLEGAL_NAMESPACE_VALUE
                                            ),
                                        };

                                        value.insert(key.as_expr().clone(), value_to_insert);

                                        continue;
                                    }
                                }
                            }
                            _ => {
                                return evaluate(path, traversal_state, functions);
                            }
                        }
                    }
                }
            }
            EvaluateResult {
                confident: true,
                deopt: None,
                value: Some(EvaluateResultValue::Map(value)),
            }
        }
        _ => {
            dbg!(path);
            evaluate(path, traversal_state, functions)
        }
    }
}
