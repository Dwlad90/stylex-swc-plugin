use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{Expr, Id, MemberExpr, MemberProp, Stmt},
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        enums::{ModuleCycle, NonNullProp, NonNullProps, StyleVarsToKeep},
        utils::common::increase_ident_count,
    },
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_member_expr_impl(
        &mut self,
        mut member_expression: MemberExpr,
    ) -> MemberExpr {
        if self.cycle == ModuleCycle::Skip {
            return member_expression;
        }

        if self.cycle == ModuleCycle::TransformExit {
            let object = member_expression.obj.as_ref();
            let property = &member_expression.prop;

            let mut obj_name: Option<Id> = Option::None;
            let mut prop_name: Option<Id> = Option::None;

            match object {
                Expr::Ident(ident) => {
                    let obj_ident_name = ident.sym.to_string();

                    obj_name = Option::Some(ident.to_id());

                    if self.state.style_map.contains_key(&obj_ident_name) {
                        match property {
                            MemberProp::Ident(ident) => {
                                let prop_ident_name = ident.sym.to_string();

                                prop_name = Option::Some(ident.to_id());
                            }
                            MemberProp::Computed(computed) => {
                                dbg!(&computed);
                                // match computed.expr.as_ref() {
                                //     Expr::Lit(lit) => {
                                //         todo!("Computed not implemented yet");
                                //     }
                                //     _ => {}
                                // }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };

            if let Some(obj_name) = obj_name {
                if let Some(prop_name) = prop_name {
                    if self.state.style_map.contains_key(obj_name.0.as_str()) {
                        dbg!(&obj_name, &prop_name);

                        increase_ident_count(
                            &mut self.state,
                            object.as_ident().expect("Object not an ident"),
                        );

                        let style_var_to_keep = StyleVarsToKeep(
                            obj_name.clone(),
                            NonNullProp::Id(prop_name),
                            NonNullProps::Vec(vec![]),
                        );

                        self.state.style_vars_to_keep.insert(style_var_to_keep);
                    }
                }
            }

            return member_expression;
        }

        member_expression.fold_children_with(self)
    }
}
