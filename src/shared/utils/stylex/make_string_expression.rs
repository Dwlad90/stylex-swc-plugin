use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::{
    shared::{
        structures::flat_compiled_styles::FlatCompiledStylesValue,
        utils::common::string_to_expression,
    },
    transform::styleq::styleq::styleq,
};

use super::parse_nallable_style::ResolvedArg;

pub(crate) fn make_string_expression(values: &Vec<ResolvedArg>) -> Option<Expr> {
    let conditions = values
        .clone()
        .into_iter()
        .filter_map(|value| match value {
            ResolvedArg::ConditionalStyle((expr, _, _, _)) => Option::Some(expr),
            _ => Option::None,
        })
        .collect::<Vec<Expr>>();

    dbg!(&conditions);

    if conditions.is_empty() {
        let result = styleq(values);

        if result.class_name.is_empty() {
            return Option::None;
        }

        return string_to_expression(result.class_name);
    }
    todo!("conditions");
}
