use rustc_hash::FxHashMap;
use stylex_shared::shared::{
  enums::{core::TransformationCycle, data_structures::evaluate_result_value::EvaluateResultValue},
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    ast::convertors::number_to_expression,
    common::{fill_state_declarations, fill_top_level_expressions, stable_hash},
    js::evaluate::evaluate,
  },
};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{ArrayLit, Decl, Expr, ExprOrSpread, ExprStmt, Module, ModuleItem, Pass, Pat, Stmt},
    visit::{Fold, FoldWith, fold_pass},
  },
};
pub(crate) struct EvaluationStyleXFirstStatementTransform {
  pub(crate) functions: FunctionMap,
  pub(crate) state: StateManager,
}

impl EvaluationStyleXFirstStatementTransform {
  pub fn default_with_pass() -> impl Pass {
    fold_pass(Self::default())
  }
}

impl Default for EvaluationStyleXFirstStatementTransform {
  fn default() -> Self {
    EvaluationStyleXFirstStatementTransform {
      functions: FunctionMap {
        identifiers: FxHashMap::default(),
        member_expressions: FxHashMap::default(),
        disable_imports: false,
      },
      state: StateManager::default(),
    }
  }
}

impl Fold for EvaluationStyleXFirstStatementTransform {
  fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
    let stmt = match &stmt {
      Stmt::Decl(Decl::Var(decl_var)) => {
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
    };

    stmt.fold_children_with(self)
  }

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    let evaluate_result = evaluate(&Box::new(expr), &mut self.state, &self.functions);

    match evaluate_result.value {
      Some(value) => match value {
        EvaluateResultValue::Expr(expr) => expr.clone(),
        EvaluateResultValue::Vec(vec) => Expr::from(ArrayLit {
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
          Some(EvaluateResultValue::Expr(number_to_expression(2.0))),
          Some(EvaluateResultValue::Expr(number_to_expression(7.0))),
        ]),
        _ => panic!("Failed to evaluate expression"),
      },
      None => panic!("Failed to evaluate expression"),
    }
  }
}

pub(crate) struct EvaluationStyleXLastStatementTransform {
  pub(crate) functions: FunctionMap,
  pub(crate) state: StateManager,
}

impl EvaluationStyleXLastStatementTransform {
  pub fn default_with_pass() -> impl Pass {
    fold_pass(Self::default())
  }
}

impl Default for EvaluationStyleXLastStatementTransform {
  fn default() -> Self {
    EvaluationStyleXLastStatementTransform {
      functions: FunctionMap {
        identifiers: FxHashMap::default(),
        member_expressions: FxHashMap::default(),
        disable_imports: false,
      },
      state: StateManager::default(),
    }
  }
}

impl Fold for EvaluationStyleXLastStatementTransform {
  fn fold_module(&mut self, module: Module) -> Module {
    self.state.cycle = TransformationCycle::StateFilling;
    let module = module.fold_children_with(self);

    fill_top_level_expressions(&module, &mut self.state);

    self.state.cycle = TransformationCycle::TransformEnter;

    module.fold_children_with(self)
  }

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    if let Some(call_expr) = expr.as_call() {
      self
        .state
        .all_call_expressions
        .insert(stable_hash(&call_expr), call_expr.clone());
    }

    if self.state.cycle == TransformationCycle::TransformEnter {
      return self.evaluate_expr(expr).fold_children_with(self);
    }

    expr.fold_children_with(self)
  }

  fn fold_module_items(
    &mut self,
    module_items: Vec<swc_core::ecma::ast::ModuleItem>,
  ) -> Vec<swc_core::ecma::ast::ModuleItem> {
    module_items.iter().for_each(|module_item| {
      if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = module_item {
        var_decl.decls.iter().for_each(|decl| {
          if let Pat::Ident(_) = &decl.name {
            fill_state_declarations(&mut self.state, decl);
          }
        });
      }
    });

    module_items.fold_children_with(self)
  }
  fn fold_var_declarator(
    &mut self,
    var_declarator: swc_core::ecma::ast::VarDeclarator,
  ) -> swc_core::ecma::ast::VarDeclarator {
    fill_state_declarations(&mut self.state, &var_declarator);

    var_declarator.fold_children_with(self)
  }
}

impl EvaluationStyleXLastStatementTransform {
  fn evaluate_expr(&mut self, expr: Expr) -> Expr {
    let evaluate_result = evaluate(&Box::new(expr), &mut self.state, &self.functions);

    if !evaluate_result.confident {
      panic!("{}", evaluate_result.reason.unwrap());
    }

    match evaluate_result.value {
      Some(value) => match value {
        EvaluateResultValue::Expr(expr) => expr.clone(),
        EvaluateResultValue::Vec(vec) => Expr::from(ArrayLit {
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
          Some(EvaluateResultValue::Expr(number_to_expression(2.0))),
          Some(EvaluateResultValue::Expr(number_to_expression(7.0))),
        ]),
        _ => panic!("Failed to evaluate expression"),
      },
      None => panic!("Failed to evaluate expression"),
    }
  }
}
