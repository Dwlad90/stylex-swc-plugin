use std::fmt;
use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::{BindingIdent, Expr, KeyValueProp, Lit};

use super::{functions::FunctionConfig, theme_ref::ThemeRef};

#[derive(Debug, Hash, PartialEq, Clone)]
pub(crate) enum ArrayJS {
    Map,
    Filter,
}

pub enum EvaluateResultValue {
    Expr(Expr),
    Vec(Vec<Option<EvaluateResultValue>>),
    Map(IndexMap<Expr, Vec<KeyValueProp>>),
    Entries(IndexMap<Lit, Lit>),
    Callback(
        Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>, // Expr,
    ),
    FunctionConfig(FunctionConfig),
    ThemeRef(ThemeRef),
}

impl Clone for EvaluateResultValue {
    fn clone(&self) -> Self {
        match self {
            Self::Expr(e) => Self::Expr(e.clone()),
            Self::Vec(v) => Self::Vec(v.clone()),
            Self::Map(m) => Self::Map(m.clone()),
            Self::Entries(e) => Self::Entries(e.clone()),
            Self::FunctionConfig(f) => Self::FunctionConfig(f.clone()),
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
            Self::ThemeRef(e) => f.debug_tuple("ThemeRef").field(e).finish(),
            Self::Callback(_) => f
                .debug_tuple("Callback")
                .field(&"Function Pointer")
                .finish(),
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
            (Self::Callback(_), Self::Callback(_)) => false,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluateResult {
    pub(crate) confident: bool,
    pub value: Option<EvaluateResultValue>,
    pub(crate) deopt: Option<Expr>,
    pub(crate) inline_styles: Option<IndexMap<String, Expr>>,
    pub(crate) fns: Option<IndexMap<String, (Vec<BindingIdent>, IndexMap<String, Expr>)>>,
}

impl EvaluateResultValue {
    pub fn as_expr(&self) -> Option<&Expr> {
        match self {
            EvaluateResultValue::Expr(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Option<EvaluateResultValue>>> {
        match self {
            EvaluateResultValue::Vec(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_map(&self) -> Option<&IndexMap<Expr, Vec<KeyValueProp>>> {
        match self {
            EvaluateResultValue::Map(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_entries(&self) -> Option<&IndexMap<Lit, Lit>> {
        match self {
            EvaluateResultValue::Entries(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_function(&self) -> Option<&FunctionConfig> {
        match self {
            EvaluateResultValue::FunctionConfig(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_callback(
        &self,
    ) -> Option<
        &Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>,
        // &Expr,
    > {
        match self {
            EvaluateResultValue::Callback(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub fn as_theme_ref(&self) -> Option<&ThemeRef> {
        match self {
            EvaluateResultValue::ThemeRef(value) => Option::Some(value),
            _ => Option::None,
        }
    }
}
