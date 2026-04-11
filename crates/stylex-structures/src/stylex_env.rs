use std::rc::Rc;

use swc_core::ecma::ast::Expr;

/// An entry in the `env` configuration map.
///
/// - `Expr(Expr)` — any static compile-time value (string, number, object, array, …)
/// - `Function(JSFunction)` — a callable that receives `Vec<Expr>` and returns `Expr`
#[derive(Clone, Debug)]
pub enum EnvEntry {
  Expr(Expr),
  Function(JSFunction),
}

impl EnvEntry {
  pub fn as_expr(&self) -> Option<&Expr> {
    match self {
      EnvEntry::Expr(e) => Some(e),
      _ => None,
    }
  }

  pub fn as_function(&self) -> Option<&JSFunction> {
    match self {
      EnvEntry::Function(f) => Some(f),
      _ => None,
    }
  }

  pub fn is_function(&self) -> bool {
    matches!(self, EnvEntry::Function(_))
  }
}

/// A compile-time function from the `env` config.
/// Wraps a closure that takes `Expr` arguments and returns an `Expr`.
#[derive(Clone)]
pub struct JSFunction {
  inner: Rc<dyn Fn(Vec<Expr>) -> Expr>,
}

impl std::fmt::Debug for JSFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "JSFunction(<closure>)")
  }
}

impl JSFunction {
  pub fn new(inner: impl Fn(Vec<Expr>) -> Expr + 'static) -> Self {
    Self {
      inner: Rc::new(inner),
    }
  }

  pub fn call(&self, args: Vec<Expr>) -> Expr {
    (self.inner)(args)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use swc_core::ecma::ast::Lit;

  fn make_str_expr(s: &str) -> Expr {
    Expr::Lit(Lit::from(s))
  }

  fn make_num_expr(v: f64) -> Expr {
    Expr::Lit(Lit::from(v))
  }

  #[test]
  fn env_entry_as_expr_returns_some_for_expr() {
    let entry = EnvEntry::Expr(make_str_expr("hello"));
    assert!(entry.as_expr().is_some());
  }

  #[test]
  fn env_entry_as_expr_returns_none_for_function() {
    let entry = EnvEntry::Function(JSFunction::new(|_| make_num_expr(0.0)));
    assert!(entry.as_expr().is_none());
  }

  #[test]
  fn env_entry_as_function_returns_some_for_function() {
    let entry = EnvEntry::Function(JSFunction::new(|_| make_num_expr(0.0)));
    assert!(entry.as_function().is_some());
  }

  #[test]
  fn env_entry_as_function_returns_none_for_expr() {
    let entry = EnvEntry::Expr(make_str_expr("x"));
    assert!(entry.as_function().is_none());
  }

  #[test]
  fn env_entry_is_function_true() {
    let entry = EnvEntry::Function(JSFunction::new(|_| make_num_expr(0.0)));
    assert!(entry.is_function());
  }

  #[test]
  fn env_entry_is_function_false() {
    let entry = EnvEntry::Expr(make_num_expr(1.0));
    assert!(!entry.is_function());
  }

  #[test]
  fn js_function_call_works() {
    let f = JSFunction::new(|args| {
      if args.is_empty() {
        make_num_expr(0.0)
      } else {
        args[0].clone()
      }
    });
    let result = f.call(vec![make_str_expr("input")]);
    assert!(matches!(result, Expr::Lit(Lit::Str(_))));
  }

  #[test]
  fn js_function_debug_format() {
    let f = JSFunction::new(|_| make_num_expr(0.0));
    let debug = format!("{:?}", f);
    assert_eq!(debug, "JSFunction(<closure>)");
  }
}
