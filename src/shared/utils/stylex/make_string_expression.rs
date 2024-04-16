use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    BinExpr, BinaryOp, ComputedPropName, Expr, Ident, KeyValueProp, Lit, MemberExpr, MemberProp,
    Number, Prop, PropName, PropOrSpread, UnaryExpr, UnaryOp,
  },
};

use crate::shared::{
  enums::FnResult,
  utils::{common::string_to_expression, css::factories::object_expression_factory},
};

use super::{
  js_to_expr::convert_object_to_ast, parse_nallable_style::ResolvedArg,
  stylex::gen_condition_permutations,
};

fn fn_result_to_expression(fn_result: &FnResult) -> Option<Expr> {
  match fn_result {
    FnResult::Stylex(string_object) => Some(string_object.clone()),
    FnResult::Props(string_object) => Some(convert_object_to_ast(string_object)),
    FnResult::Attrs(string_object) => Some(convert_object_to_ast(string_object)),
  }
}

pub(crate) fn make_string_expression(
  values: &Vec<ResolvedArg>,
  transform: fn(&Vec<ResolvedArg>) -> Option<FnResult>,
) -> Option<Expr> {
  // if values.is_empty() {
  //     // Early return if there are no values
  //     return Option::None;
  // }

  let conditions = values
    .clone()
    .into_iter()
    .filter_map(|value| match value {
      ResolvedArg::ConditionalStyle(expr, _, _, _) => Option::Some(expr),
      _ => Option::None,
    })
    .collect::<Vec<Box<Expr>>>();

  if conditions.is_empty() {
    if let Some(value) = transform(values) {
      return fn_result_to_expression(&value);
    } else {
      return string_to_expression("".to_string());
    }
  }

  let condition_permutations = gen_condition_permutations(conditions.len());

  let obj_entries = condition_permutations
    .iter()
    .filter_map(|permutation| {
      let mut i = 0;

      dbg!(&values);

      let args = values
        .into_iter()
        .filter_map(|v| match v {
          ResolvedArg::StyleObject(_, _) => {
            dbg!(&v);
            Some(v.clone())
          }
          ResolvedArg::ConditionalStyle(_test, primary, fallback, ident) => {
            dbg!(&ident);
            let result = if permutation.get(i).unwrap_or(&false) == &true {
              primary
            } else {
              fallback
            };

            i += 1;

            dbg!(&result);

            let result = if let Some(result) = result {
              Some(ResolvedArg::StyleObject(result.clone(), ident.clone()))
            } else {
              Option::None
            };

            result
          }
        })
        .collect::<Vec<ResolvedArg>>();

      let key = permutation
        .iter()
        .fold(0, |so_far, &b| (so_far << 1) | if b { 1 } else { 0 });

      if let Some(result) = fn_result_to_expression(&transform(&args).unwrap()) {
        let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(key.to_string().clone().into(), DUMMY_SP)),
          value: Box::new(result),
        })));

        return Some(prop);
      }

      Option::None
    })
    .collect::<Vec<PropOrSpread>>();

  let obj_expressions = object_expression_factory(obj_entries).unwrap();
  let conditions_to_key = gen_bitwise_or_of_conditions(conditions);

  return Some(Expr::Member(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(obj_expressions),
    prop: MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: conditions_to_key,
    }),
  }));
}

fn gen_bitwise_or_of_conditions(conditions: Vec<Box<Expr>>) -> Box<Expr> {
  let binary_expressions = conditions
    .iter()
    .enumerate()
    .map(|(i, condition)| {
      let shift = conditions.len() - i - 1;
      let shift_expr = Box::new(Expr::Bin(BinExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
          span: DUMMY_SP,
          op: UnaryOp::Bang,
          arg: Box::new(Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::Bang,
            arg: condition.clone(),
          })),
        })),
        op: BinaryOp::LShift,
        right: Box::new(Expr::Lit(Lit::Num(Number {
          value: shift as f64,
          span: DUMMY_SP,
          raw: None,
        }))),
        span: DUMMY_SP,
      }));
      shift_expr
    })
    .collect::<Vec<Box<Expr>>>();

  binary_expressions
    .into_iter()
    .reduce(|acc, expr| {
      Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::BitOr,
        left: acc,
        right: expr,
      }))
    })
    .unwrap()
}
