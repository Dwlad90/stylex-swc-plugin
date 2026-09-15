//! Style arguments the merge cases below are built from.
//!
//! `styleq`, `props` and `attrs` are one chain read by three sets of names, so
//! they are given their inputs by one set of builders.

use std::rc::Rc;

use indexmap::IndexMap;
use stylex_constants::constants::common::COMPILED_KEY;
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles,
};
use stylex_structures::pair::Pair;

use crate::shared::utils::core::parse_nullable_style::{ResolvedArg, StyleObject};

/// A compiled namespace: the marker every compiled style carries, and the
/// properties given as `(property, class name)`.
pub(crate) fn compiled(properties: &[(&str, &str)]) -> FlatCompiledStyles {
  let mut styles = inline(properties);

  styles.shift_insert(
    0,
    COMPILED_KEY.to_owned(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  styles
}

/// A style the compiler did not compile: the same properties, carrying no
/// marker, so the merge writes it out as an inline style rather than as
/// classes.
pub(crate) fn inline(properties: &[(&str, &str)]) -> FlatCompiledStyles {
  let mut styles: FlatCompiledStyles = IndexMap::new();

  for (property, value) in properties {
    styles.insert(
      (*property).to_owned(),
      Rc::new(FlatCompiledStylesValue::String((*value).to_owned())),
    );
  }

  styles
}

/// An inline style holding a value that spells no CSS text.
pub(crate) fn inline_pair(property: &str, value: &str) -> FlatCompiledStyles {
  let mut styles: FlatCompiledStyles = IndexMap::new();

  styles.insert(
    property.to_owned(),
    Rc::new(FlatCompiledStylesValue::KeyValue(Pair::new(
      property.to_owned(),
      value.to_owned(),
    ))),
  );

  styles
}

/// One style argument holding the given namespace.
pub(crate) fn style_of(properties: &[(&str, &str)]) -> ResolvedArg {
  ResolvedArg::style_object(StyleObject::Style(compiled(properties)))
}

/// One style argument holding the given namespace as it is.
pub(crate) fn styles(styles: FlatCompiledStyles) -> ResolvedArg {
  ResolvedArg::style_object(StyleObject::Style(styles))
}
