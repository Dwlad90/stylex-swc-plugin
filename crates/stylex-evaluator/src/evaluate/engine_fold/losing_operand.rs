//! The losing operand of `||`, `&&` and `??` when it calls a compiler function.
//!
//! The evaluator evaluates both operands of these three operators and then
//! picks one. So a `keyframes` call on the side that loses still writes its
//! rule. The engine stops at the left operand when that operand decides, and a
//! call on the right side then writes nothing.
//!
//! So the printed source changes such an operator into a call of the global
//! [`EITHER`]. It gets the left value, the right operand as an arrow, and the
//! operator. It runs the right operand first and then picks the operand the
//! operator picks. When the left operand decides, a throw from the right
//! operand has no effect, because the evaluator also ignores a refusal on the
//! side it does not use.
//!
//! Only an operator whose right operand the guard walked is changed, and only
//! when that operand calls a compiler function. The engine runs no source that
//! the guard did not walk. The guard records each such operator, and the
//! printer finds it again by its position in the tree (see [`ordinals`]).

use rustc_hash::FxHashSet;
use swc_core::{
  common::util::take::Take,
  ecma::{
    ast::{BinExpr, CallExpr, Expr},
    visit::{Visit, VisitMut, VisitMutWith, VisitWith},
  },
};

use stylex_ast::ast::factories::{
  create_arrow_expression, create_expr_or_spread, create_ident_call_expr,
  create_string_expr_or_spread,
};

use super::super::nodes::logical_expression::LogicalOp;

/// The name of the global that runs both operands of a logical operator.
pub(super) const EITHER: &str = "__sxEither";

/// The source that adds [`EITHER`] to the global object of an engine.
///
/// The property is not writable, not enumerable and not configurable, so the
/// source a fold runs cannot replace it. The guard refuses every name with this
/// spelling, so no binding can hide it.
///
/// The operator travels as its own text. `??` takes the right operand only for
/// `null` and `undefined`, as the language does.
pub(super) const EITHER_SOURCE: &str = r#"
Object.defineProperty(globalThis, "__sxEither", {
  value: (left, right, operator) => {
    let value;
    let error;
    let threw = false;

    try {
      value = right();
    } catch (thrown) {
      threw = true;
      error = thrown;
    }

    const leftDecides =
      operator === "||" ? !!left : operator === "&&" ? !left : left !== null && left !== undefined;

    if (leftDecides) {
      return left;
    }

    if (threw) {
      throw error;
    }

    return value;
  },
});
"#;

/// Where each recorded operator stands among the logical operators of the
/// printed trees, in the order the printer visits them: the call first, and
/// then each parameter default in its order.
///
/// The guard records an operator by its address in the source tree. The
/// printer works on a copy, so it cannot use that address. A position in the
/// visit order is the same in both trees. It is also part of the fold memo key,
/// so two folds that print the same call but change different operators do not
/// share one compiled script.
pub(super) fn ordinals<'e>(
  call: &CallExpr,
  defaults: impl Iterator<Item = &'e Expr>,
  recorded: &FxHashSet<*const BinExpr>,
) -> Vec<usize> {
  let mut count = Ordinals {
    recorded,
    next: 0,
    found: Vec::new(),
  };

  call.visit_with(&mut count);

  for default in defaults {
    default.visit_with(&mut count);
  }

  count.found
}

/// The walk of [`ordinals`].
struct Ordinals<'r> {
  recorded: &'r FxHashSet<*const BinExpr>,
  next: usize,
  found: Vec<usize>,
}

impl Visit for Ordinals<'_> {
  fn visit_bin_expr(&mut self, bin: &BinExpr) {
    if LogicalOp::of(bin.op).is_some() {
      if self.recorded.contains(&std::ptr::from_ref(bin)) {
        self.found.push(self.next);
      }

      self.next += 1;
    }

    bin.visit_children_with(self);
  }
}

/// The printer's side of [`ordinals`]: it counts the logical operators of each
/// tree it is given in the same order, and changes those at the positions in
/// `changed` into a call of [`EITHER`].
pub(super) struct Rewrite<'c> {
  changed: &'c [usize],
  next: usize,
}

impl<'c> Rewrite<'c> {
  pub(super) fn new(changed: &'c [usize]) -> Self {
    Self { changed, next: 0 }
  }

  /// Changes the recorded operators of `tree`, and counts the others.
  pub(super) fn apply(&mut self, tree: &mut Expr) {
    tree.visit_mut_with(self);
  }
}

impl VisitMut for Rewrite<'_> {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    let Expr::Bin(bin) = expr else {
      expr.visit_mut_children_with(self);

      return;
    };

    let Some(op) = LogicalOp::of(bin.op) else {
      bin.visit_mut_children_with(self);

      return;
    };

    // Counted before the operands are visited, so the order is the order of
    // `Ordinals`, which counts a node before its children too.
    let position = self.next;

    self.next += 1;

    bin.visit_mut_children_with(self);

    if self.changed.contains(&position) {
      let left = bin.left.as_mut().take();
      let right = bin.right.as_mut().take();

      *expr = Expr::Call(create_ident_call_expr(
        EITHER,
        vec![
          create_expr_or_spread(left),
          create_expr_or_spread(create_arrow_expression(right)),
          create_string_expr_or_spread(op.spelling()),
        ],
      ));
    }
  }
}

#[cfg(test)]
#[path = "tests/losing_operand_tests.rs"]
mod losing_operand_tests;
