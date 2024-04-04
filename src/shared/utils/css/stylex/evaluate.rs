// use sha2::{Digest, Sha256};
use std::{collections::HashMap, rc::Rc};

use colored::Colorize;
use indexmap::IndexMap;
use swc_core::{
    common::{Span, DUMMY_SP},
    ecma::{
        ast::{
            ArrayLit, BlockStmtOrExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Id, Ident,
            KeyValueProp, Lit, MemberProp, Number, ObjectLit, Pat, Prop, PropName, PropOrSpread,
            Str, TplElement, VarDeclarator,
        },
        utils::{ident::IdentLike, ExprExt},
        visit::{Fold, FoldWith},
    },
};

struct SpanReplacer;

impl Fold for SpanReplacer {
    fn fold_span(&mut self, n: Span) -> Span {
        DUMMY_SP
    }
}

fn replace_spans(expr: &mut Expr) -> Expr {
    expr.clone().fold_children_with(&mut SpanReplacer)
}

use crate::shared::{
    constants::{
        self,
        constants::{INVALID_METHODS, VALID_CALLEES},
    },
    enums::VarDeclAction,
    structures::{
        evaluate_result::{EvaluateResult, EvaluateResultValue},
        functions::{CallbackType, FunctionConfig, FunctionMap, FunctionType},
        injectable_style::InjectableStyle,
        named_import_source::ImportSources,
        state_manager::StateManager,
        stylex_options::StyleXOptions,
        stylex_state_options::StyleXStateOptions,
    },
    utils::{
        common::{
            binary_expr_to_num, deep_merge_props, expr_to_str, get_key_str,
            get_string_val_from_lit, get_var_decl_by_ident, get_var_decl_from,
            number_to_expression, remove_duplicates, string_to_expression,
        },
        css::factories::object_expression_factory,
        js::{
            enums::{ArrayJS, ObjectJS},
            native_functions::{evaluate_filter, evaluate_map},
        },
    },
};

#[derive(Debug)]
pub(crate) struct SeenValue {
    pub(crate) value: Option<EvaluateResultValue>,
    pub(crate) resolved: bool,
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) confident: bool,
    pub(crate) deopt_path: Option<Expr>, // Assuming this is a string identifier
    pub(crate) seen: HashMap<Expr, SeenValue>, // Assuming the values are strings
    //pub(crate) added_imports: HashSet<String>,
    pub(crate) functions: FunctionMap,
    pub(crate) traversal_state: StateManager,
}

impl State {
    pub(crate) fn default() -> Self {
        State {
            confident: true,
            deopt_path: Option::None,
            seen: HashMap::new(),
            functions: FunctionMap {
                identifiers: HashMap::new(),
                member_expressions: HashMap::new(),
            },
            traversal_state: StateManager::new(StyleXOptions::default()),
        }
    }

    pub(crate) fn new(traversal_state: &StateManager) -> Self {
        State {
            confident: true,
            deopt_path: Option::None,
            seen: HashMap::new(),
            functions: FunctionMap {
                identifiers: HashMap::new(),
                member_expressions: HashMap::new(),
            },
            traversal_state: traversal_state.clone(),
        }
    }
}

// enum KeyResult {
//     ConfidentTrue {
//         confident: bool,
//         value: String,
//     },
//     ConfidentFalse {
//         confident: bool,
//         deopt: Option<Expr>,
//     },
// }

pub(crate) fn evaluate_obj_key(
    prop_kv: &KeyValueProp,
    state: &mut StateManager,
    functions: &FunctionMap,
) -> EvaluateResult {
    let key_path = &prop_kv.key;

    let key: Expr;

    match key_path {
        PropName::Ident(ident) => {
            key = Expr::Lit(Lit::Str(Str {
                value: ident.sym.to_string().into(),
                raw: Option::None,
                span: DUMMY_SP,
            }));
        }
        PropName::Computed(computed) => {
            let computed_path = &computed.expr;
            let computed_result = evaluate(computed_path, state, functions);
            if computed_result.confident {
                key = match computed_result.value {
                    Some(EvaluateResultValue::Expr(value)) => value,
                    _ => panic!("Expected string value"),
                };
            } else {
                return EvaluateResult {
                    confident: false,
                    deopt: computed_result.deopt,
                    value: None,
                };
            }
        }
        PropName::Str(str) => {
            key = Expr::Lit(Lit::Str(str.clone()));
        }
        PropName::Num(num) => {
            key = Expr::Lit(Lit::Num(num.clone()));
        }
        PropName::BigInt(big_int) => {
            key = Expr::Lit(Lit::BigInt(big_int.clone()));
        }
    }

    let key_expr =
        string_to_expression(expr_to_str(&key, state, functions)).expect("Expected string value");

    EvaluateResult {
        confident: true,
        deopt: Option::None,
        value: Option::Some(EvaluateResultValue::Expr(key_expr)),
    }
}
pub fn evaluate(
    path: &Expr,
    traversal_state: &mut StateManager,
    fns: &FunctionMap,
) -> EvaluateResult {
    let mut state = State {
        confident: true,
        deopt_path: None,
        seen: HashMap::new(),
        // added_imports: HashSet::new(),
        functions: fns.clone(),
        traversal_state: traversal_state.clone(),
    };

    let mut value = evaluate_cached(path, &mut state);

    if !state.confident {
        value = Option::None;
    }

    *traversal_state = state.traversal_state.clone();

    EvaluateResult {
        confident: state.confident,
        value,
        deopt: state.deopt_path,
    }
}

fn deopt(path: &Expr, state: &mut State) -> Option<EvaluateResultValue> {
    if state.confident {
        state.confident = false;
        state.deopt_path = Some(path.clone());
    }

    Option::None
}

// fn evaluate_arror_function(
//     arrow: ArrowExpr,
//     state: &mut State,
//     declarations: &Vec<VarDeclarator>,
//     var_dec_count_map: &mut HashMap<Id, i8>,
// ) -> Option<FunctionConfig> {

// }

fn _evaluate(path: &Expr, state: &mut State) -> Option<EvaluateResultValue> {
    if !state.confident {
        panic!("Should not be here");
        // return Option::None;
    }

    let result = match path {
        Expr::Arrow(arrow) => {
            let body = arrow.body.clone();
            let params = arrow.params.clone();

            let ident_params = params
                .clone()
                .into_iter()
                .filter_map(|param| {
                    if param.is_ident() {
                        Option::Some(param.as_ident().unwrap().to_id())
                    } else {
                        Option::None
                    }
                })
                .collect::<Vec<Id>>();

            match body.as_ref() {
                BlockStmtOrExpr::Expr(body_expr) => {
                    if ident_params.len() == params.len() {
                        let orig_args: Vec<Pat> = params.clone();

                        let arrow_closure_fabric =
                            |orig_args: Vec<Pat>,
                             functions: HashMap<Id, FunctionConfig>,
                             ident_params: Vec<Id>,
                             body_expr: Expr,
                             traversal_state: StateManager| {
                                move |cb_args: Vec<Option<EvaluateResultValue>>| {
                                    let mut functions = functions.clone();

                                    let mut member_expressions: HashMap<
                                        ImportSources,
                                        HashMap<Id, FunctionConfig>,
                                    > = HashMap::new();
                                    println!(
                                        "!!!!__ orig_args: {:#?}, functions: {:#?}, cb_args: {:#?}",
                                        orig_args, functions, cb_args
                                    );

                                    ident_params.iter().enumerate().for_each(|(index, ident)| {
                                        match cb_args.get(index) {
                                            Some(arg) => {
                                                let arg = arg.clone();
                                                let expr = arg.unwrap().as_expr().unwrap().clone();

                                                let cl = |arg: Expr| move || arg.clone();

                                                // panic!("Check what's happening here, expr: {:#?}", expr);
                                                let result = (cl)(expr.clone());
                                                let function = FunctionConfig {
                                                    fn_ptr: FunctionType::Mapper(Rc::new(result)),
                                                    takes_path: false,
                                                };
                                                functions.insert(ident.clone(), function.clone());

                                                member_expressions.insert(
                                                    ImportSources::Regular("entry".to_string()),
                                                    functions.clone(),
                                                );
                                            }
                                            None => {}
                                        }
                                    });

                                    // panic!("Check what's happening here, body_expr: {:#?} ident_params: {:#?}", body_expr, ident_params);
                                    let result = evaluate(
                                        &body_expr,
                                        &mut traversal_state.clone(),
                                        &FunctionMap {
                                            identifiers: functions,
                                            member_expressions,
                                        },
                                    );

                                    let value = result.value;

                                    match value {
                                        Some(res) => res.as_expr().unwrap().clone(),
                                        None => todo!(),
                                    }
                                }
                            };

                        let functions = state.functions.identifiers.clone();

                        let arrow_closure = Rc::new(arrow_closure_fabric(
                            orig_args,
                            functions,
                            ident_params,
                            *body_expr.clone(),
                            state.traversal_state.clone(),
                        ));

                        return Option::Some(EvaluateResultValue::Callback(
                            arrow_closure,
                            // Expr::Arrow(arrow.clone()),
                        ));
                    }

                    Option::None
                }
                BlockStmtOrExpr::BlockStmt(_) => Option::None,
            }
        }
        Expr::Ident(ident) => {
            let ident_id = ident.to_id();

            dbg!(&ident_id, &state.functions.identifiers);

            if state.functions.identifiers.contains_key(&ident_id) {
                let func = state.functions.identifiers.get(&ident_id)?;
                dbg!(&ident_id, &func);

                let FunctionType::Mapper(func) = func.fn_ptr.clone() else {
                    panic!("Function not found");
                };

                return Some(EvaluateResultValue::Expr(func()));
            }

            let ident_binding =
                get_binding(&Expr::Ident(ident.clone()), &mut state.traversal_state);

            Option::None
        }
        Expr::TsAs(_) => todo!("TsAs not implemented yet"),
        Expr::TsSatisfies(_) => todo!("TsSatisfies not implemented yet"),
        Expr::Seq(_) => todo!("Seq not implemented yet"),
        Expr::Lit(lit_path) => Option::Some(EvaluateResultValue::Expr(Expr::Lit(lit_path.clone()))),
        Expr::Tpl(tpl) => evaluate_quasis(&Expr::Tpl(tpl.clone()), &tpl.quasis, false, state),
        Expr::TaggedTpl(tagged_tpl) => {
            todo!("TaggedTpl");
            evaluate_quasis(
                &Expr::TaggedTpl(tagged_tpl.clone()),
                &tagged_tpl.tpl.quasis,
                false,
                state,
            )
        }
        Expr::Cond(_) => todo!("Cond not implemented yet"),
        Expr::Paren(paren) => {
            let result = evaluate_cached(&paren.expr, state);

            result
        }
        Expr::Member(member) => {
            dbg!(&member.obj);

            let Some(object) = evaluate_cached(&*member.obj, state) else {
                dbg!(&member.obj);
                panic!("Object not found");
            };

            if !state.confident {
                return Option::None;
            };

            let Some(expr) = object.as_expr() else {
                panic!("Function not found");
            };

            // let Some(ArrayLit { elems, .. }) = expr.as_array() else {
            //     if let Some(object) = expr.as_object() {
            //         return Some(EvaluateResultValue::Expr(Expr::Object(object.clone())));
            //     } else {
            //         dbg!(&expr);
            //         todo!("Expression");
            //     }
            // };

            let prop_path = &member.prop;

            let propery = match prop_path {
                MemberProp::Ident(ident) => {
                    Option::Some(EvaluateResultValue::Expr(Expr::Ident(ident.clone())))
                }
                MemberProp::Computed(ComputedPropName { expr, .. }) => {
                    let result = evaluate_cached(&*expr.clone(), state);

                    if !state.confident {
                        return Option::None;
                    }

                    result
                }
                MemberProp::PrivateName(_) => {
                    return deopt(path, state);
                }
            };

            match expr {
                Expr::Array(ArrayLit { elems, .. }) => {
                    let Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(Number {
                        value, ..
                    })))) = propery
                    else {
                        panic!("Member not found");
                    };

                    let property = elems.get(value as usize)?.clone();

                    let Some(ExprOrSpread { expr, .. }) = property else {
                        panic!("Member not found");
                    };

                    Some(EvaluateResultValue::Expr(*expr))
                }
                Expr::Object(ObjectLit { props, .. }) => {
                    let Some(EvaluateResultValue::Expr(Expr::Ident(ident))) = propery else {
                        panic!("Member not found");
                    };

                    let property = props
                        .into_iter()
                        .find(|prop| match prop {
                            PropOrSpread::Spread(_) => {
                                todo!("Spread not implemented yet");
                            }
                            PropOrSpread::Prop(prop) => match prop.as_ref() {
                                Prop::KeyValue(key_value) => {
                                    let key = get_key_str(key_value);

                                    key == ident.sym.to_string()
                                }
                                _ => todo!("PropOrSpread"),
                            },
                        })?
                        .clone();

                    if let PropOrSpread::Prop(prop) = property {
                        dbg!(&prop, ident);
                        Some(EvaluateResultValue::Expr(*prop.key_value().unwrap().value))
                    } else {
                        panic!("Member not found");
                    }
                }
                _ => todo!("Expression"),
            }
        }
        Expr::Unary(_) => todo!("Unary not implemented yet"),
        Expr::Array(arr_path) => {
            let elems = arr_path.elems.clone();

            let mut arr: Vec<Option<EvaluateResultValue>> = vec![];

            for elem in elems.iter().filter_map(|elem| elem.clone()) {
                let elem_value = evaluate(&elem.expr, &mut state.traversal_state, &state.functions);

                if elem_value.confident {
                    arr.push(elem_value.value);
                } else {
                    // elem_value.deopt.is_some() && deopt(&elem_value.deopt.unwrap(), state);
                    return Option::None;
                }
            }

            Option::Some(EvaluateResultValue::Vec(arr))
        }
        Expr::Object(obj_path) => {
            let mut props = vec![];

            for prop in &obj_path.props {
                match prop {
                    PropOrSpread::Spread(prop) => {
                        let spread_expression = evaluate_cached(&prop.expr, state);

                        if !state.confident {
                            return deopt(path, state);
                        }

                        let new_props = &spread_expression.unwrap();
                        let new_props = new_props.as_expr().unwrap();
                        let new_props = new_props.as_object().unwrap();

                        let merged_object = deep_merge_props(props, new_props.props.clone());

                        props = merged_object;

                        continue;
                    }
                    PropOrSpread::Prop(prop) => {
                        if prop.is_method() {
                            return deopt(path, state);
                        }

                        match prop.as_ref() {
                            Prop::KeyValue(path_key_value) => {
                                let key_path = path_key_value.key.clone();

                                let key = match &key_path {
                                    PropName::Ident(ident) => {
                                        Option::Some(ident.clone().sym.to_string())
                                    }
                                    PropName::Str(str) => {
                                        Option::Some(str.value.clone().to_string())
                                    }
                                    PropName::Num(num) => Option::Some(num.value.to_string()),
                                    PropName::Computed(computed) => {
                                        let evaluated_result = evaluate(
                                            &computed.expr,
                                            &mut state.traversal_state,
                                            &state.functions,
                                        );

                                        if !evaluated_result.confident {
                                            if evaluated_result.deopt.is_some() {
                                                deopt(&evaluated_result.deopt.unwrap(), state);
                                            };

                                            return Option::None;
                                        }

                                        panic!("Check what's happening here");
                                        // Option::Some(evaluated_result.value.unwrap().as_expr())
                                    }
                                    PropName::BigInt(big_int) => {
                                        Option::Some(big_int.value.to_string())
                                    }
                                };

                                let value_path = path_key_value.value.clone();

                                let value = evaluate(
                                    &value_path,
                                    &mut state.traversal_state,
                                    &state.functions,
                                );

                                if !value.confident {
                                    if value.deopt.is_some() {
                                        deopt(&value.deopt.unwrap(), state);
                                    };

                                    return Option::None;
                                }

                                let value = value.value.unwrap();
                                // props = deep_merge_props(
                                //     props,
                                //     vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                //         KeyValueProp {
                                //             key: PropName::Ident(Ident {
                                //                 sym: key.unwrap().into(),
                                //                 span: DUMMY_SP,
                                //                 optional: false,
                                //             }),
                                //             value: Box::new(value.as_expr().unwrap().clone()),
                                //         },
                                //     )))],
                                // );

                                let value = match value {
                                    EvaluateResultValue::Expr(expr) => expr,
                                    EvaluateResultValue::Vec(items) => {
                                        let mut elems: Vec<Option<ExprOrSpread>> = vec![];

                                        for entry in items {
                                            let expr = entry
                                                .and_then(|entry| {
                                                    entry
                                                        .as_vec()
                                                        .and_then(|vec| {
                                                            let mut elems = vec![];

                                                            for item in vec {
                                                                if let Some(item) = item {
                                                                    let item =
                                                                        item.as_expr().unwrap();
                                                                    elems.push(Some(
                                                                        ExprOrSpread {
                                                                            spread: Option::None,
                                                                            expr: Box::new(
                                                                                item.clone(),
                                                                            ),
                                                                        },
                                                                    ));
                                                                }
                                                            }

                                                            Option::Some(Expr::Array(ArrayLit {
                                                                span: DUMMY_SP,
                                                                elems,
                                                            }))
                                                        })
                                                        .or_else(|| entry.as_expr().cloned())
                                                })
                                                .expect(
                                                    constants::messages::ILLEGAL_PROP_ARRAY_VALUE,
                                                );

                                            let expr = match expr {
                                                Expr::Array(array) => Expr::Array(array),
                                                Expr::Lit(lit) => Expr::Lit(lit),
                                                _ => panic!(
                                                    "{}",
                                                    constants::messages::ILLEGAL_PROP_ARRAY_VALUE,
                                                ),
                                            };

                                            elems.push(Option::Some(ExprOrSpread {
                                                spread: Option::None,
                                                expr: Box::new(expr),
                                            }));
                                        }

                                        let array = ArrayLit {
                                            span: DUMMY_SP,
                                            elems,
                                        };

                                        Expr::Array(array)
                                    }
                                    _ => panic!("Property value must be an expression"),
                                };

                                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                    KeyValueProp {
                                        key: PropName::Ident(Ident {
                                            sym: key.unwrap().into(),
                                            span: DUMMY_SP,
                                            optional: false,
                                        }),
                                        value: Box::new(value),
                                    },
                                ))));
                            }

                            _ => todo!(),
                        }
                    }
                }
            }

            let obj = ObjectLit {
                props: remove_duplicates(props.clone()),
                span: DUMMY_SP,
            };

            return Option::Some(EvaluateResultValue::Expr(Expr::Object(obj)));
        }
        Expr::Bin(bin) => {
            let result = match binary_expr_to_num(bin, state) {
                Some(num) => num as f64,
                None => panic!("Not implemented yet"),
            };
            let result = number_to_expression(result).unwrap();

            return Option::Some(EvaluateResultValue::Expr(result));
        }
        Expr::Call(call) => {
            let callee = call.callee.clone();

            let mut context: Option<Vec<Option<EvaluateResultValue>>> = Option::None;
            let mut func: Option<FunctionConfig> = Option::None;

            if let Callee::Expr(callee_expr) = callee {
                let callee_expr = callee_expr.as_ref();

                if get_binding(callee_expr, &mut state.traversal_state).is_none()
                    && is_valid_callee(callee_expr)
                {
                    panic!("{}", constants::messages::BUILT_IN_FUNCTION)
                } else if let Expr::Ident(ident) = callee_expr {
                    let ident_id = ident.to_id();

                    if state.functions.identifiers.contains_key(&ident_id) {
                        func = Option::Some(
                            state.functions.identifiers.get(&ident_id).unwrap().clone(),
                        );
                    }
                }

                if let Expr::Member(member) = callee_expr {
                    let object = member.obj.as_ref();
                    let property = &member.prop;

                    if object.is_ident() {
                        let obj_ident = object.as_ident().unwrap();

                        if property.is_ident() {
                            if is_valid_callee(object) && !is_invalid_method(property) {
                                let callee_name = get_callee_name(object);

                                let method_name = get_method_name(property);

                                match callee_name.as_str() {
                                    "Object" => {
                                        let args = call.args.clone();

                                        let Option::Some(arg) = args.get(0) else {
                                            panic!("Object.entries requires an argument")
                                        };

                                        if arg.spread.is_some() {
                                            panic!("Spread not implemented yet")
                                        }

                                        let cached_arg = evaluate_cached(&*arg.expr, state);

                                        match method_name.as_ref() {
                                            "fromEntries" => {
                                                func = Option::Some(FunctionConfig {
                                                    fn_ptr: FunctionType::Callback(
                                                        CallbackType::Object(ObjectJS::FromEntries),
                                                    ),
                                                    takes_path: false,
                                                });

                                                let mut entries_result = IndexMap::new();

                                                match cached_arg
                                                    .expect("Object.entries requires an argument")
                                                {
                                                    EvaluateResultValue::Expr(expr) => {
                                                        let array =
                                                            expr.as_array().cloned().expect(
                                                                "Object.entries requires an object",
                                                            );

                                                        let entries = array
                                                            .elems
                                                            .into_iter()
                                                            .filter_map(|item| item)
                                                            // .and_then(|items| items.flatten())
                                                            .collect::<Vec<ExprOrSpread>>();

                                                        for entry in entries {
                                                            assert!(
                                                                entry.spread.is_none(),
                                                                "Spread not implemented yet"
                                                            );

                                                            let array = entry
                                                                .expr
                                                                .as_array()
                                                                .expect("Entry must be an array");

                                                            let elems = array
                                                                .elems
                                                                .clone()
                                                                .into_iter()
                                                                .filter_map(|item| item)
                                                                .collect::<Vec<ExprOrSpread>>();

                                                            let key = elems
                                                                .get(0)
                                                                .and_then(|e| e.expr.as_lit())
                                                                .expect("Key must be a literal");

                                                            let value = elems
                                                                .get(1)
                                                                .and_then(|e| e.expr.as_lit())
                                                                .expect("Value must be a literal");

                                                            entries_result
                                                                .insert(key.clone(), value.clone());
                                                        }
                                                    }
                                                    EvaluateResultValue::Vec(vec) => {
                                                        for entry in vec {
                                                            let entry = entry
                                                                .and_then(|entry| {
                                                                    entry.as_vec().cloned()
                                                                })
                                                                .expect("Entry must be some");

                                                            let key = entry
                                                                .get(0)
                                                                .and_then(|item| item.clone())
                                                                .and_then(|item| {
                                                                    item.as_expr().cloned()
                                                                })
                                                                .and_then(|expr| {
                                                                    expr.as_lit().cloned()
                                                                })
                                                                .expect("Key must be a literal");

                                                            let value = entry
                                                                .get(1)
                                                                .and_then(|item| item.clone())
                                                                .and_then(|item| {
                                                                    item.as_expr().cloned()
                                                                })
                                                                .and_then(|expr| {
                                                                    expr.as_lit().cloned()
                                                                })
                                                                .expect("Value must be a literal");

                                                            entries_result.insert(
                                                                key.clone().clone(),
                                                                value.clone().clone(),
                                                            );
                                                        }
                                                    }
                                                    _ => {
                                                        panic!("Object.entries requires an object")
                                                    }
                                                };

                                                context = Option::Some(vec![Option::Some(
                                                    EvaluateResultValue::Entries(entries_result),
                                                )]);
                                            }
                                            "keys" => {
                                                func = Option::Some(FunctionConfig {
                                                    fn_ptr: FunctionType::Callback(
                                                        CallbackType::Object(ObjectJS::Keys),
                                                    ),
                                                    takes_path: false,
                                                });

                                                let object = cached_arg
                                                    .and_then(|arg| arg.as_expr().cloned())
                                                    .and_then(|expr| expr.as_object().cloned())
                                                    .expect("Object.entries requires an object");

                                                let mut keys = vec![];

                                                for prop in &object.props {
                                                    let expr = prop
                                                        .as_prop()
                                                        .and_then(|prop| {
                                                            Option::Some(*prop.clone())
                                                        })
                                                        .expect("Spread not implemented yet");

                                                    let key_values = expr.as_key_value().expect(
                                                        "Object.entries requires an object",
                                                    );

                                                    let key = get_key_str(key_values);

                                                    keys.push(Option::Some(ExprOrSpread {
                                                        spread: Option::None,
                                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                                            span: DUMMY_SP,
                                                            value: key.into(),
                                                            raw: Option::None,
                                                        }))),
                                                    }));
                                                }

                                                context = Option::Some(vec![Option::Some(
                                                    EvaluateResultValue::Expr(Expr::Array(
                                                        ArrayLit {
                                                            span: DUMMY_SP,
                                                            elems: keys,
                                                        },
                                                    )),
                                                )]);
                                            }
                                            "values" => {
                                                func = Option::Some(FunctionConfig {
                                                    fn_ptr: FunctionType::Callback(
                                                        CallbackType::Object(ObjectJS::Values),
                                                    ),
                                                    takes_path: false,
                                                });

                                                let object = cached_arg
                                                    .and_then(|arg| arg.as_expr().cloned())
                                                    .and_then(|expr| expr.as_object().cloned())
                                                    .expect("Object.entries requires an object");

                                                let mut values = vec![];

                                                for prop in &object.props {
                                                    let expr = prop
                                                        .as_prop()
                                                        .and_then(|prop| {
                                                            Option::Some(*prop.clone())
                                                        })
                                                        .expect("Spread not implemented yet");

                                                    let key_values = expr.as_key_value().expect(
                                                        "Object.entries requires an object",
                                                    );

                                                    let value = key_values
                                                        .value
                                                        .as_lit()
                                                        .expect("Object value should be a literal");

                                                    values.push(Option::Some(ExprOrSpread {
                                                        spread: Option::None,
                                                        expr: Box::new(Expr::Lit(value.clone())),
                                                    }));
                                                }

                                                context = Option::Some(vec![Option::Some(
                                                    EvaluateResultValue::Expr(Expr::Array(
                                                        ArrayLit {
                                                            span: DUMMY_SP,
                                                            elems: values,
                                                        },
                                                    )),
                                                )]);
                                            }
                                            "entries" => {
                                                func = Option::Some(FunctionConfig {
                                                    fn_ptr: FunctionType::Callback(
                                                        CallbackType::Object(ObjectJS::Entries),
                                                    ),
                                                    takes_path: false,
                                                });

                                                let object = cached_arg
                                                    .and_then(|arg| arg.as_expr().cloned())
                                                    .and_then(|expr| expr.as_object().cloned())
                                                    .expect("Object.entries requires an object");

                                                let mut entries: IndexMap<Lit, Lit> =
                                                    IndexMap::new();

                                                for prop in &object.props {
                                                    let expr = prop
                                                        .as_prop()
                                                        .and_then(|prop| {
                                                            Option::Some(*prop.clone())
                                                        })
                                                        .expect("Spread not implemented yet");

                                                    let key_values = expr.as_key_value().expect(
                                                        "Object.entries requires an object",
                                                    );

                                                    let value = key_values
                                                        .value
                                                        .as_lit()
                                                        .expect("Object value should be a literal");

                                                    let key = get_key_str(key_values);

                                                    entries.insert(
                                                        Lit::Str(Str {
                                                            span: DUMMY_SP,
                                                            value: key.into(),
                                                            raw: Option::None,
                                                        }),
                                                        value.clone(),
                                                    );
                                                }

                                                context = Option::Some(vec![Option::Some(
                                                    EvaluateResultValue::Entries(entries),
                                                )]);
                                            }
                                            _ => {
                                                panic!("{}", constants::messages::BUILT_IN_FUNCTION)
                                            }
                                        }
                                    }
                                    _ => panic!("{}", constants::messages::BUILT_IN_FUNCTION),
                                }
                            } else {
                                // let memberExpressionFn = state
                                //     .functions
                                //     .member_expressions
                                //     .get(&ImportSources::Regular(obj_ident.sym.to_string()));

                                // if let Some(memberExpressionFn) = memberExpressionFn {

                                // }

                                // dbg!(&state.functions.member_expressions);
                                // panic!("{}", constants::messages::BUILT_IN_FUNCTION);
                                let prop_ident = property.as_ident().unwrap();

                                let obj_name = obj_ident.sym.to_string();
                                let prop_id = prop_ident.to_id();

                                let member_expressions = state
                                    .functions
                                    .member_expressions
                                    .get(&ImportSources::Regular(obj_name.clone()));

                                if let Some(member_expr) = member_expressions {
                                    let member_expr = member_expr.clone();
                                    if let Some(member_expr_fn) = member_expr.get(&prop_id) {
                                        dbg!(&member_expr, &member_expr_fn,);

                                        // panic!();
                                        // context = Option::Some(vec![Option::Some(EvaluateResultValue::Expr(
                                        //     member_expr_fn.clone(),
                                        // ))]);
                                        func = Option::Some(
                                            member_expr.get(&prop_id).unwrap().clone(),
                                        );
                                    }
                                }
                            }
                        }

                        if let Option::Some(prop_id) = is_id_prop(property) {
                            let obj_name = obj_ident.sym.to_string();

                            if state
                                .functions
                                .member_expressions
                                .contains_key(&ImportSources::Regular(obj_name.clone()))
                            {
                                let member_expr = state
                                    .functions
                                    .member_expressions
                                    .get(&ImportSources::Regular(obj_name))
                                    .unwrap();
                                let member_expr = member_expr.clone();

                                if member_expr.contains_key(&prop_id) {
                                    todo!("Check what's happening here");

                                    // context = Option::Some(member_expr.clone());
                                    func = Option::Some(member_expr.get(&prop_id).unwrap().clone());
                                }
                            }
                        }
                    }

                    if object.is_lit() {
                        let obj_lit = object.as_lit().unwrap();

                        if property.is_ident() {
                            let prop_ident = property.as_ident().unwrap();
                            let prop_name = prop_ident.sym.to_string();

                            match obj_lit {
                                Lit::Str(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                                Lit::Bool(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                                _ => {}
                            }
                        }
                    }

                    let parsed_obj = evaluate(object, &mut state.traversal_state, &state.functions);

                    // println!("!!!!__ obj: {:#?}, parsed_obj: {:#?}", obj, parsed_obj);

                    if parsed_obj.confident {
                        if property.is_ident() {
                            let prop_ident = property.as_ident().unwrap().clone();
                            let prop_name = prop_ident.sym.to_string();

                            let value = parsed_obj.value.unwrap();

                            match value.clone() {
                                EvaluateResultValue::Map(map) => {
                                    let result_fn = map.get(&Expr::Ident(prop_ident.clone()));

                                    func = match result_fn {
                                        Some(_) => panic!("Not implemented yet"),
                                        None => Option::None,
                                    };
                                }
                                EvaluateResultValue::Vec(expr) => {
                                    func = Option::Some(FunctionConfig {
                                        fn_ptr: FunctionType::Callback(
                                            match prop_name.as_str() {
                                                "map" => CallbackType::Array(ArrayJS::Map),
                                                "filter" => CallbackType::Array(ArrayJS::Filter),
                                                "entries" => {
                                                    CallbackType::Object(ObjectJS::Entries)
                                                }
                                                _ => todo!(
                                                    "Array method '{}' implemented yet",
                                                    prop_name
                                                ),
                                            },
                                            // obj.clone(),
                                        ),
                                        takes_path: false,
                                    });

                                    // panic!("Array method not implemented yet, {:#?}",expr);

                                    context = Option::Some(expr)
                                }
                                EvaluateResultValue::Expr(expr) => match expr {
                                    Expr::Array(ArrayLit { elems, .. }) => {
                                        func = Option::Some(FunctionConfig {
                                            fn_ptr: FunctionType::Callback(
                                                match prop_name.as_str() {
                                                    "map" => CallbackType::Array(ArrayJS::Map),
                                                    "filter" => {
                                                        CallbackType::Array(ArrayJS::Filter)
                                                    }
                                                    "entries" => {
                                                        CallbackType::Object(ObjectJS::Entries)
                                                    }
                                                    _ => todo!(
                                                        "Method '{}' implemented yet",
                                                        prop_name
                                                    ),
                                                },
                                                // obj.clone(),
                                            ),
                                            takes_path: false,
                                        });

                                        let expr = elems
                                            .into_iter()
                                            .map(|elem| {
                                                Option::Some(EvaluateResultValue::Expr(
                                                    *elem.unwrap().expr.clone(),
                                                ))
                                            })
                                            .collect::<Vec<Option<EvaluateResultValue>>>();
                                        // panic!("Array method not implemented yet, {:#?}",expr);

                                        context = Option::Some(vec![Option::Some(
                                            EvaluateResultValue::Vec(expr),
                                        )]);
                                    }
                                    _ => {}
                                },
                                _ => {
                                    println!("!!!!__ Evaluation result value: {:#?}", value);
                                    panic!("Evaluation result not implemented yet")
                                }
                            }
                        } else if let Option::Some(prop_id) = is_id_prop(property) {
                            let prop_id = prop_id.clone();
                            let value = parsed_obj.value.unwrap();
                            let map = value.as_map().unwrap();

                            let result_fn = map.get(&Expr::Lit(Lit::Str(Str {
                                value: prop_id.clone().0,
                                raw: Option::None,
                                span: DUMMY_SP,
                            })));

                            func = match result_fn {
                                Some(_) => panic!("Not implemented yet"),
                                None => Option::None,
                            };
                        }
                    }
                }
            }

            if let Some(func) = func {
                if func.takes_path {
                    let args = call
                        .args
                        .clone()
                        .into_iter()
                        .filter_map(|arg| Option::Some(*arg.expr))
                        .collect::<Vec<Expr>>();

                    match func.fn_ptr {
                        FunctionType::ArrayArgs(func) => {
                            let func_result = (func)(args.clone());

                            dbg!(&func_result, &args);
                            return Option::Some(EvaluateResultValue::Expr(func_result));
                        }
                        FunctionType::StylexFns(func) => {
                            let func_result =
                                (func)(args.get(0).unwrap().clone(), state.traversal_state.clone());
                            state.traversal_state = func_result.1;
                            return Option::Some(EvaluateResultValue::Expr(func_result.0));
                        }
                        FunctionType::Callback(_) => {
                            panic!("Arrow function not implemented yet");
                            // let func_result = (cb)(args.get(0).unwrap().clone());
                            // return Option::Some(EvaluateResultValue::Expr(func_result));
                        }
                        FunctionType::Mapper(_) => {
                            panic!("Mapper not implemented yet");
                            // let func_result = (cb)(args.get(0).unwrap().clone());
                            // return Option::Some(EvaluateResultValue::Expr(func_result));
                        }
                    }
                } else {
                    let args: Vec<EvaluateResultValue> = call
                        .args
                        .clone()
                        .into_iter()
                        .filter_map(|arg| {
                            let cached_arg = evaluate_cached(&arg.expr, state);

                            println!("!!!!__ cached_arg: {:#?}, arg: {:#?}", cached_arg, arg);
                            cached_arg
                        })
                        .collect();

                    if !state.confident {
                        return Option::None;
                    }

                    match func.fn_ptr {
                        FunctionType::ArrayArgs(func) => {
                            let func_result = (func)(
                                args.into_iter()
                                    .map(|arg| arg.as_expr().unwrap().clone())
                                    .collect(),
                            );
                            return Option::Some(EvaluateResultValue::Expr(func_result));
                        }
                        FunctionType::StylexFns(func) => {
                            let func_result = (func)(
                                args.get(0).unwrap().clone().as_expr().unwrap().clone(),
                                state.traversal_state.clone(),
                            );

                            state.traversal_state = func_result.1;

                            return Option::Some(EvaluateResultValue::Expr(func_result.0));
                        }
                        FunctionType::Callback(func) => {
                            let context = context.expect("Object.entries requires a context");

                            match func {
                                CallbackType::Array(ArrayJS::Map) => {
                                    return evaluate_map(&args, &context);
                                }
                                CallbackType::Array(ArrayJS::Filter) => {
                                    return evaluate_filter(&args, &context);
                                }
                                CallbackType::Object(ObjectJS::Entries) => {
                                    let Some(Some(EvaluateResultValue::Entries(entries))) =
                                        context.get(0)
                                    else {
                                        panic!("Object.entries requires an argument")
                                    };

                                    let mut entry_elems: Vec<Option<ExprOrSpread>> = vec![];

                                    for (key, value) in entries {
                                        let key: ExprOrSpread = ExprOrSpread {
                                            spread: Option::None,
                                            expr: Box::new(Expr::Lit(key.clone())),
                                        };

                                        let value: ExprOrSpread = ExprOrSpread {
                                            spread: Option::None,
                                            expr: Box::new(Expr::Lit(value.clone())),
                                        };

                                        entry_elems.push(Option::Some(ExprOrSpread {
                                            spread: Option::None,
                                            expr: Box::new(Expr::Array(ArrayLit {
                                                span: DUMMY_SP,
                                                elems: vec![Option::Some(key), Option::Some(value)],
                                            })),
                                        }));
                                    }

                                    return Option::Some(EvaluateResultValue::Expr(Expr::Array(
                                        ArrayLit {
                                            span: DUMMY_SP,
                                            elems: entry_elems,
                                        },
                                    )));
                                }
                                CallbackType::Object(ObjectJS::Keys) => {
                                    let Some(Some(EvaluateResultValue::Expr(keys))) =
                                        context.get(0)
                                    else {
                                        panic!("Object.keys requires an argument")
                                    };

                                    return Option::Some(EvaluateResultValue::Expr(keys.clone()));
                                }
                                CallbackType::Object(ObjectJS::Values) => {
                                    let Some(Some(EvaluateResultValue::Expr(values))) =
                                        context.get(0)
                                    else {
                                        panic!("Object.values requires an argument")
                                    };

                                    return Option::Some(EvaluateResultValue::Expr(values.clone()));
                                }
                                CallbackType::Object(ObjectJS::FromEntries) => {
                                    let Some(Some(EvaluateResultValue::Entries(entries))) =
                                        context.get(0)
                                    else {
                                        panic!("Object.fromEntries requires an argument")
                                    };

                                    let mut entry_elems = vec![];

                                    for (key, value) in entries {
                                        let ident = if let Lit::Str(lit_str) = key {
                                            Ident::new(lit_str.value.clone(), DUMMY_SP)
                                        } else {
                                            panic!("Expected a string literal")
                                        };

                                        let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                            KeyValueProp {
                                                key: PropName::Ident(ident),
                                                value: Box::new(Expr::Lit(value.clone())),
                                            },
                                        )));

                                        entry_elems.push(prop);
                                    }

                                    return Option::Some(EvaluateResultValue::Expr(
                                        object_expression_factory(entry_elems)
                                            .expect("Object creation failed"),
                                    ));
                                }
                            }
                        }
                        _ => panic!("Function type not implemented yet"),
                    }
                }
            }

            // let args = call
            //     .args
            //     .clone()
            //     .into_iter()
            //     .map(|a| a.expr.as_ref().clone())
            //     .collect();
            // let elems = (state.functions.identifiers.get("makeArray").unwrap().fn_ptr)(args);
            return deopt(path, state);
        }
        _ => {
            println!("!!!!__ path_not_implemented: {:#?}", path);
            panic!("Not implemented yet, return something");
        }
    };

    if result.is_none() && path.is_ident() {
        let ident = path.as_ident().expect("Identifier not found");

        let binding = get_var_decl_by_ident(
            ident,
            &mut state.traversal_state,
            &state.functions,
            VarDeclAction::Reduce,
        );

        match binding {
            Some(binding) => {
                if path.eq(&Expr::Ident(binding.name.as_ident().unwrap().id.clone())) {
                    todo!("Check what's happening here")
                }
                eprintln!("{}", Colorize::yellow("!!!! binding: {:#?} !!!!"));
                dbg!(&binding.init);

                return evaluate_cached(&*binding.init.expect("Binding nof found").clone(), state);
            }
            None => {
                let name = ident.sym.to_string();

                if name == "undefined" || name == "infinity" || name == "NaN" {
                    return Option::Some(EvaluateResultValue::Expr(Expr::Ident(ident.clone())));
                }
            }
        }
    }

    if result.is_none() {
        deopt(path, state);
    }

    result
}

fn get_binding(callee: &Expr, state: &mut StateManager) -> Option<VarDeclarator> {
    match callee {
        Expr::Ident(ident) => get_var_decl_from(state, &ident).cloned(),
        _ => Option::None,
    }
}

fn is_valid_callee(callee: &Expr) -> bool {
    match callee {
        Expr::Ident(ident) => {
            let name = ident.sym.to_string();
            VALID_CALLEES.contains(name.as_str())
        }
        _ => false,
    }
}

fn get_callee_name(callee: &Expr) -> String {
    match callee {
        Expr::Ident(ident) => ident.sym.to_string(),
        _ => panic!("Callee is not an identifier"),
    }
}

fn is_invalid_method(prop: &MemberProp) -> bool {
    match prop {
        MemberProp::Ident(ident_prop) => {
            INVALID_METHODS.contains(ident_prop.sym.to_string().as_str())
        }
        _ => false,
    }
}

fn get_method_name(prop: &MemberProp) -> String {
    match prop {
        MemberProp::Ident(ident_prop) => ident_prop.sym.to_string(),
        _ => panic!("Method is not an identifier"),
    }
}

fn is_id_prop(prop: &MemberProp) -> Option<Id> {
    match prop {
        MemberProp::Computed(comp_prop) => match comp_prop.expr.as_ref() {
            Expr::Lit(Lit::Str(str)) => Option::Some(str.value.to_id()),
            _ => Option::None,
        },
        _ => Option::None,
    }
}

pub(crate) fn evaluate_quasis(
    tpl_expr: &Expr,
    quasis: &Vec<TplElement>,
    raw: bool,
    state: &mut State,
) -> Option<EvaluateResultValue> {
    let mut str = "".to_string();

    let mut i = 0;

    let exprs = match tpl_expr {
        Expr::Tpl(tpl) => tpl.exprs.clone(),
        Expr::TaggedTpl(tagged_tpl) => tagged_tpl.tpl.exprs.clone(),
        _ => panic!("The expression is not a template"),
    };

    for elem in quasis {
        if !state.confident {
            return Option::None;
        };

        str += (if raw {
            elem.raw.to_string()
        } else {
            elem.cooked
                .clone()
                .expect("Cooked should be some")
                .to_string()
        })
        .as_str();

        let expr = exprs.get(i);
        i = i + 1;

        if let Some(expr) = expr {
            let evaluated_expr = evaluate_cached(expr, state);

            if let Some(expr) = evaluated_expr {
                let expr = expr.as_expr().expect("Expression not found");

                let lit = expr.as_lit().expect("Literal not found");

                let lit_str = get_string_val_from_lit(lit);

                str += &lit_str;
            }
        }
    }

    if !state.confident {
        return Option::None;
    };

    Option::Some(EvaluateResultValue::Expr(string_to_expression(str)?))
}

pub(crate) fn evaluate_cached(path: &Expr, state: &mut State) -> Option<EvaluateResultValue> {
    let existing = state.seen.get(&path);

    match existing {
        Some(value) => {
            // panic!("Should not be here");
            if value.resolved {
                let a = value.value.clone();
                return a;
            }
            deopt(path, state)
            // value.value.unwrap().clone()
        }
        None => {
            let item = SeenValue {
                value: Option::None,
                resolved: false,
            };
            state.seen.insert(path.clone(), item);

            let val = _evaluate(path, state);

            if state.confident {
                state.seen.insert(
                    path.clone(),
                    SeenValue {
                        value: val.clone(),
                        resolved: true,
                    },
                );
            }

            val
        }
    }
}

fn stylex_keyframes(
    animation: HashMap<String, HashMap<String, String>>,
    options: &StyleXStateOptions,
) -> (String, InjectableStyle) {
    panic!(
        "{}",
        Colorize::yellow("!!!! stylex_keyframes not implemented yet !!!!")
    );

    // (
    //     "".to_string(),
    //     InjectableStyle {
    //         ltr: "".to_string(),
    //         priority: None,
    //         rtl: None,
    //     },
    // )
}

fn keyframes(
    animation: HashMap<String, HashMap<String, String>>,
    state: &mut StateManager,
) -> String {
    panic!(
        "{}",
        Colorize::yellow("!!!! keyframes not implemented yet !!!!")
    );
    // let (animation_name, injected_style) = stylex_keyframes(animation, &state.options);
    // state.stylex_keyframes_import.insert(animation_name.clone());
    // animation_name
}
