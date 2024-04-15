use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, Expr, ExprOrSpread, ObjectLit, PropOrSpread},
};

pub fn object_expression_factory(props: Vec<PropOrSpread>) -> Option<Expr> {
    Some(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
}

pub(crate) fn array_expression_factory(elems: Vec<Option<ExprOrSpread>>) -> Option<Expr> {
    Some(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    }))
}
