use std::collections::HashMap;

use stylex_swc_plugin::shared::{
    structures::{
        evaluate_result::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
        stylex_options::StyleXOptions,
    },
    utils::css::stylex::evaluate,
};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{ArrayLit, Decl, Expr, ExprOrSpread, ExprStmt, Stmt},
        visit::{Fold, FoldWith},
    },
};
pub(crate) struct EvaluationModuleTransformVisitor {
    pub(crate) functions: FunctionMap,
}

impl Default for EvaluationModuleTransformVisitor {
    fn default() -> Self {
        EvaluationModuleTransformVisitor {
            functions: FunctionMap {
                identifiers: HashMap::new(),
                member_expressions: HashMap::new(),
            },
        }
    }
}

impl Fold for EvaluationModuleTransformVisitor {
    fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
        let stmt = match &stmt {
            Stmt::Decl(decl) => match decl {
                Decl::Var(decl_var) => {
                    let decl = decl_var.decls.get(0);
                    match decl {
                        Some(decl) => match decl.init.as_ref() {
                            Some(expr) => {
                                return Stmt::Expr(ExprStmt {
                                    span: DUMMY_SP,
                                    expr: expr.clone().fold_with(self),
                                });
                            }
                            None => stmt,
                        },
                        None => stmt,
                    }
                }
                _ => stmt,
            },

            _ => stmt,
        };

        stmt.fold_children_with(self)
    }

    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let evaluate_result = evaluate(
            &expr,
            &StateManager::new(StyleXOptions::default()),
            &self.functions,
            &vec![],
            &mut HashMap::new(),
        );
        println!(
            "!!!!!expr {:?}, evaluate_result: {:?}",
            expr, evaluate_result
        );

        match evaluate_result.value {
            Some(value) => match value {
                EvaluateResultValue::Expr(expr) => expr.clone().fold_children_with(self),
                EvaluateResultValue::Vec(vec) => Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: vec
                        .iter()
                        .map(|value| match value {
                            Some(value) => match value.as_expr() {
                                Some(expr) => Option::Some(ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(expr.clone()),
                                }),
                                None => None,
                            },
                            None => None,
                        })
                        .collect(),
                }),
                _ => panic!("Failed to evaluate expression"),
            },
            None => panic!("Failed to evaluate expression"),
        }
    }
}
