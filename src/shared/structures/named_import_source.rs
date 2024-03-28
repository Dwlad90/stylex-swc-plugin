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
}

#[derive(Deserialize, Clone, Debug)]
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
            RuntimeInjection::Boolean(value) => Option::Some(value),
            RuntimeInjection::Regular(_) => Option::None,
            RuntimeInjection::Named(_named) => Option::None,
        }
    }
    pub(crate) fn _as_regular(&self) -> Option<&String> {
        match self {
            RuntimeInjection::Boolean(_) => Option::None,
            RuntimeInjection::Regular(value) => Option::Some(value),
            RuntimeInjection::Named(_) => Option::None,
        }
    }
    pub(crate) fn _as_named(&self) -> Option<&NamedImportSource> {
        match self {
            RuntimeInjection::Boolean(_) => Option::None,
            RuntimeInjection::Regular(_) => Option::None,
            RuntimeInjection::Named(value) => Option::Some(value),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum RuntimeInjectionState {
    Regular(String),
    Named(NamedImportSource),
}
