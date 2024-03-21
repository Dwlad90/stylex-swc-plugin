use std::collections::HashMap;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{BinExpr, BinaryOp, CallExpr, Callee, Expr, MemberExpr},
        visit::{noop_fold_type, Fold, FoldWith},
    },
};

use crate::{
    shared::{
        enums::NonNullProps,
        structures::{
            functions::FunctionMap, named_import_source::ImportSources, state_manager::StateManager,
        },
        utils::{
            common::reduce_ident_count,
            stylex::{
                make_string_expression::make_string_expression,
                member_expression::member_expression,
                parse_nallable_style::{parse_nullable_style, ResolvedArg, StyleObject},
            },
        },
    },
    ModuleTransformVisitor,
};

#[derive(Clone, Debug)]
struct MemberTransform {
    pub(crate) index: i32,
    pub(crate) bail_out_index: Option<i32>,
    pub(crate) non_null_props: NonNullProps,
    pub(crate) state: StateManager,
}

impl Fold for MemberTransform {
    noop_fold_type!();

    fn fold_member_expr(&mut self, member: MemberExpr) -> MemberExpr {
        member_expression(
            &member,
            &mut self.index,
            &mut self.bail_out_index,
            &mut self.non_null_props,
            &mut self.state,
            &FunctionMap {
                identifiers: HashMap::new(),
                member_expressions: HashMap::new(),
            },
        );

        member
    }
}

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
                        let mut conditional = 0;

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
                                    dbg!(&member, &resolved);
                                    match resolved {
                                        StyleObject::Other => {
                                            dbg!(&arg);
                                            bail_out_index = Option::Some(current_index);
                                            bail_out = true;
                                            todo!("StyleObject::Other case");
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
                                Expr::Bin(BinExpr {
                                    left, op, right, ..
                                }) => {
                                    if !op.eq(&BinaryOp::LogicalAnd) {
                                        bail_out_index = Some(current_index);
                                        bail_out = true;
                                        break;
                                    }

                                    let left_resolved = parse_nullable_style(&left, &self.state);
                                    let right_resolved = parse_nullable_style(&right, &self.state);
                                    dbg!(&left, &right_resolved);

                                      if !left_resolved.eq(&StyleObject::Other)
                                        || right_resolved.eq(&StyleObject::Other)
                                    {
                                        bail_out_index = Some(current_index);
                                        bail_out = true;
                                    } else {
                                        let ident = match right.as_ref() {
                                            Expr::Ident(ident) => ident.clone(),
                                            Expr::Member(meber) => meber
                                                .obj
                                                .as_ident()
                                                .expect("Member obj is not an ident")
                                                .clone(),
                                            _ => panic!("Illegal argument"),
                                        };

                                        resolved_args.push(ResolvedArg::ConditionalStyle(
                                            left,
                                            Some(right_resolved),
                                            None,
                                            ident,
                                        ));

                                        conditional = conditional + 1;
                                    }
                                }

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

                        dbg!(&resolved_args, &conditional, &bail_out_index, &bail_out);

                        dbg!(&self.state.gen_conditional_classes());

                        if !self.state.gen_conditional_classes() && conditional > 0 {
                            bail_out = true;
                        }

                        if bail_out {
                            let arguments_path = call.args.clone();

                            let mut non_null_props: NonNullProps = NonNullProps::Vec(vec![]);

                            let mut index = -1;

                            for mut arg_path in arguments_path.into_iter() {
                                index = index + 1;

                                assert!(arg_path.spread.is_none(), "Spread not implemented yet");

                                // let mut arg = arg_path.expr.as_ref();

                                let mut member_transfom = MemberTransform {
                                    index: index.clone(),
                                    bail_out_index: bail_out_index.clone(),
                                    non_null_props: non_null_props.clone(),
                                    state: self.state.clone(),
                                };

                                arg_path.expr =
                                    arg_path.expr.clone().fold_with(&mut member_transfom);

                                index = member_transfom.index;
                                bail_out_index = member_transfom.bail_out_index;
                                non_null_props = member_transfom.non_null_props;
                                self.state = member_transfom.state;

                                // match arg {
                                //     Expr::Member(member) => {
                                //         dbg!(&member);

                                //         member_expression(
                                //             member,
                                //             &mut index,
                                //             &mut bail_out_index,
                                //             &mut non_null_props,
                                //             &mut self.state,
                                //             &FunctionMap {
                                //                 identifiers: HashMap::new(),
                                //                 member_expressions: HashMap::new(),
                                //             },
                                //         )
                                //     }
                                //     Expr::Ident(_) => {}
                                //     _ => {
                                //         dbg!(&arg);
                                //         panic!("Illegal argument");
                                //         arg.clone().fold_with(self);
                                //     }
                                // }
                            }
                        } else {
                            if resolved_args.is_empty() {
                                return Option::None;
                            }

                            let string_expression = make_string_expression(&resolved_args);
                            reduce_ident_count(&mut self.state, &ident);

                            for arg in &resolved_args {
                                match arg {
                                    ResolvedArg::StyleObject(_, ident) => {
                                        dbg!(&ident);
                                        reduce_ident_count(&mut self.state, &ident);
                                    }
                                    ResolvedArg::ConditionalStyle(expr, _, _, ident) => {
                                        dbg!(&ident);
                                        reduce_ident_count(&mut self.state, &ident);
                                    }
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
