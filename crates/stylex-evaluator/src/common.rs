use std::path::PathBuf;

use stylex_macros::stylex_panic;
use swc_core::ecma::{
  ast::{BinaryOp, Expr, VarDeclarator},
  utils::drop_span,
};

/// Extracts the initializer expression from a variable declarator, panicking
/// if no initializer is present.
pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> &Expr {
  match &var_decl.init {
    Some(var_decl_init) => var_decl_init,
    None => stylex_panic!("Variable declaration must be initialized with an expression."),
  }
}

/// Evaluates a binary expression with the given operator and numeric operands.
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

/// Unwraps parenthesized expressions and drops source spans.
pub fn normalize_expr(expr: &mut Expr) -> &mut Expr {
  match expr {
    Expr::Paren(paren) => normalize_expr(paren.expr.as_mut()),
    _ => {
      *expr = drop_span(expr.clone());
      expr
    },
  }
}

/// Resolves a Node.js package path relative to the current working directory.
///
/// The `current_dir()` call can only fail if the working directory has been
/// deleted while the process is running — unreachable in any realistic test
/// scenario. The thin delegation to `resolve_node_package_path_from_basedir`
/// is tested directly.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn resolve_node_package_path(package_name: &str) -> Result<PathBuf, String> {
  let basedir = std::env::current_dir()
    .map_err(|error| format!("Error determining current working directory: {:?}", error))?;

  resolve_node_package_path_from_basedir(package_name, basedir)
}

fn resolve_node_package_path_from_basedir(
  package_name: &str,
  basedir: PathBuf,
) -> Result<PathBuf, String> {
  let resolver = node_resolve::Resolver::default();
  let resolver = resolver.with_basedir(basedir);
  let resolver = resolver.preserve_symlinks(true);
  let resolver = resolver.with_extensions([".ts", ".tsx", ".js", ".jsx", ".json"]);
  let resolver = resolver.with_main_fields(vec![String::from("main"), String::from("module")]);

  match resolver.resolve(package_name) {
    Ok(path) => Ok(path),
    Err(error) => Err(format!(
      "Error resolving package {}: {:?}",
      package_name, error
    )),
  }
}

#[cfg(test)]
#[path = "tests/common_tests.rs"]
mod tests;
