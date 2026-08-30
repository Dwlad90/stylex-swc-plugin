use rustc_hash::FxHashMap;
use stylex_ast::ast::convertors::create_number_expr;
use stylex_enums::core::TransformationCycle;
use stylex_transform::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    common::{fill_state_declarations, fill_top_level_expressions},
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
/// An evaluated list as the array literal it stands for, at every depth.
///
/// A nested list is a list of its own — an array literal evaluates to one
/// element per element, whatever those elements are — so the rendering has to
/// recurse or a nested array prints as a hole. Written once here because both
/// harnesses below render the same shape.
fn render_array(items: &[EvaluateResultValue]) -> Expr {
  Expr::from(ArrayLit {
    span: DUMMY_SP,
    elems: items
      .iter()
      .map(|value| match value {
        EvaluateResultValue::Null => None,
        EvaluateResultValue::Vec(nested) => Some(ExprOrSpread {
          spread: None,
          expr: Box::new(render_array(nested)),
        }),
        value => value.as_expr().map(|expr| ExprOrSpread {
          spread: None,
          expr: Box::new(expr.clone()),
        }),
      })
      .collect(),
  })
}

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
            },
            None => stmt,
          },
          None => stmt,
        }
      },

      _ => stmt,
    };

    stmt.fold_children_with(self)
  }

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    let evaluate_result = evaluate(&Box::new(expr.clone()), &mut self.state, &self.functions);

    // A refusal is an ordinary answer from the evaluator — the expression has
    // no compile-time value and belongs in the output as written. Rendering it
    // unchanged is what the compiler itself does, so this harness mirrors it
    // rather than aborting; a deopt is not a failed test.
    if !evaluate_result.confident {
      return expr;
    }

    match evaluate_result.value {
      Some(value) => match value {
        EvaluateResultValue::Expr(expr) => expr,
        EvaluateResultValue::Vec(vec) => render_array(&vec),
        // A callback that could not fold its body answers nothing, which for a
        // harness that renders one value means the input under test was wrong.
        EvaluateResultValue::Callback(func) => match func(
          vec![
            EvaluateResultValue::Expr(create_number_expr(2.0)),
            EvaluateResultValue::Expr(create_number_expr(7.0)),
          ],
          &mut self.state,
        ) {
          Some(expr) => expr,
          None => panic!("the callback folded no value"),
        },
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
    self.state.cycle = TransformationCycle::Discover;
    let module = module.fold_children_with(self);

    fill_top_level_expressions(&module, &mut self.state);

    self.state.cycle = TransformationCycle::TransformProducers;

    module.fold_children_with(self)
  }

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    if let Some(call_expr) = expr.as_call() {
      self.state.add_call_expression(call_expr);
    }

    if self.state.cycle == TransformationCycle::TransformProducers {
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
        EvaluateResultValue::Expr(expr) => expr,
        EvaluateResultValue::Vec(vec) => render_array(&vec),
        // A callback that could not fold its body answers nothing, which for a
        // harness that renders one value means the input under test was wrong.
        EvaluateResultValue::Callback(func) => match func(
          vec![
            EvaluateResultValue::Expr(create_number_expr(2.0)),
            EvaluateResultValue::Expr(create_number_expr(7.0)),
          ],
          &mut self.state,
        ) {
          Some(expr) => expr,
          None => panic!("the callback folded no value"),
        },
        _ => panic!("Failed to evaluate expression"),
      },
      None => panic!("Failed to evaluate expression"),
    }
  }
}
