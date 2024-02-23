use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, KeyValueProp};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EvaluateResultValue {
    Expr(Expr),
    Vec(Vec<Option<EvaluateResultValue>>),
    Map(IndexMap<Expr, Vec<KeyValueProp>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EvaluateResult {
    pub(crate) confident: bool,
    pub value: Option<EvaluateResultValue>,
    pub(crate) deopt: Option<Expr>,
    // fns: Option<HashMap<String, Vec<(Vec<String>, HashMap<String, ExpressionOrPatternLike>)>>>,
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
}
