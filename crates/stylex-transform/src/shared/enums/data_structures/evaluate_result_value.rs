use std::{fmt, rc::Rc};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::{
  Deserialize, Deserializer, Serialize,
  de::{Error, Visitor},
  ser::Serializer,
};
use stylex_macros::stylex_unimplemented;
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
use stylex_structures::stylex_env::EnvEntry;

pub enum EvaluateResultValue {
  Null,
  Expr(Expr),
  Vec(Vec<EvaluateResultValue>),
  Map(IndexMap<Expr, Vec<KeyValueProp>>),
  Entries(IndexMap<Lit, Box<Expr>>),
  Callback(EvaluationCallback),
  FunctionConfig(FunctionConfig),
  FunctionConfigMap(FxHashMap<Atom, FunctionConfig>),
  ThemeRef(ThemeRef),
  /// An env object from the `env` config option.
  EnvObject(IndexMap<String, EnvEntry>),
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Serialize for EvaluateResultValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Null => serializer.serialize_none(),
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
      },
      Self::Map(_) => stylex_unimplemented!("Serialization of Map values is not yet supported."),
      Self::Entries(_) => {
        stylex_unimplemented!("Serialization of Entries values is not yet supported.")
      },
      Self::Callback(_) => {
        stylex_unimplemented!("Serialization of Callback values is not yet supported.")
      },
      Self::FunctionConfig(_) => {
        stylex_unimplemented!("Serialization of FunctionConfig values is not yet supported.")
      },
      Self::FunctionConfigMap(_) => {
        stylex_unimplemented!("Serialization of FunctionConfigMap values is not yet supported.")
      },
      Self::ThemeRef(_) => {
        stylex_unimplemented!("Serialization of ThemeRef values is not yet supported.")
      },
      Self::Vec(_) => {
        stylex_unimplemented!("Serialization of Vec values is not yet supported.")
      },
      Self::EnvObject(_) => {
        stylex_unimplemented!("Serialization of EnvObject values is not yet supported.")
      },
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl<'de> Deserialize<'de> for EvaluateResultValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct EvaluateResultValueVisitor;

    impl<'de> Visitor<'de> for EvaluateResultValueVisitor {
      type Value = EvaluateResultValue;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("null")
      }

      fn visit_none<E>(self) -> Result<Self::Value, E>
      where
        E: Error,
      {
        Ok(EvaluateResultValue::Null)
      }

      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: Error,
      {
        Ok(EvaluateResultValue::Null)
      }

      fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
      where
        D: Deserializer<'de>,
      {
        let _ = deserializer;
        Err(Error::custom(
          "only null EvaluateResultValue deserialization is supported",
        ))
      }
    }

    deserializer.deserialize_option(EvaluateResultValueVisitor)
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Clone for EvaluateResultValue {
  fn clone(&self) -> Self {
    match self {
      Self::Null => Self::Null,
      Self::Expr(e) => Self::Expr(e.clone()),
      Self::Vec(v) => Self::Vec(v.clone()),
      Self::Map(m) => Self::Map(m.clone()),
      Self::Entries(e) => Self::Entries(e.clone()),
      Self::FunctionConfig(f) => Self::FunctionConfig(f.clone()),
      Self::FunctionConfigMap(f) => Self::FunctionConfigMap(f.clone()),
      Self::Callback(c) => Self::Callback(Rc::clone(c)),
      Self::ThemeRef(tr) => Self::ThemeRef(tr.clone()),
      Self::EnvObject(e) => Self::EnvObject(e.clone()),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl fmt::Debug for EvaluateResultValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Null => f.debug_tuple("Null").finish(),
      Self::Expr(e) => f.debug_tuple("Expr").field(e).finish(),
      Self::Vec(v) => f.debug_tuple("Vec").field(v).finish(),
      Self::Map(m) => f.debug_tuple("Map").field(m).finish(),
      Self::Entries(e) => f.debug_tuple("Entries").field(e).finish(),
      Self::FunctionConfig(e) => f.debug_tuple("FunctionConfig").field(e).finish(),
      Self::FunctionConfigMap(e) => f.debug_tuple("FunctionConfigMap").field(e).finish(),
      Self::ThemeRef(e) => f.debug_tuple("ThemeRef").field(e).finish(),
      Self::Callback(_) => f.debug_tuple("Callback").field(&"Callback").finish(),
      Self::EnvObject(e) => f.debug_tuple("EnvObject").field(e).finish(),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for EvaluateResultValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Null, Self::Null) => true,
      (Self::Expr(e1), Self::Expr(e2)) => e1 == e2,
      (Self::Vec(v1), Self::Vec(v2)) => v1 == v2,
      (Self::ThemeRef(v1), Self::ThemeRef(v2)) => v1 == v2,
      (Self::Map(m1), Self::Map(m2)) => m1 == m2,
      (Self::FunctionConfig(f1), Self::FunctionConfig(f2)) => f1 == f2,
      (Self::FunctionConfigMap(f1), Self::FunctionConfigMap(f2)) => f1 == f2,
      (Self::Callback(_), Self::Callback(_)) => false,
      (Self::EnvObject(_), Self::EnvObject(_)) => false,
      _ => false,
    }
  }
}

impl EvaluateResultValue {
  /// Extracts an ObjectLit from an EvaluateResultValue if it contains an
  /// Expr(Object).
  ///
  /// This is a common pattern when evaluating spread expressions or object
  /// literals.
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

  /// Extracts an ArrayLit from an EvaluateResultValue if it contains an
  /// Expr(Array).
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

  /// Extracts a string key from an `EvaluateResultValue::Expr` variant.
  ///
  /// Handles the common pattern of resolving a property name from an evaluated
  /// expression:
  /// - `Expr::Ident` → symbol name as string
  /// - `Expr::Lit(Str)` → string value
  /// - `Expr::Lit(Num)` → number formatted as string
  /// - `Expr::Lit(BigInt)` → bigint formatted as string
  /// - All other variants → `None`
  ///
  /// # Example
  /// ```ignore
  /// let key = property.as_string_key().expect("Property must be a string key");
  /// ```
  #[inline]
  pub fn as_string_key(&self) -> Option<String> {
    match self {
      Self::Expr(expr) => match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Lit(Lit::Str(s)) => s.value.as_str().map(str::to_string),
        Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
        Expr::Lit(Lit::BigInt(bi)) => Some(bi.value.to_string()),
        _ => None,
      },
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::EvaluateResultValue;

  #[test]
  fn serializes_null_as_json_null() {
    let json = match serde_json::to_string(&EvaluateResultValue::Null) {
      Ok(json) => json,
      Err(error) => panic!("failed to serialize null evaluate result: {error}"),
    };

    assert_eq!(json, "null");
  }

  #[test]
  fn deserializes_json_null_as_null() {
    let value = match serde_json::from_str::<EvaluateResultValue>("null") {
      Ok(value) => value,
      Err(error) => panic!("failed to deserialize null evaluate result: {error}"),
    };

    assert_eq!(value, EvaluateResultValue::Null);
  }
}
