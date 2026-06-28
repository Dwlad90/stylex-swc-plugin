use stylex_macros::stylex_panic;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{BinaryOp, Expr, KeyValueProp, Prop, PropName, PropOrSpread, UnaryExpr, UnaryOp},
    utils::quote_ident,
  },
};

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  utils::ast::convertors::{create_number_expr, create_string_expr},
};
use stylex_ast::ast::factories::{
  create_bin_expr, create_computed_member_prop, create_member_expr, create_object_expression,
};

use super::{js_to_ast::convert_object_to_ast, parse_nullable_style::ResolvedArg};

fn fn_result_to_expression(fn_result: FnResult) -> Option<Expr> {
  match fn_result {
    FnResult::Stylex(string_object) => Some(string_object),
    FnResult::Props(string_object) | FnResult::Attrs(string_object) => {
      Some(convert_object_to_ast(&string_object))
    },
  }
}

pub(crate) fn make_string_expression(
  values: &[ResolvedArg],
  props_like_fn: fn(&[ResolvedArg]) -> Option<FnResult>,
) -> Option<Expr> {
  let conditions = values
    .iter()
    .filter_map(|value| match value {
      ResolvedArg::ConditionalStyle(expr, _, _) => Some(expr),
      _ => None,
    })
    .collect::<Vec<_>>();

  if conditions.is_empty() {
    match props_like_fn(values) {
      Some(value) => {
        return fn_result_to_expression(value);
      },
      _ => {
        return Some(create_string_expr(""));
      },
    }
  }

  let condition_permutations = gen_condition_permutations(conditions.len());

  let obj_entries = condition_permutations
    .iter()
    .filter_map(|permutation| {
      let mut i = 0;

      let args = values
        .iter()
        .filter_map(|arg| match arg {
          ResolvedArg::StyleObject(_) => Some(arg.clone()),
          ResolvedArg::ConditionalStyle(_test, primary, fallback) => {
            let result = if permutation.get(i).unwrap_or(&false) == &true {
              primary
            } else {
              fallback
            };

            i += 1;

            result
              .as_ref()
              .map(|result| ResolvedArg::StyleObject(result.clone()))
          },
        })
        .collect::<Vec<ResolvedArg>>();

      let key = permutation
        .iter()
        .fold(0, |so_far, &b| (so_far << 1) | if b { 1 } else { 0 });

      if let Some(result) = fn_result_to_expression(match props_like_fn(&args) {
        Some(r) => r,
        None => stylex_panic!(
          "Style transformation returned no result for the given condition permutation."
        ),
      }) {
        let prop = PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
          key: PropName::Ident(quote_ident!(key.to_string())),
          value: Box::new(result),
        })));
        return Some(prop);
      }

      None
    })
    .collect::<Vec<PropOrSpread>>();

  let obj_expressions = create_object_expression(obj_entries);
  let conditions_to_key =
    gen_bitwise_or_of_conditions(&conditions.into_iter().cloned().collect::<Vec<_>>());

  Some(Expr::from(create_member_expr(
    obj_expressions,
    create_computed_member_prop(*conditions_to_key),
  )))
}

fn gen_bitwise_or_of_conditions(conditions: &[Expr]) -> Box<Expr> {
  let binary_expressions = conditions.iter().enumerate().map(|(i, condition)| {
    let shift = conditions.len() - i - 1;

    create_bin_expr(
      BinaryOp::LShift,
      Expr::from(UnaryExpr {
        span: DUMMY_SP,
        op: UnaryOp::Bang,
        arg: Box::new(Expr::from(UnaryExpr {
          span: DUMMY_SP,
          op: UnaryOp::Bang,
          arg: Box::new(condition.clone()),
        })),
      }),
      create_number_expr(shift as f64),
    )
  });

  Box::new(
    match binary_expressions.reduce(|acc, expr| create_bin_expr(BinaryOp::BitOr, acc, expr)) {
      Some(expr) => expr,
      None => stylex_panic!("Cannot generate condition mask from an empty conditions list."),
    },
  )
}

fn gen_condition_permutations(count: usize) -> Vec<Vec<bool>> {
  (0..2usize.pow(count as u32))
    .map(|i| (0..count).map(|j| i & (1 << j) != 0).collect())
    .collect()
}
