use std::collections::HashSet;

use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{CallExpr, Expr, Id, Ident, KeyValueProp, Pat, PropName, VarDeclarator},
};

use crate::shared::{
    constants,
    enums::{TopLevelExpression, TopLevelExpressionKind},
    regex::INCLUDED_IDENT_REGEX,
    structures::{evaluate_result::EvaluateResultValue, state_manager::StateManager},
    utils::common::get_var_decl_by_ident_or_member,
};

use super::common::{get_key_str, get_key_values_from_object};

pub(crate) fn validate_stylex_create_indent(call: &CallExpr, state: &mut StateManager) {
    if !is_create_call(call, state) {
        return;
    }

    let ident = Ident::new("create".into(), DUMMY_SP);

    dbg!(&ident);
    assert!(
        get_var_decl_by_ident_or_member(state, &ident).is_some()
            || state
                .top_level_expressions
                .clone()
                .into_iter()
                .any(|TopLevelExpression(_, call_item, _)| call_item
                    .eq(&Box::new(Expr::Call(call.clone())))),
        "{}",
        constants::messages::UNBOUND_STYLEX_CALL_VALUE
    );

    assert!(
        &call.args.len() == &1,
        "{}",
        constants::messages::ILLEGAL_ARGUMENT_LENGTH
    );

    let first_args = &call.args[0];

    assert!(
        first_args.expr.is_object(),
        "{}",
        constants::messages::NON_OBJECT_FOR_STYLEX_CALL
    )
}

pub(crate) fn validate_stylex_keyframes_indent(var_decl: &VarDeclarator, state: &mut StateManager) {
    let init = match &var_decl.init {
        Some(init) => init
            .clone()
            .call()
            .expect(constants::messages::NON_STATIC_KEYFRAME_VALUE),
        None => panic!("{}", constants::messages::NON_STATIC_KEYFRAME_VALUE),
    };

    if !is_keyframes_call(var_decl, state) {
        return;
    }

    let ident = Ident::new("keyframes".into(), DUMMY_SP);

    dbg!(&ident);
    assert!(
        get_var_decl_by_ident_or_member(state, &ident).is_some()
            || state
                .top_level_expressions
                .clone()
                .into_iter()
                .any(|TopLevelExpression(_, call_item, _)| call_item
                    .eq(&Box::new(Expr::Call(init.clone())))),
        "{}",
        constants::messages::UNBOUND_STYLEX_CALL_VALUE
    );

    assert!(
        &init.args.len() == &1,
        "{}",
        constants::messages::ILLEGAL_ARGUMENT_LENGTH
    );

    let first_args = &init.args[0];

    assert!(
        first_args.expr.is_object(),
        "{}",
        constants::messages::NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL
    )
}

pub(crate) fn validate_stylex_define_vars(call: &CallExpr, state: &mut StateManager) {
    if !is_define_vars_call(call, state) {
        return;
    }

    let ident = Ident::new("defineVars".into(), DUMMY_SP);

    assert!(
        get_var_decl_by_ident_or_member(state, &ident).is_some()
            || state
                .top_level_expressions
                .clone()
                .into_iter()
                .any(|TopLevelExpression(_, call_item, _)| call_item
                    .eq(&Box::new(Expr::Call(call.clone())))),
        "{}",
        constants::messages::UNBOUND_STYLEX_CALL_VALUE
    );

    assert!(
        &call.args.len() == &1,
        "{}",
        constants::messages::ILLEGAL_ARGUMENT_LENGTH
    );

    assert!(
        state
            .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call)
            .is_some(),
        "{}",
        constants::messages::NON_EXPORT_NAMED_DECLARATION
    );

    // let first_args = &call.args[0];

    // assert!(
    //     first_args.expr.is_object(),
    //     "{}",
    //     constants::messages::NON_OBJECT_FOR_STYLEX_CALL
    // )
}

pub(crate) fn is_create_call(call: &CallExpr, state: &StateManager) -> bool {
    is_target_call(("create", &state.stylex_create_import), call, state)
}

pub(crate) fn is_props_call(call: &CallExpr, state: &StateManager) -> bool {
    dbg!(&state.stylex_props_import);
    is_target_call(("props", &state.stylex_props_import), call, state)
}

pub(crate) fn is_attrs_call(call: &CallExpr, state: &StateManager) -> bool {
    dbg!(&state.stylex_props_import);
    is_target_call(("attrs", &state.stylex_attrs_import), call, state)
}

pub(crate) fn is_keyframes_call(var_decl: &VarDeclarator, state: &StateManager) -> bool {
    let init = match &var_decl.init {
        Some(init) => init.clone().call(),
        None => Option::None,
    };

    if let Some(call) = init {
        is_target_call(("keyframes", &state.stylex_keyframes_import), &call, state)
    } else {
        false
    }
}

pub(crate) fn is_define_vars_call(call: &CallExpr, state: &StateManager) -> bool {
    dbg!(&state.stylex_props_import);
    is_target_call(
        ("defineVars", &state.stylex_define_vars_import),
        call,
        state,
    )
}

pub(crate) fn is_target_call(
    (call_name, imports_map): (&str, &HashSet<Id>),
    call: &CallExpr,
    state: &StateManager,
) -> bool {
    let is_create_ident = call
        .callee
        .as_expr()
        .and_then(|expr| expr.as_ident())
        .map_or(false, |ident| imports_map.contains(&ident.to_id()));

    let is_create_member = call
        .callee
        .as_expr()
        .and_then(|expr| expr.as_member())
        .map_or(false, |member| {
            member.obj.is_ident()
                && member.prop.as_ident().map_or(false, |ident| {
                    ident.sym.eq(call_name)
                        && state
                            .stylex_import_stringified()
                            .contains(&member.obj.as_ident().unwrap().sym.to_string())
                })
        });

    is_create_ident || is_create_member
}
pub(crate) fn validate_namespace(namespaces: &[KeyValueProp], conditions: &Vec<String>) {
    for namespace in namespaces {
        let key = match &namespace.key {
            PropName::Ident(key) => format!("{}", key.sym),
            PropName::Str(key) => {
                if !(key.value.starts_with("@")
                    || key.value.starts_with(":")
                    || key.value == "default")
                {
                    panic!("{}", constants::messages::INVALID_PSEUDO_OR_AT_RULE)
                }
                key.value.to_string()
            }
            _ => panic!("{}", constants::messages::NON_STATIC_VALUE),
        };

        match namespace.value.as_ref() {
            Expr::Lit(_) => {}
            Expr::Array(array) => {
                for elem in &array.elems {
                    if let Some(elem) = elem {
                        assert!(
                            elem.spread.is_none(),
                            "{}",
                            "Spread operator not implemented"
                        );

                        if let Expr::Lit(_) = elem.expr.as_ref() {
                            // Do nothing
                        } else {
                            panic!("{}", constants::messages::ILLEGAL_PROP_ARRAY_VALUE);
                        }
                    }
                }
            }
            Expr::Object(object) => {
                let key = get_key_str(namespace);

                if key.starts_with("@") || key.starts_with(":") {
                    if conditions.contains(&key) {
                        panic!("{}", constants::messages::DUPLICATE_CONDITIONAL);
                    }

                    let nested_key_values = get_key_values_from_object(object);

                    let mut extended_conditions = conditions.to_vec();
                    extended_conditions.push(key);

                    validate_namespace(&nested_key_values, &extended_conditions);
                } else {
                    let conditional_styles_key_values = get_key_values_from_object(object);

                    for conditional_style in &conditional_styles_key_values {
                        validate_conditional_styles(conditional_style, conditions);
                    }
                }
            }
            _ => {
                if INCLUDED_IDENT_REGEX.is_match(&key) {
                    assert!(
                        conditions.is_empty(),
                        "{}",
                        constants::messages::ONLY_TOP_LEVEL_INCLUDES
                    )
                }
            }
        }
    }
}

pub(crate) fn validate_dynamic_style_params(params: &Vec<Pat>) {
    if params.iter().any(|param| !param.is_ident()) {
        panic!(
            "{}",
            constants::messages::ONLY_NAMED_PARAMETERS_IN_DYNAMIC_STYLE_FUNCTIONS
        );
    }
}

pub(crate) fn validate_conditional_styles(
    inner_key_value: &KeyValueProp,
    conditions: &Vec<String>,
) {
    let inner_key = get_key_str(inner_key_value);
    let inner_value = inner_key_value.value.clone();

    dbg!(inner_key.clone());

    assert!(
        (inner_key.starts_with(":") || inner_key.starts_with("@") || inner_key == "default"),
        "{}",
        constants::messages::INVALID_PSEUDO_OR_AT_RULE,
    );

    if conditions.contains(&inner_key) {
        panic!("{}", constants::messages::DUPLICATE_CONDITIONAL);
    }

    match inner_value.as_ref() {
        Expr::Lit(_) => {}
        Expr::Array(array) => {
            for elem in array.elems.iter() {
                match elem {
                    Some(elem) => match elem.expr.as_ref() {
                        Expr::Lit(_) => {}
                        _ => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
                    },
                    None => {}
                }
            }
        }
        Expr::Object(object) => {
            let nested_key_values = get_key_values_from_object(object);

            let mut extended_conditions = conditions.clone();
            extended_conditions.push(inner_key);

            for nested_key_value in nested_key_values.iter() {
                validate_conditional_styles(nested_key_value, &extended_conditions);
            }
        }
        Expr::Ident(_) => {
            if INCLUDED_IDENT_REGEX.is_match(&inner_key) {
                panic!("{}", constants::messages::ONLY_TOP_LEVEL_INCLUDES);
            }
        }
        _ => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
    }
}

pub(crate) fn assert_valid_keyframes(obj: &EvaluateResultValue) {
    match obj {
        EvaluateResultValue::Expr(expr) => match expr {
            Expr::Object(object) => {
                let key_values = get_key_values_from_object(object);

                for key_value in key_values.iter() {
                    match key_value.value.as_ref() {
                        Expr::Object(_) => {}
                        _ => panic!("{}", constants::messages::NON_OBJECT_KEYFRAME),
                    }
                }
            }
            _ => panic!(
                "{}",
                constants::messages::NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL
            ),
        },
        _ => panic!(
            "{}",
            constants::messages::NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL
        ),
    }
}
