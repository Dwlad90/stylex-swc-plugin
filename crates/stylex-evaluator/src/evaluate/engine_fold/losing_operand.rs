//! The losing operand of `||`, `&&` and `??` when it calls a compiler function.
//!
//! The evaluator evaluates both operands of these three operators and then
//! picks one. So a `keyframes` call on the side that loses still writes its
//! rule. The engine stops at the left operand when that operand decides, and a
//! call on the right side then writes nothing.
//!
//! So the printed source changes such an operator into a call of the global
//! [`EITHER`]. It gets each operand as an arrow, and the operator. It runs the
//! left operand and then the right operand, as the evaluator does, and then
//! picks the operand the operator picks. When the left operand decides, a throw
//! from the right operand has no effect, because the evaluator also ignores a
//! refusal on the side it does not use.
//!
//! A put-off refusal in the left operand is the answer, but the right operand
//! still runs first, because the evaluator evaluates the right side when the
//! left side refuses. Any other throw of the left operand stops at once, as a
//! throw does in the language.
//!
//! A `??` is changed too where the guard cannot show that its left operand
//! decides. The helper then applies the rule of the logical node: a left
//! operand that is falsy and not nullish decides nothing, and the fold
//! refuses.
//!
//! Only an operator whose right operand the guard walked is changed, and only
//! when that operand calls a compiler function or the operator is such a `??`.
//! The engine runs no source that the guard did not walk. The guard records
//! each such operator by its span (see [`ordinals`]).
//!
//! A right operand that runs only where the engine says so, or that runs where
//! it loses, can hold a part that the walk refuses. The walk records the
//! decline as a *put-off refusal* and goes on. The printed source has a call
//! of [`PUT_OFF`] with the position of that part in its place, and the call
//! throws the position. The parts in front of it run first, so a compiler call
//! there still writes its rule, as in the evaluator. Where the left operand
//! decides, the throw has no effect. Where it does not decide, the fold gives
//! the decline the walk recorded (see [`engine::threw_in_fold`]). ADR 0012
//! gives the reason that no other source throws a number.
//!
//! [`engine::threw_in_fold`]: super::engine::threw_in_fold

use indexmap::IndexMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use swc_core::{
  common::{Span, Spanned, util::take::Take},
  ecma::{
    ast::{BinExpr, CallExpr, Expr, Stmt},
    visit::{Visit, VisitMut, VisitMutWith, VisitWith},
  },
};

use stylex_ast::ast::factories::{
  create_arrow_expression, create_expr_or_spread, create_ident_call_expr,
  create_number_expr_or_spread, create_string_expr_or_spread,
};

use super::super::nodes::logical_expression::LogicalOp;
use super::Decline;
use crate::growable_stack::grown_per_level;

/// What the guard recorded at a node.
pub(super) enum Change {
  /// A logical operator that becomes a call of [`EITHER`].
  Either(LogicalOp),
  /// A part that the walk declined, which becomes a call of [`PUT_OFF`]. It
  /// holds the decline: a refusal, or [`Decline::NotACandidate`], which gives
  /// the call back to the dispatch below.
  PutOff(Decline),
}

impl Change {
  /// What the printer does at the node, without the decline.
  fn kind(&self) -> ChangeKind {
    match self {
      Self::Either(op) => ChangeKind::Either(*op),
      Self::PutOff(_) => ChangeKind::PutOff,
    }
  }
}

/// What the printer does at a recorded node. It is part of the memo key.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum ChangeKind {
  Either(LogicalOp),
  PutOff,
}

/// The nodes the guard recorded, by span, in the order it recorded them.
///
/// Ordered, so a walk that refuses can drop what it recorded by a length.
pub(super) type Recorded = IndexMap<Span, Change, FxBuildHasher>;

/// A recorded node as the printer and the memo key read it.
///
/// The span finds the node in the tree the printer changes, and the decline of
/// a put-off refusal in [`Recorded`]. The position is where the node stands
/// among the expressions of the printed trees, so it does not depend on the
/// file. It is part of the memo key, and a call of [`PUT_OFF`] throws it.
#[derive(Clone, Copy)]
pub(super) struct Changed {
  pub(super) span: Span,
  pub(super) position: usize,
  pub(super) kind: ChangeKind,
}

/// The name of the global that runs both operands of a logical operator.
pub(super) const EITHER: &str = "__sxEither";

/// The name of the global that throws the position of a put-off refusal.
pub(super) const PUT_OFF: &str = "__sxPutOff";

/// The source that adds [`EITHER`] and [`PUT_OFF`] to the global object of an
/// engine.
///
/// The properties are not writable, not enumerable and not configurable, so
/// the source a fold runs cannot replace them. The guard refuses every name
/// with these spellings, so no binding can hide them.
///
/// The operator travels as its own text. `??` takes the right operand only for
/// `null` and `undefined`, as the language does. A left operand of `??` that is
/// falsy and not nullish throws with the reason the logical node gives, because
/// that node does not fold it. A throw of the right operand comes first, as the
/// reason of the right operand comes first in that node.
///
/// [`PUT_OFF`] throws a number. ADR 0012 gives the reason that no other source
/// throws one.
pub(super) const EITHER_SOURCE: &str = r#"
Object.defineProperty(globalThis, "__sxEither", {
  value: (leftOperand, right, operator) => {
    let left;
    let leftError;
    let leftThrew = false;

    try {
      left = leftOperand();
    } catch (thrown) {
      if (typeof thrown !== "number") {
        throw thrown;
      }

      leftThrew = true;
      leftError = thrown;
    }

    let value;
    let error;
    let threw = false;

    try {
      value = right();
    } catch (thrown) {
      threw = true;
      error = thrown;
    }

    if (leftThrew) {
      throw leftError;
    }

    const leftDecides =
      operator === "||" ? !!left : operator === "&&" ? !left : left !== null && left !== undefined;

    if (leftDecides) {
      if (operator === "??" && !left) {
        throw threw ? error : new TypeError("unknown error");
      }

      return left;
    }

    if (threw) {
      throw error;
    }

    return value;
  },
});

Object.defineProperty(globalThis, "__sxPutOff", {
  value: (position) => {
    throw position;
  },
});
"#;

/// Each recorded node with its position among the expressions of the printed
/// trees, in the order the printer visits them: the call first, and then each
/// parameter default in its order.
///
/// The position is the same in each tree that has the same shape, so two folds
/// that print the same call but change different nodes do not share one
/// compiled script. Where two nodes have the same span, as an optional chain
/// and its base do, only the first in that order is the recorded node, and
/// [`Rewrite`] changes only that node.
pub(super) fn ordinals<'e>(
  call: &CallExpr,
  defaults: impl Iterator<Item = &'e Expr>,
  recorded: &Recorded,
) -> Vec<Changed> {
  if recorded.is_empty() {
    return Vec::new();
  }

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
///
/// It asks for room at each level, so a deep tree cannot overflow the stack.
struct Ordinals<'r> {
  recorded: &'r Recorded,
  next: usize,
  found: Vec<Changed>,
}

impl Visit for Ordinals<'_> {
  fn visit_expr(&mut self, expr: &Expr) {
    let span = expr.span();

    if let Some(change) = self.recorded.get(&span)
      && self.found.iter().all(|found| found.span != span)
    {
      self.found.push(Changed {
        span,
        position: self.next,
        kind: change.kind(),
      });
    }

    self.next += 1;

    grown_per_level(|| expr.visit_children_with(self));
  }

  fn visit_stmt(&mut self, stmt: &Stmt) {
    grown_per_level(|| stmt.visit_children_with(self));
  }
}

/// The printer's side of [`ordinals`]: it changes each recorded node of the
/// trees it is given, in the same order.
///
/// A node is changed once. Where a second node has the same span, it is left
/// as it is, so the printer changes the node that [`ordinals`] counted.
pub(super) struct Rewrite {
  changes: FxHashMap<Span, (usize, ChangeKind)>,
}

impl Rewrite {
  pub(super) fn new(changes: &[Changed]) -> Self {
    Self {
      changes: changes
        .iter()
        .map(|changed| (changed.span, (changed.position, changed.kind)))
        .collect(),
    }
  }

  /// Changes the recorded nodes of `tree`.
  pub(super) fn apply(&mut self, tree: &mut Expr) {
    if !self.changes.is_empty() {
      tree.visit_mut_with(self);
    }
  }
}

impl VisitMut for Rewrite {
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    let change = self.changes.remove(&expr.span());

    let replacement = match (change, &mut *expr) {
      // The walk did not carry the names of a declined part, so it is not
      // printed. A call that throws its position stands in its place.
      (Some((position, ChangeKind::PutOff)), _) => Some(Expr::Call(create_ident_call_expr(
        PUT_OFF,
        vec![create_number_expr_or_spread(position as f64)],
      ))),
      (Some((_, ChangeKind::Either(op))), Expr::Bin(bin)) => {
        bin.visit_mut_children_with(self);

        Some(either(bin, op))
      },
      // An `Either` is recorded only on a logical operator, and in a parsed
      // tree no other node above it has its span, so the first node with that
      // span is the operator. A tree that gives one span to two nodes could
      // drop the change here.
      (_, other) => {
        other.visit_mut_children_with(self);

        None
      },
    };

    if let Some(replacement) = replacement {
      *expr = replacement;
    }
  }
}

/// The logical operator `bin` as a call of [`EITHER`].
fn either(bin: &mut BinExpr, op: LogicalOp) -> Expr {
  Expr::Call(create_ident_call_expr(
    EITHER,
    vec![
      create_expr_or_spread(create_arrow_expression(bin.left.as_mut().take())),
      create_expr_or_spread(create_arrow_expression(bin.right.as_mut().take())),
      create_string_expr_or_spread(op.spelling()),
    ],
  ))
}

#[cfg(test)]
#[path = "tests/losing_operand_tests.rs"]
mod losing_operand_tests;
