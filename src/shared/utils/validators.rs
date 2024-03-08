use core::panic;

use colored::Colorize;
use regex::Regex;
use swc_core::ecma::ast::{
    CallExpr, Callee, Decl, Expr, Id, Ident, KeyValueProp, Module, ModuleDecl, ModuleItem, Pat,
    Prop, PropName, PropOrSpread, Stmt,
};

use crate::shared::{
    constants,
    regex::INCLUDED_IDENT_REGEX,
    structures::{named_import_source::ImportSources, state_manager::StateManager},
};

use super::common::{get_key_str, get_key_values_from_object};
use once_cell::sync::Lazy;

pub(crate) fn validate_style_x_create(module: &Module, state: &StateManager) {
    let mut has_assignment = false;

    module.clone().body.iter().for_each(|item| match &item {
        ModuleItem::ModuleDecl(decl) => match &decl {
            ModuleDecl::ExportDecl(export_decl) => match &export_decl.decl {
                Decl::Var(decl_var) => {
                    decl_var
                        .decls
                        .iter()
                        .for_each(|decl| match decl.init.as_ref() {
                            Some(decl) => validate_style_x_create_call_expression(
                                &decl,
                                state,
                                &mut has_assignment,
                            ),
                            None => {}
                        })
                }
                _ => {}
            },
            ModuleDecl::ExportDefaultExpr(export_decl) => match export_decl.expr.as_ref() {
                Expr::Paren(paren) => {
                    validate_style_x_create_call_expression(&paren.expr, state, &mut has_assignment)
                }

                _ => validate_style_x_create_call_expression(
                    &export_decl.expr,
                    state,
                    &mut has_assignment,
                ),
            },
            _ => {}
        },
        ModuleItem::Stmt(stmp) => match &stmp {
            Stmt::Decl(decl) => match &decl {
                Decl::Var(var) => var.decls.iter().for_each(|decl| match decl.init.as_ref() {
                    Some(decl) => {
                        validate_style_x_create_call_expression(&decl, state, &mut has_assignment)
                    }
                    None => {}
                }),
                _ => {}
            },
            _ => {}
        },
    });

    assert!(
        has_assignment,
        "{}",
        constants::messages::UNBOUND_STYLEX_CALL_VALUE
    );
}

pub(crate) fn validate_style_x_create_call_expression(
    expr: &Expr,
    state: &StateManager,
    has_assignment: &mut bool,
) {
    match expr {
        Expr::Call(call) => match &call.callee {
            Callee::Expr(expr) => match expr.as_ref() {
                Expr::Ident(ident) => {
                    validate_style_x_create_indent(state, ident, has_assignment, call);
                }
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        validate_style_x_create_indent(state, ident, has_assignment, call);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

fn validate_style_x_create_indent(
    state: &StateManager,
    ident: &Ident,
    has_assignment: &mut bool,
    call: &CallExpr,
) {
    let stylex_imports = state.stylex_import_stringified();

    println!("!!!! ident: {:?}\n\n", ident.sym.to_string());
    println!("!!!! stylex_imports: {:?}\n\n", stylex_imports);
    println!(
        "!!!! stylex_create_import: {:?}\n\n",
        state.stylex_create_import
    );
    println!("!!!! call: {:?}\n\n", call);

    let is_equal = stylex_imports.contains(&&ident.sym.to_string())
        || state.stylex_create_import.contains(&ident.to_id());

    println!("is_equal: {:?}", is_equal);
    if is_equal && !*has_assignment {
        assert!(
            &call.args.len() == &1,
            "{}",
            constants::messages::ILLEGAL_ARGUMENT_LENGTH
        );

        let first_args = &call.args[0];

        match first_args.expr.as_ref() {
            Expr::Object(_) => {}
            _ => panic!("{}", constants::messages::NON_OBJECT_FOR_STYLEX_CALL),
        }
        *has_assignment = true;
    }
}

pub(crate) fn validate_namespace(namespaces: &Vec<KeyValueProp>, conditions: &Vec<String>) {
    for namespace in namespaces.iter() {
        let key = namespace.key.clone();

        match &key {
            PropName::Ident(key) => {
                let key = format!("{}", key.sym);

                key
            }
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
        dbg!(&key, namespace.value.clone());

        match namespace.value.as_ref() {
            Expr::Lit(_) => {}
            Expr::Array(array) => {
                for elem in array.elems.iter() {
                    match elem {
                        Some(elem) => {
                            assert!(
                                elem.spread.is_none(),
                                "{}",
                                "Spread operator not implemented"
                            );

                            match elem.expr.as_ref() {
                                Expr::Lit(_) => {}
                                _ => panic!("{}", constants::messages::ILLEGAL_PROP_ARRAY_VALUE),
                            }
                        }
                        None => {}
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

                    let mut extended_conditions = conditions.clone();
                    extended_conditions.push(key);

                    validate_namespace(&nested_key_values, &extended_conditions);
                } else {
                    let conditional_styles_key_values = get_key_values_from_object(object);

                    for conditional_style in conditional_styles_key_values.iter() {
                        validate_conditional_styles(conditional_style, conditions);
                    }
                }
            }

            _ => {
                if let Some(ident) = key.as_ident() {
                    let key = ident.sym.to_string();

                    if INCLUDED_IDENT_REGEX.is_match(&key) {
                        assert!(
                            conditions.is_empty(),
                            "{}",
                            constants::messages::ONLY_TOP_LEVEL_INCLUDES
                        )
                    }
                } else {
                    panic!("{}", constants::messages::ILLEGAL_PROP_VALUE)
                }
            }
        }
    }
}

pub(crate) fn validate_and_return_property(property: &KeyValueProp) -> String {
    let key = property.key.clone();

    let class_name = match &key {
        PropName::Ident(key) => {
            let key = format!("{}", key.sym);

            key
        }
        PropName::Str(key) => {
            eprintln!(
                "{}",
                Colorize::yellow("!!!! flatMapExpandedShorthands not implemented yet !!!!")
            );

            key.value.to_string()
        }
        _ => panic!("{}", constants::messages::NON_STATIC_VALUE),
    };

    class_name
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
        Expr::Lit(object) => {}
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
