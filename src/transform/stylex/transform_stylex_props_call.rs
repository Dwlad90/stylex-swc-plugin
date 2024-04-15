use std::collections::HashMap;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{CallExpr, Expr, MemberExpr},
        visit::{noop_fold_type, AstNodePath, Fold, FoldWith, Visit},
    },
};

use crate::{
    shared::{
        enums::NonNullProps,
        structures::{functions::FunctionMap, state_manager::StateManager},
        utils::{
            stylex::{
                member_expression::member_expression, props::props, stylex_merge::stylex_merge,
            },
            validators::is_props_call,
        },
    },
    ModuleTransformVisitor,
};

// #[derive(Clone, Debug)]
// struct MemberTransform {
//     pub(crate) index: i32,
//     pub(crate) bail_out_index: Option<i32>,
//     pub(crate) non_null_props: NonNullProps,
//     pub(crate) state: StateManager,
//     parents: Vec<Expr>,
// }

// impl Fold for MemberTransform {
//     noop_fold_type!();

//     fn fold_expr(&mut self, expr: Expr) -> Expr {
//         self.parents.push(expr.clone());
// panic!("MemberTransform::fold_expr");
//         expr.fold_children_with(self)
//     }

//     fn fold_member_expr(&mut self, member: MemberExpr) -> MemberExpr {
//         member_expression(
//             &member,
//             &mut self.index,
//             &mut self.bail_out_index,
//             &mut self.non_null_props,
//             &mut self.state,
//             &FunctionMap {
//                 identifiers: HashMap::new(),
//                 member_expressions: HashMap::new(),
//             },
//             &self.parents,
//         );

//         member
//     }
// }

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn transform_stylex_props_call(&mut self, call: &CallExpr) -> Option<Expr> {
        let is_props_call = is_props_call(call, &mut self.state);

        if is_props_call {
            return stylex_merge(call, props, &mut self.state);
        }

        Option::None
    }
}
