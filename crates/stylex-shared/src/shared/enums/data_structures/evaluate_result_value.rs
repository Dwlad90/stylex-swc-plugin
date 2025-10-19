use std::{fmt, rc::Rc};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::{Serialize, ser::Serializer};
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{Expr, KeyValueProp, Lit},
    codegen::Config,
  },
};

use crate::shared::{
  structures::{functions::FunctionConfig, theme_ref::ThemeRef, types::EvaluationCallback},
  utils::log::build_code_frame_error::{CodeFrame, create_module, print_module},
};

pub enum EvaluateResultValue {
  Expr(Expr),
  Vec(Vec<Option<EvaluateResultValue>>),
  Map(IndexMap<Expr, Vec<KeyValueProp>>),
  Entries(IndexMap<Lit, Box<Expr>>),
  Callback(EvaluationCallback),
  FunctionConfig(FunctionConfig),
  FunctionConfigMap(FxHashMap<Atom, FunctionConfig>),
  ThemeRef(ThemeRef),
}

impl Serialize for EvaluateResultValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Expr(expr) => {
        let module = create_module(expr);
        let code_frame = CodeFrame::new();

        let printed_module = print_module(
          &code_frame,
          module,
          Some(
            Config::default()
              .with_minify(true)
              .with_omit_last_semi(true)
              .with_reduce_escaped_newline(true),
          ),
        );

        serializer.serialize_str(&printed_module)
      }
      Self::Map(_) => unimplemented!("map serialization is not implemented yet"),
      Self::Entries(_) => unimplemented!("entries serialization is not implemented yet"),
      Self::Callback(_) => unimplemented!("callback serialization is not implemented yet"),
      Self::FunctionConfig(_) => {
        unimplemented!("function_config serialization is not implemented yet")
      }
      Self::FunctionConfigMap(_) => {
        unimplemented!("function_config_map serialization is not implemented yet")
      }
      Self::ThemeRef(_) => unimplemented!("theme_ref serialization is not implemented yet"),
      Self::Vec(_) => {
        unimplemented!("Vec serialization is not implemented yet");
        // let mut map = serializer.serialize_map(Some(1))?;
        // map.serialize_entry("type", "vec")?;
        // map.end()
      }
    }
  }
}

impl Clone for EvaluateResultValue {
  fn clone(&self) -> Self {
    match self {
      Self::Expr(e) => Self::Expr(e.clone()),
      Self::Vec(v) => Self::Vec(v.clone()),
      Self::Map(m) => Self::Map(m.clone()),
      Self::Entries(e) => Self::Entries(e.clone()),
      Self::FunctionConfig(f) => Self::FunctionConfig(f.clone()),
      Self::FunctionConfigMap(f) => Self::FunctionConfigMap(f.clone()),
      Self::Callback(c) => Self::Callback(Rc::clone(c)),
      Self::ThemeRef(tr) => Self::ThemeRef(tr.clone()),
    }
  }
}

impl fmt::Debug for EvaluateResultValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Expr(e) => f.debug_tuple("Expr").field(e).finish(),
      Self::Vec(v) => f.debug_tuple("Vec").field(v).finish(),
      Self::Map(m) => f.debug_tuple("Map").field(m).finish(),
      Self::Entries(e) => f.debug_tuple("Entries").field(e).finish(),
      Self::FunctionConfig(e) => f.debug_tuple("FunctionConfig").field(e).finish(),
      Self::FunctionConfigMap(e) => f.debug_tuple("FunctionConfigMap").field(e).finish(),
      Self::ThemeRef(e) => f.debug_tuple("ThemeRef").field(e).finish(),
      Self::Callback(_) => f.debug_tuple("Callback").field(&"Callback").finish(),
    }
  }
}

impl PartialEq for EvaluateResultValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Expr(e1), Self::Expr(e2)) => e1 == e2,
      (Self::Vec(v1), Self::Vec(v2)) => v1 == v2,
      (Self::ThemeRef(v1), Self::ThemeRef(v2)) => v1 == v2,
      (Self::Map(m1), Self::Map(m2)) => m1 == m2,
      (Self::FunctionConfig(f1), Self::FunctionConfig(f2)) => f1 == f2,
      (Self::FunctionConfigMap(f1), Self::FunctionConfigMap(f2)) => f1 == f2,
      (Self::Callback(_), Self::Callback(_)) => false,
      _ => false,
    }
  }
}

impl EvaluateResultValue {
  /// Extracts an ObjectLit from an EvaluateResultValue if it contains an Expr(Object).
  ///
  /// This is a common pattern when evaluating spread expressions or object literals.
  ///
  /// # Example
  /// ```ignore
  /// let Some(obj) = spread_expression.into_object() else {
  ///   return None;
  /// };
  /// ```
  #[inline]
  pub fn into_object(self) -> Option<swc_core::ecma::ast::ObjectLit> {
    match self {
      Self::Expr(Expr::Object(obj)) => Some(obj),
      _ => None,
    }
  }

  /// Extracts an ArrayLit from an EvaluateResultValue if it contains an Expr(Array).
  ///
  /// This is a common pattern when evaluating array expressions.
  ///
  /// # Example
  /// ```ignore
  /// let Some(arr) = value.into_array() else {
  ///   return None;
  /// };
  /// ```
  #[inline]
  pub fn into_array(self) -> Option<swc_core::ecma::ast::ArrayLit> {
    match self {
      Self::Expr(Expr::Array(arr)) => Some(arr),
      _ => None,
    }
  }
}
