use colored::Colorize;
use swc_core::{
    css::ast::Namespace,
    ecma::ast::{
        Callee, Decl, Expr, Id, KeyValueProp, Module, ModuleDecl, ModuleItem, Pat, PropName, Stmt,
    },
};

use crate::shared::{constants, structures::evaluate_result::EvaluateResultValue};

pub(crate) fn validate_style_x_create(module: &Module, declaration: &Id) {
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
                                &declaration,
                                &mut has_assignment,
                            ),
                            None => {}
                        })
                }
                _ => {}
            },
            ModuleDecl::ExportDefaultExpr(export_decl) => match export_decl.expr.as_ref() {
                Expr::Paren(paren) => validate_style_x_create_call_expression(
                    &paren.expr,
                    &declaration,
                    &mut has_assignment,
                ),

                _ => validate_style_x_create_call_expression(
                    &export_decl.expr,
                    &declaration,
                    &mut has_assignment,
                ),
            },
            _ => {}
        },
        ModuleItem::Stmt(stmp) => match &stmp {
            Stmt::Decl(decl) => match &decl {
                Decl::Var(var) => var.decls.iter().for_each(|decl| match decl.init.as_ref() {
                    Some(decl) => validate_style_x_create_call_expression(
                        &decl,
                        &declaration,
                        &mut has_assignment,
                    ),
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
    declaration: &Id,
    has_assignment: &mut bool,
) {
    match expr {
        Expr::Call(call) => match &call.callee {
            Callee::Expr(expr) => match expr.as_ref() {
                Expr::Ident(ident) => {
                    validate_style_x_create_indent(declaration, ident, has_assignment, call);
                }
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        validate_style_x_create_indent(declaration, ident, has_assignment, call);
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
    declaration: &(swc_core::atoms::Atom, swc_core::common::SyntaxContext),
    ident: &swc_core::ecma::ast::Ident,
    has_assignment: &mut bool,
    call: &swc_core::ecma::ast::CallExpr,
) {
    if declaration.clone().eq(&ident.to_id()) && !*has_assignment {
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

pub(crate) fn validate_namespace(namespaces: &Vec<KeyValueProp>) {
    for namespace in namespaces.iter() {
        let key = namespace.key.clone();

        let class_name = match &key {
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
