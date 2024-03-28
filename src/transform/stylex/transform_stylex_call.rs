use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr},
};

use crate::{
    shared::{
        structures::named_import_source::ImportSources,
        utils::stylex::{stylex::stylex, stylex_merge::stylex_merge},
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_call(&mut self, call: &CallExpr) -> Option<Expr> {
        match &call.callee {
            Callee::Expr(expr) => match expr.as_ref() {
                Expr::Ident(ident) => {
                    if self
                        .state
                        .stylex_import
                        .contains(&ImportSources::Regular(ident.sym.to_string()))
                    {
                        if let Some(value) = stylex_merge(call, stylex, &mut self.state) {
                            return Some(value);
                        }
                    }
                    Option::None
                }
                _ => Option::None,
            },
            _ => Option::None,
        }
    }
}
