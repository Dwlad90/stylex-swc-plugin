use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedImportSource {
  pub r#as: String,
  pub from: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImportSources {
  Regular(String),
  Named(NamedImportSource),
}

impl ImportSources {
  pub fn is_named_export(&self) -> bool {
    match self {
      ImportSources::Regular(_) => false,
      ImportSources::Named(_named) => true,
    }
  }

  pub fn get_import_str(&self) -> &str {
    match self {
      ImportSources::Regular(regular) => regular,
      ImportSources::Named(named) => named.r#as.as_str(),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum RuntimeInjection {
  Boolean(bool),
  Regular(String),
  Named(NamedImportSource),
}

impl RuntimeInjection {
  pub(crate) fn _is_named_export(&self) -> bool {
    match self {
      RuntimeInjection::Boolean(_) => false,
      RuntimeInjection::Regular(_) => false,
      RuntimeInjection::Named(_named) => true,
    }
  }

  pub(crate) fn _is_regular_export(&self) -> bool {
    match self {
      RuntimeInjection::Boolean(_) => false,
      RuntimeInjection::Regular(_) => true,
      RuntimeInjection::Named(_) => false,
    }
  }

  pub(crate) fn _is_boolean_export(&self) -> bool {
    match self {
      RuntimeInjection::Boolean(_) => true,
      RuntimeInjection::Regular(_) => false,
      RuntimeInjection::Named(_) => false,
    }
  }

  pub(crate) fn _as_boolean(&self) -> Option<&bool> {
    match self {
      RuntimeInjection::Boolean(value) => Some(value),
      RuntimeInjection::Regular(_) => None,
      RuntimeInjection::Named(_named) => None,
    }
  }
  pub(crate) fn _as_regular(&self) -> Option<&String> {
    match self {
      RuntimeInjection::Boolean(_) => None,
      RuntimeInjection::Regular(value) => Some(value),
      RuntimeInjection::Named(_) => None,
    }
  }
  pub(crate) fn _as_named(&self) -> Option<&NamedImportSource> {
    match self {
      RuntimeInjection::Boolean(_) => None,
      RuntimeInjection::Regular(_) => None,
      RuntimeInjection::Named(value) => Some(value),
    }
  }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RuntimeInjectionState {
  Boolean(bool),
  Regular(String),
  Named(NamedImportSource),
}
