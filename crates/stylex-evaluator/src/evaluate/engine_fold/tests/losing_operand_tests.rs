//! What the printed source comes out as for a logical operator whose losing
//! operand runs.

use super::*;

use swc_core::common::{GLOBALS, Globals};

use stylex_constants::constants::evaluation_errors::reserved_compiler_name;

use super::super::engine::{FoldKey, print_fold};
use crate::evaluate::source_evaluation::assert_deopt_reason_contains;
use crate::tests::scaffolding::parse_expr;

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
