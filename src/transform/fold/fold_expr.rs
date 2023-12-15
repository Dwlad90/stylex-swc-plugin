use swc_core::{
    common::comments::Comments,
    ecma::{ast::Expr, visit::FoldWith},
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_expr_impl(&mut self, mut expr: Expr) -> Expr {
        if self.cycle == ModuleCycle::Skip {
            return expr;
        }

        if self.cycle == ModuleCycle::Processing {
            if let Some(value) = self.transform_call_expression(&mut expr) {
                return value;
            }
        }

        expr.fold_children_with(self)
    }
}
