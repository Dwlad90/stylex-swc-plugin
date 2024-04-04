use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Id, ObjectLit};

use super::{
    structures::{included_style::IncludedStyle, injectable_style::InjectableStyle, pair::Pair},
    utils::stylex::js_to_expr::NestedStringObject,
};

// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub(crate) enum ModuleCycle {
    // The plugin is being processed.
    TransformEnter,
    TransformExit,
    // The plugin has been processed and the file is being cleaned.
    Cleaning,
    // The file has been processed and the plugin is skipped.
    Initializing,
    Skip,
    InjectStyles,
    InjectClassName,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InjectedStylesDeclarationType {
    NamedDeclarationExport,
    NamedPropertyOrDefaultExport,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProp {
    Id(Id),
    True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum NonNullProps {
    Vec(Vec<Id>),
    True,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct StyleVarsToKeep(
    pub(crate) Id,
    pub(crate) NonNullProp,
    pub(crate) NonNullProps,
);

pub(crate) enum VarDeclAction {
    Increase,
    Reduce,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum TopLevelExpressionKind {
    NamedExport,
    DefaultExport,
    Stmt,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct TopLevelExpression(
    pub(crate) TopLevelExpressionKind,
    pub(crate) Expr,
    pub(crate) Option<Id>,
);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum ConditionPermutationsValue {
    Single(String),
    Triple((String, String, String)),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FnResult {
    Attrs(NestedStringObject),
    Props(NestedStringObject),
    Stylex(Expr),
}

impl FnResult {
    pub(crate) fn as_props(&self) -> Option<&NestedStringObject> {
        match self {
            FnResult::Props(props) => Some(props),
            _ => None,
        }
    }

    pub(crate) fn as_stylex(&self) -> Option<&Expr> {
        match self {
            FnResult::Stylex(expr) => Some(expr),
            _ => None,
        }
    }

    pub(crate) fn as_attrs(&self) -> Option<&NestedStringObject> {
        match self {
            FnResult::Attrs(attrs) => Some(attrs),
            _ => None,
        }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub(crate) enum FlatCompiledStylesValueNested {
//     String(String),
//     Object(IndexMap<String, String>),
// }

#[derive(Debug, PartialEq, Clone, Hash)]
pub(crate) enum FlatCompiledStylesValue {
    String(String),
    KeyValue(Pair),
    Null,
    IncludedStyle(IncludedStyle),
    InjectableStyle(InjectableStyle),
    Bool(bool),
    Tuple(String, Box<Expr>),
}

impl FlatCompiledStylesValue {
    pub(crate) fn as_tuple(&self) -> Option<(&String, &Box<Expr>)> {
        match self {
            FlatCompiledStylesValue::Tuple(key, value) => Some((key, value)),
            _ => None,
        }
    }

    pub(crate) fn as_string(&self) -> Option<&String> {
        match self {
            FlatCompiledStylesValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_injectable_style(&self) -> Option<&InjectableStyle> {
        match self {
            FlatCompiledStylesValue::InjectableStyle(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_bool(&self) -> Option<&bool> {
        match self {
            FlatCompiledStylesValue::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_null(&self) -> Option<()> {
        match self {
            FlatCompiledStylesValue::Null => Some(()),
            _ => None,
        }
    }

    pub(crate) fn as_included_style(&self) -> Option<&IncludedStyle> {
        match self {
            FlatCompiledStylesValue::IncludedStyle(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_key_value(&self) -> Option<&Pair> {
        match self {
            FlatCompiledStylesValue::KeyValue(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ObjMapType {
    Object(ObjectLit),
    Map(IndexMap<String, FlatCompiledStylesValue>),
}
