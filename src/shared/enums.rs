use swc_core::ecma::ast::{Expr, Id};

use super::utils::stylex::js_to_expr::NestedStringObject;

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
pub(crate) struct TopLevelExpression(pub(crate) TopLevelExpressionKind, pub(crate) Expr);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) enum ConditionPermutationsValue {
    Single(String),
    Triple((String, String, String)),
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
