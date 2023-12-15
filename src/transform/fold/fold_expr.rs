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
            match &mut expr {
                Expr::Call(ex) => {
                    let declaration = self.process_declaration(&ex);

                    if declaration.is_some() {
                        let value = if self.config.runtime_injection {
                            self.transform_call_expression_to_styles_expr(&ex)
                        } else {
                            self.transform_call_expression_to_css_map_expr(&ex)
                        };

                        match value {
                            Some(value) => {
                                return value;
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        expr.fold_children_with(self)
    }
}
