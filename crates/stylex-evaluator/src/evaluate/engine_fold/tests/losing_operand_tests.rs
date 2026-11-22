//! What the printed source comes out as for a logical operator whose losing
//! operand runs, and for a part that holds a put-off refusal.

use super::*;

use swc_core::common::{DUMMY_SP, GLOBALS, Globals};
use swc_core::ecma::ast::{
  ArrayLit, BinaryOp, Callee, ExprOrSpread, IdentName, MemberExpr, MemberProp,
};

use stylex_constants::constants::evaluation_errors::reserved_compiler_name;

use super::super::engine::{FoldKey, print_fold};
use crate::evaluate::source_evaluation::assert_deopt_reason_contains;
use crate::tests::scaffolding::{SMALL_THREAD, deep_or, on_a_thread_of, parse_expr};

/// The logical operators of a tree and its calls named `f`, in the order the
/// tree is visited.
#[derive(Default)]
struct Nodes {
  operators: Vec<(Span, LogicalOp)>,
  calls: Vec<Span>,
}

impl Visit for Nodes {
  fn visit_bin_expr(&mut self, bin: &BinExpr) {
    if let Some(op) = LogicalOp::of(bin.op) {
      self.operators.push((bin.span, op));
    }

    bin.visit_children_with(self);
  }

  fn visit_call_expr(&mut self, call: &CallExpr) {
    if call
      .callee
      .as_expr()
      .and_then(|callee| callee.as_ident())
      .is_some_and(|callee| callee.sym == "f")
    {
      self.calls.push(call.span);
    }

    call.visit_children_with(self);
  }
}

/// The source `print_fold` writes for `expression` when the operators at
/// `operators` run their losing operand, and the calls of `f` at `put_off`
/// hold a put-off refusal. Both count from zero in the order of the tree.
#[track_caller]
fn printed(expression: &str, operators: &[usize], put_off: &[usize]) -> String {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Call(call) = parse_expr(expression) else {
      panic!("`{}` is not a call", expression);
    };

    let mut nodes = Nodes::default();
    call.visit_with(&mut nodes);

    let mut recorded = Recorded::default();

    for &index in operators {
      let (span, op) = nodes.operators[index];
      recorded.insert(span, Change::Either(op));
    }

    for &index in put_off {
      recorded.insert(nodes.calls[index], Change::PutOff(Decline::NotACandidate));
    }

    let changed = ordinals(&call, std::iter::empty(), &recorded);

    print_fold(&call, Vec::new(), &changed).source
  })
}

/// Each operator travels as its own spelling.
#[test]
fn each_operator_is_printed_as_a_call_of_the_global() {
  for (source, expected) in [
    (
      "[a || f(1)].join()",
      r#"[__sxEither(()=>a,()=>f(1),"||")].join()"#,
    ),
    (
      "[a && f(1)].join()",
      r#"[__sxEither(()=>a,()=>f(1),"&&")].join()"#,
    ),
    (
      "[a ?? f(1)].join()",
      r#"[__sxEither(()=>a,()=>f(1),"??")].join()"#,
    ),
  ] {
    assert_eq!(printed(source, &[0], &[]), expected);
  }
}

/// Only the recorded operators change.
#[test]
fn only_the_given_operators_change() {
  let source = "[a || (b || f(1)), c + d, e && f(2)].join()";

  assert_eq!(
    printed(source, &[1, 2], &[]),
    r#"[a||(__sxEither(()=>b,()=>f(1),"||")),c+d,__sxEither(()=>e,()=>f(2),"&&")].join()"#
  );
  assert_eq!(
    printed(source, &[], &[]),
    "[a||(b||f(1)),c+d,e&&f(2)].join()"
  );
}

/// A part that holds a put-off refusal is not printed. A call that throws its
/// position stands in its place, and the parts in front of it stay.
#[test]
fn a_put_off_part_is_printed_as_a_call_that_throws_its_position() {
  assert_eq!(
    printed("[a || [g(1), f(2)]].join()", &[0], &[0]),
    r#"[__sxEither(()=>a,()=>[g(1),__sxPutOff(8)],"||")].join()"#
  );
}

/// Where two nodes have one span, only the first in the order of the tree is
/// changed, as only that node has a position.
#[test]
fn of_two_nodes_with_one_span_only_the_first_changes() {
  struct SameSpan(Option<Span>);

  impl VisitMut for SameSpan {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
      call.visit_mut_children_with(self);

      match self.0 {
        Some(span) => call.span = span,
        None => self.0 = Some(call.span),
      }
    }
  }

  GLOBALS.set(&Globals::new(), || {
    let Expr::Array(mut array) = parse_expr("[f(1), f(2)]") else {
      panic!("the source is not an array");
    };

    array.visit_mut_with(&mut SameSpan(None));

    let span = array.elems[0].as_ref().map(|elem| elem.expr.span());
    let mut recorded = Recorded::default();
    recorded.insert(
      span.unwrap_or(DUMMY_SP),
      Change::PutOff(Decline::NotACandidate),
    );

    let call = CallExpr {
      span: DUMMY_SP,
      ctxt: Default::default(),
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Array(array)),
        prop: MemberProp::Ident(IdentName::new("join".into(), DUMMY_SP)),
      }))),
      args: Vec::new(),
      type_args: None,
    };

    let changed = ordinals(&call, std::iter::empty(), &recorded);

    assert_eq!(changed.len(), 1);
    assert_eq!(
      print_fold(&call, Vec::new(), &changed).source,
      "[__sxPutOff(2),f(2)].join()"
    );
  });
}

/// The positions and the kind of change are part of the key, so two prints of
/// one call do not share a script. No changes leave the key as it was.
#[test]
fn the_changes_are_part_of_the_key() {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Call(call) = parse_expr("[a || f(1)].join()") else {
      panic!("the source is not a call");
    };

    let changed = |position, kind| Changed {
      span: DUMMY_SP,
      position,
      kind,
    };
    let either = changed(0, ChangeKind::Either(LogicalOp::Or));
    let put_off = changed(0, ChangeKind::PutOff);
    let plain = FoldKey::new(&call, 7, &[]);

    assert!(plain == FoldKey::new(&call, 7, &[]));
    assert!(plain != FoldKey::new(&call, 7, std::slice::from_ref(&either)));
    assert!(
      FoldKey::new(&call, 7, std::slice::from_ref(&either))
        != FoldKey::new(&call, 7, std::slice::from_ref(&put_off))
    );
  });
}

/// A binding that spells either global is refused, so it cannot hide it.
#[test]
fn a_binding_that_spells_a_global_refuses() {
  for global in [EITHER, PUT_OFF] {
    assert_deopt_reason_contains(
      &format!("[1].map(({global}) => {global})"),
      &reserved_compiler_name(global),
    );
  }
}

/// A call nested far deeper than a test thread can recurse is counted on a
/// stack that grows, so counting cannot overflow.
#[test]
fn a_deep_call_is_counted_without_overflow() {
  on_a_thread_of(SMALL_THREAD, || {
    GLOBALS.set(&Globals::new(), || {
      let Expr::Call(mut call) = parse_expr("[].join()") else {
        panic!("the source is not a call");
      };
      let Some(Expr::Member(member)) = call.callee.as_mut_expr().map(|callee| &mut **callee) else {
        panic!("the callee is not a member read");
      };

      let deep = deep_or(100_000);
      let Expr::Bin(BinExpr {
        span,
        op: BinaryOp::LogicalOr,
        ..
      }) = &deep
      else {
        panic!("the deep tree is not an operator");
      };
      let mut recorded = Recorded::default();
      recorded.insert(*span, Change::Either(LogicalOp::Or));

      *member.obj = Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: vec![Some(ExprOrSpread {
          spread: None,
          expr: Box::new(deep),
        })],
      });

      assert_eq!(ordinals(&call, std::iter::empty(), &recorded).len(), 1);

      // A tree this deep drops by recursion too, so it is not dropped here.
      std::mem::forget(call);
    });
  });
}
