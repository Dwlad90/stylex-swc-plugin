use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      BinExpr, BinaryOp, ComputedPropName, Expr, KeyValueProp, MemberExpr, MemberProp, Prop,
      PropName, PropOrSpread, UnaryExpr, UnaryOp,
    },
    utils::quote_ident,
  },
};

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  utils::ast::{
    convertors::{number_to_expression, string_to_expression},
    factories::object_expression_factory,
  },
};

use super::{js_to_expr::convert_object_to_ast, parse_nullable_style::ResolvedArg};

fn fn_result_to_expression(fn_result: FnResult) -> Option<Expr> {
  match fn_result {
    FnResult::Stylex(string_object) => Some(string_object),
    FnResult::Props(string_object) | FnResult::Attrs(string_object) => {
      Some(convert_object_to_ast(&string_object))
    }
  }
}

pub(crate) fn make_string_expression(
  values: &[ResolvedArg],
  transform: fn(&[ResolvedArg]) -> Option<FnResult>,
) -> Option<Expr> {
  let conditions = values
    .iter()
    .filter_map(|value| match value {
      ResolvedArg::ConditionalStyle(expr, _, _, _, _) => Some(expr),
      _ => None,
    })
    .collect::<Vec<_>>();

  if conditions.is_empty() {
    if let Some(value) = transform(values) {
      return fn_result_to_expression(value);
    } else {
      return Some(string_to_expression(""));
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
          ResolvedArg::StyleObject(_, _, _) => Some(arg.clone()),
          ResolvedArg::ConditionalStyle(_test, primary, fallback, ident, member) => {
            let result = if permutation.get(i).unwrap_or(&false) == &true {
              primary
            } else {
              fallback
            };

            i += 1;

            result
              .as_ref()
              .map(|result| ResolvedArg::StyleObject(result.clone(), ident.clone(), member.clone()))
          }
        })
        .collect::<Vec<ResolvedArg>>();

      let key = permutation
        .iter()
        .fold(0, |so_far, &b| (so_far << 1) | if b { 1 } else { 0 });

      if let Some(result) = fn_result_to_expression(transform(&args).unwrap()) {
        let prop = PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
          key: PropName::Ident(quote_ident!(key.to_string())),
          value: Box::new(result),
        })));

        return Some(prop);
      }

      None
    })
    .collect::<Vec<PropOrSpread>>();

  let obj_expressions = object_expression_factory(obj_entries);
  let conditions_to_key = gen_bitwise_or_of_conditions(
    &conditions
      .into_iter()
      .map(|c| (**c).clone())
      .collect::<Vec<_>>(),
  );

  Some(Expr::from(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(obj_expressions),
    prop: MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: conditions_to_key,
    }),
  }))
}

fn gen_bitwise_or_of_conditions(conditions: &[Expr]) -> Box<Expr> {
  let binary_expressions = conditions
    .iter()
    .enumerate()
    .map(|(i, condition)| {
      let shift = conditions.len() - i - 1;

      Expr::from(BinExpr {
        left: Box::new(Expr::from(UnaryExpr {
          span: DUMMY_SP,
          op: UnaryOp::Bang,
          arg: Box::new(Expr::from(UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::Bang,
            arg: Box::new(condition.clone()),
          })),
        })),
        op: BinaryOp::LShift,
        right: Box::new(number_to_expression(shift as f64)),
        span: DUMMY_SP,
      })
    })
    .collect::<Vec<Expr>>();

  Box::new(
    binary_expressions
      .into_iter()
      .reduce(|acc, expr| {
        Expr::from(BinExpr {
          span: DUMMY_SP,
          op: BinaryOp::BitOr,
          left: Box::new(acc),
          right: Box::new(expr),
        })
      })
      .unwrap(),
  )
}

fn gen_condition_permutations(count: usize) -> Vec<Vec<bool>> {
  (0..2usize.pow(count as u32))
    .map(|i| (0..count).map(|j| i & (1 << j) != 0).collect())
    .collect()
}
