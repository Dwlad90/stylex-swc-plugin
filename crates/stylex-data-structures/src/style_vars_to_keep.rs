use swc_core::atoms::Atom;

use stylex_enums::style_vars_to_keep::{NonNullProp, NonNullProps};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StyleVarsToKeep(
  pub Atom,
  pub NonNullProp,
  pub NonNullProps,
);
