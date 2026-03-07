use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    Bool, Expr, IdentName, KeyValueProp, Lit, Null, Number, ObjectLit, Prop, PropName,
    PropOrSpread, Str,
  },
};

/// Represents a JavaScript value passed via the `env` configuration option.
///
/// `env` values can be:
/// - Simple primitives (strings, numbers, booleans, null)
/// - Nested objects (for structured tokens)
/// - Functions that take string/number arguments and return strings
///   (for compile-time utility functions like `colorMix`, `opacity`, etc.)
#[derive(Clone, Debug)]
pub enum EnvValue {
  String(String),
  Number(f64),
  Bool(bool),
  Null,
  Object(IndexMap<String, EnvValue>),
  /// A compile-time function that receives evaluated arguments as `EnvValue`s
  /// and returns a string result.
  Function(JSFunction),
}

/// A compile-time function from the `env` config.
/// Wraps a closure that takes a list of `EnvValue` arguments and returns a string.
#[derive(Clone)]
pub struct JSFunction {
  inner: Rc<dyn Fn(Vec<EnvValue>) -> String>,
}

impl JSFunction {
  pub fn new(f: impl Fn(Vec<EnvValue>) -> String + 'static) -> Self {
    Self { inner: Rc::new(f) }
  }

  pub fn call(&self, args: Vec<EnvValue>) -> String {
    (self.inner)(args)
  }
}

impl std::fmt::Debug for JSFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "EnvFunction(<closure>)")
  }
}

impl EnvValue {
  /// Converts this `EnvValue` into a string representation suitable for CSS values.
  pub fn to_css_string(&self) -> Option<String> {
    match self {
      EnvValue::String(s) => Some(s.clone()),
      EnvValue::Number(n) => {
        if n.fract() == 0.0 {
          Some(format!("{}", *n as i64))
        } else {
          Some(n.to_string())
        }
      }
      EnvValue::Bool(b) => Some(b.to_string()),
      EnvValue::Null => None,
      EnvValue::Object(_) => None,
      EnvValue::Function(_) => None,
    }
  }

  /// Converts this `EnvValue` to a SWC `Expr`.
  /// Functions are not convertible and return `None`.
  pub fn to_expr(&self) -> Option<Expr> {
    match self {
      EnvValue::String(s) => Some(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: s.as_str().into(),
        raw: None,
      }))),
      EnvValue::Number(n) => Some(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: *n,
        raw: None,
      }))),
      EnvValue::Bool(b) => Some(Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value: *b,
      }))),
      EnvValue::Null => Some(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
      EnvValue::Object(map) => {
        let props: Vec<PropOrSpread> = map
          .iter()
          .filter_map(|(key, val)| {
            val.to_expr().map(|expr| {
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(IdentName::new(key.as_str().into(), DUMMY_SP)),
                value: Box::new(expr),
              })))
            })
          })
          .collect();

        Some(Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props,
        }))
      }
      EnvValue::Function(_) => None,
    }
  }

  /// Returns `true` if this is a `Function` variant.
  pub fn is_function(&self) -> bool {
    matches!(self, EnvValue::Function(_))
  }

  /// Returns the inner `EnvFunction` if this is a `Function` variant.
  pub fn as_function(&self) -> Option<&JSFunction> {
    match self {
      EnvValue::Function(f) => Some(f),
      _ => None,
    }
  }
}

/// Converts an `Expr` (from evaluated arguments) back to an `EnvValue` for passing to env functions.
pub fn expr_to_env_value(expr: &Expr) -> EnvValue {
  match expr {
    Expr::Lit(Lit::Str(s)) => {
      EnvValue::String(s.value.as_str().map(|s| s.to_string()).unwrap_or_default())
    }
    Expr::Lit(Lit::Num(n)) => EnvValue::Number(n.value),
    Expr::Lit(Lit::Bool(b)) => EnvValue::Bool(b.value),
    Expr::Lit(Lit::Null(_)) => EnvValue::Null,
    _ => EnvValue::String(format!("{:?}", expr)),
  }
}
