use core::panic;
// use sha2::{Digest, Sha256};
use std::collections::HashMap;

use colored::Colorize;
use indexmap::IndexMap;
use swc_core::{
    common::{Span, DUMMY_SP},
    ecma::{
        ast::{
            Callee, Expr, Id, Ident, KeyValueProp, Lit, MemberProp, ObjectLit, Prop, PropName,
            PropOrSpread, Str, VarDeclarator,
        },
        utils::ExprExt,
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
    structures::{
        evaluate_result::{EvaluateResult, EvaluateResultValue},
        functions::{FunctionConfig, FunctionMap, Functions},
        injectable_style::InjectableStyle,
        named_import_source::ImportSources,
        pre_rule::{CompiledResult, PreRule, PreRules},
        state_manager::StateManager,
        stylex_options::StyleXOptions,
        stylex_state_options::StyleXStateOptions,
    },
    utils::{
        common::{
            binary_expr_to_num, deep_merge_props, expr_to_str, get_var_decl_by_ident,
            get_var_decl_from, number_to_expression, remove_duplicates,
        },
        css::flatten_raw_style_object,
        validators::{validate_dynamic_style_params, validate_namespace},
    },
};

pub(crate) fn stylex_create(
    namespaces: &EvaluateResultValue,
    prefix: &str,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    options: &StyleXStateOptions,
) -> (
    IndexMap<String, IndexMap<String, Option<String>>>,
    IndexMap<String, InjectableStyle>,
) {
    let mut resolved_namespaces: IndexMap<String, IndexMap<String, Option<String>>> =
        IndexMap::new();
    let mut injected_styles_map: IndexMap<String, InjectableStyle> = IndexMap::new();

    for (namespace_name, namespace) in namespaces.as_map().unwrap() {
        validate_namespace(&namespace);

        let mut pseudos = vec![];
        let mut at_rules = vec![];

        let flattened_namespace = flatten_raw_style_object(
            namespace,
            declarations,
            var_dec_count_map,
            &mut pseudos,
            &mut at_rules,
            options,
        );

        let compiled_namespace_tuples = flattened_namespace
            .iter()
            .map(|(key, value)| match value {
                PreRules::PreRuleSet(rule_set) => {
                    (key.to_string(), rule_set.clone().compiled(prefix))
                }
                PreRules::StylesPreRule(styles_pre_rule) => {
                    (key.to_string(), styles_pre_rule.clone().compiled(prefix))
                }
                PreRules::NullPreRule(rule_set) => {
                    (key.to_string(), rule_set.clone().compiled(prefix))
                }
            })
            .collect::<Vec<(String, CompiledResult)>>();

        let compiled_namespace = compiled_namespace_tuples
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    match value {
                        CompiledResult::ComputedStyles(styles) => {
                            CompiledResult::ComputedStyles(styles.clone())
                        }
                        CompiledResult::Null(null) => CompiledResult::Null(null.clone()),
                        _ => todo!("handle other cases"),
                    },
                )
            })
            .collect::<IndexMap<String, CompiledResult>>();

        let mut namespace_obj: IndexMap<String, Option<String>> = IndexMap::new();
        for key in compiled_namespace.keys() {
            let value = compiled_namespace.get(key).unwrap();

            if let Some(included_styles) = value.as_included_style() {
                todo!("handle included style")
            } else if let Some(styles) = value.as_computed_styles() {
                let class_name_tuples = styles.clone();

                let class_name = &class_name_tuples
                    .iter()
                    .map(|computed_style| {
                        let class_name = computed_style.0.clone();

                        class_name
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                namespace_obj.insert(key.clone(), Some(class_name.clone()));

                for item in &class_name_tuples {
                    let class_name = item.0.clone();
                    let injectable_styles = item.1.clone();
                    if !injected_styles_map.contains_key(class_name.as_str()) {
                        injected_styles_map.insert(class_name.clone(), injectable_styles.clone());
                    }
                }
            } else {
                namespace_obj.insert(key.clone(), Option::None);
            }
        }

        resolved_namespaces.insert(
            expr_to_str(namespace_name, declarations, var_dec_count_map),
            namespace_obj,
        );
    }

    (resolved_namespaces, injected_styles_map)
}

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
}

pub(crate) fn evaluate_style_x_create_arg(
    path: &Expr,
    traversal_state: &StateManager,
    functions: &FunctionMap,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> EvaluateResult {
    match path {
        Expr::Object(object) => {
            let mut style_object = object.clone();

            let mut value: IndexMap<Expr, Vec<KeyValueProp>> = IndexMap::new();
            let mut fns: IndexMap<String, (Vec<Expr>, IndexMap<String, Expr>)> = IndexMap::new();

            for prop in &mut style_object.props {
                match prop {
                    PropOrSpread::Spread(_) => todo!("Spread not implemented yet"),
                    PropOrSpread::Prop(prop) => {
                        let obj_prop_path = &prop.clone();

                        match prop.as_ref() {
                            Prop::KeyValue(key_value_prop) => {
                                let key_result = evaluate_obj_key(
                                    key_value_prop,
                                    traversal_state,
                                    functions,
                                    declarations,
                                    var_dec_count_map,
                                );

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
                                        let val = evaluate(
                                            value_path,
                                            traversal_state,
                                            functions,
                                            declarations,
                                            var_dec_count_map,
                                        );

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
                                                _ => panic!("Unexpected value type"),
                                            },
                                            _ => panic!("Unexpected value type"),
                                        };

                                        value.insert(key.as_expr().clone(), value_to_insert);

                                        continue;
                                    }
                                }
                            }
                            _ => {
                                return evaluate(
                                    path,
                                    traversal_state,
                                    functions,
                                    declarations,
                                    var_dec_count_map,
                                );
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
        _ => evaluate(
            path,
            traversal_state,
            functions,
            declarations,
            var_dec_count_map,
        ),
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

fn evaluate_obj_key(
    prop_kv: &KeyValueProp,
    traversal_state: &StateManager,
    functions: &FunctionMap,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
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
            let computed_result = evaluate(
                computed_path,
                traversal_state,
                functions,
                declarations,
                var_dec_count_map,
            );
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

    EvaluateResult {
        confident: true,
        deopt: Option::None,
        value: Option::Some(EvaluateResultValue::Expr(key.clone())),
    }
}
pub fn evaluate(
    path: &Expr,
    traversal_state: &StateManager,
    fns: &FunctionMap,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> EvaluateResult {
    let mut state = State {
        confident: true,
        deopt_path: None,
        seen: HashMap::new(),
        // added_imports: HashSet::new(),
        functions: fns.clone(),
        traversal_state: traversal_state.clone(),
    };

    let mut value = evaluate_cached(path, &mut state, declarations, var_dec_count_map);

    if !state.confident {
        value = Option::None;
    }

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

fn _evaluate(
    path: &Expr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> Option<EvaluateResultValue> {
    if !state.confident {
        panic!("Should not be here");
        // return Option::None;
    }

    let result = match path {
        Expr::Arrow(_) => todo!("Arrow function not implemented yet"),
        Expr::Ident(ident) => {
            let name = ident.sym.to_string();
            if state.functions.identifiers.contains_key(&name) {
                panic!("Not implemented yet");
            }

            Option::None
        }
        Expr::TsAs(_) => todo!("TsAs not implemented yet"),
        Expr::TsSatisfies(_) => todo!("TsSatisfies not implemented yet"),
        Expr::Seq(_) => todo!("Seq not implemented yet"),
        Expr::Lit(lit_path) => Option::Some(EvaluateResultValue::Expr(Expr::Lit(lit_path.clone()))),
        Expr::Tpl(_) => todo!("Tpl not implemented yet"),
        Expr::TaggedTpl(_) => todo!("TaggedTpl not implemented yet"),
        Expr::Cond(_) => todo!("Cond not implemented yet"),
        Expr::Paren(paren) => {
            let result = evaluate_cached(&paren.expr, state, declarations, var_dec_count_map);

            result
        }
        Expr::Member(_) => todo!("Member not implemented yet"),
        Expr::Unary(_) => todo!("Unary not implemented yet"),
        Expr::Array(arr_path) => {
            let elems = arr_path.elems.clone();

            let mut arr: Vec<Option<EvaluateResultValue>> = vec![];

            for elem in elems.iter().filter_map(|elem| elem.clone()) {
                let elem_value = evaluate(
                    &elem.expr,
                    &state.traversal_state,
                    &state.functions,
                    declarations,
                    var_dec_count_map,
                );

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
                        let spread_expression =
                            evaluate_cached(&prop.expr, state, declarations, var_dec_count_map);

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
                                            &state.traversal_state,
                                            &state.functions,
                                            declarations,
                                            var_dec_count_map,
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
                                    &state.traversal_state,
                                    &state.functions,
                                    declarations,
                                    var_dec_count_map,
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

                                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                    KeyValueProp {
                                        key: PropName::Ident(Ident {
                                            sym: key.unwrap().into(),
                                            span: DUMMY_SP,
                                            optional: false,
                                        }),
                                        value: Box::new(value.as_expr().unwrap().clone()),
                                    },
                                ))));
                            }

                            _ => todo!(),
                        }
                    }
                }
            }

            println!("!!!!__ props: {:#?}", props);

            let obj = ObjectLit {
                props: remove_duplicates(props.clone()),
                span: DUMMY_SP,
            };

            return Option::Some(EvaluateResultValue::Expr(Expr::Object(obj)));
        }
        Expr::Bin(bin) => {
            let result = match binary_expr_to_num(bin, state, declarations, var_dec_count_map) {
                Some(num) => num as f64,
                None => panic!("Not implemented yet"),
            };
            let result = number_to_expression(result).unwrap();

            return Option::Some(EvaluateResultValue::Expr(result));
        }
        Expr::Call(call) => {
            let callee = call.callee.clone();

            let mut context: Option<HashMap<String, FunctionConfig>> = Option::None;
            let mut func: Option<FunctionConfig> = Option::None;

            if let Callee::Expr(callee_expr) = callee {
                let callee_expr = callee_expr.as_ref();

                if get_binding(callee_expr, &declarations).is_none() && is_valid_callee(callee_expr)
                {
                    panic!("{}", constants::messages::BUILT_IN_FUNCTION)
                } else if let Expr::Ident(ident) = callee_expr {
                    let name = ident.sym.to_string();
                    if state.functions.identifiers.contains_key(&name) {
                        func =
                            Option::Some(state.functions.identifiers.get(&name).unwrap().clone());
                    }
                }

                if let Expr::Member(member) = callee_expr {
                    let obj = member.obj.as_ref();
                    let prop = &member.prop;

                    if obj.is_ident() {
                        let obj_ident = obj.as_ident().unwrap();

                        if prop.is_ident() {
                            if is_valid_callee(obj) && !is_invalid_method(prop) {
                                panic!("{}", constants::messages::BUILT_IN_FUNCTION)
                            } else if obj.is_ident() && prop.is_ident() {
                                let prop_ident = prop.as_ident().unwrap();

                                let obj_name = obj_ident.sym.to_string();
                                let prop_name = prop_ident.sym.to_string();

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

                                    if member_expr.contains_key(&prop_name) {
                                        context = Option::Some(member_expr.clone());
                                        func = Option::Some(
                                            member_expr.get(&prop_name).unwrap().clone(),
                                        );
                                    }
                                }
                            }
                        }

                        if let Option::Some(prop_name) = is_string_prop(prop) {
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

                                if member_expr.contains_key(&prop_name) {
                                    context = Option::Some(member_expr.clone());
                                    func =
                                        Option::Some(member_expr.get(&prop_name).unwrap().clone());
                                }
                            }
                        }
                    }

                    if obj.is_lit() {
                        let obj_lit = obj.as_lit().unwrap();

                        if prop.is_ident() {
                            let prop_ident = prop.as_ident().unwrap();
                            let prop_name = prop_ident.sym.to_string();

                            match obj_lit {
                                Lit::Str(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                                Lit::Bool(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                                _ => {}
                            }
                        }
                    }

                    let parsed_obj = evaluate(
                        obj,
                        &state.traversal_state,
                        &state.functions,
                        declarations,
                        var_dec_count_map,
                    );

                    if parsed_obj.confident {
                        if prop.is_ident() {
                            let prop_ident = prop.as_ident().unwrap().clone();
                            let prop_name = prop_ident.sym.to_string();

                            let value = parsed_obj.value.unwrap();
                            let map = value.as_map().unwrap();

                            let result_fn = map.get(&Expr::Ident(prop_ident.clone()));

                            func = match result_fn {
                                Some(_) => panic!("Not implemented yet"),
                                None => Option::None,
                            };
                        } else if let Option::Some(prop_name) = is_string_prop(prop) {
                            let prop_name = prop_name.clone();
                            let value = parsed_obj.value.unwrap();
                            let map = value.as_map().unwrap();

                            let result_fn = map.get(&Expr::Lit(Lit::Str(Str {
                                value: prop_name.clone().into(),
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
                    let args = call.args.clone();
                    todo!("Function takes path not implemented yet")
                } else {
                    let args = call
                        .args
                        .clone()
                        .into_iter()
                        .filter_map(|arg| {
                            let cached_arg =
                                evaluate_cached(&arg.expr, state, declarations, var_dec_count_map);

                            if cached_arg.is_none() {
                                return Option::None;
                            }

                            cached_arg.unwrap().as_expr().cloned()
                        })
                        .collect();

                    if !state.confident {
                        return Option::None;
                    }

                    let func_result = (func.fn_ptr)(args);

                    return Option::Some(EvaluateResultValue::Expr(func_result));
                }
            }

            // let args = call
            //     .args
            //     .clone()
            //     .into_iter()
            //     .map(|a| a.expr.as_ref().clone())
            //     .collect();
            // let elems = (state.functions.identifiers.get("makeArray").unwrap().fn_ptr)(args);
            return Option::None;
        }
        _ => {
            panic!("Not implemented yet, return something");
        }
    };

    if result.is_none() && path.is_ident() {
        let ident = path.as_ident().unwrap();
        let binding = get_var_decl_by_ident(ident, declarations, var_dec_count_map);

        match binding {
            Some(_) => todo!("Binding not implemented yet"),
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

fn get_binding(callee: &Expr, declarations: &Vec<VarDeclarator>) -> Option<VarDeclarator> {
    match callee {
        Expr::Ident(ident) => get_var_decl_from(&declarations, &ident).cloned(),
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

fn is_invalid_method(prop: &MemberProp) -> bool {
    match prop {
        MemberProp::Ident(ident_prop) => {
            INVALID_METHODS.contains(ident_prop.sym.to_string().as_str())
        }
        _ => false,
    }
}

fn is_string_prop(prop: &MemberProp) -> Option<String> {
    match prop {
        MemberProp::Computed(comp_prop) => match comp_prop.expr.as_ref() {
            Expr::Lit(Lit::Str(str)) => Option::Some(str.value.to_string().clone()),
            _ => Option::None,
        },
        _ => Option::None,
    }
}

pub(crate) fn evaluate_cached(
    path: &Expr,
    state: &mut State,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
) -> Option<EvaluateResultValue> {
    // let seen = &mut state.seen;

    let existing = state.seen.get(&path);

    match existing {
        Some(_value) => {
            panic!("Should not be here");
            // if value.resolved {
            //     return value.value.unwrap().clone();
            // }
            // deopt(path, state)
            // // value.value.unwrap().clone()
        }
        None => {
            let item = SeenValue {
                value: Option::None,
                resolved: false,
            };
            state.seen.insert(path.clone(), item);

            let val = _evaluate(path, state, declarations, var_dec_count_map);

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
