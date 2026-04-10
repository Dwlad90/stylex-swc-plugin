use stylex_macros::stylex_panic;
use swc_core::ecma::{
  ast::{BinaryOp, Expr},
  utils::drop_span,
};

pub fn evaluate_bin_expr(op: BinaryOp, left: f64, right: f64) -> f64 {
  match &op {
    BinaryOp::Add => left + right,
    BinaryOp::Sub => left - right,
    BinaryOp::Div => left / right,
    BinaryOp::Mul => left * right,
    BinaryOp::Mod => left % right,
    BinaryOp::Exp => left.powf(right),
    BinaryOp::BitOr => (left as i64 | right as i64) as f64,
    BinaryOp::BitXor => (left as i64 ^ right as i64) as f64,
    BinaryOp::BitAnd => (left as i64 & right as i64) as f64,
    BinaryOp::LShift => ((left as i64) << (right as u64)) as f64,
    BinaryOp::RShift => ((left as i64) >> (right as u64)) as f64,
    BinaryOp::ZeroFillRShift => ((left as u64) >> (right as u64)) as f64,
    _ => stylex_panic!("Unsupported binary operator: {:?}", op),
  }
}
pub fn normalize_expr(expr: &mut Expr) -> &mut Expr {
  match expr {
    Expr::Paren(paren) => normalize_expr(paren.expr.as_mut()),
    _ => {
      *expr = drop_span(expr.clone());
      expr
    },
  }
}
