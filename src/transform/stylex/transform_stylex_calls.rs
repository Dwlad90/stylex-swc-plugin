use swc_core::ecma::ast::Ident;
use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr, MemberProp},
};

use crate::shared::enums::ModuleCycle;
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_call_expression_to_stylex_expr(
        &mut self,
        ex: &CallExpr,
    ) -> Option<Expr> {
        if let Callee::Expr(callee) = &ex.callee {
            match callee.as_ref() {
                Expr::Member(member) => match member.prop.clone() {
                    MemberProp::Ident(ident) => {
                        return self.transform_stylex_fns(ident, ex);
                    }
                    _ => {}
                },
                Expr::Ident(ident) => return self.transform_stylex_fns(ident.clone(), ex),
                _ => {}
            }
        }

        Option::None
    }

    fn transform_stylex_fns(&mut self, ident: Ident, call_expr: &CallExpr) -> Option<Expr> {
        if self.cycle == ModuleCycle::TransformEnter {
            let (_, parent_var_decl) = &self.get_call_var_name(call_expr);

            if let Some(parent_var_decl) = parent_var_decl {
                if let Some(value) = self.transform_stylex_keyframes_call(parent_var_decl) {
                    return Option::Some(value);
                }
            }

            if let Some(value) = self.transform_stylex_define_vars(call_expr) {
                return Option::Some(value);
            }

            if let Some(value) = self.transform_stylex_create(call_expr) {
                return Option::Some(value);
            }

            if let Some(value) = self.transform_stylex_create(call_expr) {
                return Option::Some(value);
            }
        }

        if self.cycle == ModuleCycle::TransformExit {
            dbg!(&self.state.stylex_props_import);

            if self.state.stylex_props_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_stylex_props_call(call_expr) {
                    return Option::Some(value);
                }
            }

            if self.state.stylex_attrs_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_stylex_attrs_call(call_expr) {
                    return Option::Some(value);
                }
            }

            if let Some(value) = self.transform_stylex_call(call_expr) {
                return Option::Some(value);
            }

            if let Some(value) = self.transform_stylex_attrs_call(call_expr) {
                return Option::Some(value);
            }

            if let Some(value) = self.transform_stylex_props_call(call_expr) {
                return Option::Some(value);
            }
        }

        // Option::Some(Expr::Call(ex.clone()))
        Option::None
    }
}
