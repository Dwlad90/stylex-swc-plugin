// use sha2::{Digest, Sha256};
use std::{collections::HashMap, hash::Hash, rc::Weak};

use colored::Colorize;
use indexmap::IndexMap;
use swc_core::{
    common::{util::take::Take, Span, DUMMY_SP},
    ecma::{
        ast::{
            Expr, Id, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
            VarDeclarator,
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
    structures::{
        evaluate_result::{EvaluateResult, EvaluateResultValue},
        functions::{FunctionMap, Functions},
        injectable_style::{InjectableStyle, InjectableStyleBase},
        pre_rule::{CompiledResult, PreRule, PreRules},
        state_manager::StateManager,
        stylex_state_options::StyleXStateOptions,
    },
    utils::{
        common::{
            deep_merge_props, expr_to_str, expr_tpl_to_string, get_key_str,
            object_expression_factory,
        },
        css::flatten_raw_style_object,
        validators::{
            validate_and_return_property, validate_dynamic_style_params, validate_namespace,
        },
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
        println!("!!!!__ namespace: {:#?}", namespace);

        // let css_property = get_key_str(property);

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
        println!("!!!!__ flattened_namespace: {:#?}", flattened_namespace);

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

        println!(
            "!!!!__ compiled_namespace_tuples: {:#?}",
            compiled_namespace_tuples
        );

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

        println!("!!!!__ compiled_namespace: {:#?}", compiled_namespace);

        let mut namespace_obj: IndexMap<String, Option<String>> = IndexMap::new();
        for key in compiled_namespace.keys() {
            let value = compiled_namespace.get(key).unwrap();

            // ...

            if let Some(included_styles) = value.as_included_style() {
                todo!("handle included style")
                // namespace_obj.insert(key.clone(), Some(included_styles));
            } else if let Some(styles) = value.as_computed_styles() {
                // let mut class_name_tuples = styles
                //     .iter()
                //     .map(|item| item)
                //     .collect::<Vec<ComputedStyle>>();

                let class_name_tuples = styles.clone();

                let class_name = &class_name_tuples
                    .iter()
                    .map(|computed_style| {
                        let class_name = computed_style.0.clone();

                        println!("!!!!__ class_name to join: {:#?}", class_name);

                        return class_name;
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                namespace_obj.insert(key.clone(), Some(class_name.clone()));

                println!(
                    "!!!!__ class_name_tuples: {:#?}, namespace_obj: {:#?}",
                    class_name_tuples, namespace_obj
                );

                for item in &class_name_tuples {
                    let class_name = item.0.clone();
                    let injectable_styles = item.1.clone();
                    if !injected_styles_map.contains_key(class_name.as_str()) {
                        injected_styles_map.insert(class_name.clone(), injectable_styles.clone());
                    }
                }
            } else {
                println!("!!!!__ value: {:#?}", value);
                namespace_obj.insert(key.clone(), Option::None);
            }
        }

        resolved_namespaces.insert(
            expr_to_str(namespace_name, declarations, var_dec_count_map),
            namespace_obj,
        );
        // let values: Vec<String> = flattened_namespace.values().map(|v| v.get_value().to_string()).collect();

        // for vec in flattened_namespace.values() {
        //     for pre_rule in vec {
        //         values.push(pre_rule);
        //     }
        // }

        // let (css_style, class_name_hashed, rules) = convert_style_to_class_name(
        //     (&css_property, &values),
        //     &mut pseudos,
        //     &mut at_rules,
        //     prefix,
        // );

        // println!("!!!!__ rules: {:?}", rules);

        // MetaData {
        //     class_name: class_name_hashed.clone(),
        //     style: InjectableStyleBase {
        //         // priority: Option::None, //MetaData::set_priority(&css_property),
        //         rtl: None,
        //         ltr: format!(".{}{}", class_name_hashed, css_style),
        //     },
        //     priority: MetaData::set_priority(&css_property),
        // }
    }

    (resolved_namespaces, injected_styles_map)
}

#[derive(Debug)]
pub(crate) struct SeenValue {
    pub(crate) value: Option<EvaluateResultValue>,
    pub(crate) resolved: bool,
}

#[derive(Debug)]
struct State {
    confident: bool,
    deopt_path: Option<Expr>,       // Assuming this is a string identifier
    seen: HashMap<Expr, SeenValue>, // Assuming the values are strings
    // added_imports: HashSet<String>,
    functions: FunctionMap,
    traversal_state: StateManager,
}

pub(crate) fn evaluate_style_x_create_arg(
    path: &Expr,
    traversal_state: &StateManager,
    functions: &FunctionMap,
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
                                let key_result =
                                    evaluate_obj_key(key_value_prop, traversal_state, functions);

                                if !key_result.confident {
                                    return EvaluateResult {
                                        confident: false,
                                        deopt: key_result.deopt,
                                        value: None,
                                    };
                                }

                                print!("!!!!keykeykey {:#?}", key_result);
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
                                        println!(
                                            "!!!!__ val: {:#?}, value_path: {:#?}",
                                            val, value_path
                                        );
                                        if !val.confident {
                                            return val;
                                        }

                                        println!("!!!!__ val: {:#?}", val);

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
                                return evaluate(path, traversal_state, functions);
                            }
                        }
                    }
                }
            }
            println!("!!!!__ value: {:#?}", value);
            EvaluateResult {
                confident: true,
                deopt: None,
                value: Some(EvaluateResultValue::Map(value)),
            }
        }
        _ => evaluate(path, traversal_state, functions),
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
            let computed_result = evaluate(computed_path, traversal_state, functions);
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
pub(crate) fn evaluate(
    path: &Expr,
    traversal_state: &StateManager,
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
    println!(
        "!!!!__!!! value: {:#?}, path: {:#?} confident: {:#?}, deopt_path: {:#?}",
        value, path, state.confident, state.deopt_path
    );
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
fn _evaluate(path: &Expr, state: &mut State) -> Option<EvaluateResultValue> {
    if !state.confident {
        panic!("Should not be here");
        // return Option::None;
    }

    let result = match path {
        Expr::Arrow(_) => todo!("Arrow function not implemented yet"),
        Expr::Ident(_) => todo!("Ident not implemented yet"),
        Expr::TsAs(_) => todo!("TsAs not implemented yet"),
        Expr::TsSatisfies(_) => todo!("TsSatisfies not implemented yet"),
        Expr::Seq(_) => todo!("Seq not implemented yet"),
        Expr::Lit(lit_path) => Option::Some(EvaluateResultValue::Expr(Expr::Lit(lit_path.clone()))),
        Expr::Tpl(_) => todo!("Tpl not implemented yet"),
        Expr::TaggedTpl(_) => todo!("TaggedTpl not implemented yet"),
        Expr::Cond(_) => todo!("Cond not implemented yet"),
        Expr::Paren(_) => todo!("Paren not implemented yet"),
        Expr::Member(_) => todo!("Member not implemented yet"),
        Expr::Unary(_) => todo!("Unary not implemented yet"),
        Expr::Array(_) => todo!("Array not implemented yet"),
        Expr::Object(obj_path) => {
            let mut props = vec![];

            for prop in &obj_path.props {
                match prop {
                    PropOrSpread::Spread(prop) => {
                        todo!("Check if it's working");

                        let spread_expression = evaluate_cached(&prop.expr, state);

                        if state.confident {
                            return deopt(path, state);
                        }
                        let merged_object = deep_merge_props(
                            &props,
                            &spread_expression
                                .unwrap()
                                .as_expr()
                                .unwrap()
                                .as_object()
                                .unwrap()
                                .props,
                        );
                        props = merged_object.props;

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

                                let value =
                                    evaluate(&value_path, &state.traversal_state, &state.functions);

                                println!(
                                    "!!!!__ key: {:#?} value: {:#?}, value_path: {:#?}",
                                    key, value, value_path
                                );

                                if !value.confident {
                                    if value.deopt.is_some() {
                                        deopt(&value.deopt.unwrap(), state);
                                    };

                                    return Option::None;
                                }

                                let value = value.value.unwrap();

                                let _ = &props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
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

            let obj = ObjectLit {
                props: props.clone(),
                span: DUMMY_SP,
            };

            return Option::Some(EvaluateResultValue::Expr(Expr::Object(obj)));
        }
        Expr::Bin(_) => todo!("Bin not implemented yet"),
        Expr::Call(_) => todo!("Call not implemented yet"),
        _ => {
            panic!("Not implemented yet, return somthing");
        }
    };

    if result.is_none() {
        deopt(path, state);
    }

    println!("!!!!__ path: {:#?}, result: {:#?}", path, result);

    result
}

fn evaluate_cached(path: &Expr, state: &mut State) -> Option<EvaluateResultValue> {
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
