use crate::coercions::{to_int32, to_shift_count, to_uint32};
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::BinaryOp;

/// Applies a binary operator to two numeric operands.
///
/// Only the operators that produce a number are handled. A comparison or a
/// logical operator reaches a different reader, so one arriving here is a bug
/// in the caller rather than in the source being compiled.
pub fn evaluate_bin_expr(op: BinaryOp, left: f64, right: f64) -> f64 {
  match &op {
    BinaryOp::Add => left + right,
    BinaryOp::Sub => left - right,
    BinaryOp::Div => left / right,
    BinaryOp::Mul => left * right,
    BinaryOp::Mod => left % right,
    BinaryOp::Exp => left.powf(right),
    // Every bitwise operator reads its sides through `ToInt32` and its count
    // through `ToShiftCount`, which is what makes them 32-bit: a 64-bit cast
    // saturates where the language wraps, and it answered `0` for `-1 >>> 0`
    // where JavaScript answers 4294967295.
    BinaryOp::BitOr => f64::from(to_int32(left) | to_int32(right)),
    BinaryOp::BitXor => f64::from(to_int32(left) ^ to_int32(right)),
    BinaryOp::BitAnd => f64::from(to_int32(left) & to_int32(right)),
    BinaryOp::LShift => f64::from(to_int32(left) << to_shift_count(right)),
    BinaryOp::RShift => f64::from(to_int32(left) >> to_shift_count(right)),
    BinaryOp::ZeroFillRShift => f64::from(to_uint32(left) >> to_shift_count(right)),
    _ => stylex_panic!("Unsupported binary operator: {:?}", op),
  }
}

#[cfg(test)]
#[path = "tests/operators_tests.rs"]
mod tests;
