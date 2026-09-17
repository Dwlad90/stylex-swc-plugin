use std::hash::{Hash, Hasher};
use std::mem::discriminant;

use rustc_hash::FxHasher;

use swc_core::ecma::ast::Expr;

use stylex_styleq::StyleqValue;

use stylex_structures::base_css_type::BaseCSSType;
use stylex_types::structures::injectable_style::InjectableStyle;

use crate::types::FlatCompiledStyles;

/// One value a compiled style map holds.
///
/// Two readers share the type. A compiled namespace holds a class name, a
/// `null` for a property it clears, the compiled marker, an injectable style or
/// a variable tuple. An inline style -- a plain object a `props` call was given
/// -- holds what the author wrote instead, which is text, a number, a boolean,
/// a `null` or an object of the same kind again.
#[derive(Debug, PartialEq, Clone)]
pub enum FlatCompiledStylesValue {
  String(String),
  /// A number an inline declaration holds. Kept as a number, because the value
  /// a `style` property carries is the one the author wrote: `opacity: 0.5` is
  /// a number and `opacity: '0.5'` is text.
  Number(f64),
  /// An object an inline declaration holds, such as the body of `:hover`, and
  /// the inline style itself once a merge has written it under `style`.
  Object(FlatCompiledStyles),
  Null,
  InjectableStyle(InjectableStyle),
  Bool(bool),
  Tuple(String, Box<Expr>, Option<BaseCSSType>),
}

/// Written out rather than derived, because two of the kinds hold something
/// the derived spelling reads wrongly or not at all.
///
/// A number has no `Hash` of its own, so its bits stand for it. That agrees
/// with equality everywhere but the two zeroes and `NaN`: `-0` and `0` are one
/// value with two bit patterns, and two `NaN`s are two values with one. Both
/// are rare, and both cost a lookup rather than an answer -- a merge that
/// misses reads the style again.
///
/// An object holds an `IndexMap`, which compares without regard to the order
/// its names were written in. The hash is combined without regard to it too,
/// so two objects that hold the same names and values stay one value. The
/// count goes in as well, so an object holding an empty object beside a `null`
/// does not write the bytes of one holding an object that holds the `null`.
impl Hash for FlatCompiledStylesValue {
  fn hash<H: Hasher>(&self, state: &mut H) {
    discriminant(self).hash(state);

    match self {
      FlatCompiledStylesValue::String(value) => value.hash(state),
      FlatCompiledStylesValue::Number(value) => value.to_bits().hash(state),
      FlatCompiledStylesValue::Object(values) => {
        values.len().hash(state);

        let combined = values.iter().fold(0, |combined, (key, value)| {
          let mut entry = FxHasher::default();

          key.hash(&mut entry);
          value.hash(&mut entry);

          combined ^ entry.finish()
        });

        combined.hash(state);
      },
      FlatCompiledStylesValue::Null => {},
      FlatCompiledStylesValue::InjectableStyle(value) => value.hash(state),
      FlatCompiledStylesValue::Bool(value) => value.hash(state),
      FlatCompiledStylesValue::Tuple(key, value, css_type) => {
        key.hash(state);
        value.hash(state);
        css_type.hash(state);
      },
    }
  }
}

impl FlatCompiledStylesValue {
  pub fn as_tuple(&self) -> Option<(&String, &Expr, &Option<BaseCSSType>)> {
    match self {
      FlatCompiledStylesValue::Tuple(key, value, css_type) => Some((key, value, css_type)),
      _ => None,
    }
  }

  pub fn as_string(&self) -> Option<&String> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value),
      _ => None,
    }
  }

  pub fn as_injectable_style(&self) -> Option<&InjectableStyle> {
    match self {
      FlatCompiledStylesValue::InjectableStyle(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn _as_bool(&self) -> Option<&bool> {
    match self {
      FlatCompiledStylesValue::Bool(value) => Some(value),
      _ => None,
    }
  }

  pub(crate) fn _as_null(&self) -> Option<()> {
    match self {
      FlatCompiledStylesValue::Null => Some(()),
      _ => None,
    }
  }

  pub fn as_object(&self) -> Option<&FlatCompiledStyles> {
    match self {
      FlatCompiledStylesValue::Object(values) => Some(values),
      _ => None,
    }
  }
}

impl StyleqValue for FlatCompiledStylesValue {
  fn as_class_name(&self) -> Option<&str> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value.as_str()),
      _ => None,
    }
  }

  fn is_null(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Null)
  }

  fn is_true_bool(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Bool(true))
  }

  /// No variant stands for a property that was not given. A compiled style
  /// carries what the author wrote, and a property they left out is absent from
  /// the map rather than present with nothing in it.
  fn is_undefined(&self) -> bool {
    false
  }
}
