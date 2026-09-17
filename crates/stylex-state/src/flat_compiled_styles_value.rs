use std::hash::{Hash, Hasher};
use std::mem::discriminant;

use rustc_hash::FxHasher;

use std::rc::Rc;

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
/// a `null`, an absence, a list, or an object of the same kinds again.
///
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
  /// A list an inline declaration holds, such as the fallbacks an author gives
  /// one property.
  List(Vec<Rc<FlatCompiledStylesValue>>),
  Null,
  /// A declaration the author wrote with no value. The merge drops it whole,
  /// so it declares nothing and leaves the property for a later style.
  Undefined,
  InjectableStyle(InjectableStyle),
  Bool(bool),
  Tuple(String, Box<Expr>, Option<BaseCSSType>),
}

/// Written out rather than derived, because two of the kinds hold something
/// the derived spelling reads wrongly or not at all.
///
/// A number has no `Hash` of its own, so its bits stand for it, and the two
/// zeroes are folded onto one pattern first: `-0` and `0` are one value to
/// equality, so they have to be one hash as well. `NaN` is the one value left
/// where the two disagree, and it disagrees the other way -- two `NaN`s are
/// never equal and always hash alike. The language gives no way to close that,
/// because one `NaN` cannot be told from another.
///
/// An object holds an `IndexMap`, which compares without regard to the order
/// its names were written in. The hash is combined without regard to it too,
/// so two objects that hold the same names and values stay one value. The
/// count goes in as well, so an object holding an empty object beside a `null`
/// does not write the bytes of one holding an object that holds the `null`. A
/// list is the other way round, because order is part of what a list is.
///
/// Nothing hashes an inline value today. The one caller is the merge cache in
/// `stylex-styleq`, which hashes a compiled namespace and never an inline
/// style, and this compiler runs that cache off. The rule is written here all
/// the same: nothing compares the value a hit came from, so a hash that
/// disagrees with equality answers with another style's class names.
impl Hash for FlatCompiledStylesValue {
  fn hash<H: Hasher>(&self, state: &mut H) {
    discriminant(self).hash(state);

    match self {
      FlatCompiledStylesValue::String(value) => value.hash(state),
      FlatCompiledStylesValue::Number(value) => zero_without_a_sign(*value).to_bits().hash(state),
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
      // A list keeps the order it was written in, so it hashes in that order.
      FlatCompiledStylesValue::List(elements) => {
        elements.len().hash(state);

        for element in elements {
          element.hash(state);
        }
      },
      FlatCompiledStylesValue::Null | FlatCompiledStylesValue::Undefined => {},
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

/// The same number, with a negative zero folded onto the plain one.
///
/// The two are one value to equality and two bit patterns to a hash, so the
/// fold is what holds the two answers together.
fn zero_without_a_sign(value: f64) -> f64 {
  match value == 0.0 {
    true => 0.0,
    false => value,
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

  /// Kept beside the readers of the other kinds, so the set of them is one
  /// list rather than a list with two names missing from it. The compiler asks
  /// `styleq` about these two instead, and only the tests ask here.
  #[allow(dead_code)]
  pub(crate) fn as_bool(&self) -> Option<&bool> {
    match self {
      FlatCompiledStylesValue::Bool(value) => Some(value),
      _ => None,
    }
  }

  /// Kept for the same reason [`FlatCompiledStylesValue::as_bool`] is.
  #[allow(dead_code)]
  pub(crate) fn as_null(&self) -> Option<()> {
    match self {
      FlatCompiledStylesValue::Null => Some(()),
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

  /// An inline style is the object the author wrote, so a declaration they
  /// gave no value stands in it. A compiled style has no such value: a
  /// property the author left out is absent from the map rather than present
  /// holding nothing.
  fn is_undefined(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Undefined)
  }
}
