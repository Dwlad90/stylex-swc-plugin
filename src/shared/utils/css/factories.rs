use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{Expr, ObjectLit, PropOrSpread},
};

pub(crate) fn object_expression_factory(props: Vec<PropOrSpread>) -> Option<Expr> {
    Some(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
}
