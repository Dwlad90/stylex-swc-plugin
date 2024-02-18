use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, KeyValueProp};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum EvaluateResultValue {
    Expr(Expr),
    Map(IndexMap<Expr, Vec<KeyValueProp>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct EvaluateResult {
    pub(crate) confident: bool,
    pub(crate) value: Option<EvaluateResultValue>,
    pub(crate) deopt: Option<Expr>,
    // fns: Option<HashMap<String, Vec<(Vec<String>, HashMap<String, ExpressionOrPatternLike>)>>>,
}

impl EvaluateResultValue {
    pub(crate) fn as_expr(&self) -> Option<&Expr> {
        match self {
            EvaluateResultValue::Expr(value) => Option::Some(value),
            _ => Option::None,
        }
    }

    pub(crate) fn as_map(&self) -> Option<&IndexMap<Expr, Vec<KeyValueProp>>> {
        match self {
            EvaluateResultValue::Map(value) => Option::Some(value),
            _ => Option::None,
        }
    }
}
