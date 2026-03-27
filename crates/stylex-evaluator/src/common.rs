use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use radix_fmt::radix;
use stylex_macros::stylex_panic;
use swc_core::ecma::{
  ast::{BinaryOp, Expr, VarDeclarator},
  utils::drop_span,
};

pub fn create_hash(value: &str) -> String {
  radix(murmur2::murmur2(value.as_bytes(), 1), 36).to_string()
}

pub fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: bool) -> String {
  if should_wrap_in_quotes {
    format!("\"{}\"", key)
  } else {
    key.to_string()
  }
}

pub fn get_expr_from_var_decl(var_decl: &VarDeclarator) -> &Expr {
  match &var_decl.init {
    Some(var_decl_init) => var_decl_init,
    None => stylex_panic!("Variable declaration must be initialized with an expression."),
  }
}

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

pub fn hash_f64(value: f64) -> u64 {
  let bits = value.to_bits();
  let mut hasher = DefaultHasher::new();
  bits.hash(&mut hasher);
  hasher.finish()
}

pub fn round_f64(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10f64.powi(decimal_places as i32);
  (value * multiplier).round() / multiplier
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

pub fn sort_numbers_factory() -> impl FnMut(&f64, &f64) -> std::cmp::Ordering {
  |a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

pub fn char_code_at(s: &str, index: usize) -> Option<u32> {
  s.chars().nth(index).map(|c| c as u32)
}

pub fn stable_hash<T: Hash>(t: &T) -> u64 {
  let mut hasher = DefaultHasher::new();
  t.hash(&mut hasher);
  hasher.finish()
}

pub fn find_and_swap_remove<T, F>(vec: &mut Vec<T>, predicate: F) -> Option<T>
where
  F: FnMut(&T) -> bool,
{
  vec
    .iter()
    .position(predicate)
    .map(|index| vec.swap_remove(index))
}

pub fn create_short_hash(value: &str) -> String {
  let hash = murmur2::murmur2(value.as_bytes(), 1) % (62u32.pow(5));
  base62::encode(hash)
}

pub fn resolve_node_package_path(package_name: &str) -> Result<PathBuf, String> {
  match node_resolve::Resolver::default()
    .with_basedir(PathBuf::from("./cwd"))
    .preserve_symlinks(true)
    .with_extensions([".ts", ".tsx", ".js", ".jsx", ".json"])
    .with_main_fields(vec![String::from("main"), String::from("module")])
    .resolve(package_name)
  {
    Ok(path) => Ok(path),
    Err(error) => Err(format!(
      "Error resolving package {}: {:?}",
      package_name, error
    )),
  }
}
