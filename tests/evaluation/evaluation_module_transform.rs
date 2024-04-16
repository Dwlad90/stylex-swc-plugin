use std::collections::HashMap;

use stylex_swc_plugin::shared::{
  structures::{
    evaluate_result::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
    stylex_options::StyleXOptions,
  },
  utils::css::stylex::evaluate::evaluate,
};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{ArrayLit, Decl, Expr, ExprOrSpread, ExprStmt, Lit, Number, Pat, Stmt, VarDeclarator},
    visit::{Fold, FoldWith},
  },
};
pub(crate) struct EvaluationModuleTransformVisitor {
  pub(crate) functions: FunctionMap,
  pub(crate) declarations: Vec<VarDeclarator>,
  pub(crate) state: StateManager,
}

impl Default for EvaluationModuleTransformVisitor {
  fn default() -> Self {
    EvaluationModuleTransformVisitor {
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
      declarations: vec![],
      state: StateManager::new(StyleXOptions::default()),
    }
  }
}

impl Fold for EvaluationModuleTransformVisitor {
  fn fold_var_declarators(&mut self, var_declarators: Vec<VarDeclarator>) -> Vec<VarDeclarator> {
    var_declarators.iter().for_each(|decl| {
      if let Pat::Ident(_) = &decl.name {
        let var = decl.clone();

        if !self.declarations.contains(&var) {
          self.declarations.push(var);
        }
      }
    });

    var_declarators.fold_children_with(self)
  }

  fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
    let stmt = match &stmt {
      Stmt::Decl(decl) => match decl {
        Decl::Var(decl_var) => {
          let decl = decl_var.decls.first();
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
    println!("!!!!!expr: {:?}", expr);
    let evaluate_result = evaluate(&expr, &mut self.state, &self.functions);
    println!("!!!!!evaluate_result: {:?}", evaluate_result);

    match evaluate_result.value {
      Some(value) => match value {
        EvaluateResultValue::Expr(expr) => expr.clone(), //.fold_children_with(self),
        EvaluateResultValue::Vec(vec) => Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: vec
            .iter()
            .map(|value| match value {
              Some(value) => value.as_expr().map(|expr| ExprOrSpread {
                  spread: None,
                  expr: Box::new(expr.clone()),
                }),
              None => None,
            })
            .collect(),
        }),
        EvaluateResultValue::Callback(func) => func(vec![
          Option::Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: 2.0,
            raw: Option::None,
          })))),
          Option::Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: 7.0,
            raw: Option::None,
          })))),
        ]),
        _ => panic!("Failed to evaluate expression"),
      },
      None => panic!("Failed to evaluate expression"),
    }
  }
}
