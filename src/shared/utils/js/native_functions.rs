use crate::shared::{structures::evaluate_result::EvaluateResultValue, utils::common::lit_to_num};
use std::rc::Rc;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, Expr, ExprOrSpread},
};

pub(crate) fn evaluate_map(
    funcs: &Vec<EvaluateResultValue>,
    args: &Vec<Option<EvaluateResultValue>>,
) -> Option<EvaluateResultValue> {
    let cb = funcs.get(0)?.clone();

    let Some(cb) = cb.as_callback() else {
        return Option::None;
    };

    let func_result = args
        .into_iter()
        .filter_map(|arg| {
            let Some(result) = &arg else {
                return Option::None;
            };

            match result {
                EvaluateResultValue::Expr(_) => Option::Some(evaluate_map_cb(cb, &arg)),
                EvaluateResultValue::Vec(vec) => {
                    let func_result = vec
                        .into_iter()
                        .filter_map(|expr| {
                            let expr = evaluate_map_cb(cb, &expr.clone());

                            Option::Some(EvaluateResultValue::Expr(expr))
                        })
                        .collect::<Vec<EvaluateResultValue>>();

                    let elems = func_result
                        .into_iter()
                        .map(|item| {
                            Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(item.as_expr()?.clone()),
                            })
                        })
                        .collect::<Vec<Option<ExprOrSpread>>>();

                    Some(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems,
                    }))
                }
                _ => todo!("Not implemented yet"),
            }
        })
        .collect::<Vec<Expr>>();

    match func_result.get(0) {
        Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::Array(array.clone()))),
        _ => Some(EvaluateResultValue::Expr(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: func_result
                .into_iter()
                .map(|expr| {
                    Option::Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(expr),
                    })
                })
                .collect(),
        }))),
    }
}

pub(crate) fn evaluate_filter(
    funcs: &Vec<EvaluateResultValue>,
    args: &Vec<Option<EvaluateResultValue>>,
) -> Option<EvaluateResultValue> {
    let cb = funcs.get(0)?.clone();

    let Some(cb) = cb.as_callback() else {
        return Option::None;
    };

    let func_result = args
        .into_iter()
        .filter_map(|arg| {
            let Some(result) = &arg else {
                return Option::None;
            };

            match result {
                EvaluateResultValue::Expr(expr) => evaluate_filter_cb(cb, &arg, &expr),
                EvaluateResultValue::Vec(vec) => {
                    let func_result = vec
                        .into_iter()
                        .filter_map(|expr| {
                            let result = evaluate_filter_cb(
                                cb,
                                &expr.clone(),
                                &expr.as_ref()?.clone().as_expr()?.clone(),
                            );

                            result.map(|expr| EvaluateResultValue::Expr(expr))
                        })
                        .collect::<Vec<EvaluateResultValue>>();

                    let elems = func_result
                        .into_iter()
                        .map(|item| {
                            Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(item.as_expr()?.clone()),
                            })
                        })
                        .collect::<Vec<Option<ExprOrSpread>>>();

                    Some(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems,
                    }))
                }
                _ => todo!("Not implemented yet"),
            }
        })
        .collect::<Vec<Expr>>();

    match func_result.get(0) {
        Some(Expr::Array(array)) => Some(EvaluateResultValue::Expr(Expr::Array(array.clone()))),
        _ => Some(EvaluateResultValue::Expr(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: func_result
                .into_iter()
                .map(|expr| {
                    Option::Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(expr),
                    })
                })
                .collect(),
        }))),
    }
}

pub(crate) fn evaluate_map_cb(
    cb: &Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr>,
    cb_arg: &Option<EvaluateResultValue>,
) -> Expr {
    (cb)(vec![cb_arg.clone()])
}

pub(crate) fn evaluate_filter_cb(
    cb: &Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr>,
    cb_arg: &Option<EvaluateResultValue>,
    item: &Expr,
) -> Option<Expr> {
    let result = evaluate_map_cb(cb, cb_arg);

    let Some(lit) = result.as_lit() else {
        panic!("Expr is not a literal");
    };

    if lit_to_num(&lit) == 0.0 {
        Option::None
    } else {
        Option::Some(item.clone())
    }

    // Option::Some(Expr::Lit(Lit::Num(Number {
    //     span: DUMMY_SP,
    //     value: ,
    //     raw: Option::None,
    // })));
}
