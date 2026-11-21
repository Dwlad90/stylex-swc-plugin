//! What the printed source comes out as for a logical operator whose losing
//! operand runs.

use super::*;

use swc_core::common::{DUMMY_SP, GLOBALS, Globals};
use swc_core::ecma::ast::{ArrayLit, ExprOrSpread};

use stylex_constants::constants::evaluation_errors::reserved_compiler_name;

use super::super::engine::{FoldKey, print_fold};
use crate::evaluate::source_evaluation::assert_deopt_reason_contains;
use crate::tests::scaffolding::{SMALL_THREAD, deep_or, on_a_thread_of, parse_expr};

/// The source `print_fold` writes for `expression` when the operators at
/// `positions` run their losing operand.
#[track_caller]
fn printed(expression: &str, positions: &[usize]) -> String {
  GLOBALS.set(&Globals::new(), || match parse_expr(expression) {
    Expr::Call(call) => print_fold(&call, Vec::new(), positions).source,
    other => panic!("`{}` parsed as {:?}", expression, other),
  })
}

/// Each operator travels as its own spelling.
#[test]
fn each_operator_is_printed_as_a_call_of_the_global() {
  for (source, expected) in [
    (
      "[a || f(1)].join()",
      r#"[__sxEither(a,()=>f(1),"||")].join()"#,
    ),
    (
      "[a && f(1)].join()",
      r#"[__sxEither(a,()=>f(1),"&&")].join()"#,
    ),
    (
      "[a ?? f(1)].join()",
      r#"[__sxEither(a,()=>f(1),"??")].join()"#,
    ),
  ] {
    assert_eq!(printed(source, &[0]), expected);
  }
}

/// Only the operators at the given positions change. The positions count the
/// logical operators in the order the tree is visited, an operator before its
/// operands.
#[test]
fn only_the_given_positions_change() {
  let source = "[a || (b || f(1)), c + d, e && f(2)].join()";

  assert_eq!(
    printed(source, &[1, 2]),
    r#"[a||(__sxEither(b,()=>f(1),"||")),c+d,__sxEither(e,()=>f(2),"&&")].join()"#
  );
  assert_eq!(printed(source, &[]), "[a||(b||f(1)),c+d,e&&f(2)].join()");
}

/// The positions are part of the memo key, so two prints of one call do not
/// share a script. No positions leave the key as it was.
#[test]
fn the_positions_are_part_of_the_key() {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Call(call) = parse_expr("[a || f(1)].join()") else {
      panic!("the source is not a call");
    };

    let plain = FoldKey::new(&call, 7, &[]);

    assert!(plain == FoldKey::new(&call, 7, &[]));
    assert!(plain != FoldKey::new(&call, 7, &[0]));
  });
}

/// A binding that spells the global is refused, so it cannot hide it.
#[test]
fn a_binding_that_spells_the_global_refuses() {
  assert_deopt_reason_contains(
    "[1].map((__sxEither) => __sxEither)",
    &reserved_compiler_name(EITHER),
  );
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

      *member.obj = Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: vec![Some(ExprOrSpread {
          spread: None,
          expr: Box::new(deep_or(100_000)),
        })],
      });

      assert!(ordinals(&call, std::iter::empty(), &Recorded::default()).is_empty());

      // A tree this deep drops by recursion too, so it is not dropped here.
      std::mem::forget(call);
    });
  });
}
