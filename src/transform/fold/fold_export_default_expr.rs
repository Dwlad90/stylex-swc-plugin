use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{ExportDefaultExpr, Expr},
        visit::FoldWith,
    },
};

use crate::{shared::enums::ModuleCycle, ModuleTransformVisitor};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_export_default_expr_impl(
        &mut self,
        mut export_default_expr: ExportDefaultExpr,
    ) -> ExportDefaultExpr {
        if self.cycle == ModuleCycle::Skip {
            return export_default_expr;
        }

        if self.cycle == ModuleCycle::Processing {
            match &mut export_default_expr.expr.as_mut() {
                Expr::Paren(paren) => {
                    if let Some(value) = self.transform_call_expression(&mut paren.expr) {
                        *export_default_expr.expr = value;
                    }
                }
                _ => {}
            }
        }

        export_default_expr.fold_children_with(self)
    }
}
