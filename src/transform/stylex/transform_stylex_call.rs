use swc_core::{
    common::comments::Comments,
    ecma::ast::{CallExpr, Callee, Expr},
};

use crate::{
    shared::{
        structures::named_import_source::ImportSources,
        utils::{
            common::reduce_ident_count,
            stylex::{
                make_string_expression::make_string_expression,
                parse_nallable_style::{parse_nullable_style, ResolvedArg, StyleObject},
            },
        },
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
                        let mut bail_out = false;
                        let conditional = 0;

                        let mut current_index = -1;

                        let mut bail_out_index: Option<i32> = Option::None;

                        let mut resolved_args: Vec<ResolvedArg> = vec![];

                        for arg in call.args.iter() {
                            current_index = current_index + 1;

                            assert!(arg.spread.is_none(), "Spread not implemented yet");

                            let arg = arg.expr.as_ref();

                            match arg.clone() {
                                Expr::Member(member) => {
                                    let resolved = parse_nullable_style(arg, &self.state);
                                    match resolved {
                                        StyleObject::Other => {
                                            bail_out_index = Option::Some(current_index);
                                            bail_out = true;
                                        }
                                        StyleObject::Style(_) => {
                                            resolved_args.push(ResolvedArg::StyleObject(
                                                resolved,
                                                member
                                                    .obj
                                                    .as_ident()
                                                    .expect("Member obj is not an ident")
                                                    .clone(),
                                            ));
                                        }
                                        StyleObject::Nullable => {
                                            resolved_args.push(ResolvedArg::StyleObject(
                                                resolved,
                                                member
                                                    .obj
                                                    .as_ident()
                                                    .expect("Member obj is not an ident")
                                                    .clone(),
                                            ));
                                        }
                                    }
                                }
                                Expr::Cond(_) => todo!("Condition"),
                                Expr::Bin(_) => todo!("Logical"),
                                _ => {
                                    bail_out_index = Option::Some(current_index);
                                    bail_out = true;
                                }
                            }

                            if conditional > 4 {
                                bail_out = true;
                            }

                            if bail_out {
                                break;
                            }
                        }

                        if !self.state.options.gen_conditional_classes && conditional > 0 {
                            bail_out = true;
                        }

                        if bail_out {
                            let arguments_path = call.args.clone();

                            let non_null_props: Vec<String> = vec![];

                            let mut index = -1;

                            for arg_path in arguments_path.into_iter() {
                                index = index + 1;

                                assert!(arg_path.spread.is_none(), "Spread not implemented yet");

                                let arg = arg_path.expr.as_ref();

                                if let Some(member) = arg.as_member() {
                                    todo!("Member not implemented yet");
                                } else {
                                    todo!("Not member not implemented yet");
                                }
                            }
                        } else {
                            dbg!(&call.args);
                            let string_expression = make_string_expression(&resolved_args);

                            for arg in &resolved_args {
                                match arg {
                                    ResolvedArg::StyleObject(_, ident) => {
                                        reduce_ident_count(&mut self.var_decl_count_map, &ident);
                                    }
                                    ResolvedArg::ConditionalStyle(_) => todo!("ConditionalStyle"),
                                }
                            }

                            return string_expression;
                        }

                        dbg!(ident);
                    }
                    Option::None
                }
                _ => Option::None,
            },
            _ => Option::None,
        }
    }
}
