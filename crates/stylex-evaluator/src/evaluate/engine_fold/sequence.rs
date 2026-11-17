//! A sequence in a folded expression, written as its last operand.
//!
//! A sequence is worth its last operand, and the evaluator reads that operand
//! alone (see `nodes::sequence_expression`). The other operands are never
//! evaluated, so a value that has no compile-time form may stand there. The
//! engine would evaluate every operand, so each sequence is replaced by its
//! last operand before the engine reads the source. The guard walks only that
//! operand, for the same reason.

use swc_core::ecma::{
  ast::Expr,
  visit::{VisitMut, VisitMutWith},
};

/// `tree` with every sequence in it replaced by its last operand.
pub(super) fn last_operand_of_each_sequence(tree: &mut Expr) {
  tree.visit_mut_with(&mut LastOperand);
}

/// The walk of [`last_operand_of_each_sequence`].
struct LastOperand;

impl VisitMut for LastOperand {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    // A last operand can itself be a sequence, so the replacement repeats.
    while let Expr::Seq(sequence) = expr
      && let Some(last) = sequence.exprs.pop()
    {
      *expr = *last;
    }

    expr.visit_mut_children_with(self);
  }
}
