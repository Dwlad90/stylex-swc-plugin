use std::{collections::HashMap, rc::Rc};

use swc_core::ecma::ast::{ArrayLit, ArrowExpr, Expr, Id};

use crate::shared::utils::js::enums::{ArrayJS, ObjectJS};

use super::named_import_source::ImportSources;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum CallbackType {
    Array(ArrayJS),
    Object(ObjectJS),
}

pub enum FunctionType {
    ArrayArgs(fn(Vec<Expr>) -> Expr),
    OneArg(fn(Expr) -> Expr),
    Mapper(Rc<dyn Fn() -> Expr + 'static>),
    // Callback(CallbackType, Expr),
    Callback(CallbackType),
}

impl Clone for FunctionType {
    fn clone(&self) -> Self {
        match self {
            Self::ArrayArgs(e) => Self::ArrayArgs(e.clone()),
            Self::OneArg(e) => Self::OneArg(e.clone()),
            Self::Callback(v) => Self::Callback(v.clone()),
            Self::Mapper(c) => Self::Mapper(Rc::clone(c)),
        }
    }
}

impl std::fmt::Debug for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionType::ArrayArgs(_) => write!(f, "ArrayArgs"),
            FunctionType::OneArg(_) => write!(f, "OneArg"),
            FunctionType::Mapper(_) => write!(f, "Mapper"),
            FunctionType::Callback(_) => write!(f, "Callback"),
        }
    }
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FunctionType::ArrayArgs(_), FunctionType::ArrayArgs(_)) => true,
            (FunctionType::OneArg(_), FunctionType::OneArg(_)) => true,
            (FunctionType::Mapper(_), FunctionType::OneArg(_)) => true,
            (FunctionType::Callback(_), FunctionType::Callback(_)) => true,
            _ => false,
        }
    }
}

impl Eq for FunctionType {}

impl std::hash::Hash for FunctionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            FunctionType::ArrayArgs(_) => {
                std::mem::discriminant(self).hash(state);
            }
            FunctionType::OneArg(_) => {
                std::mem::discriminant(self).hash(state);
            }
            FunctionType::Mapper(_) => {
                std::mem::discriminant(self).hash(state);
            }
            FunctionType::Callback(_) => {
                std::mem::discriminant(self).hash(state);
            }
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct FunctionConfig {
    pub fn_ptr: FunctionType,
    pub takes_path: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Functions {
    pub(crate) include: FunctionConfig,
    pub(crate) first_that_works: FunctionConfig,
    pub(crate) keyframes: FunctionConfig,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionMap {
    pub identifiers: HashMap<Id, FunctionConfig>,
    pub member_expressions: HashMap<ImportSources, HashMap<Id, FunctionConfig>>,
}

impl Default for FunctionMap {
    fn default() -> Self {
        Self {
            identifiers: HashMap::new(),
            member_expressions: HashMap::new(),
        }
    }
}
