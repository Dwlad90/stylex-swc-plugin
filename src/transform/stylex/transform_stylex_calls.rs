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
                        if let Some(value) = self.transform_stylex_fns(ident, ex) {
                            return value;
                        }
                    }
                    _ => {}
                },
                Expr::Ident(ident) => {
                    if let Some(value) = self.transform_stylex_fns(ident.clone(), ex) {
                        return value;
                    }
                }
                _ => {}
            }
        }

        Option::None
    }

    fn transform_stylex_fns(&mut self, ident: Ident, ex: &CallExpr) -> Option<Option<Expr>> {
        if self.cycle == ModuleCycle::TransformEnter {
            if self.state.stylex_create_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_stylex_create(ex) {
                    return Some(Option::Some(value));
                }
            }

            if self
                .state
                .stylex_define_vars_import
                .contains(&ident.to_id())
            {
                if let Some(value) = self.transform_stylex_define_vars(ex) {
                    return Some(Option::Some(value));
                }
            }

            if let Some(value) = self.transform_stylex_create(ex) {
                return Some(Option::Some(value));
            }

            if let Some(value) = self.transform_stylex_define_vars(ex) {
                return Some(Option::Some(value));
            }
        }

        if self.cycle == ModuleCycle::TransformExit {
            dbg!(&self.state.stylex_props_import);

            if self.state.stylex_props_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_stylex_props_call(ex) {
                    return Some(Option::Some(value));
                }
            }

            if self.state.stylex_attrs_import.contains(&ident.to_id()) {
                if let Some(value) = self.transform_stylex_attrs_call(ex) {
                    return Some(Option::Some(value));
                }
            }

            if let Some(value) = self.transform_stylex_call(ex) {
                return Some(Option::Some(value));
            }

            if let Some(value) = self.transform_stylex_attrs_call(ex) {
                return Some(Option::Some(value));
            }

            if let Some(value) = self.transform_stylex_props_call(ex) {
                return Some(Option::Some(value));
            }
        }

        None
    }
}
