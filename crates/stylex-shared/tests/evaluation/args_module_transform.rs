use rustc_hash::FxHashMap;
use stylex_shared::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    ast::{
      convertors::{expr_to_str, number_to_expression},
      factories::{
        array_expression_factory, object_expression_factory, prop_or_spread_expression_factory,
      },
    },
    core::evaluate_stylex_create_arg::evaluate_stylex_create_arg,
  },
};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{Decl, Expr, ExprOrSpread, ExprStmt, Pass, Pat, Prop, PropOrSpread, Stmt, VarDeclarator},
    visit::{fold_pass, Fold, FoldWith},
  },
};
pub(crate) struct ArgsStyleXTransform {
  pub(crate) functions: FunctionMap,
  pub(crate) declarations: Vec<VarDeclarator>,
  pub(crate) state: StateManager,
}

impl ArgsStyleXTransform {
  pub fn default_with_pass() -> impl Pass {
    fold_pass(Self::default())
  }
}

impl Default for ArgsStyleXTransform {
  fn default() -> Self {
    ArgsStyleXTransform {
      functions: FunctionMap {
        identifiers: FxHashMap::default(),
        member_expressions: FxHashMap::default(),
      },
      declarations: vec![],
      state: StateManager::default(),
    }
  }
}

impl Fold for ArgsStyleXTransform {
  fn fold_var_declarators(&mut self, var_declarators: Vec<VarDeclarator>) -> Vec<VarDeclarator> {
    var_declarators.iter().for_each(|decl| {
      if let Pat::Ident(_) = &decl.name {
        if !self.declarations.contains(decl) {
          self.declarations.push(decl.clone());
        }
      }
    });

    var_declarators.fold_children_with(self)
  }

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
    let evaluate_result =
      evaluate_stylex_create_arg(&mut Box::new(expr), &mut self.state, &self.functions);

    match evaluate_result.value {
      Some(value) => match value {
        EvaluateResultValue::Expr(expr) => expr.clone(), //.fold_children_with(self),
        EvaluateResultValue::Vec(vec) => array_expression_factory(
          vec
            .iter()
            .map(|value| match value {
              Some(value) => value.as_expr().map(|expr| ExprOrSpread {
                spread: None,
                expr: Box::new(expr.clone()),
              }),
              None => None,
            })
            .collect(),
        ),
        EvaluateResultValue::Callback(func) => func(vec![
          Some(EvaluateResultValue::Expr(number_to_expression(2.0))),
          Some(EvaluateResultValue::Expr(number_to_expression(7.0))),
        ]),
        EvaluateResultValue::Map(map) => {
          let mut props = vec![];

          for (key, value) in map.iter() {
            let prop = prop_or_spread_expression_factory(
              expr_to_str(key, &mut self.state, &FunctionMap::default()).as_str(),
              object_expression_factory(
                value
                  .iter()
                  .map(|key_value| PropOrSpread::from(Prop::from(key_value.clone())))
                  .collect(),
              ),
            );

            props.push(prop);
          }

          object_expression_factory(props)
        }
        _ => panic!("Failed to evaluate expression"),
      },
      None => panic!("Failed to evaluate expression"),
    }
  }
}
